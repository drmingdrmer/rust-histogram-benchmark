use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_bench_with_merge(
        "hdrhistogram",
        || hdrhistogram::Histogram::<u64>::new(2).unwrap(),
        |h, v| {
            h.record(v).ok();
        },
        |h, q| h.value_at_quantile(q),
        |target, other| {
            target.add(other).ok();
        },
    );
}
