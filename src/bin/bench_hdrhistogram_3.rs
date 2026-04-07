use rust_histogram_benchmark::bench_harness;

fn main() {
    bench_harness::run_full_bench(
        "hdrhistogram-3",
        || hdrhistogram::Histogram::<u64>::new(3).unwrap(),
        |h, v| {
            h.record(v).ok();
        },
        |h, q| h.value_at_quantile(q),
    );
}
