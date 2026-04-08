# Benchmark Highlights

6 Rust histogram implementations, 2M samples, median of 20 iterations.
[Full results](report.md) | [Source](https://github.com/drmingdrmer/rust-histogram-benchmark)

| Histogram | Memory | Record (ns) | P99 Query (ns) | Merge (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|---:|
| base2histogram | 🥈 2.1 KB | 🥈 1.8 | 🥈 14.2 | 🏆 33.2 | 🏆 0.000% | 🏆 0.018% | 0.130% |
| h2histogram | 7.6 KB | 🏆 1.5 | 475.1 | 203.0 | 1.628% | 1.839% | 2.673% |
| hdrhistogram | 15.0 KB | 2.2 | 199.0 | 897.0 | 0.388% | 🥈 0.141% | 🏆 0.109% |
| quantogram | 31.4 KB | 69.1 | 62.8 | — | 0.310% | 0.424% | 0.739% |
| ddsketch | 🏆 2.0 KB | 4.5 | 71.9 | 🥈 153.1 | 0.782% | 0.999% | 0.401% |
| tdigest | 9.4 KB | 15.6 | 🏆 4.4 | 1312.5 | 🥈 0.223% | 0.184% | 🥈 0.118% |
