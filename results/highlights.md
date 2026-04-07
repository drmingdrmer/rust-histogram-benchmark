# Benchmark Highlights

7 Rust histogram configurations, 2M samples, median of 20 iterations.
[Full results](report.md) | [Source](https://github.com/drmingdrmer/rust-histogram-benchmark)

| Histogram | Memory | Record (ns) | P99 Query (ns) | Merge (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|---:|
| base2histogram | 2.1 KB | 🥈 1.8 | 🥈 14.0 | 🏆 35.5 | 🏆 0.000% | 🥈 0.018% | 0.130% |
| h2histogram | 🥈 2.0 KB | 🏆 1.4 | 154.1 | 🥈 64.7 | 9.070% | 0.424% | 🥈 0.109% |
| hdrhistogram | 8.0 KB | 2.0 | 194.2 | 502.5 | 0.388% | 0.141% | 🥈 0.109% |
| hdrhistogram-3 | 32.0 KB | 2.1 | 454.2 | 1919.2 | 🏆 0.000% | 🏆 0.000% | 🏆 0.022% |
| quantogram | 31.6 KB | 71.0 | 61.5 | — | 0.310% | 0.424% | 0.739% |
| ddsketch | 🥈 2.0 KB | 4.5 | 72.5 | 177.3 | 0.782% | 0.999% | 0.401% |
| tdigest | 🏆 1.6 KB | 14.1 | 🏆 4.5 | 1214.3 | 0.223% | 0.184% | 0.118% |
