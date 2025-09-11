cargo build --release

chain-spec-builder --chain-spec-path ./artifacts/dev/dev_plain_balances.json create --relay-chain "paseo" --para-id 4883 --runtime \
    target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm named-preset development