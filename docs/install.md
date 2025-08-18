# Dependencies

[The Polkadot Developer Documentation details how to install the necessary prerequisites](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/), including Rust and its tooling. You will need `cargo`, amongst other dependencies, before progressing.

## The Omninode

The `polkadot-omni-node` can be used a universal collator instance for running most of the parachain and can be installed follows:

```sh
cargo install polkadot-omni-node
```

## Chain Spec Builder

The [`chain-spec-builder`](https://paritytech.github.io/polkadot-sdk/master/staging_chain_spec_builder/index.html) tool is used for building a chain specification from a Wasm binary. It can be installed as follows:

```sh
cargo install staging-chain-spec-builder
```

You may find more information in the link above and in the [Polkadot Developer Documentation](https://docs.polkadot.com/develop/parachains/deployment/generate-chain-specs/).

## pop! CLI

[`pop-cli`](https://github.com/r0gue-io/pop-cli?tab=readme-ov-file#install) can be used to spin up a local development network with a parachain and relay chain setup.
