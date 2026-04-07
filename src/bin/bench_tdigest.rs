use std::hint::black_box;
use std::time::Instant;

use rust_histogram_benchmark::bench_harness::accuracy_distributions;
use rust_histogram_benchmark::bench_harness::compute_accuracy;
use rust_histogram_benchmark::bench_harness::measure_merge_ns;
use rust_histogram_benchmark::bench_harness::measure_percentile_ns;
use rust_histogram_benchmark::distributions;
use rust_histogram_benchmark::output::BenchResult;
use rust_histogram_benchmark::output::PercentileLatency;
use rust_histogram_benchmark::output::RecordThroughput;
use tdigest::TDigest;

const N: usize = 2_000_000;
const BATCH_SIZE: usize = 1_000;
const MAX_SIZE: usize = 100;

fn build_tdigest(values: &[u64]) -> TDigest {
    let mut td = TDigest::new_with_size(MAX_SIZE);
    for chunk in values.chunks(BATCH_SIZE) {
        let batch: Vec<f64> = chunk.iter().map(|&v| v as f64).collect();
        td = td.merge_unsorted(batch);
    }
    td
}

fn measure_tdigest_record_ns(values: &[u64]) -> f64 {
    let warmup = 5;
    let measure = 20;
    let mut timings = Vec::with_capacity(warmup + measure);

    for _ in 0..(warmup + measure) {
        let start = Instant::now();
        let td = build_tdigest(values);
        let elapsed = start.elapsed();
        black_box(&td);
        timings.push(elapsed);
    }

    let mut nanos: Vec<f64> = timings[warmup..].iter().map(|d| d.as_nanos() as f64 / values.len() as f64).collect();
    nanos.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nanos[nanos.len() / 2]
}

fn main() {
    let name = "tdigest";

    eprintln!("[{name}] generating distributions...");
    let seq = distributions::sequential(N);
    let uni = distributions::uniform(N, 1_000_000);
    let lnorm = distributions::log_normal_api(N);

    // --- Record throughput (batched) ---
    eprintln!("[{name}] measuring record throughput...");
    let seq_ns = measure_tdigest_record_ns(&seq);
    let uni_ns = measure_tdigest_record_ns(&uni);
    let ln_ns = measure_tdigest_record_ns(&lnorm);

    // --- Percentile latency ---
    eprintln!("[{name}] measuring percentile latency...");
    let state = build_tdigest(&lnorm);

    let query = |td: &TDigest, q: f64| td.estimate_quantile(q);
    let p50_ns = measure_percentile_ns(&state, 0.50, query);
    let p90_ns = measure_percentile_ns(&state, 0.90, query);
    let p95_ns = measure_percentile_ns(&state, 0.95, query);
    let p99_ns = measure_percentile_ns(&state, 0.99, query);
    let p999_ns = measure_percentile_ns(&state, 0.999, query);

    // --- Merge ---
    eprintln!("[{name}] measuring merge...");
    let merge_ns = measure_merge_ns(&state, |target: &mut TDigest, other: &TDigest| {
        *target = TDigest::merge_digests(vec![target.clone(), other.clone()]);
    });

    // --- Accuracy ---
    eprintln!("[{name}] measuring accuracy...");
    let mut accuracy = Vec::new();
    for (dist_name, values) in accuracy_distributions() {
        let td = build_tdigest(&values);
        accuracy.push(compute_accuracy(dist_name, &values, &td, |td, q| {
            td.estimate_quantile(q)
        }));
    }

    let result = BenchResult {
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
        merge_ns: Some(merge_ns),
        accuracy,
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
