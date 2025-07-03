cargo build --release

chain-spec-builder --chain-spec-path ./artifacts/latest_plain_chain_spec.json create --relay-chain "paseo-local" --para-id 4518 --runtime \
    target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm patch educhain.patch.json


