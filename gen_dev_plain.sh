#!/bin/sh

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