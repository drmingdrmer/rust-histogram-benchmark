//! Greenwald-Khanna (GK) stream benchmark integration (quantiles crate).
//!
//! Notes:
//! - Uses one balanced configuration: `epsilon=0.01`.
//! - The `Stream` API does not expose a native merge path in this crate version, so this benchmark
//!   reports merge as `N/A`.

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use quantiles::greenwald_khanna::Stream;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_u64(
        "gkstream",
        "epsilon=0.01",
        |_| Stream::<u64>::new(0.01),
        |summary, value| {
            summary.insert(value);
        },
        bench_harness::noop_finish,
        |summary, quantile| *summary.quantile(quantile),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
