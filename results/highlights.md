# Benchmark Highlights

5 Rust histogram crates, 2M samples, median of 20 iterations.
[Full results](report.md) | [Source](https://github.com/drmingdrmer/rust-histogram-benchmark)

| Histogram | Memory | Record (ns) | P99 Query (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|
| base2histogram | 2 KB | 1.9 | 14.2 | 🏆 0.000% | 🏆 0.018% | 0.130% |
| h2histogram | 2 KB | 🏆 1.5 | 153.4 | 9.070% | 0.424% | 🏆 0.109% |
| hdrhistogram | ~56 KB | 2.4 | 195.2 | 0.388% | 0.141% | 0.109% |
| ddsketch | 🏆 ~1.6 KB | 4.5 | 73.0 | 0.782% | 0.999% | 0.401% |
| tdigest | ~1.6 KB | 14.2 | 🏆 4.1 | 0.223% | 0.184% | 0.118% |
