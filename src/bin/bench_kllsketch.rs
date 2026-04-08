//! KLL sketch benchmark integration (sketch_oxide).
//!
//! This benchmark uses a single balanced configuration (`k=200`) to keep
//! memory, throughput, and tail accuracy in the same regime as other
//! approximate quantile sketches in this repository.

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;
use sketch_oxide::Mergeable;
use sketch_oxide::quantiles::KllSketch;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "kllsketch",
        "k=200",
        |_| KllSketch::new(200).unwrap(),
        |sketch, value| {
            sketch.update(value as f64);
        },
        bench_harness::noop_finish,
        |sketch, quantile| sketch.quantile(quantile).unwrap_or(0.0),
        |target, other| {
            target.merge(other).unwrap();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
