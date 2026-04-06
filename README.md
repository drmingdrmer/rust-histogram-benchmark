# rust-histogram-benchmark

Benchmark suite for Rust histogram implementations, measuring recording
throughput, percentile query latency, and accuracy across distributions.

## Implementations

| Crate | Version | Algorithm | Value Type | Repo |
|-------|---------|-----------|------------|------|
| [base2histogram] | 0.1 | Base-2 log-linear + trapezoidal interpolation | `u64` | [drmingdrmer/base2histogram](https://github.com/drmingdrmer/base2histogram) |
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

## Results

Raw JSON results per implementation: [`results/`](results/)
([full markdown report](results/report.md))

### Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 1.9 | 0.8 | 0.8 |
| ddsketch | 4.9 | 4.5 | 4.5 |
| h2histogram | 2.1 | 1.5 | 1.5 |
| hdrhistogram | 2.4 | 2.1 | 2.1 |
| tdigest | 2.1 | 14.6 | 14.3 |

### Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 36.1 | 35.6 | 45.3 | 47.7 | 39.5 |
| ddsketch | 56.6 | 64.8 | 64.2 | 69.7 | 80.2 |
| h2histogram | 137.3 | 143.3 | 142.0 | 146.3 | 160.4 |
| hdrhistogram | 125.8 | 165.0 | 179.9 | 193.7 | 204.0 |
| tdigest | 39.0 | 9.6 | 7.6 | 4.5 | 2.5 |

### Accuracy: Relative Error %

#### uniform

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.008% | 1.059% | 3.733% |
| ddsketch | 0.846% | 0.621% | 0.495% |
| h2histogram | 1.657% | 3.478% | 0.704% |
| hdrhistogram | 0.392% | 0.028% | 0.123% |
| tdigest | 0.003% | 0.110% | 0.033% |

#### log_normal_api

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.109% | 0.000% |
| ddsketch | 0.875% | 0.801% | 0.782% |
| h2histogram | 2.978% | 4.466% | 9.070% |
| hdrhistogram | 0.000% | 0.109% | 0.388% |
| tdigest | 0.266% | 0.034% | 0.133% |

#### bimodal

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.070% |
| ddsketch | 0.681% | 0.553% | 0.519% |
| h2histogram | 7.143% | 6.010% | 0.898% |
| hdrhistogram | 0.000% | 0.401% | 0.229% |
| tdigest | 0.251% | 0.559% | 0.423% |

#### exponential

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.152% |
| ddsketch | 0.794% | 0.520% | 0.530% |
| h2histogram | 1.737% | 6.041% | 0.022% |
| hdrhistogram | 0.000% | 0.367% | 0.673% |
| tdigest | 0.185% | 0.113% | 0.178% |

#### pareto

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.000% | 0.000% |
| ddsketch | 1.000% | 0.411% | 0.554% |
| h2histogram | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.000% | 0.000% |
| tdigest | 0.000% | 0.557% | 0.568% |

## Methodology

### Recording Throughput

Measures time per `record(value)` call. Each histogram is pre-created, then
values are recorded in a tight loop. 3 warmup iterations + 10 measured
iterations, median reported.

**Workloads:**
- Sequential: values `1..N`
- Random uniform: `u64` drawn from `[1, 10^6]`
- Log-normal: `μ=6, σ=0.5` (typical API latency shape)

Note: t-digest does not support per-value recording. Its benchmark uses
batched `merge_unsorted()` with batch size 1000 and reports amortized
ns per value.

### Percentile Query Latency

Measures time to compute a single percentile after recording 1M values
from the log-normal API distribution. 10K queries per measurement iteration.

### Accuracy

Records 1M samples from known distributions, compares histogram percentile
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
