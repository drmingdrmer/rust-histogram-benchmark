#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;
use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "ddsketch",
        "alpha=0.01,max_num_bins=2048,min_value=1.0",
        |_| DDSketch::new(Config::new(0.01, 2048, 1.0)),
        |h, v| h.add(v as f64),
        bench_harness::noop_finish,
        |h, q| h.quantile(q).unwrap().unwrap_or(0.0),
        |target, other| {
            target.merge(other).unwrap();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
