## Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.0 | 2.0 | 1.8 |
| ddsketch | 4.6 | 4.4 | 4.3 |
| h2histogram | 2.0 | 1.4 | 1.5 |
| hdrhistogram-3 | 2.3 | 2.0 | 2.1 |
| hdrhistogram | 2.3 | 2.1 | 2.0 |
| quantogram | 46.0 | 76.9 | 70.1 |
| tdigest | 2.1 | 14.0 | 14.4 |

## Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 19.6 | 12.2 | 13.3 | 14.4 | 15.0 |
| ddsketch | 51.5 | 61.2 | 68.7 | 74.1 | 80.3 |
| h2histogram | 138.0 | 141.7 | 147.8 | 147.0 | 153.9 |
| hdrhistogram-3 | 142.0 | 263.9 | 318.6 | 444.0 | 661.7 |
| hdrhistogram | 126.1 | 161.3 | 169.6 | 188.8 | 227.6 |
| quantogram | 86.5 | 83.1 | 109.0 | 58.2 | 105.2 |
| tdigest | 44.7 | 9.8 | 7.3 | 4.1 | 3.0 |

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

