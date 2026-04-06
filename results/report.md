## Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.0 | 0.9 | 0.7 |
| ddsketch | 4.9 | 4.6 | 4.5 |
| h2histogram | 2.0 | 1.5 | 1.5 |
| hdrhistogram | 2.4 | 2.3 | 2.0 |
| tdigest | 2.2 | 14.1 | 14.1 |

## Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 39.5 | 38.4 | 39.1 | 40.2 | 42.4 |
| ddsketch | 64.1 | 67.3 | 71.8 | 69.5 | 81.9 |
| h2histogram | 135.4 | 142.9 | 148.7 | 152.5 | 153.3 |
| hdrhistogram | 126.4 | 168.1 | 182.6 | 199.6 | 222.8 |
| tdigest | 40.1 | 9.7 | 7.8 | 5.0 | 2.5 |

## Accuracy: Relative Error %

### uniform

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.008% | 1.059% | 3.733% |
| ddsketch | 0.846% | 0.621% | 0.495% |
| h2histogram | 1.657% | 3.478% | 0.704% |
| hdrhistogram | 0.392% | 0.028% | 0.123% |
| tdigest | 0.003% | 0.110% | 0.033% |

### log_normal_api

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.109% | 0.000% |
| ddsketch | 0.875% | 0.801% | 0.782% |
| h2histogram | 2.978% | 4.466% | 9.070% |
| hdrhistogram | 0.000% | 0.109% | 0.388% |
| tdigest | 0.266% | 0.034% | 0.133% |

### bimodal

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.070% |
| ddsketch | 0.681% | 0.553% | 0.519% |
| h2histogram | 7.143% | 6.010% | 0.898% |
| hdrhistogram | 0.000% | 0.401% | 0.229% |
| tdigest | 0.251% | 0.559% | 0.423% |

### exponential

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.152% |
| ddsketch | 0.794% | 0.520% | 0.530% |
| h2histogram | 1.737% | 6.041% | 0.022% |
| hdrhistogram | 0.000% | 0.367% | 0.673% |
| tdigest | 0.185% | 0.113% | 0.178% |

### pareto

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.000% | 0.000% |
| ddsketch | 1.000% | 0.411% | 0.554% |
| h2histogram | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.000% | 0.000% |
| tdigest | 0.000% | 0.557% | 0.568% |

