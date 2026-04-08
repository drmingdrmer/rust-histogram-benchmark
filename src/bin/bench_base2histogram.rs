#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_u64_with_merge(
        "base2histogram",
        "width=3",
        |_| base2histogram::Histogram::<()>::with_log_scale(3, 1),
        |h, v| h.record(v),
        bench_harness::noop_finish,
        |h, q| h.percentile(q),
        |target, other| target.merge(other),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
