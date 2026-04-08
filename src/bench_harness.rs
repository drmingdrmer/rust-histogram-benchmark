use std::hint::black_box;
use std::time::Duration;
use std::time::Instant;

use crate::alloc_tracker;
use crate::distributions;
use crate::output::AccuracyResult;
use crate::output::BenchResult;
use crate::output::PercentileLatency;
use crate::output::RecordThroughput;

const N: usize = 2_000_000;
const WARMUP_ITERS: usize = 5;
const MEASURE_ITERS: usize = 20;
const PERCENTILE_ITERS: usize = 20_000;
const MERGE_ITERS: usize = 1_000;

struct BenchInputs {
    seq: Vec<u64>,
    uni: Vec<u64>,
    lnorm: Vec<u64>,
    accuracy: Vec<(&'static str, Vec<u64>)>,
    max_value: u64,
}

fn accuracy_distributions() -> Vec<(&'static str, Vec<u64>)> {
    vec![
        ("uniform", distributions::uniform(N, 1_000_000)),
        ("log_normal_api", distributions::log_normal_api(N)),
        ("bimodal", distributions::bimodal(N)),
        ("exponential", distributions::exponential(N)),
        ("pareto", distributions::pareto_heavy(N)),
    ]
}

pub fn noop_finish<S>(_state: &mut S) {}

fn exact_percentile(sorted: &[u64], p: f64) -> f64 {
    let rank = p * (sorted.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - lo as f64;
    sorted[lo] as f64 * (1.0 - frac) + sorted[hi] as f64 * frac
}

fn relative_error_pct(exact: f64, estimated: f64) -> f64 {
    if exact == 0.0 {
        if estimated == 0.0 { 0.0 } else { 100.0 }
    } else {
        ((exact - estimated) / exact).abs() * 100.0
    }
}

fn measure_record_ns<S>(
    values: &[u64],
    setup: &impl Fn() -> S,
    record_one: &mut impl FnMut(&mut S, u64),
    finish: &mut impl FnMut(&mut S),
) -> f64 {
    let mut timings = Vec::with_capacity(WARMUP_ITERS + MEASURE_ITERS);

    for _ in 0..(WARMUP_ITERS + MEASURE_ITERS) {
        let mut state = setup();
        let start = Instant::now();
        for &v in values {
            record_one(&mut state, v);
        }
        finish(&mut state);
        let elapsed = start.elapsed();
        black_box(&state);
        timings.push(elapsed);
    }

    median_ns_per_op(&timings[WARMUP_ITERS..], values.len())
}

fn measure_percentile_ns<S, R, F>(state: &S, quantile: f64, query: F) -> f64
where
    R: 'static,
    F: Fn(&S, f64) -> R,
{
    let mut timings = Vec::with_capacity(WARMUP_ITERS + MEASURE_ITERS);

    for _ in 0..(WARMUP_ITERS + MEASURE_ITERS) {
        let start = Instant::now();
        for _ in 0..PERCENTILE_ITERS {
            black_box(query(state, quantile));
        }
        let elapsed = start.elapsed();
        timings.push(elapsed);
    }

    median_ns_per_op(&timings[WARMUP_ITERS..], PERCENTILE_ITERS)
}

/// Measure ns/op for merging `source` into a clone of itself.
fn measure_merge_ns<S: Clone, MergeFn>(source: &S, merge: MergeFn) -> f64
where MergeFn: Fn(&mut S, &S) {
    let mut timings = Vec::with_capacity(WARMUP_ITERS + MEASURE_ITERS);

    for _ in 0..(WARMUP_ITERS + MEASURE_ITERS) {
        let mut target = source.clone();
        let start = Instant::now();
        for _ in 0..MERGE_ITERS {
            merge(&mut target, source);
        }
        let elapsed = start.elapsed();
        black_box(&target);
        timings.push(elapsed);
    }

    median_ns_per_op(&timings[WARMUP_ITERS..], MERGE_ITERS)
}

/// Measure retained heap bytes while the populated histogram is alive.
fn measure_memory_bytes<S>(
    setup: &impl Fn() -> S,
    record_one: &mut impl FnMut(&mut S, u64),
    finish: &mut impl FnMut(&mut S),
    values: &[u64],
) -> usize {
    // Warm up to avoid measuring one-time lazy allocations
    {
        let mut h = setup();
        for &v in values {
            record_one(&mut h, v);
        }
        finish(&mut h);
        black_box(&h);
    }

    let live_before = alloc_tracker::live_bytes();

    let mut h = setup();
    for &v in values {
        record_one(&mut h, v);
    }
    finish(&mut h);

    let live_after = alloc_tracker::live_bytes();
    black_box(&h);

    live_after.saturating_sub(live_before)
}

fn compute_accuracy<S, F>(name: &str, values: &[u64], state: &S, query: F) -> AccuracyResult
where F: Fn(&S, f64) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    let percentiles = [0.50, 0.95, 0.99];
    let exact: Vec<f64> = percentiles.iter().map(|&p| exact_percentile(&sorted, p)).collect();
    let estimated: Vec<f64> = percentiles.iter().map(|&p| query(state, p)).collect();

    AccuracyResult {
        distribution: name.to_string(),
        p50_error_pct: relative_error_pct(exact[0], estimated[0]),
        p95_error_pct: relative_error_pct(exact[1], estimated[1]),
        p99_error_pct: relative_error_pct(exact[2], estimated[2]),
    }
}

pub fn bench_u64<S, SetupFn, RecordFn, FinishFn, QueryFn>(
    family: &str,
    config: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    mut finish: FinishFn,
    query: QueryFn,
) -> BenchResult
where
    SetupFn: Fn(u64) -> S,
    RecordFn: FnMut(&mut S, u64),
    FinishFn: FnMut(&mut S),
    QueryFn: Fn(&S, f64) -> u64,
{
    let query_f64 = |s: &S, q: f64| -> f64 { query(s, q) as f64 };
    let inputs = prepare_inputs();
    run_core(
        family,
        config,
        &inputs,
        &setup,
        &mut record_one,
        &mut finish,
        &query_f64,
    )
}

pub fn bench_u64_with_merge<S: Clone, SetupFn, RecordFn, FinishFn, QueryFn, MergeFn>(
    family: &str,
    config: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    mut finish: FinishFn,
    query: QueryFn,
    merge_fn: MergeFn,
) -> BenchResult
where
    SetupFn: Fn(u64) -> S,
    RecordFn: FnMut(&mut S, u64),
    FinishFn: FnMut(&mut S),
    QueryFn: Fn(&S, f64) -> u64,
    MergeFn: Fn(&mut S, &S),
{
    let query_f64 = |s: &S, q: f64| -> f64 { query(s, q) as f64 };
    let inputs = prepare_inputs();
    let mut result = run_core(
        family,
        config,
        &inputs,
        &setup,
        &mut record_one,
        &mut finish,
        &query_f64,
    );

    eprintln!("[{}] measuring merge...", result.name);
    let make_state = || setup(inputs.max_value);
    let state = build_state(&make_state, &mut record_one, &mut finish, &inputs.lnorm);
    result.merge_ns = Some(measure_merge_ns(&state, &merge_fn));
    result
}

pub fn bench_f64<S, SetupFn, RecordFn, FinishFn, QueryFn>(
    family: &str,
    config: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    mut finish: FinishFn,
    query: QueryFn,
) -> BenchResult
where
    SetupFn: Fn(u64) -> S,
    RecordFn: FnMut(&mut S, u64),
    FinishFn: FnMut(&mut S),
    QueryFn: Fn(&S, f64) -> f64,
{
    let inputs = prepare_inputs();
    run_core(family, config, &inputs, &setup, &mut record_one, &mut finish, &query)
}

pub fn bench_f64_with_merge<S: Clone, SetupFn, RecordFn, FinishFn, QueryFn, MergeFn>(
    family: &str,
    config: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    mut finish: FinishFn,
    query: QueryFn,
    merge_fn: MergeFn,
) -> BenchResult
where
    SetupFn: Fn(u64) -> S,
    RecordFn: FnMut(&mut S, u64),
    FinishFn: FnMut(&mut S),
    QueryFn: Fn(&S, f64) -> f64,
    MergeFn: Fn(&mut S, &S),
{
    let inputs = prepare_inputs();
    let mut result = run_core(family, config, &inputs, &setup, &mut record_one, &mut finish, &query);

    eprintln!("[{}] measuring merge...", result.name);
    let make_state = || setup(inputs.max_value);
    let state = build_state(&make_state, &mut record_one, &mut finish, &inputs.lnorm);
    result.merge_ns = Some(measure_merge_ns(&state, &merge_fn));
    result
}

fn build_state<S>(
    setup: &impl Fn() -> S,
    record_one: &mut impl FnMut(&mut S, u64),
    finish: &mut impl FnMut(&mut S),
    values: &[u64],
) -> S {
    let mut state = setup();
    for &v in values {
        record_one(&mut state, v);
    }
    finish(&mut state);
    state
}

fn run_core<S, SetupFn, RecordFn, FinishFn, QueryFn>(
    family: &str,
    config: &str,
    inputs: &BenchInputs,
    setup: &SetupFn,
    record_one: &mut RecordFn,
    finish: &mut FinishFn,
    query: &QueryFn,
) -> BenchResult
where
    SetupFn: Fn(u64) -> S,
    RecordFn: FnMut(&mut S, u64),
    FinishFn: FnMut(&mut S),
    QueryFn: Fn(&S, f64) -> f64,
{
    let name = family.to_string();
    let make_state = || setup(inputs.max_value);

    // --- Record throughput ---
    eprintln!("[{name}] measuring record throughput...");
    let seq_ns = measure_record_ns(&inputs.seq, &make_state, &mut *record_one, &mut *finish);
    let uni_ns = measure_record_ns(&inputs.uni, &make_state, &mut *record_one, &mut *finish);
    let ln_ns = measure_record_ns(&inputs.lnorm, &make_state, &mut *record_one, &mut *finish);

    // --- Memory ---
    eprintln!("[{name}] measuring memory...");
    let memory_bytes = measure_memory_bytes(&make_state, record_one, finish, &inputs.lnorm);

    // --- Percentile latency ---
    eprintln!("[{name}] measuring percentile latency...");
    let state = build_state(&make_state, record_one, finish, &inputs.lnorm);
    let p50_ns = measure_percentile_ns(&state, 0.50, query);
    let p90_ns = measure_percentile_ns(&state, 0.90, query);
    let p95_ns = measure_percentile_ns(&state, 0.95, query);
    let p99_ns = measure_percentile_ns(&state, 0.99, query);
    let p999_ns = measure_percentile_ns(&state, 0.999, query);

    // --- Accuracy ---
    eprintln!("[{name}] measuring accuracy...");
    let mut accuracy = Vec::new();
    for (dist_name, values) in &inputs.accuracy {
        let mut h = make_state();
        for &v in values {
            record_one(&mut h, v);
        }
        finish(&mut h);
        accuracy.push(compute_accuracy(dist_name, values, &h, query));
    }

    BenchResult {
        name,
        family: family.to_string(),
        config: config.to_string(),
        record_throughput: RecordThroughput {
            sequential_ns: seq_ns,
            uniform_ns: uni_ns,
            log_normal_ns: ln_ns,
        },
        percentile_latency: PercentileLatency {
            p50_ns,
            p90_ns,
            p95_ns,
            p99_ns,
            p999_ns,
        },
        memory_bytes,
        merge_ns: None,
        accuracy,
    }
}

fn prepare_inputs() -> BenchInputs {
    eprintln!("[bench] generating distributions...");
    let seq = distributions::sequential(N);
    let uni = distributions::uniform(N, 1_000_000);
    let lnorm = distributions::log_normal_api(N);
    let accuracy = accuracy_distributions();

    let max_value = seq
        .iter()
        .chain(&uni)
        .chain(&lnorm)
        .copied()
        .chain(accuracy.iter().flat_map(|(_, values)| values.iter().copied()))
        .max()
        .unwrap_or(2);

    BenchInputs {
        seq,
        uni,
        lnorm,
        accuracy,
        max_value: max_value.max(2),
    }
}

fn median_ns_per_op(timings: &[Duration], ops: usize) -> f64 {
    let mut nanos: Vec<f64> = timings.iter().map(|d| d.as_nanos() as f64 / ops as f64).collect();
    nanos.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nanos[nanos.len() / 2]
}
