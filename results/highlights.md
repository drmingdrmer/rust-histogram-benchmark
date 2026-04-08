# Benchmark Highlights

8 Rust histogram implementations, 2M samples, median of 20 iterations.
[Full results](report.md) | [Source](https://github.com/drmingdrmer/rust-histogram-benchmark)

| Histogram | Memory | Record (ns) | P99 Query (ns) | Merge (ns) | P99 Error: log-normal | bimodal | exponential |
|---|---:|---:|---:|---:|---:|---:|---:|
| base2histogram | 🥈 2.1 KB | 🥈 1.8 | 13.5 | 🏆 34.8 | 🏆 0.000% | 🏆 0.018% | 0.130% |
| ddsketch | 🏆 2.0 KB | 4.5 | 82.0 | 🥈 146.8 | 0.782% | 0.999% | 0.401% |
| h2histogram | 7.6 KB | 🏆 1.4 | 476.9 | 215.7 | 1.628% | 1.839% | 2.673% |
| hdrhistogram | 15.0 KB | 2.2 | 175.8 | 795.2 | 0.388% | 🥈 0.141% | 🏆 0.109% |
| kllsketch | 398.6 KB | 25.0 | 201792.7 | 21467.0 | 7.597% | 11.459% | 8.627% |
| quantogram | 31.5 KB | 71.5 | 76.6 | — | 0.310% | 0.424% | 0.739% |
| reqsketch | 61.9 KB | 33.2 | 🥈 9.9 | 44738.8 | 🥈 0.078% | 0.159% | 0.543% |
| tdigest | 9.4 KB | 15.9 | 🏆 4.5 | 1203.3 | 0.223% | 0.184% | 🥈 0.118% |
