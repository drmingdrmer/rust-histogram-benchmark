use std::env;
use std::fmt::Write;
use std::fs;

use rust_histogram_benchmark::output::BenchResult;

fn main() {
    let mut output_dir = String::from("results");
    let mut paths = Vec::new();

    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--output-dir" | "-o" => {
                i += 1;
                output_dir = args[i].clone();
            }
            _ => paths.push(args[i].clone()),
        }
        i += 1;
    }

    if paths.is_empty() {
        eprintln!("Usage: gen-charts [--output-dir DIR] <json-files...>");
        std::process::exit(1);
    }

    let mut results: Vec<BenchResult> = paths
        .iter()
        .map(|p| {
            let content = fs::read_to_string(p).unwrap_or_else(|e| panic!("read {p}: {e}"));
            parse_result(&content)
        })
        .collect();

    sort_results(&mut results);

    let perf_svg = perf_bars_svg(&results);
    let radar_svgs = radar_svgs(&results);
    let heatmap_svg = heatmap_svg(&results);

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(format!("{output_dir}/chart-perf.svg"), &perf_svg).unwrap();
    for (name, svg) in &radar_svgs {
        fs::write(format!("{output_dir}/chart-radar-{}.svg", slugify(name)), svg).unwrap();
    }
    fs::write(format!("{output_dir}/chart-accuracy.svg"), &heatmap_svg).unwrap();

    let html = html_dashboard(&perf_svg, &radar_svgs, &heatmap_svg);
    fs::write(format!("{output_dir}/charts.html"), &html).unwrap();

    eprintln!(
        "Generated charts: perf, radar (x{}), accuracy, dashboard",
        radar_svgs.len()
    );
}

// ---------------------------------------------------------------------------
// Colors
// ---------------------------------------------------------------------------

fn color_for(family: &str) -> &'static str {
    match family {
        "base2histogram" => "#4878CF",
        "ddsketch" => "#D4A942",
        "h2histogram" => "#E87D2B",
        "hdrhistogram" => "#D65F5F",
        "quantogram" => "#56A354",
        "reqsketch" => "#4E9AA8",
        "tdigest" => "#9F7FBA",
        _ => "#888888",
    }
}

fn error_color(pct: f64) -> String {
    // Light green (0%) -> Yellow (~1%) -> Red (2%+)
    let t = (pct / 2.0).clamp(0.0, 1.0);
    let (r, g, b) = if t <= 0.5 {
        let s = t * 2.0;
        (
            (130.0 + s * 125.0) as u8,
            (210.0 + s * 10.0) as u8,
            (120.0 * (1.0 - s) + 50.0 * s) as u8,
        )
    } else {
        let s = (t - 0.5) * 2.0;
        (255, (220.0 * (1.0 - s) + 60.0 * s) as u8, (50.0 * (1.0 - s)) as u8)
    };
    format!("#{r:02x}{g:02x}{b:02x}")
}

fn parse_result(input: &str) -> BenchResult {
    serde_json::from_str(input).unwrap_or_else(|e| panic!("failed to parse benchmark result: {e}"))
}

fn sort_results(results: &mut [BenchResult]) {
    results.sort_by(|a, b| a.family.cmp(&b.family).then_with(|| a.name.cmp(&b.name)));
}

// ---------------------------------------------------------------------------
// SVG building helpers — avoid # inside format strings
// ---------------------------------------------------------------------------

