chain-spec-builder create --relay-chain "rococo-local" --para-id 2000 --runtime \
    target/release/wbuild/educhain-runtime/educhain_runtime.wasm default

zombienet --provider native spawn zombienet-omni-node.toml