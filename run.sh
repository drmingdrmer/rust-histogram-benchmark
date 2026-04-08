#!/bin/bash
set -e
cd "$(dirname "$0")"

RESULTS_DIR="results"
mkdir -p "$RESULTS_DIR"

echo "Building release binaries..."
cargo build --release 2>&1 | tail -1

BINS=(
    bench-base2histogram
    bench-hdrhistogram
    bench-h2histogram
    bench-quantogram
    bench-ddsketch
    bench-reqsketch
    bench-kllsketch
    bench-tdigest
)

JSON_PATHS=()

for bin in "${BINS[@]}"; do
    json_path="$RESULTS_DIR/$bin.json"
    echo "Running $bin..."
    ./target/release/"$bin" > "$json_path" 2>/dev/null
    JSON_PATHS+=("$json_path")
done

echo ""
echo "Generating reports..."

# Markdown report for embedding in README
./target/release/report --markdown "${JSON_PATHS[@]}" > "$RESULTS_DIR/report.md"

# Charts (SVG + HTML dashboard + highlights markdown)
./target/release/gen-charts --output-dir "$RESULTS_DIR" "${JSON_PATHS[@]}"

# Text report to terminal
echo ""
./target/release/report --text "${JSON_PATHS[@]}"
