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
    bench-hdrhistogram-3
    bench-h2histogram
    bench-quantogram
    bench-ddsketch
    bench-tdigest
)

for bin in "${BINS[@]}"; do
    echo "Running $bin..."
    ./target/release/"$bin" > "$RESULTS_DIR/$bin.json" 2>/dev/null
done

echo ""
echo "Generating reports..."

# Markdown report for embedding in README
./target/release/report --markdown "$RESULTS_DIR"/*.json > "$RESULTS_DIR/report.md"

# Charts (SVG + HTML dashboard)
./target/release/gen-charts --output-dir "$RESULTS_DIR" "$RESULTS_DIR"/*.json

# Text report to terminal
echo ""
./target/release/report --text "$RESULTS_DIR"/*.json
