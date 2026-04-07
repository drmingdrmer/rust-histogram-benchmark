# Benchmark Highlights

7 Rust histogram configurations, 2M samples, median of 20 iterations.
[Full results](report.md) | [Source](https://github.com/drmingdrmer/rust-histogram-benchmark)

| Histogram | Memory | Record (ns) | P99 Query (ns) | Merge (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|---:|
| base2histogram | 2 KB | 1.8 | 13.0 | 🏆 37.6 | 🏆 0.000% | 0.018% | 0.130% |
| h2histogram | 2 KB | 🏆 1.5 | 150.0 | 67.5 | 9.070% | 0.424% | 🏆 0.109% |
| hdrhistogram | ~56 KB | 2.2 | 203.2 | 504.6 | 0.388% | 0.141% | 0.109% |
| hdrhistogram-3 | ~450 KB | 2.1 | 428.2 | 2060.8 | 🏆 0.000% | 🏆 0.000% | 0.022% |
| quantogram | dynamic | 72.1 | 63.2 | — | 0.310% | 0.424% | 0.739% |
| ddsketch | 🏆 ~1.6 KB | 4.3 | 74.5 | 153.4 | 0.782% | 0.999% | 0.401% |
| tdigest | ~1.6 KB | 14.0 | 🏆 4.5 | 1313.3 | 0.223% | 0.184% | 0.118% |
