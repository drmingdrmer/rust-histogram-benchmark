use std::env;
use std::fs;
use std::io::Read;

use rust_histogram_benchmark::output::BenchResult;

enum Format {
    Text,
    Markdown,
}

fn main() {
    let mut format = Format::Text;
    let mut paths = Vec::new();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--markdown" | "-m" => format = Format::Markdown,
            "--text" | "-t" => format = Format::Text,
            _ => paths.push(arg),
        }
    }

    let mut results: Vec<BenchResult> = if paths.is_empty() {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        vec![parse_result(&input)]
    } else {
        paths
            .iter()
            .map(|path| {
                let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
                parse_result(&content)
            })
            .collect()
    };

    if results.is_empty() {
        eprintln!("no results found");
        std::process::exit(1);
    }

    sort_results(&mut results);

    match format {
        Format::Text => print_text(&results),
        Format::Markdown => print_markdown(&results),
    }
}

fn parse_result(input: &str) -> BenchResult {
    serde_json::from_str(input).unwrap_or_else(|e| panic!("failed to parse benchmark result: {e}"))
}

fn fmt_optional_f64(v: Option<f64>) -> String {
    match v {
        Some(ns) => format!("{ns:.1}"),
        None => "—".to_string(),
    }
}

fn fmt_memory(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}

fn sort_results(results: &mut [BenchResult]) {
    results.sort_by(|a, b| a.family.cmp(&b.family).then_with(|| a.name.cmp(&b.name)));
}

// ---------------------------------------------------------------------------
// Text output (terminal)
// ---------------------------------------------------------------------------

fn print_text(results: &[BenchResult]) {
    println!("Configurations\n");
    println!("{:<26} {}", "Histogram", "config");
    println!("{}", "-".repeat(86));
    for r in results {
        println!("{:<26} {}", r.name, r.config);
    }

    println!("Recording Throughput (ns/op)\n");
    println!("{:<26} {:>12} {:>12} {:>12}", "", "sequential", "uniform", "log-normal");
    println!("{}", "-".repeat(64));
    for r in results {
        println!(
            "{:<26} {:>12.1} {:>12.1} {:>12.1}",
            r.name,
            r.record_throughput.sequential_ns,
            r.record_throughput.uniform_ns,
            r.record_throughput.log_normal_ns,
        );
    }

    println!("\nPercentile Query Latency (ns/op)\n");
    println!(
        "{:<26} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "", "P50", "P90", "P95", "P99", "P99.9"
    );
    println!("{}", "-".repeat(78));
    for r in results {
        println!(
            "{:<26} {:>10.1} {:>10.1} {:>10.1} {:>10.1} {:>10.1}",
            r.name,
            r.percentile_latency.p50_ns,
            r.percentile_latency.p90_ns,
            r.percentile_latency.p95_ns,
            r.percentile_latency.p99_ns,
            r.percentile_latency.p999_ns,
        );
    }

    println!("\nMemory (retained heap bytes after recording 2M log-normal values)\n");
    println!("{:<26} {:>14}", "", "memory");
    println!("{}", "-".repeat(42));
    for r in results {
        println!("{:<26} {:>14}", r.name, fmt_memory(r.memory_bytes));
    }

    println!("\nMerge Latency (ns/op)\n");
    println!("{:<26} {:>12}", "", "merge");
    println!("{}", "-".repeat(40));
    for r in results {
        println!("{:<26} {:>12}", r.name, fmt_optional_f64(r.merge_ns));
    }

    println!("\nAccuracy: Relative Error %\n");
    let dist_names: Vec<&str> = results[0].accuracy.iter().map(|a| a.distribution.as_str()).collect();
    for dist in &dist_names {
        println!("  {dist}");
        println!("  {:<24} {:>12} {:>12} {:>12}", "", "P50", "P95", "P99");
        println!("  {}", "-".repeat(62));
        for r in results {
            if let Some(a) = r.accuracy.iter().find(|a| a.distribution == *dist) {
                println!(
                    "  {:<24} {:>11.3}% {:>11.3}% {:>11.3}%",
                    r.name, a.p50_error_pct, a.p95_error_pct, a.p99_error_pct,
                );
            }
        }
        println!();
    }
}

// ---------------------------------------------------------------------------
// Markdown output
// ---------------------------------------------------------------------------

fn print_markdown(results: &[BenchResult]) {
    println!("## Configurations\n");
    println!("| Histogram | Config |");
    println!("|---|---|");
    for r in results {
        println!("| {} | `{}` |", r.name, r.config);
    }

    println!("## Recording Throughput (ns/op)\n");
    print!("| Histogram |");
    for label in ["sequential", "uniform", "log-normal"] {
        print!(" {label} |");
    }
    println!();
    println!("|---|---:|---:|---:|");
    for r in results {
        println!(
            "| {} | {:.1} | {:.1} | {:.1} |",
            r.name,
            r.record_throughput.sequential_ns,
            r.record_throughput.uniform_ns,
            r.record_throughput.log_normal_ns,
        );
    }

    println!("\n## Percentile Query Latency (ns/op)\n");
    println!("| Histogram | P50 | P90 | P95 | P99 | P99.9 |");
    println!("|---|---:|---:|---:|---:|---:|");
    for r in results {
        println!(
            "| {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |",
            r.name,
            r.percentile_latency.p50_ns,
            r.percentile_latency.p90_ns,
            r.percentile_latency.p95_ns,
            r.percentile_latency.p99_ns,
            r.percentile_latency.p999_ns,
        );
    }

    println!("\n## Memory (retained heap bytes after recording 2M log-normal values)\n");
    println!("| Histogram | memory |");
    println!("|---|---:|");
    for r in results {
        println!("| {} | {} |", r.name, fmt_memory(r.memory_bytes));
    }

    println!("\n## Merge Latency (ns/op)\n");
    println!("| Histogram | merge |");
    println!("|---|---:|");
    for r in results {
        println!("| {} | {} |", r.name, fmt_optional_f64(r.merge_ns));
    }

    println!("\n## Accuracy: Relative Error %\n");
    let dist_names: Vec<&str> = results[0].accuracy.iter().map(|a| a.distribution.as_str()).collect();
    for dist in &dist_names {
        println!("### {dist}\n");
        println!("| Histogram | P50 | P95 | P99 |");
        println!("|---|---:|---:|---:|");
        for r in results {
            if let Some(a) = r.accuracy.iter().find(|a| a.distribution == *dist) {
                println!(
                    "| {} | {:.3}% | {:.3}% | {:.3}% |",
                    r.name, a.p50_error_pct, a.p95_error_pct, a.p99_error_pct,
                );
            }
        }
        println!();
    }
}
