//! Atomic histogram benchmark integration (histogram crate).
//!
//! Query path:
//! - AtomicHistogram has no direct percentile API.
//! - We load one snapshot Histogram lazily and answer percentile queries from it.
//! - Merge is reported as `N/A` because there is no native AtomicHistogram merge API.

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use histogram::AtomicHistogram;
use histogram::Histogram;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_u64(
        "atomichistogram",
        "grouping_power=4,max_value_power=64,query=load_once_snapshot",
        |_| AtomicState::new(4, 64),
        |state, value| state.record(value),
        bench_harness::noop_finish,
        |state, quantile| state.quantile_midpoint(quantile),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

struct AtomicState {
    histogram: AtomicHistogram,
    snapshot: Option<Histogram>,
}

impl AtomicState {
    fn new(grouping_power: u8, max_value_power: u8) -> Self {
        Self {
            histogram: AtomicHistogram::new(grouping_power, max_value_power).unwrap(),
            snapshot: None,
        }
    }

    fn record(&mut self, value: u64) {
        self.histogram.increment(value).unwrap();
    }

    fn quantile_midpoint(&mut self, quantile: f64) -> u64 {
        let snapshot = self.snapshot.get_or_insert_with(|| self.histogram.load());
        match snapshot.percentile(quantile) {
            Ok(Some(bucket)) => {
                let lo = *bucket.range().start();
                let hi = *bucket.range().end();
                (lo + hi) / 2
            }
            _ => 0,
        }
    }
}
