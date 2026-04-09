//! metrics-util `Summary` benchmark integration.
//!
//! `Summary` is a DDSketch-backed quantile summary implementation from the
//! metrics ecosystem.
//!
//! Balanced config in this suite:
//! - alpha = 0.01
//! - max_buckets = 2048
//! - min_value = 1.0

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use metrics_util::storage::Summary;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "metricssummary",
        "alpha=0.01,max_buckets=2048,min_value=1.0",
        |_| Summary::new(0.01, 2_048, 1.0),
        |summary, value| {
            summary.add(value as f64);
        },
        bench_harness::noop_finish,
        |summary, quantile| summary.quantile(quantile).unwrap_or(0.0),
        |target, other| {
            target.merge(other).unwrap();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
