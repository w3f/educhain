cargo build --release

# Check if we're generating a raw chain spec
if [ "$1" == "raw" ]; then
    echo "Generating raw chain spec... make sure you have already generated the plain chain spec first!"
    cd deploy
    chain-spec-builder convert-to-raw ./artifacts/latest_plain_chain_spec.json
else
    echo "Generating regular chain spec..."
    chain-spec-builder --chain-spec-path ./artifacts/latest_plain_chain_spec.json create --relay-chain "paseo" --para-id 4883 --runtime \
    target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm patch educhain.patch.json
fi