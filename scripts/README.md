# EduChain Build Scripts

Generate chain specifications for EduChain using `chain-spec-builder` and `polkadot-omni-node`.

## Quick Start

```bash
# Development chain with funded accounts
./scripts/build-chain-spec.sh dev with-balances

# Production chain with patch
./scripts/build-chain-spec.sh live
```

## Prerequisites

```bash
# Install chain-spec-builder
cargo install --force staging-chain-spec-builder

# Install polkadot-omni-node
cargo install --force --locked polkadot-omni-node
```

## Chain Spec Types

### Development with Balances

Uses the `development` named preset with pre-funded accounts (Alice, Bob, Charlie, Dave, Eve, Ferdie).

**Output:** `artifacts/dev_plain_balances.json`

### Production

Applies `educhain.patch.json` for Paseo testnet deployment.

**Outputs:**
- `artifacts/latest_plain_chain_spec.json` - Human-readable spec
- `artifacts/latest_raw_chain_spec.json` - Encoded for validators
- `artifacts/para-genesis-state` - Genesis state for relay registration
- `artifacts/para-genesis-wasm` - Genesis runtime for relay registration

## Configuration

The `educhain.patch.json` file contains:
- Parachain ID: 4883
- Collator invulnerables and session keys
- XCM safe version: 4
- Token symbol: PAS (10 decimals)

## Testing

Test your chain spec locally with pop CLI:

```bash
pop up network -f ./pop-paseo-testnet-toml
```

## Resources

- [Building Chain Specs](../docs/build-chain-spec.md) - Detailed documentation
- [Using pop CLI](../docs/pop-cli.md) - Local testing
- [chain-spec-builder Guide](https://paritytech.github.io/polkadot-sdk/master/staging_chain_spec_builder/)
