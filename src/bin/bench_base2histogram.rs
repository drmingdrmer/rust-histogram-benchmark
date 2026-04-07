use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_bench_with_merge(
        "base2histogram",
        base2histogram::Histogram::<()>::new,
        |h, v| h.record(v),
        |h, q| h.percentile(q),
        |target, other| target.merge(other),
    );
}
