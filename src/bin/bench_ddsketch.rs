use rust_histogram_benchmark::bench_harness;
use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    bench_harness::run_full_bench_f64(
        "ddsketch",
        || DDSketch::new(Config::defaults()),
        |h, v| h.add(v as f64),
        |h, q| h.quantile(q).unwrap().unwrap_or(0.0),
    );
}