/// Write an SVG text element. `attrs` contains everything after `<text `.
fn svg_text(buf: &mut String, x: f64, y: f64, attrs: &str, content: &str) {
    writeln!(buf, r#"<text x="{x:.1}" y="{y:.1}" {attrs}>{content}</text>"#).unwrap();
}

fn svg_rect(buf: &mut String, x: f64, y: f64, w: f64, h: f64, attrs: &str) {
    writeln!(
        buf,
        r#"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" {attrs}/>"#
    )
    .unwrap();
}

// ---------------------------------------------------------------------------
// Performance bar charts (4 panels)
// ---------------------------------------------------------------------------

struct BarEntry {
    name: String,
    family: String,
    value: f64,
    skip: bool,
}

fn perf_bars_svg(results: &[BenchResult]) -> String {
    let panels: Vec<(&str, &str, Vec<BarEntry>)> = vec![
        (
            "Record Speed (log-normal, ns/op)",
            "ns",
            results
                .iter()
                .map(|r| BarEntry {
                    name: r.name.clone(),
                    family: r.family.clone(),
                    value: r.record_throughput.log_normal_ns,
                    skip: false,
                })
                .collect(),
        ),
        (
            "P99 Query Latency (ns/op)",
            "ns",
            results
                .iter()
                .map(|r| BarEntry {
                    name: r.name.clone(),
                    family: r.family.clone(),
                    value: r.percentile_latency.p99_ns,
                    skip: false,
                })
                .collect(),
        ),
        (
            "Memory Usage (bytes)",
            "",
            results
                .iter()
                .map(|r| BarEntry {
                    name: r.name.clone(),
                    family: r.family.clone(),
                    value: r.memory_bytes as f64,
                    skip: false,
                })
                .collect(),
        ),
        (
            "Merge Latency (ns/op)",
            "ns",
            results
                .iter()
                .map(|r| BarEntry {
                    name: r.name.clone(),
                    family: r.family.clone(),
                    value: r.merge_ns.unwrap_or(0.0),
                    skip: r.merge_ns.is_none(),
                })
                .collect(),
        ),
    ];

    let chart_w = 780.0_f64;
    let panel_gap = 35.0;
    let left_margin = 140.0;
    let right_margin = 85.0;
    let bar_area_w = chart_w - left_margin - right_margin;
    let bar_h = 26.0;
    let bar_gap = 8.0;
    let title_h = 32.0;

    let n = results.len() as f64;
    let panel_h = title_h + n * (bar_h + bar_gap) - bar_gap + 10.0;
    let total_h = panels.len() as f64 * (panel_h + panel_gap) - panel_gap + 30.0;

    let mut svg = String::new();
    write_svg_open(&mut svg, chart_w, total_h);
    svg_rect(&mut svg, 0.0, 0.0, chart_w, total_h, r#"fill="white" rx="8""#);

    let mut y_offset = 15.0;

    for (title, unit, mut entries) in panels {
        entries.sort_by(|a, b| {
            if a.skip && !b.skip {
                return std::cmp::Ordering::Greater;
            }
            if !a.skip && b.skip {
                return std::cmp::Ordering::Less;
            }
            a.value.partial_cmp(&b.value).unwrap()
        });

        let max_val = entries.iter().filter(|e| !e.skip).map(|e| e.value).fold(0.0_f64, f64::max);

        let title_attrs = r##"font-family="system-ui,sans-serif" font-size="14" font-weight="600" fill="#333""##;
        svg_text(&mut svg, left_margin, y_offset + 16.0, title_attrs, title);
        y_offset += title_h;

        for entry in &entries {
            let bar_y = y_offset;
            let text_y = bar_y + bar_h / 2.0 + 5.0;
            let name = esc(&entry.name);

            let label_attrs = r##"font-family="system-ui,sans-serif" font-size="12" fill="#444" text-anchor="end""##;
            svg_text(&mut svg, left_margin - 12.0, text_y, label_attrs, &name);

            if entry.skip {
                let na_attrs = r##"font-family="system-ui,sans-serif" font-size="11" fill="#999" font-style="italic""##;
                svg_text(&mut svg, left_margin + 8.0, text_y, na_attrs, "N/A");
            } else {
                let bar_w = (entry.value / max_val * bar_area_w).max(3.0);
                let color = color_for(&entry.family);

                // Background track
                svg_rect(
                    &mut svg,
                    left_margin,
                    bar_y,
                    bar_area_w,
                    bar_h,
                    r##"fill="#f5f5f5" rx="3""##,
                );

                // Colored bar
                let bar_attrs = format!(r#"fill="{color}" rx="3" opacity="0.85""#);
                svg_rect(&mut svg, left_margin, bar_y, bar_w, bar_h, &bar_attrs);

                // Value
                let label = format_value(entry.value, unit);
                let val_attrs = r##"font-family="system-ui,sans-serif" font-size="11" fill="#333" font-weight="500""##;
                svg_text(&mut svg, left_margin + bar_w + 8.0, text_y, val_attrs, &label);
            }

            y_offset += bar_h + bar_gap;
        }

        y_offset += panel_gap - bar_gap + 10.0;
    }

    write_svg_close(&mut svg);
    svg
}

fn format_value(v: f64, unit: &str) -> String {
    if unit.is_empty() {
        if v >= 1024.0 * 1024.0 {
            format!("{:.1} MB", v / (1024.0 * 1024.0))
        } else if v >= 1024.0 {
            format!("{:.1} KB", v / 1024.0)
        } else {
            format!("{v:.0} B")
        }
    } else if v >= 1000.0 {
        format!("{v:.0} {unit}")
    } else if v >= 10.0 {
        format!("{v:.1} {unit}")
    } else {
        format!("{v:.2} {unit}")
    }
}

// ---------------------------------------------------------------------------
// Radar chart
// ---------------------------------------------------------------------------

/// Returns one standalone SVG per histogram, all sharing the same normalization.
fn radar_svgs(results: &[BenchResult]) -> Vec<(String, String)> {
    let axes = ["Record", "Query", "Lightweight", "Merge", "Accuracy"];
    let n_axes = axes.len();

    let max_merge = results.iter().filter_map(|r| r.merge_ns).fold(0.0_f64, f64::max);

    let raw: Vec<RadarSeries> = results
        .iter()
        .map(|r| {
            let key_dists = ["log_normal_api", "bimodal", "exponential"];
            let key_errors: Vec<f64> = r
                .accuracy
                .iter()
                .filter(|a| key_dists.contains(&a.distribution.as_str()))
                .map(|a| a.p99_error_pct)
                .collect();
            let avg_p99_error = key_errors.iter().sum::<f64>() / key_errors.len() as f64;
            RadarSeries {
                name: r.name.clone(),
                family: r.family.clone(),
                values: [
                    r.record_throughput.log_normal_ns,
                    r.percentile_latency.p99_ns,
                    r.memory_bytes as f64,
                    r.merge_ns.unwrap_or(max_merge * 2.0),
                    avg_p99_error,
                ],
            }
        })
        .collect();

    let mut scores: Vec<RadarSeries> = raw
        .iter()
        .map(|series| RadarSeries {
            name: series.name.clone(),
            family: series.family.clone(),
            values: [0.0; 5],
        })
        .collect();

    for axis_i in 0..n_axes {
        let vals: Vec<f64> = raw.iter().map(|series| series.values[axis_i]).collect();
        let min_v = vals.iter().copied().fold(f64::INFINITY, f64::min);
        let max_v = vals.iter().copied().fold(0.0_f64, f64::max);
        let ln_range = (max_v / min_v).ln();

        for (j, series) in raw.iter().enumerate() {
            let score = if ln_range < 0.01 {
                1.0
            } else {
                1.0 - (series.values[axis_i] / min_v).ln() / ln_range
            };
            scores[j].values[axis_i] = score.clamp(0.08, 1.0);
        }
    }

    let w = 240.0_f64;
    let h = 210.0;
    let cx = 125.0;
    let cy = 115.0;
    let radius = 44.0;

    scores
        .iter()
        .map(|series| {
            let color = color_for(&series.family);
            let mut svg = String::new();
            write_svg_open(&mut svg, w, h);
            svg_rect(&mut svg, 0.0, 0.0, w, h, r#"fill="white" rx="6""#);

            let name_attrs = format!(
                r##"font-family="system-ui,sans-serif" font-size="18" font-weight="600" fill="{color}" text-anchor="middle""##
            );
            svg_text(&mut svg, cx, 28.0, &name_attrs, &esc(&series.name));

            for ring in 1..=5 {
                let r = radius * ring as f64 / 5.0;
                let points = polygon_points(cx, cy, r, n_axes);
                writeln!(
                    svg,
                    r##"<polygon points="{points}" fill="none" stroke="#e8e8e8" stroke-width="0.5"/>"##
                )
                .unwrap();
            }

            for i in 0..n_axes {
                let angle = axis_angle(i, n_axes);
                let ex = cx + radius * angle.cos();
                let ey = cy + radius * angle.sin();
                writeln!(
                    svg,
                    r##"<line x1="{cx}" y1="{cy}" x2="{ex:.1}" y2="{ey:.1}" stroke="#ddd" stroke-width="0.5"/>"##
                )
                .unwrap();
            }

            for (i, label) in axes.iter().enumerate() {
                let angle = axis_angle(i, n_axes);
                let label_r = radius + 10.0;
                let lx = cx + label_r * angle.cos();
                let ly = cy + label_r * angle.sin() + 5.0;
                let anchor = if angle.cos().abs() < 0.15 {
                    "middle"
                } else if angle.cos() > 0.0 {
                    "start"
                } else {
                    "end"
                };
                let attrs = format!(
                    r##"font-family="system-ui,sans-serif" font-size="15" fill="#999" text-anchor="{anchor}""##
                );
                svg_text(&mut svg, lx, ly, &attrs, label);
            }

            let mut points = String::new();
            for (i, &score) in series.values.iter().enumerate() {
                let angle = axis_angle(i, n_axes);
                let r = radius * score;
                if i > 0 {
                    points.push(' ');
                }
                write!(points, "{:.1},{:.1}", cx + r * angle.cos(), cy + r * angle.sin()).unwrap();
            }
            writeln!(
                svg,
                r#"<polygon points="{points}" fill="{color}" fill-opacity="0.20" stroke="{color}" stroke-width="1.5"/>"#
            )
            .unwrap();

            for (i, &score) in series.values.iter().enumerate() {
                let angle = axis_angle(i, n_axes);
                let r = radius * score;
                let px = cx + r * angle.cos();
                let py = cy + r * angle.sin();
                writeln!(svg, r#"<circle cx="{px:.1}" cy="{py:.1}" r="3" fill="{color}"/>"#)
                    .unwrap();
            }

            write_svg_close(&mut svg);
            (series.name.clone(), svg)
        })
        .collect()
}

#[derive(Clone)]
struct RadarSeries {
    name: String,
    family: String,
    values: [f64; 5],
}

fn axis_angle(i: usize, total: usize) -> f64 {
    std::f64::consts::PI * 2.0 * i as f64 / total as f64 - std::f64::consts::FRAC_PI_2
}

fn polygon_points(cx: f64, cy: f64, r: f64, n: usize) -> String {
    let mut pts = String::new();
    for i in 0..n {
        let angle = axis_angle(i, n);
        if i > 0 {
            pts.push(' ');
        }
        write!(pts, "{:.1},{:.1}", cx + r * angle.cos(), cy + r * angle.sin()).unwrap();
    }
    pts
}

// ---------------------------------------------------------------------------
// Accuracy heatmap
// ---------------------------------------------------------------------------

fn heatmap_svg(results: &[BenchResult]) -> String {
    let dists = ["log_normal_api", "bimodal", "exponential", "pareto", "uniform"];
    let dist_labels = ["Log-Normal", "Bimodal", "Exponential", "Pareto", "Uniform"];

    let cell_w = 90.0_f64;
    let cell_h = 42.0;
    let label_w = 110.0;
    let header_h = 140.0;

    let n_cols = results.len();
    let n_rows = dists.len();
    let w = label_w + n_cols as f64 * cell_w + 20.0;
    let h = header_h + n_rows as f64 * cell_h + 55.0;

    let mut svg = String::new();
    write_svg_open(&mut svg, w, h);
    svg_rect(&mut svg, 0.0, 0.0, w, h, r#"fill="white" rx="8""#);

    let title_attrs =
        r##"font-family="system-ui,sans-serif" font-size="14" font-weight="600" fill="#333" text-anchor="middle""##;
    svg_text(&mut svg, w / 2.0, 24.0, title_attrs, "P99 Accuracy Error %");

    // Column headers (rotated)
    for (j, r) in results.iter().enumerate() {
        let x = label_w + j as f64 * cell_w + cell_w / 2.0;
        let y = header_h - 12.0;
        writeln!(
            svg,
            r##"<text x="{x:.1}" y="{y:.1}" font-family="system-ui,sans-serif" font-size="11" fill="#444" text-anchor="start" transform="rotate(-55,{x:.1},{y:.1})">{}</text>"##,
            esc(&r.name),
        ).unwrap();
    }

    // Rows
    for (i, dist) in dists.iter().enumerate() {
        let y = header_h + i as f64 * cell_h;

        let row_attrs = r##"font-family="system-ui,sans-serif" font-size="11" fill="#444" text-anchor="end""##;
        svg_text(
            &mut svg,
            label_w - 10.0,
            y + cell_h / 2.0 + 4.0,
            row_attrs,
            dist_labels[i],
        );

        for (j, r) in results.iter().enumerate() {
            let x = label_w + j as f64 * cell_w;
            let acc = r.accuracy.iter().find(|a| a.distribution == *dist);
            let p99 = acc.map(|a| a.p99_error_pct).unwrap_or(f64::NAN);

            let fill = if p99.is_nan() {
                "#f0f0f0".to_string()
            } else {
                error_color(p99)
            };

            let cell_attrs = format!(r#"fill="{fill}" stroke="white" stroke-width="2" rx="3""#);
            svg_rect(&mut svg, x, y, cell_w - 2.0, cell_h, &cell_attrs);

            let text_color = if p99.is_nan() || p99 < 0.8 { "#333" } else { "#fff" };
            let label = if p99.is_nan() {
                "\u{2014}".to_string()
            } else if p99 < 0.001 {
                "0.000%".to_string()
            } else {
                format!("{p99:.3}%")
            };
            let val_attrs = format!(
                r#"font-family="system-ui,sans-serif" font-size="10" font-weight="500" fill="{text_color}" text-anchor="middle""#
            );
            svg_text(
                &mut svg,
                x + (cell_w - 2.0) / 2.0,
                y + cell_h / 2.0 + 4.0,
                &val_attrs,
                &label,
            );
        }
    }

    // Color scale legend
    let legend_y = header_h + n_rows as f64 * cell_h + 18.0;
    let legend_x = label_w;
    let legend_w = 200.0;
    let legend_h = 12.0;
    let steps = 40;

    for s in 0..steps {
        let t = s as f64 / steps as f64 * 2.0;
        let fill = error_color(t);
        let sx = legend_x + s as f64 / steps as f64 * legend_w;
        let sw = legend_w / steps as f64 + 0.5;
        let attrs = format!(r#"fill="{fill}""#);
        svg_rect(&mut svg, sx, legend_y, sw, legend_h, &attrs);
    }

    let scale_attrs = r##"font-family="system-ui,sans-serif" font-size="9" fill="#666""##;
    svg_text(&mut svg, legend_x, legend_y + legend_h + 12.0, scale_attrs, "0%");
    let mid_attrs = r##"font-family="system-ui,sans-serif" font-size="9" fill="#666" text-anchor="middle""##;
    svg_text(
        &mut svg,
        legend_x + legend_w / 2.0,
        legend_y + legend_h + 12.0,
        mid_attrs,
        "1%",
    );
    let end_attrs = r##"font-family="system-ui,sans-serif" font-size="9" fill="#666" text-anchor="end""##;
    svg_text(
        &mut svg,
        legend_x + legend_w,
        legend_y + legend_h + 12.0,
        end_attrs,
        "2%+",
    );

    write_svg_close(&mut svg);
    svg
}

// ---------------------------------------------------------------------------
// HTML dashboard
// ---------------------------------------------------------------------------

fn html_dashboard(perf: &str, radars: &[(String, String)], heatmap: &str) -> String {
    let radar_items: String = radars
        .iter()
        .map(|(_, svg)| format!(r#"<div class="radar-item">{svg}</div>"#))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Rust Histogram Benchmark</title>
<style>
  body {{
    font-family: system-ui, -apple-system, sans-serif;
    max-width: 900px;
    margin: 0 auto;
    padding: 20px 16px 60px;
    background: #fafafa;
    color: #333;
  }}
  h1 {{
    font-size: 1.5rem;
    font-weight: 600;
    border-bottom: 2px solid #e0e0e0;
    padding-bottom: 8px;
  }}
  h2 {{
    font-size: 1.1rem;
    font-weight: 600;
    margin-top: 32px;
    color: #555;
  }}
  .chart {{
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 4px rgba(0,0,0,0.08);
    padding: 8px;
    margin: 16px 0;
    overflow-x: auto;
  }}
  .chart svg {{
    display: block;
    max-width: 100%;
    height: auto;
  }}
  .radar-grid {{
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin: 16px 0;
  }}
  .radar-item {{
    background: white;
    border-radius: 6px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.06);
    padding: 4px;
  }}
  .radar-item svg {{
    display: block;
  }}
  footer {{
    margin-top: 40px;
    font-size: 0.8rem;
    color: #999;
    text-align: center;
  }}
</style>
</head>
<body>
<h1>Rust Histogram Benchmark</h1>

<h2>Performance Comparison</h2>
<div class="chart">{perf}</div>

<h2>Normalized Radar (outer = better)</h2>
<div class="radar-grid">
{radar_items}
</div>

<h2>Accuracy Heatmap (P99 Error)</h2>
<div class="chart">{heatmap}</div>

<footer>Generated by rust-histogram-benchmark</footer>
</body>
</html>
"#
    )
}

// ---------------------------------------------------------------------------
// SVG helpers
// ---------------------------------------------------------------------------

fn write_svg_open(buf: &mut String, w: f64, h: f64) {
    writeln!(
        buf,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">"#,
    )
    .unwrap();
}

fn write_svg_close(buf: &mut String) {
    writeln!(buf, "</svg>").unwrap();
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}
