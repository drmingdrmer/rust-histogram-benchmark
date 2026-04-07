## Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.0 | 1.9 | 1.8 |
| ddsketch | 4.7 | 4.5 | 4.5 |
| h2histogram | 1.9 | 1.4 | 1.5 |
| hdrhistogram-3 | 2.3 | 2.1 | 2.0 |
| hdrhistogram | 2.3 | 2.0 | 2.0 |
| quantogram | 50.1 | 76.8 | 70.6 |
| tdigest | 2.1 | 13.9 | 14.2 |

## Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 18.4 | 11.2 | 12.4 | 13.5 | 14.7 |
| ddsketch | 53.2 | 63.2 | 65.3 | 73.5 | 80.7 |
| h2histogram | 132.8 | 141.9 | 145.3 | 144.2 | 146.9 |
| hdrhistogram-3 | 150.6 | 276.3 | 320.9 | 458.1 | 666.4 |
| hdrhistogram | 120.0 | 156.7 | 181.0 | 196.5 | 227.2 |
| quantogram | 87.2 | 76.6 | 103.4 | 72.4 | 113.0 |
| tdigest | 44.4 | 8.6 | 7.3 | 4.8 | 3.1 |

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

