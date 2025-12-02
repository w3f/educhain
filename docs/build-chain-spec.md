# Building Chain Specifications

Generate chain specifications for EduChain using `chain-spec-builder` and `polkadot-omni-node`.

## Prerequisites

Install required tools:

```bash
# Install chain-spec-builder
cargo install --force staging-chain-spec-builder

# Install polkadot-omni-node
cargo install --force --locked polkadot-omni-node
```

Build the runtime:

```bash
cargo build --release
```

## Quick Start

### Development with Balances

For local testing with pre-funded accounts:

```bash
./scripts/build-chain-spec.sh dev with-balances
```

Output: `./artifacts/dev_plain_balances.json`

### Production Chain

For Paseo testnet with `educhain.patch.json`:

```bash
./scripts/build-chain-spec.sh live
```

Outputs:

- `./artifacts/latest_plain_chain_spec.json` - Human-readable spec
- `./artifacts/latest_raw_chain_spec.json` - Encoded spec for validators
- `./artifacts/para-genesis-state` - Genesis state for relay registration
- `./artifacts/para-genesis-wasm` - Genesis runtime for relay registration
- Manual account configuration

### Production Configuration

Applies `educhain.patch.json`. Patch files can be used to define the state of certain pallets at genesis:

```json
{
  "collatorSelection": {
    "candidacyBond": 16000000000,
    "invulnerables": [
      "5Fzv6yDyfR5hZP83XGSEmWvCzqWH5gWPRZ5YNPWA7rAj8ZNu",
      "5DXMUyggiDhHhN4P7ZN4sXHu1VfBXcT2cz3cYLJe7sYe3SSR"
    ]
  },
  "session": {
    "keys": [
      [
        "5Fzv6yDyfR5hZP83XGSEmWvCzqWH5gWPRZ5YNPWA7rAj8ZNu",
        "5Fzv6yDyfR5hZP83XGSEmWvCzqWH5gWPRZ5YNPWA7rAj8ZNu",
        {
          "aura": "0x6656fbe8f31f1e4225e3e8adf5bbc16f0b0d54b0cf3a5d5f9eede6d1a6e7a581"
        }
      ],
      // Second collator...
    ]
  },
  "sudo": {
    "key": "5EqrDfW9HxBNfUjUiL8fNxRiDJ1ueKr5TdCVzCRgVqRFCvKV"
  },
  "parachainInfo": {
    "parachainId": 4883
  },
  "polkadotXcm": {
    "safeXcmVersion": 5
  }
}
```

## Detailed Usage

## Manual Commands

### Development with Balances

Create a development chain spec with pre-funded accounts (Alice, Bob, etc.):

```bash
chain-spec-builder \
  --chain-spec-path ./artifacts/dev_plain_balances.json \
  create \
  --relay-chain "paseo" \
  --para-id 4883 \
  --runtime target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm \
  named-preset development
```

### Production Chain

**Step 1: Generate plain chain spec**

Create a human-readable chain spec with your custom configuration:

```bash
chain-spec-builder \
  --chain-spec-path ./artifacts/latest_plain_chain_spec.json \
  create \
  --relay-chain "paseo" \
  --para-id 4883 \
  --runtime target/release/wbuild/educhain-runtime/educhain_runtime.compact.compressed.wasm \
  patch educhain.patch.json
```

> The metadata still has to be applied, which can be done manually, see: [Chain Spec Metadata](#chain-spec-metadata)

**Step 2: Convert to raw format**

Encode the chain spec for use by validators:

```bash
chain-spec-builder convert-to-raw ./artifacts/latest_plain_chain_spec.json && \
  mv chain_spec.json ./artifacts/latest_raw_chain_spec.json
```

This converts the plain spec and moves the output to `./artifacts/latest_raw_chain_spec.json`

**Step 3: Export genesis state**

Generate the genesis state needed for parachain registration:

```bash
polkadot-omni-node export-genesis-head \
  --chain ./artifacts/latest_raw_chain_spec.json \
  ./artifacts/para-genesis-state
```

**Step 4: Export genesis wasm**

Generate the runtime wasm needed for parachain registration:

```bash
polkadot-omni-node export-genesis-wasm \
  --chain ./artifacts/latest_raw_chain_spec.json \
  ./artifacts/para-genesis-wasm
```

## Configuration

The `educhain.patch.json` file contains:

- Parachain ID: 4883
- Collator invulnerables and session keys
- XCM safe version: 5
- Token: PAS (10 decimals)

### Chain Spec Metadata

**Production (live) networks:**

```json
{
  "name": "Educhain Paseo",
  "id": "live",
  "chainType": "Live",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "educhain-paseo-live",
  "para_id": 4883,
  "relay_chain": "paseo",
  "properties": {
    "tokenDecimals": 10,
    "tokenSymbol": "PAS"
  }
}
```

**Local testing networks:**

```json
{
  "name": "Educhain Paseo Local",
  "id": "local",
  "chainType": "Local",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "educhain-paseo-local",
  "para_id": 4883,
  "relay_chain": "paseo-local",
  "properties": {
    "tokenDecimals": 10,
    "tokenSymbol": "PAS"
  }
}
```

## Related Resources

- [Using pop CLI](pop-cli.md) - Local testing with pop CLI
- [Launch Documentation](launch.md) - Running your parachain
- [XCM Configuration](xcm.md) - Cross-chain messaging
