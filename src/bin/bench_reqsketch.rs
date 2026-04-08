//! REQ sketch benchmark integration.
//!
//! `reqsketch` quantile/rank APIs require `&mut self`.
//! The harness query callback now accepts `FnMut(&mut S, ...)`, so we can
//! benchmark `ReqSketch<f64>` directly without a dedicated adapter state.

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use reqsketch::ReqSketch;
use reqsketch::SearchCriteria;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "reqsketch",
        "k=12,rank_accuracy=high",
        |_| ReqSketch::with_k(12).unwrap(),
        |sketch, value| sketch.update(value as f64),
        bench_harness::noop_finish,
        |sketch, quantile| sketch.quantile(quantile, SearchCriteria::Inclusive).unwrap_or(0.0),
        |target, other| {
            target.merge(other).unwrap();
        },
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
