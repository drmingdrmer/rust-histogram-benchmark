#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_u64_with_merge(
        "hdrhistogram",
        "fixed_bounds,max=observed_max,sigfig=2",
        |observed_max| hdrhistogram::Histogram::<u64>::new_with_max(observed_max, 2).unwrap(),
        |h, v| {
            h.record(v).unwrap();
        },
        bench_harness::noop_finish,
        |h, q| h.value_at_quantile(q),
        |target, other| {
            target.add(other).unwrap();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
