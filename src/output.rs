use serde::Deserialize;
use serde::Serialize;

/// Output format shared by all benchmark binaries.
///
/// Each binary prints one JSON object to stdout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchResult {
    pub name: String,
    pub record_throughput: RecordThroughput,
    pub percentile_latency: PercentileLatency,
    pub memory_bytes: usize,
    pub merge_ns: Option<f64>,
    pub accuracy: Vec<AccuracyResult>,
}

/// Nanoseconds per `record()` call for each workload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordThroughput {
    pub sequential_ns: f64,
    pub uniform_ns: f64,
    pub log_normal_ns: f64,
}

/// Nanoseconds per percentile query after recording 1M values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentileLatency {
    pub p50_ns: f64,
    pub p90_ns: f64,
    pub p95_ns: f64,
    pub p99_ns: f64,
    pub p999_ns: f64,
}

/// Accuracy for one distribution at several percentile points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyResult {
    pub distribution: String,
    pub p50_error_pct: f64,
    pub p95_error_pct: f64,
    pub p99_error_pct: f64,
}
