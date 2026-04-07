## Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.0 | 2.0 | 1.8 |
| ddsketch | 4.7 | 4.5 | 4.5 |
| h2histogram | 2.0 | 1.5 | 1.4 |
| hdrhistogram-3 | 2.3 | 2.1 | 2.1 |
| hdrhistogram | 2.3 | 2.1 | 2.0 |
| quantogram | 52.4 | 79.0 | 71.0 |
| tdigest | 2.1 | 13.9 | 14.1 |

## Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 19.6 | 11.0 | 12.2 | 14.0 | 14.8 |
| ddsketch | 52.4 | 62.8 | 66.5 | 72.5 | 77.6 |
| h2histogram | 137.2 | 144.4 | 150.9 | 154.1 | 163.4 |
| hdrhistogram-3 | 152.5 | 272.9 | 308.6 | 454.2 | 647.6 |
| hdrhistogram | 125.1 | 166.4 | 167.3 | 194.2 | 226.1 |
| quantogram | 118.3 | 108.4 | 138.4 | 61.5 | 111.8 |
| tdigest | 37.8 | 9.2 | 7.5 | 4.5 | 3.2 |

## Memory (after recording 2M log-normal values)

| Histogram | heap bytes |
|---|---:|
| base2histogram | 2.1 KB |
| ddsketch | 2.0 KB |
| h2histogram | 2.0 KB |
| hdrhistogram-3 | 32.0 KB |
| hdrhistogram | 8.0 KB |
| quantogram | 31.6 KB |
| tdigest | 1.6 KB |

## Merge Latency (ns/op)

| Histogram | merge |
|---|---:|
| base2histogram | 35.5 |
| ddsketch | 177.3 |
| h2histogram | 64.7 |
| hdrhistogram-3 | 1919.2 |
| hdrhistogram | 502.5 |
| quantogram | — |
| tdigest | 1214.3 |

## Accuracy: Relative Error %

### uniform

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.016% | 1.063% | 3.740% |
| ddsketch | 0.783% | 0.596% | 0.494% |
| h2histogram | 1.718% | 3.452% | 0.705% |
| hdrhistogram-3 | 0.022% | 0.003% | 0.019% |
| hdrhistogram | 0.329% | 0.003% | 0.122% |
| quantogram | 0.230% | 0.940% | 0.798% |
| tdigest | 0.109% | 0.020% | 0.020% |

### log_normal_api

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.109% | 0.000% |
| ddsketch | 0.875% | 0.801% | 0.782% |
| h2histogram | 2.978% | 4.466% | 9.070% |
| hdrhistogram-3 | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.109% | 0.388% |
| quantogram | 0.744% | 0.000% | 0.310% |
| tdigest | 0.027% | 0.039% | 0.223% |

### bimodal

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.033% | 0.018% |
| ddsketch | 0.681% | 0.553% | 0.999% |
| h2histogram | 7.143% | 6.010% | 0.424% |
| hdrhistogram-3 | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.401% | 0.141% |
| quantogram | 0.649% | 0.601% | 0.424% |
| tdigest | 0.091% | 0.982% | 0.184% |

### exponential

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.130% |
| ddsketch | 0.918% | 0.520% | 0.401% |
| h2histogram | 1.443% | 6.041% | 0.109% |
| hdrhistogram-3 | 0.000% | 0.033% | 0.022% |
| hdrhistogram | 0.289% | 0.367% | 0.109% |
| quantogram | 0.433% | 0.567% | 0.739% |
| tdigest | 0.020% | 0.042% | 0.118% |

### pareto

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.000% | 0.000% |
| ddsketch | 1.000% | 0.411% | 0.554% |
| h2histogram | 0.000% | 0.000% | 0.000% |
| hdrhistogram-3 | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.000% | 0.000% |
| quantogram | 0.000% | 0.000% | 0.000% |
| tdigest | 0.000% | 0.745% | 1.280% |

