use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_full_bench(
        "base2histogram",
        base2histogram::Histogram::<()>::new,
        |h, v| h.record(v),
        |h, q| h.percentile(q),
    );
}
