# Using pop CLI

[pop CLI](https://github.com/r0gue-io/pop-cli) is used for local testing of EduChain.

## Installation

```bash
cargo install --force --locked pop-cli
```

## Local Testing

### Development Network (Recommended)

Launch with pre-funded development accounts (Alice, Bob, etc.):

```bash
pop up network -f ./pop-paseo-testnet-dev-toml
```

This uses `./artifacts/dev_plain_balances.json` which includes:
- Pre-funded accounts (Alice, Bob, Charlie, Dave, Eve, Ferdie)
- Alice as sudo
- Ready for immediate testing

### Production Network

Launch with production chain spec:

```bash
pop up network -f ./pop-paseo-testnet-toml
```

This uses `./artifacts/latest_plain_chain_spec.json` with your custom configuration from `educhain.patch.json`.

## Resources

- [pop CLI Documentation](https://learn.onpop.io)
- [Launching a Chain](https://learn.onpop.io/chains/guides/launch-a-chain)
