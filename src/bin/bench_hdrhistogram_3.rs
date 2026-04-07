#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_bench_with_merge(
        "hdrhistogram-3",
        || hdrhistogram::Histogram::<u64>::new(3).unwrap(),
        |h, v| {
            h.record(v).ok();
        },
        |h, q| h.value_at_quantile(q),
        |target, other| {
            target.add(other).ok();
        },
    );
}
