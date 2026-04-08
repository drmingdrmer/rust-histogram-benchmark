#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_u64_with_merge(
        "h2histogram",
        "grouping_power=4,max_value_power=64",
        |_| histogram::Histogram::new(4, 64).unwrap(),
        |h, v| {
            h.increment(v).unwrap();
        },
        bench_harness::noop_finish,
        |h, q| estimate_bucket_midpoint(h, q),
        |target, other| {
            *target = target.wrapping_add(other).unwrap();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn estimate_bucket_midpoint(hist: &histogram::Histogram, quantile: f64) -> u64 {
    match hist.percentile(quantile) {
        Ok(Some(bucket)) => {
            let lo = *bucket.range().start();
            let hi = *bucket.range().end();
            (lo + hi) / 2
        }
        _ => 0,
    }
}
