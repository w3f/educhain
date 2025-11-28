# Installation & Prerequisites

Before you start building, you'll need to set up your development environment.

## 1. Basic Dependencies

First, ensure you have the necessary prerequisites installed, including Rust and its tooling.

[Polkadot SDK Installation Guide](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/){ .md-button }

You will need `cargo` and other standard development tools.

## 2. The Omninode

The `polkadot-omni-node` is a universal collator instance that can run most parachains. Install it using Cargo:

```sh
cargo install polkadot-omni-node
```

## 3. Chain Spec Builder

The [`chain-spec-builder`](https://paritytech.github.io/polkadot-sdk/master/staging_chain_spec_builder/index.html) tool is essential for creating a chain specification from a Wasm binary.

```sh
cargo install staging-chain-spec-builder
```

For more details, refer to the [Polkadot Developer Documentation](https://docs.polkadot.com/develop/parachains/deployment/generate-chain-specs/).

## 4. pop! CLI

The [`pop-cli`](https://github.com/r0gue-io/pop-cli) is a powerful tool to spin up local development networks with a parachain and relay chain setup.

=== "Cargo"

    ```sh
    cargo install --force --locked pop-cli
    ```

=== "Binary"

    Check the [pop-cli releases page](https://github.com/r0gue-io/pop-cli/releases) for pre-built binaries.

Once installed, you're ready to launch your chain!

[Launch Your Chain](./launch.md){ .md-button .md-button--primary }
