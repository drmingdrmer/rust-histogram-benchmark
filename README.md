# rust-histogram-benchmark

Benchmark suite for Rust histogram implementations, measuring recording
throughput, percentile query latency, and accuracy across distributions.

## Implementations

| Crate | Version | Algorithm | Value Type | Repo |
|-------|---------|-----------|------------|------|
| [base2histogram] | 0.2 | Base-2 log-linear + trapezoidal interpolation | `u64` | [drmingdrmer/base2histogram](https://github.com/drmingdrmer/base2histogram) |
| [hdrhistogram] | 7 | Linear sub-buckets within power-of-2 ranges | `u64` | [HdrHistogram/HdrHistogram_rust](https://github.com/HdrHistogram/HdrHistogram_rust) |
| [histogram] (H2) | 1 | Base-2 log-linear | `u64` | [iopsystems/histogram](https://github.com/iopsystems/histogram) |
| [sketches-ddsketch] | 0.4 | Logarithmic with relative accuracy guarantee | `f64` | [mheffner/rust-sketches-ddsketch](https://github.com/mheffner/rust-sketches-ddsketch) |
| [tdigest] | 0.2 | t-digest merging with centroid compression | `f64` | [MnO2/t-digest](https://github.com/MnO2/t-digest) |

## Feature Matrix

| Feature | base2histogram | hdrhistogram | H2 histogram | DDSketch | t-digest |
|---------|:-:|:-:|:-:|:-:|:-:|
| Native u64 recording | ✓ | ✓ | ✓ | | |
| Native f64 recording | | | | ✓ | ✓ |
| Negative values | | | | ✓ | ✓ |
| Percentile point estimate | ✓ | ✓ | | ✓ | ✓ |
| Percentile bucket range | | | ✓ | | |
| Interpolation | Trapezoidal | Linear | None | None | Centroid |
| Formal error guarantee | | | | ✓ (α) | |
| Configurable precision | Compile-time | Runtime | Runtime | Runtime | Runtime |
| Fixed memory | ✓ | ✓ | ✓ | | |
| Atomic / concurrent | | | ✓ | | |
| Sliding window | ✓ | | | | |
| Merge support | ✓ | ✓ | ✓ | ✓ | ✓ |
| Serde / serialization | | ✓ | ✓ | | |
| Sparse representation | | | ✓ | | |
| Value removal | | | ✓ | | |
| Inverse query (prank) | | ✓ | | | |

## Highlights

| Histogram | Memory | Record (ns) | P99 Query (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|
| base2histogram | 2 KB | 1.9 | 14.2 | 🏆 0.000% | 🏆 0.018% | 0.130% |
| h2histogram | 2 KB | 🏆 1.5 | 153.4 | 9.070% | 0.424% | 🏆 0.109% |
| hdrhistogram | ~56 KB | 2.4 | 195.2 | 0.388% | 0.141% | 0.109% |
| ddsketch | 🏆 ~1.6 KB | 4.5 | 73.0 | 0.782% | 0.999% | 0.401% |
| tdigest | ~1.6 KB | 14.2 | 🏆 4.1 | 0.223% | 0.184% | 0.118% |

## Results

Raw JSON results per implementation: [`results/`](results/)
([full markdown report](results/report.md) |
[highlights](results/highlights.md))

### Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.0 | 2.0 | 1.8 |
| ddsketch | 4.8 | 4.6 | 4.5 |
| h2histogram | 2.1 | 1.5 | 1.5 |
| hdrhistogram | 2.4 | 2.1 | 2.0 |
| tdigest | 2.1 | 14.0 | 14.2 |

### Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 19.7 | 11.7 | 12.4 | 14.6 | 15.1 |
| ddsketch | 52.5 | 63.2 | 68.2 | 75.2 | 79.7 |
| h2histogram | 130.9 | 138.0 | 148.7 | 152.5 | 157.7 |
| hdrhistogram | 125.7 | 167.9 | 180.7 | 201.7 | 227.8 |
| tdigest | 39.6 | 9.3 | 7.3 | 4.2 | 3.0 |

### Accuracy: Relative Error %

#### uniform

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.016% | 1.063% | 3.740% |
| ddsketch | 0.783% | 0.596% | 0.494% |
| h2histogram | 1.718% | 3.452% | 0.705% |
| hdrhistogram | 0.329% | 0.003% | 0.122% |
| tdigest | 0.109% | 0.020% | 0.020% |

#### log_normal_api

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.109% | 0.000% |
| ddsketch | 0.875% | 0.801% | 0.782% |
| h2histogram | 2.978% | 4.466% | 9.070% |
| hdrhistogram | 0.000% | 0.109% | 0.388% |
| tdigest | 0.027% | 0.039% | 0.223% |

#### bimodal

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.033% | 0.018% |
| ddsketch | 0.681% | 0.553% | 0.999% |
| h2histogram | 7.143% | 6.010% | 0.424% |
| hdrhistogram | 0.000% | 0.401% | 0.141% |
| tdigest | 0.091% | 0.982% | 0.184% |

#### exponential

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.130% |
| ddsketch | 0.918% | 0.520% | 0.401% |
| h2histogram | 1.443% | 6.041% | 0.109% |
| hdrhistogram | 0.289% | 0.367% | 0.109% |
| tdigest | 0.020% | 0.042% | 0.118% |

#### pareto

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.000% | 0.000% |
| ddsketch | 1.000% | 0.411% | 0.554% |
| h2histogram | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.000% | 0.000% |
| tdigest | 0.000% | 0.745% | 1.280% |

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
benchmark uses the midpoint `(lo + hi) / 2` for comparison. DDSketch
and t-digest accept `f64`; u64 values are cast via `as f64`.

## Configuration

| Crate | Config | Buckets | Memory |
|-------|--------|---------|--------|
| base2histogram | `WIDTH=3` (default) | 252 | ~2 KB |
| hdrhistogram | `sigfig=2` (auto-resize) | variable | variable |
| H2 histogram | `grouping_power=2` | 252 | ~2 KB |
| DDSketch | default config (`α≈0.01`) | dynamic | dynamic |
| t-digest | `max_size=100` | 100 centroids | ~1 KB |

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
