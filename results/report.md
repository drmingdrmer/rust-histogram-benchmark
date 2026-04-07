## Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.1 | 1.9 | 1.8 |
| ddsketch | 4.8 | 4.5 | 4.5 |
| h2histogram | 2.1 | 1.4 | 1.5 |
| hdrhistogram-3 | 2.3 | 2.1 | 2.0 |
| hdrhistogram | 2.4 | 2.3 | 2.0 |
| quantogram | 50.7 | 80.6 | 70.1 |
| tdigest | 2.1 | 14.2 | 14.2 |

## Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 18.6 | 14.0 | 12.2 | 13.1 | 14.4 |
| ddsketch | 53.9 | 63.2 | 63.0 | 79.2 | 80.5 |
| h2histogram | 139.1 | 146.3 | 150.6 | 155.7 | 158.3 |
| hdrhistogram-3 | 150.8 | 276.9 | 332.7 | 450.5 | 666.9 |
| hdrhistogram | 122.4 | 164.7 | 176.5 | 197.7 | 225.5 |
| quantogram | 83.1 | 87.9 | 107.9 | 68.6 | 117.8 |
| tdigest | 44.1 | 8.2 | 8.5 | 5.0 | 3.6 |

## Memory (after recording 2M log-normal values)

| Histogram | heap bytes |
|---|---:|
| base2histogram | 2.1 KB |
| ddsketch | 2.0 KB |
| h2histogram | 2.0 KB |
| hdrhistogram-3 | 32.0 KB |
| hdrhistogram | 8.0 KB |
| quantogram | 31.3 KB |
| tdigest | 1.6 KB |

## Merge Latency (ns/op)

| Histogram | merge |
|---|---:|
| base2histogram | 33.4 |
| ddsketch | 149.7 |
| h2histogram | 68.7 |
| hdrhistogram-3 | 2042.7 |
| hdrhistogram | 501.5 |
| quantogram | — |
| tdigest | 1275.4 |

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

