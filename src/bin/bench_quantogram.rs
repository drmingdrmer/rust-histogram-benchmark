#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use quantogram::QuantogramBuilder;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_f64(
        "quantogram",
        "bins_per_doubling=35,smallest_power=0,largest_power=observed_max_power",
        |observed_max| build_quantogram(observed_max, 35),
        |h, v| h.add(v as f64),
        bench_harness::noop_finish,
        |h, q| h.quantile(q).unwrap_or(0.0),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn build_quantogram(observed_max: u64, bins_per_doubling: usize) -> quantogram::Quantogram {
    let max_power = u64::BITS - observed_max.leading_zeros();
    QuantogramBuilder::new()
        .with_bins_per_doubling(bins_per_doubling)
        .with_smallest_power(0)
        .with_largest_power(max_power as isize)
        .build()
}
