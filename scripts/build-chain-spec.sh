#!/usr/bin/env bash
# Generate chain specifications using chain-spec-builder and polkadot-omni-node
# Usage: ./scripts/build-chain-spec.sh [dev|live] [with-balances|no-balances]

set -e

CHAIN_TYPE=${1:-live}
BALANCES=${2:-no-balances}
OUTPUT_DIR="./artifacts"
RUNTIME_WASM="target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm"

mkdir -p "$OUTPUT_DIR"

echo "Building runtime..."
cargo build --release

if [ ! -f "$RUNTIME_WASM" ]; then
    echo "Error: Runtime WASM not found at $RUNTIME_WASM"
    exit 1
fi

if [ "$CHAIN_TYPE" = "dev" ]; then
    if [ "$BALANCES" = "with-balances" ]; then
        chain-spec-builder \
            --chain-spec-path "$OUTPUT_DIR/dev_plain_balances.json" \
            create \
            --relay-chain "paseo" \
            --para-id 4883 \
            --runtime "$RUNTIME_WASM" \
            named-preset development
        
        echo "✓ Created: $OUTPUT_DIR/dev_plain_balances.json"
    else
        polkadot-omni-node build-spec --chain dev > "$OUTPUT_DIR/dev_plain.json"
        echo "✓ Created: $OUTPUT_DIR/dev_plain.json"
    fi
elif [ "$CHAIN_TYPE" = "live" ]; then
    # Generate plain chain spec with patch
    chain-spec-builder \
        --chain-spec-path "$OUTPUT_DIR/latest_plain_chain_spec.json" \
        create \
        --relay-chain "paseo" \
        --para-id 4883 \
        --runtime "$RUNTIME_WASM" \
        patch educhain.patch.json
    
    # Apply correct metadata
    if command -v jq &> /dev/null; then
        TMP_FILE=$(mktemp)
        jq '.name = "Educhain Paseo" |
            .id = "live" |
            .chainType = "Live" |
            .protocolId = "educhain-paseo-live" |
            .para_id = 4883 |
            .relay_chain = "paseo" |
            .properties.tokenDecimals = 10 |
            .properties.tokenSymbol = "PAS"' \
            "$OUTPUT_DIR/latest_plain_chain_spec.json" > "$TMP_FILE" && \
            mv "$TMP_FILE" "$OUTPUT_DIR/latest_plain_chain_spec.json"
    fi
    
    echo "✓ Created: $OUTPUT_DIR/latest_plain_chain_spec.json"
    
    # Convert to raw chain spec (creates chain_spec.json in current directory)
    chain-spec-builder convert-to-raw "$OUTPUT_DIR/latest_plain_chain_spec.json"
    
    # Move and rename the raw spec to artifacts directory
    if [ -f "chain_spec.json" ]; then
        mv chain_spec.json "$OUTPUT_DIR/latest_raw_chain_spec.json"
    elif [ -f "latest_raw_chain_spec.json" ]; then
        mv latest_raw_chain_spec.json "$OUTPUT_DIR/latest_raw_chain_spec.json"
    fi
    
    if [ ! -f "$OUTPUT_DIR/latest_raw_chain_spec.json" ]; then
        echo "Error: Failed to create raw chain spec"
        exit 1
    fi
    
    echo "✓ Created: $OUTPUT_DIR/latest_raw_chain_spec.json"
    
    # Export genesis state using polkadot-omni-node
    polkadot-omni-node export-genesis-head \
        --chain "$OUTPUT_DIR/latest_raw_chain_spec.json" \
        "$OUTPUT_DIR/para-genesis-state"
    echo "✓ Created: $OUTPUT_DIR/para-genesis-state"
    
    # Export genesis wasm using polkadot-omni-node
    polkadot-omni-node export-genesis-wasm \
        --chain "$OUTPUT_DIR/latest_raw_chain_spec.json" \
        "$OUTPUT_DIR/para-genesis-wasm"
    echo "✓ Created: $OUTPUT_DIR/para-genesis-wasm"
else
    echo "Error: Use 'dev' or 'live' for chain type"
    exit 1
fi