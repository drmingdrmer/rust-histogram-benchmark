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

pub fn accuracy_distributions() -> Vec<(&'static str, Vec<u64>)> {
    vec![
        ("uniform", distributions::uniform(N, 1_000_000)),
        ("log_normal_api", distributions::log_normal_api(N)),
        ("bimodal", distributions::bimodal(N)),
        ("exponential", distributions::exponential(N)),
        ("pareto", distributions::pareto_heavy(N)),
    ]
}

pub fn exact_percentile(sorted: &[u64], p: f64) -> f64 {
    let rank = p * (sorted.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - lo as f64;
    sorted[lo] as f64 * (1.0 - frac) + sorted[hi] as f64 * frac
}

pub fn relative_error_pct(exact: f64, estimated: f64) -> f64 {
    if exact == 0.0 {
        if estimated == 0.0 { 0.0 } else { 100.0 }
    } else {
        ((exact - estimated) / exact).abs() * 100.0
    }
}

pub fn measure_record_ns<S, F, G>(values: &[u64], setup: F, mut record_one: G) -> f64
where
    F: Fn() -> S,
    G: FnMut(&mut S, u64),
{
    let mut timings = Vec::with_capacity(WARMUP_ITERS + MEASURE_ITERS);

    for _ in 0..(WARMUP_ITERS + MEASURE_ITERS) {
        let mut state = setup();
        let start = Instant::now();
        for &v in values {
            record_one(&mut state, v);
        }
        let elapsed = start.elapsed();
        black_box(&state);
        timings.push(elapsed);
    }

    median_ns_per_op(&timings[WARMUP_ITERS..], values.len())
}

pub fn measure_percentile_ns<S, R, F>(state: &S, quantile: f64, query: F) -> f64
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
pub fn measure_merge_ns<S: Clone, MergeFn>(source: &S, merge: MergeFn) -> f64
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

/// Measure heap bytes used by creating and populating a histogram.
pub fn measure_memory_bytes<S>(
    setup: &impl Fn() -> S,
    record_one: &mut impl FnMut(&mut S, u64),
    values: &[u64],
) -> usize {
    // Warm up to avoid measuring one-time lazy allocations
    let _ = {
        let mut h = setup();
        for &v in values {
            record_one(&mut h, v);
        }
        h
    };

    let before = alloc_tracker::live_bytes();
    let mut h = setup();
    for &v in values {
        record_one(&mut h, v);
    }
    let after = alloc_tracker::live_bytes();
    black_box(&h);
    drop(h);
    let _ = after.saturating_sub(before);

    // Measure again more precisely: the delta while h is alive
    let before = alloc_tracker::live_bytes();
    let mut h = setup();
    for &v in values {
        record_one(&mut h, v);
    }
    let after = alloc_tracker::live_bytes();
    let memory = after.saturating_sub(before);
    black_box(&h);
    memory
}

pub fn compute_accuracy<S, F>(name: &str, values: &[u64], state: &S, query: F) -> AccuracyResult
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

/// Run the full benchmark suite for a u64-returning histogram (no merge).
pub fn run_full_bench<S, SetupFn, RecordFn, QueryFn>(
    name: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    query: QueryFn,
) where
    SetupFn: Fn() -> S,
    RecordFn: FnMut(&mut S, u64),
    QueryFn: Fn(&S, f64) -> u64,
{
    let query_f64 = |s: &S, q: f64| -> f64 { query(s, q) as f64 };
    let result = run_core(name, &setup, &mut record_one, &query_f64);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

/// Run the full benchmark suite for a u64-returning histogram with merge.
pub fn run_bench_with_merge<S: Clone, SetupFn, RecordFn, QueryFn, MergeFn>(
    name: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    query: QueryFn,
    merge_fn: MergeFn,
) where
    SetupFn: Fn() -> S,
    RecordFn: FnMut(&mut S, u64),
    QueryFn: Fn(&S, f64) -> u64,
    MergeFn: Fn(&mut S, &S),
{
    let query_f64 = |s: &S, q: f64| -> f64 { query(s, q) as f64 };
    let mut result = run_core(name, &setup, &mut record_one, &query_f64);

    eprintln!("[{name}] measuring merge...");
    let state = build_state(&setup, &mut record_one);
    result.merge_ns = Some(measure_merge_ns(&state, &merge_fn));

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

/// Run the full benchmark suite for an f64-returning histogram (no merge).
pub fn run_full_bench_f64<S, SetupFn, RecordFn, QueryFn>(
    name: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    query: QueryFn,
) where
    SetupFn: Fn() -> S,
    RecordFn: FnMut(&mut S, u64),
    QueryFn: Fn(&S, f64) -> f64,
{
    let result = run_core(name, &setup, &mut record_one, &query);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

/// Run the full benchmark suite for an f64-returning histogram with merge.
pub fn run_bench_f64_with_merge<S: Clone, SetupFn, RecordFn, QueryFn, MergeFn>(
    name: &str,
    setup: SetupFn,
    mut record_one: RecordFn,
    query: QueryFn,
    merge_fn: MergeFn,
) where
    SetupFn: Fn() -> S,
    RecordFn: FnMut(&mut S, u64),
    QueryFn: Fn(&S, f64) -> f64,
    MergeFn: Fn(&mut S, &S),
{
    let mut result = run_core(name, &setup, &mut record_one, &query);

    eprintln!("[{name}] measuring merge...");
    let state = build_state(&setup, &mut record_one);
    result.merge_ns = Some(measure_merge_ns(&state, &merge_fn));

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn build_state<S>(setup: &impl Fn() -> S, record_one: &mut impl FnMut(&mut S, u64)) -> S {
    let lnorm = distributions::log_normal_api(N);
    let mut state = setup();
    for &v in &lnorm {
        record_one(&mut state, v);
    }
    state
}

fn run_core<S, SetupFn, RecordFn, QueryFn>(
    name: &str,
    setup: &SetupFn,
    record_one: &mut RecordFn,
    query: &QueryFn,
) -> BenchResult
where
    SetupFn: Fn() -> S,
    RecordFn: FnMut(&mut S, u64),
    QueryFn: Fn(&S, f64) -> f64,
{
    eprintln!("[{name}] generating distributions...");
    let seq = distributions::sequential(N);
    let uni = distributions::uniform(N, 1_000_000);
    let lnorm = distributions::log_normal_api(N);

    // --- Record throughput ---
    eprintln!("[{name}] measuring record throughput...");
    let seq_ns = measure_record_ns(&seq, setup, &mut *record_one);
    let uni_ns = measure_record_ns(&uni, setup, &mut *record_one);
    let ln_ns = measure_record_ns(&lnorm, setup, &mut *record_one);

    // --- Memory ---
    eprintln!("[{name}] measuring memory...");
    let memory_bytes = measure_memory_bytes(setup, record_one, &lnorm);

    // --- Percentile latency ---
    eprintln!("[{name}] measuring percentile latency...");
    let mut state = setup();
    for &v in &lnorm {
        record_one(&mut state, v);
    }
    let p50_ns = measure_percentile_ns(&state, 0.50, query);
    let p90_ns = measure_percentile_ns(&state, 0.90, query);
    let p95_ns = measure_percentile_ns(&state, 0.95, query);
    let p99_ns = measure_percentile_ns(&state, 0.99, query);
    let p999_ns = measure_percentile_ns(&state, 0.999, query);

    // --- Accuracy ---
    eprintln!("[{name}] measuring accuracy...");
    let mut accuracy = Vec::new();
    for (dist_name, values) in accuracy_distributions() {
        let mut h = setup();
        for &v in &values {
            record_one(&mut h, v);
        }
        accuracy.push(compute_accuracy(dist_name, &values, &h, query));
    }

    BenchResult {
        name: name.to_string(),
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

fn median_ns_per_op(timings: &[Duration], ops: usize) -> f64 {
    let mut nanos: Vec<f64> = timings.iter().map(|d| d.as_nanos() as f64 / ops as f64).collect();
    nanos.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nanos[nanos.len() / 2]
}
