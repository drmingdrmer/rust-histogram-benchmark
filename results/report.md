## Configurations

| Histogram | Config |
|---|---|
| base2histogram | `width=3` |
| ckms | `epsilon=0.1` |
| ddsketch | `alpha=0.01,max_num_bins=2048,min_value=1.0` |
| h2histogram | `grouping_power=4,max_value_power=64` |
| hdrhistogram | `fixed_bounds,max=observed_max,sigfig=2` |
| kllsketch | `k=200` |
| quantogram | `bins_per_doubling=35,smallest_power=0,largest_power=observed_max_power` |
| reqsketch | `k=12,rank_accuracy=high` |
| tdigest | `max_size=100,batch_size=1000,local_sort+merge_sorted` |
## Recording Throughput (ns/op)

| Histogram | sequential | uniform | log-normal |
|---|---:|---:|---:|
| base2histogram | 2.1 | 2.0 | 1.8 |
| ckms | 788.2 | 3882.8 | 5048.4 |
| ddsketch | 4.6 | 4.5 | 4.5 |
| h2histogram | 2.0 | 1.4 | 1.4 |
| hdrhistogram | 2.3 | 2.2 | 2.2 |
| kllsketch | 7.5 | 24.9 | 25.0 |
| quantogram | 46.7 | 78.2 | 71.5 |
| reqsketch | 13.0 | 33.5 | 33.2 |
| tdigest | 3.0 | 15.1 | 15.9 |

## Percentile Query Latency (ns/op)

| Histogram | P50 | P90 | P95 | P99 | P99.9 |
|---|---:|---:|---:|---:|---:|
| base2histogram | 22.1 | 13.7 | 13.1 | 13.5 | 14.4 |
| ckms | 176443.9 | 252119.1 | 261938.9 | 276570.5 | 276368.8 |
| ddsketch | 51.0 | 62.7 | 67.3 | 82.0 | 78.8 |
| h2histogram | 408.1 | 455.0 | 458.2 | 476.9 | 521.4 |
| hdrhistogram | 115.8 | 149.3 | 159.4 | 175.8 | 197.9 |
| kllsketch | 199201.3 | 201616.0 | 201286.0 | 201792.7 | 202202.8 |
| quantogram | 83.5 | 99.4 | 122.9 | 76.6 | 127.3 |
| reqsketch | 10.2 | 9.8 | 9.8 | 9.9 | 9.8 |
| tdigest | 39.1 | 8.9 | 7.3 | 4.5 | 3.3 |

## Memory (retained heap bytes after recording 2M log-normal values)

| Histogram | memory |
|---|---:|
| base2histogram | 2.1 KB |
| ckms | 981.1 KB |
| ddsketch | 2.0 KB |
| h2histogram | 7.6 KB |
| hdrhistogram | 15.0 KB |
| kllsketch | 398.6 KB |
| quantogram | 31.5 KB |
| reqsketch | 61.9 KB |
| tdigest | 9.4 KB |

## Merge Latency (ns/op)

| Histogram | merge |
|---|---:|
| base2histogram | 34.8 |
| ckms | 895198460.0 |
| ddsketch | 146.8 |
| h2histogram | 215.7 |
| hdrhistogram | 795.2 |
| kllsketch | 21467.0 |
| quantogram | — |
| reqsketch | 44738.8 |
| tdigest | 1203.3 |

## Accuracy: Relative Error %

### uniform

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.016% | 1.063% | 3.740% |
| ckms | 6.349% | 3.750% | 7.569% |
| ddsketch | 0.783% | 0.596% | 0.494% |
| h2histogram | 0.080% | 1.721% | 0.950% |
| hdrhistogram | 0.329% | 0.003% | 0.122% |
| kllsketch | 0.980% | 0.537% | 0.498% |
| quantogram | 0.225% | 0.945% | 0.803% |
| reqsketch | 0.543% | 0.002% | 0.011% |
| tdigest | 0.109% | 0.020% | 0.020% |

### log_normal_api

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.109% | 0.000% |
| ckms | 6.203% | 21.351% | 225.194% |
| ddsketch | 0.875% | 0.801% | 0.782% |
| h2histogram | 0.993% | 0.763% | 1.628% |
| hdrhistogram | 0.000% | 0.109% | 0.388% |
| kllsketch | 0.744% | 2.288% | 7.597% |
| quantogram | 0.744% | 0.000% | 0.310% |
| reqsketch | 0.000% | 0.109% | 0.078% |
| tdigest | 0.027% | 0.039% | 0.223% |

### bimodal

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.033% | 0.018% |
| ckms | 10.390% | 815.426% | 384.828% |
| ddsketch | 0.681% | 0.553% | 0.999% |
| h2histogram | 0.649% | 0.401% | 1.839% |
| hdrhistogram | 0.000% | 0.401% | 0.141% |
| kllsketch | 0.000% | 6.110% | 11.459% |
| quantogram | 0.649% | 0.601% | 0.424% |
| reqsketch | 0.649% | 0.367% | 0.159% |
| tdigest | 0.091% | 0.982% | 0.184% |

### exponential

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.100% | 0.130% |
| ckms | 10.823% | 5.507% | 31.312% |
| ddsketch | 0.918% | 0.520% | 0.401% |
| h2histogram | 0.866% | 0.367% | 2.673% |
| hdrhistogram | 0.289% | 0.367% | 0.109% |
| kllsketch | 1.443% | 3.138% | 8.627% |
| quantogram | 0.433% | 0.567% | 0.739% |
| reqsketch | 1.587% | 0.067% | 0.543% |
| tdigest | 0.020% | 0.042% | 0.118% |

### pareto

| Histogram | P50 | P95 | P99 |
|---|---:|---:|---:|
| base2histogram | 0.000% | 0.000% | 0.000% |
| ckms | 0.000% | 383757.143% | 127852.381% |
| ddsketch | 100.000% | 0.411% | 0.554% |
| h2histogram | 0.000% | 0.000% | 0.000% |
| hdrhistogram | 0.000% | 0.000% | 0.000% |
| kllsketch | 0.000% | 14.286% | 23.810% |
| quantogram | 0.000% | 0.000% | 0.000% |
| reqsketch | 0.000% | 0.000% | 0.000% |
| tdigest | 0.000% | 0.745% | 1.280% |

