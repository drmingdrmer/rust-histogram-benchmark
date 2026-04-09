//! Sparse histogram benchmark integration (histogram crate).
//!
//! Adapter semantics for this benchmark:
//! - `SparseHistogram` is a sparse representation and does not support online recording.
//! - Recording is done into a dense `histogram::Histogram`.
//! - `finish()` materializes one sparse snapshot and drops dense state.
//! - Query/merge/memory are measured on the sparse representation.

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use histogram::Histogram;
use histogram::SparseHistogram;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_u64_with_merge(
        "sparsehistogram",
        "grouping_power=4,max_value_power=64,dense_record+freeze_sparse",
        |_| SparseState::new(4, 64),
        |state, value| state.record(value),
        |state| state.finish(),
        |state, quantile| state.quantile_midpoint(quantile),
        |target, other| target.merge_from(other),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

#[derive(Clone)]
struct SparseState {
    dense: Option<Histogram>,
    sparse: Option<SparseHistogram>,
}

impl SparseState {
    fn new(grouping_power: u8, max_value_power: u8) -> Self {
        Self {
            dense: Some(Histogram::new(grouping_power, max_value_power).unwrap()),
            sparse: None,
        }
    }

    fn record(&mut self, value: u64) {
        self.dense
            .as_mut()
            .expect("record called after sparse snapshot finalized")
            .increment(value)
            .unwrap();
    }

    fn finish(&mut self) {
        if self.sparse.is_none() {
            let dense = self.dense.take().expect("missing dense histogram");
            self.sparse = Some(SparseHistogram::from(&dense));
        }
    }

    fn quantile_midpoint(&mut self, quantile: f64) -> u64 {
        self.finish();
        match self.sparse.as_ref().unwrap().percentile(quantile) {
            Ok(Some(bucket)) => {
                let lo = *bucket.range().start();
                let hi = *bucket.range().end();
                (lo + hi) / 2
            }
            _ => 0,
        }
    }

    fn merge_from(&mut self, other: &Self) {
        self.finish();
        let merged = self
            .sparse
            .as_ref()
            .unwrap()
            .wrapping_add(other.sparse.as_ref().expect("rhs sparse snapshot missing"))
            .unwrap();
        self.sparse = Some(merged);
        self.dense = None;
    }
}
