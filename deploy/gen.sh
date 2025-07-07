chain-spec-builder \
--chain-spec-path ./artifacts/latest_raw_chain_spec.json \
convert-to-raw artifacts/latest_plain_chain_spec.json
    
polkadot-omni-node export-genesis-wasm \
--chain  ./artifacts/latest_raw_chain_spec.json artifacts/latest-para-wasm

polkadot-omni-node export-genesis-head \
--chain ./artifacts/latest_raw_chain_spec.json artifacts/latest-para-state