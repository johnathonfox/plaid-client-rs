#!/usr/bin/env bash
set -euo pipefail

echo "Running tests with coverage..."
cargo tarpaulin --out Html --output-dir target/coverage

echo "Coverage report: target/coverage/tarpaulin-report.html"
