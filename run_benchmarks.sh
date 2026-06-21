#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

RESULTS_DIR="$SCRIPT_DIR/results"
mkdir -p "$RESULTS_DIR"

CRATES=(
    "bench-mermaid-rs-renderer"
    "bench-merman"
    "bench-rusty-mermaid"
    "bench-selkie"
)

LABELS=(
    "mermaid-rs-renderer"
    "merman"
    "rusty-mermaid"
    "selkie"
)

VERSIONS=(
    "0.2.2"
    "0.8.0-alpha.1"
    "0.2.0"
    "0.3.0"
)

DESCRIPTIONS=(
    "Fast native Mermaid renderer, 23 diagram types, SVG output"
    "Parity-focused headless Mermaid, SVG/PNG/ASCII output"
    "Pure Rust Mermaid, parse+layout+render to SVG/PNG/GPU"
    "Full Rust Mermaid parser & renderer, SVG/ASCII/text"
)

echo "============================================"
echo "  Mermaid Rust SVG Libraries Benchmark"
echo "============================================"
echo ""
echo "Date: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "Rust: $(rustc --version)"
echo "OS:   $(uname -srm)"
echo ""

> "$RESULTS_DIR/all_results.jsonl"

for i in "${!CRATES[@]}"; do
    crate="${CRATES[$i]}"
    label="${LABELS[$i]}"
    version="${VERSIONS[$i]}"
    desc="${DESCRIPTIONS[$i]}"

    echo "--------------------------------------------"
    echo "  Benchmarking: $label v$version"
    echo "  $desc"
    echo "--------------------------------------------"

    if cargo bench -p "$crate" --bench bench -- --noplot 2>&1 | tee "$RESULTS_DIR/${label}.log"; then
        echo "{\"status\":\"ok\",\"crate\":\"$label\",\"version\":\"$version\",\"description\":\"$desc\"}" >> "$RESULTS_DIR/all_results.jsonl"
    else
        echo "{\"status\":\"error\",\"crate\":\"$label\",\"version\":\"$version\",\"description\":\"$desc\"}" >> "$RESULTS_DIR/all_results.jsonl"
        echo "  [WARN] $label benchmarks had errors (partial results may exist)"
    fi
    echo ""
done

echo "============================================"
echo "  Collecting criterion results..."
echo "============================================"

python3 "$SCRIPT_DIR/generate_report.py" \
    --criterion-dir "$SCRIPT_DIR/target/criterion" \
    --results-dir "$RESULTS_DIR" \
    --output "$SCRIPT_DIR/report.html"

echo ""
echo "Report generated: $SCRIPT_DIR/report.html"
echo "Done!"
