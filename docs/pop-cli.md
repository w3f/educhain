# Using pop CLI

[pop CLI](https://github.com/r0gue-io/pop-cli) is used for local testing of EduChain.

## Installation

```bash
cargo install --force --locked pop-cli
```

## Local Testing

Launch a local testnet with relay chain and parachain:

```bash
pop up network -f ./pop-paseo-testnet-toml
```

This command spawns a local Paseo relay chain and launches EduChain as a parachain.

## Resources

- [pop CLI Documentation](https://learn.onpop.io)
- [Launching a Chain](https://learn.onpop.io/chains/guides/launch-a-chain)
