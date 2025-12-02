#!/usr/bin/env bash

# =====================================================================
# DEPRECATED: This script is deprecated as of January 2025
# 
# Please use the new formalized build scripts instead:
#   ./scripts/build-spec-pop.sh dev with-balances        (recommended)
#   ./scripts/build-spec-traditional.sh dev with-balances
#
# See scripts/README.md for detailed documentation.
# =====================================================================

echo "⚠️  WARNING: This script is deprecated!"
echo "Please use: ./scripts/build-spec-pop.sh dev with-balances"
echo ""
read -p "Continue with deprecated script? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

cargo build --release

chain-spec-builder --chain-spec-path ./artifacts/dev/dev_plain_balances.json create --relay-chain "paseo" --para-id 4883 --runtime \
    target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm named-preset development