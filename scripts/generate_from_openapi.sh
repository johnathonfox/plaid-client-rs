#!/usr/bin/env bash
set -euo pipefail

SPEC_URL="https://raw.githubusercontent.com/plaid/plaid-openapi/master/openapi.yaml"
OUT_DIR="src/models/generated"

echo "Downloading Plaid OpenAPI spec..."
curl -sL "$SPEC_URL" -o /tmp/plaid-openapi.yaml

echo "Generating Rust models..."
# TODO: Add progenitor or openapi-generator command here
# progenitor generate -i /tmp/plaid-openapi.yaml -o $OUT_DIR

echo "Done. Review generated code in $OUT_DIR"
