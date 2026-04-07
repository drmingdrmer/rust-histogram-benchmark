#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_full_bench_f64(
        "quantogram",
        quantogram::Quantogram::new,
        |h, v| h.add(v as f64),
        |h, q| h.quantile(q).unwrap_or(0.0),
    );
}
