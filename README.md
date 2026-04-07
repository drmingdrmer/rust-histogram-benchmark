# rust-histogram-benchmark

Benchmark suite for Rust histogram implementations, measuring recording
throughput, percentile query latency, and accuracy across distributions.

## Implementations

| Crate | Version | Algorithm | Value Type | Repo |
|-------|---------|-----------|------------|------|
| [base2histogram] | 0.2 | Base-2 log-linear + trapezoidal interpolation | `u64` | [drmingdrmer/base2histogram](https://github.com/drmingdrmer/base2histogram) |
| [hdrhistogram] | 7 | Linear sub-buckets within power-of-2 ranges | `u64` | [HdrHistogram/HdrHistogram_rust](https://github.com/HdrHistogram/HdrHistogram_rust) |
| [histogram] (H2) | 1 | Base-2 log-linear | `u64` | [iopsystems/histogram](https://github.com/iopsystems/histogram) |
| [quantogram] | 0.4 | Log-bin histogram with absolute error guarantee | `f64` | [paulchernoch/quantogram](https://github.com/paulchernoch/quantogram) |
| [sketches-ddsketch] | 0.4 | Logarithmic with relative accuracy guarantee | `f64` | [mheffner/rust-sketches-ddsketch](https://github.com/mheffner/rust-sketches-ddsketch) |
| [tdigest] | 0.2 | t-digest merging with centroid compression | `f64` | [MnO2/t-digest](https://github.com/MnO2/t-digest) |

hdrhistogram is benchmarked at two precision levels: `sigfig=2` and `sigfig=3`.

## Feature Matrix

| Feature | base2histogram | hdrhistogram | H2 histogram | quantogram | DDSketch | t-digest |
|---------|:-:|:-:|:-:|:-:|:-:|:-:|
| Native u64 recording | ✓ | ✓ | ✓ | | | |
| Native f64 recording | | | | ✓ | ✓ | ✓ |
| Negative values | | | | ✓ | ✓ | ✓ |
| Percentile point estimate | ✓ | ✓ | | ✓ | ✓ | ✓ |
| Percentile bucket range | | | ✓ | | | |
| Interpolation | Trapezoidal | Linear | None | None | None | Centroid |
| Formal error guarantee | | | | ✓ (abs) | ✓ (α) | |
| Configurable precision | Compile-time | Runtime | Runtime | Runtime | Runtime | Runtime |
| Fixed memory | ✓ | ✓ | ✓ | | | |
| Atomic / concurrent | | | ✓ | | | |
| Sliding window | ✓ | | | | | |
| Merge support | ✓ | ✓ | ✓ | | ✓ | ✓ |
| Serde / serialization | | ✓ | ✓ | | | |
| Sparse representation | | | ✓ | | | |
| Value removal | | | ✓ | | | |
| Inverse query (prank) | | ✓ | | | | |

## Highlights

| Histogram | Memory | Record (ns) | P99 Query (ns) | Merge (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|---:|
| base2histogram | 2 KB | 1.8 | 13.0 | 🏆 37.6 | 🏆 0.000% | 0.018% | 0.130% |
| h2histogram | 2 KB | 🏆 1.5 | 150.0 | 67.5 | 9.070% | 0.424% | 🏆 0.109% |
| hdrhistogram | ~56 KB | 2.2 | 203.2 | 504.6 | 0.388% | 0.141% | 0.109% |
| hdrhistogram-3 | ~450 KB | 2.1 | 428.2 | 2060.8 | 🏆 0.000% | 🏆 0.000% | 0.022% |
| quantogram | dynamic | 72.1 | 63.2 | — | 0.310% | 0.424% | 0.739% |
| ddsketch | 🏆 ~1.6 KB | 4.3 | 74.5 | 153.4 | 0.782% | 0.999% | 0.401% |
| tdigest | ~1.6 KB | 14.0 | 🏆 4.5 | 1313.3 | 0.223% | 0.184% | 0.118% |

## Detailed Results

- [Full benchmark report](results/report.md)
- [Highlights](results/highlights.md)
- [Raw JSON per implementation](results/)

## Methodology

### Recording Throughput

Measures time per `record(value)` call. Each histogram is pre-created, then
values are recorded in a tight loop. 2M values per workload, 5 warmup
iterations + 20 measured iterations, median reported.

**Workloads:**
- Sequential: values `1..N`
- Random uniform: `u64` drawn from `[1, 10^6]`
- Log-normal: `μ=6, σ=0.5` (typical API latency shape)

Note: t-digest does not support per-value recording. Its benchmark uses
batched `merge_unsorted()` with batch size 1000 and reports amortized
ns per value.

### Percentile Query Latency

Measures time to compute a single percentile after recording 2M values
from the log-normal API distribution. 20K queries per measurement iteration.

### Accuracy

Records 2M samples from known distributions, compares histogram percentile
estimates against exact values computed from the sorted sample.
Relative error = `|exact - estimated| / exact × 100%`.

Note: H2 histogram returns a bucket range, not a point estimate. The
benchmark uses the midpoint `(lo + hi) / 2` for comparison. DDSketch,
quantogram, and t-digest accept `f64`; u64 values are cast via `as f64`.

## Configuration

| Crate | Config | Buckets | Memory |
|-------|--------|---------|--------|
| base2histogram | `WIDTH=3` (default) | 252 | ~2 KB |
| hdrhistogram | `sigfig=2` (auto-resize) | variable | ~56 KB |
| hdrhistogram-3 | `sigfig=3` (auto-resize) | variable | ~450 KB |
| H2 histogram | `grouping_power=2` | 252 | ~2 KB |
| quantogram | default (1% error) | dynamic | dynamic |
| DDSketch | default config (`α≈0.01`) | dynamic | ~1.6 KB |
| t-digest | `max_size=100` | 100 centroids | ~1.6 KB |

## Usage

```bash
# Run all benchmarks and generate report
./run.sh

# Run a single histogram benchmark
cargo run --release --bin bench-base2histogram

# Generate report from saved JSON results
cargo run --release --bin report -- results/*.json

# Generate markdown report
cargo run --release --bin report -- --markdown results/*.json
```

## License

MIT
