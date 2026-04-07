# Benchmark Highlights

7 Rust histogram configurations, 2M samples, median of 20 iterations.
[Full results](report.md) | [Source](https://github.com/drmingdrmer/rust-histogram-benchmark)

| Histogram | Memory | Record (ns) | P99 Query (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|
| base2histogram | 2 KB | 1.8 | 14.4 | 🏆 0.000% | 0.018% | 0.130% |
| h2histogram | 2 KB | 🏆 1.5 | 147.0 | 9.070% | 0.424% | 🏆 0.109% |
| hdrhistogram | ~56 KB | 2.0 | 188.8 | 0.388% | 0.141% | 0.109% |
| hdrhistogram-3 | ~450 KB | 2.1 | 444.0 | 🏆 0.000% | 🏆 0.000% | 0.022% |
| quantogram | dynamic | 70.1 | 58.2 | 0.310% | 0.424% | 0.739% |
| ddsketch | 🏆 ~1.6 KB | 4.3 | 74.1 | 0.782% | 0.999% | 0.401% |
| tdigest | ~1.6 KB | 14.4 | 🏆 4.1 | 0.223% | 0.184% | 0.118% |
