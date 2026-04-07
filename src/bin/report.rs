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

    let results: Vec<BenchResult> = if paths.is_empty() {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        parse_results(&input)
    } else {
        paths
            .iter()
            .flat_map(|path| {
                let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
                parse_results(&content)
            })
            .collect()
    };

    if results.is_empty() {
        eprintln!("no results found");
        std::process::exit(1);
    }

    match format {
        Format::Text => print_text(&results),
        Format::Markdown => print_markdown(&results),
    }
}

fn parse_results(input: &str) -> Vec<BenchResult> {
    if let Ok(r) = serde_json::from_str::<BenchResult>(input) {
        return vec![r];
    }
    if let Ok(rs) = serde_json::from_str::<Vec<BenchResult>>(input) {
        return rs;
    }
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<BenchResult>(line).ok())
        .collect()
}

fn fmt_merge_ns(v: Option<f64>) -> String {
    match v {
        Some(ns) => format!("{ns:.1}"),
        None => "—".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Text output (terminal)
// ---------------------------------------------------------------------------

fn print_text(results: &[BenchResult]) {
    println!("Recording Throughput (ns/op)\n");
    println!("{:<20} {:>12} {:>12} {:>12}", "", "sequential", "uniform", "log-normal");
    println!("{}", "-".repeat(58));
    for r in results {
        println!(
            "{:<20} {:>12.1} {:>12.1} {:>12.1}",
            r.name,
            r.record_throughput.sequential_ns,
            r.record_throughput.uniform_ns,
            r.record_throughput.log_normal_ns,
        );
    }

    println!("\nPercentile Query Latency (ns/op)\n");
    println!(
        "{:<20} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "", "P50", "P90", "P95", "P99", "P99.9"
    );
    println!("{}", "-".repeat(72));
    for r in results {
        println!(
            "{:<20} {:>10.1} {:>10.1} {:>10.1} {:>10.1} {:>10.1}",
            r.name,
            r.percentile_latency.p50_ns,
            r.percentile_latency.p90_ns,
            r.percentile_latency.p95_ns,
            r.percentile_latency.p99_ns,
            r.percentile_latency.p999_ns,
        );
    }

    println!("\nMerge Latency (ns/op)\n");
    println!("{:<20} {:>12}", "", "merge");
    println!("{}", "-".repeat(34));
    for r in results {
        println!("{:<20} {:>12}", r.name, fmt_merge_ns(r.merge_ns));
    }

    println!("\nAccuracy: Relative Error %\n");
    let dist_names: Vec<&str> = results[0].accuracy.iter().map(|a| a.distribution.as_str()).collect();
    for dist in &dist_names {
        println!("  {dist}");
        println!("  {:<18} {:>12} {:>12} {:>12}", "", "P50", "P95", "P99");
        println!("  {}", "-".repeat(56));
        for r in results {
            if let Some(a) = r.accuracy.iter().find(|a| a.distribution == *dist) {
                println!(
                    "  {:<18} {:>11.3}% {:>11.3}% {:>11.3}%",
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

    println!("\n## Merge Latency (ns/op)\n");
    println!("| Histogram | merge |");
    println!("|---|---:|");
    for r in results {
        println!("| {} | {} |", r.name, fmt_merge_ns(r.merge_ns));
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
