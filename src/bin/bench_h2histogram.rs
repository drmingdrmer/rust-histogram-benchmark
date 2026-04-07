use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_bench_with_merge(
        "h2histogram",
        || histogram::Histogram::new(2, 64).unwrap(),
        |h, v| {
            let _ = h.increment(v);
        },
        |h, q| match h.percentile(q) {
            Ok(Some(bucket)) => {
                let lo = *bucket.range().start();
                let hi = *bucket.range().end();
                (lo + hi) / 2
            }
            _ => 0,
        },
        |target, other| {
            if let Ok(merged) = target.wrapping_add(other) {
                *target = merged;
            }
        },
    );
}
