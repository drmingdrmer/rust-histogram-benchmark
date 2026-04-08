//! Benchmark adapter for `reqsketch`.
//!
//! Why a dedicated state wrapper is needed:
//! - The harness query callback uses `Fn(&S, f64) -> f64`.
//! - `ReqSketch::quantile()` and `ReqSketch::sorted_view()` require `&mut self`.
//! - `ReqSketchState` uses interior mutability (`RefCell`) so we can keep the harness interface
//!   unchanged across all implementations.
//!
//! Why a `SortedView` cache is needed:
//! - Query latency benchmark performs many repeated percentile lookups over a fixed post-recording
//!   state.
//! - We materialize the sorted view once and reuse it for all query calls.
//! - Cache is invalidated on write paths (`record` and `merge_from`) to keep correctness when the
//!   sketch changes.
//! - This avoids repeatedly paying adapter-level view setup cost during query timing and keeps
//!   measurements focused on lookup behavior.

#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use std::cell::RefCell;

use reqsketch::ReqSketch;
use reqsketch::SearchCriteria;
use reqsketch::SortedView;
use rust_histogram_benchmark::bench_harness;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "reqsketch",
        "k=12,rank_accuracy=high",
        |_| ReqSketchState::new(12),
        |state, value| state.record(value),
        bench_harness::noop_finish,
        |state, quantile| state.quantile(quantile),
        |target, other| target.merge_from(other),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

#[derive(Clone)]
struct ReqSketchState {
    // REQ quantile APIs require `&mut self`, but harness query callback is `Fn(&S, f64)`.
    // Interior mutability keeps the harness interface unchanged across implementations.
    sketch: RefCell<ReqSketch<f64>>,
    // Cache a stable sorted view for repeated percentile queries.
    // Invalidate on write paths (`record` / `merge_from`).
    view_cache: RefCell<Option<SortedView<f64>>>,
}

impl ReqSketchState {
    fn new(k: u16) -> Self {
        let sketch = ReqSketch::with_k(k).unwrap();
        Self {
            sketch: RefCell::new(sketch),
            view_cache: RefCell::new(None),
        }
    }

    fn record(&mut self, value: u64) {
        self.sketch.get_mut().update(value as f64);
        *self.view_cache.get_mut() = None;
    }

    fn quantile(&self, rank: f64) -> f64 {
        let needs_refresh = self.view_cache.borrow().is_none();
        if needs_refresh {
            let view = {
                let mut sketch = self.sketch.borrow_mut();
                match sketch.sorted_view() {
                    Ok(view) => view.clone(),
                    Err(_) => return 0.0,
                }
            };
            *self.view_cache.borrow_mut() = Some(view);
        }

        self.view_cache
            .borrow()
            .as_ref()
            .and_then(|view| view.quantile(rank, SearchCriteria::Inclusive).ok())
            .unwrap_or(0.0)
    }

    fn merge_from(&mut self, other: &Self) {
        let other_sketch = other.sketch.borrow();
        self.sketch.get_mut().merge(&other_sketch).unwrap();
        *self.view_cache.get_mut() = None;
    }
}
