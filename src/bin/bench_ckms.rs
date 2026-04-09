//! CKMS benchmark integration (quantiles crate).
//!
//! CKMS is a classical streaming quantile summary with rank-error guarantees.
//! This benchmark uses one balanced configuration (`epsilon=0.1`).

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use quantiles::ckms::CKMS;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "ckms",
        "epsilon=0.1",
        |_| CKMS::<f64>::new(0.1),
        |summary, value| {
            summary.insert(value as f64);
        },
        bench_harness::noop_finish,
        |summary, quantile| summary.query(quantile).map(|(_, v)| v).unwrap_or(0.0),
        |target, other| {
            *target += other.clone();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
