#!/bin/sh

# Build release runtime benchmarks
cargo build --release --features=runtime-benchmarks

# Collect all pallets needed for benchmarking
# Makes the assumption all pallets are present at: /pallets/$PALLET_NAME
pallets=(
    "frame_system",,
    "pallet_identity",
    "pallet_assets",
    "pallet_balances",
    "pallet_session",
    "pallet_timestamp",
    "pallet_message_queue",
    "pallet_sudo",
    "pallet_collator_selection",
    "cumulus_pallet_xcmp_queue",
    "cumulus_pallet_parachain_system",
    "pallet_proxy",
    "pallet_utility",
    "pallet_multisig",
    "pallet_xcm"
)



# Generate weights
for pallet_name in $pallets; do
    ./target/release/educhain-node benchmark pallet \
        --pallet $pallet_name \
        --extrinsic "*" \
        --steps 50 \
        --repeat 20 \
        --output ./runtime/src/weights/$pallet_name.rs
done