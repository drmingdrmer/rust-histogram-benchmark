#[global_allocator]
static ALLOC: rust_histogram_benchmark::alloc_tracker::TrackingAllocator =
    rust_histogram_benchmark::alloc_tracker::TrackingAllocator;

use rust_histogram_benchmark::bench_harness;
use tdigest::TDigest;

fn main() {
    let result = bench_harness::bench_f64_with_merge(
        "tdigest",
        "max_size=100,batch_size=1000,local_sort+merge_sorted",
        |_| TDigestState::new(100, 1_000),
        |state, value| state.record(value),
        |state| state.finish(),
        |state, q| state.quantile(q),
        |target, other| target.merge_from(other),
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

#[derive(Clone)]
struct TDigestState {
    tdigest: TDigest,
    batch: Vec<f64>,
    batch_size: usize,
}

impl TDigestState {
    fn new(max_size: usize, batch_size: usize) -> Self {
        Self {
            tdigest: TDigest::new_with_size(max_size),
            batch: Vec::with_capacity(batch_size),
            batch_size,
        }
    }

    fn record(&mut self, value: u64) {
        self.batch.push(value as f64);
        if self.batch.len() == self.batch_size {
            self.flush();
        }
    }

    fn finish(&mut self) {
        if !self.batch.is_empty() {
            self.flush();
        }
    }

    fn quantile(&self, quantile: f64) -> f64 {
        self.tdigest.estimate_quantile(quantile)
    }

    fn merge_from(&mut self, other: &Self) {
        let max_size = self.tdigest.max_size();
        let current = std::mem::replace(&mut self.tdigest, TDigest::new_with_size(max_size));
        self.tdigest = TDigest::merge_digests(vec![current, other.tdigest.clone()]);
    }

    fn flush(&mut self) {
        self.batch.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let batch = std::mem::replace(&mut self.batch, Vec::with_capacity(self.batch_size));
        self.tdigest = self.tdigest.merge_sorted(batch);
    }
}
