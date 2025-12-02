#!/usr/bin/env bash

# =====================================================================
# DEPRECATED: This script is deprecated as of January 2025
# 
# Please use the new formalized build scripts instead:
#   ./scripts/build-spec-pop.sh dev no-balances        (recommended)
#   ./scripts/build-spec-traditional.sh dev no-balances
#
# See scripts/README.md for detailed documentation.
# =====================================================================

echo "⚠️  WARNING: This script is deprecated!"
echo "Please use: ./scripts/build-spec-pop.sh dev no-balances"
echo ""
read -p "Continue with deprecated script? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

NODE_BIN=./target/release/educhain-node
OUT=artifacts/dev_plain.json

# Generate the plain chainspec
$NODE_BIN build-spec --chain dev > $OUT

# Patch the fields to match pop! CLI convention
jq '
  .name = "Educhain Paseo Local" |
  .id = "local" |
  .chainType = "Local" |
  .protocolId = "educhain-paseo-local" |
  .relay_chain = "paseo-local" |
  .para_id = 4883 |
  .properties.tokenDecimals = 10 |
  .properties.tokenSymbol = "PAS"
' $OUT > ${OUT}.tmp && mv ${OUT}.tmp $OUT

echo "dev_plain.json generated and patched for pop! CLI."