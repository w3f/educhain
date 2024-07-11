# Dependencies

[The Wiki details how to install the necessary prerequisites](https://guide.kusama.network/docs/build-guides-install-deps/), including Rust and its tooling. You will need `cargo`, amongst other dependencies, before progressing.

## The "Omninode"

The `polkadot-parachain` can be used a universal collator instance for running most of the parachains (an "[omninode beta](https://forum.polkadot.network/t/polkadot-parachain-omni-node-gathering-ideas-and-feedback/7823)"), and can be installed follows:

```sh
cargo install --git https://github.com/paritytech/polkadot-sdk --tag polkadot-v1.10.0 --force polkadot-parachain-bin
```



> **You can change `--tag` to the specific release of your choice.**

Which will allow you to run the `polkadot-parachain`:

```sh
❯ polkadot-parachain --version
polkadot-parachain 1.10.0-7049c3c9883 # or, which ever release you cloned.
```

## Chain Spec Builder

The [`chain-spec-builder`](https://paritytech.github.io/polkadot-sdk/master/staging_chain_spec_builder/index.html) tool is used for building a chain specification from a Wasm binary. It can be installed as follows:

```sh
cargo install staging-chain-spec-builder
```

You may find more information in the link above and in the [Parity Devops documentation](https://paritytech.github.io/devops-guide/explanations/chainspecs.html):

```sh
❯ chain-spec-builder --help
A utility to easily create a chain spec definition

Usage: chain-spec-builder [OPTIONS] <COMMAND>

Commands:
  create          Create a new chain spec by interacting with the provided runtime wasm blob
  verify          Verifies the provided input chain spec
  update-code     Updates the code in the provided input chain spec
  convert-to-raw  Converts the given chain spec into the raw format
  list-presets    Lists available presets
  display-preset  Displays given preset
  help            Print this message or the help of the given subcommand(s)

Options:
  -c, --chain-spec-path <CHAIN_SPEC_PATH>
          The path where the chain spec should be saved [default: ./chain_spec.json]
  -h, --help
          Print help
```

## pop! CLI

[`pop-cli`](https://github.com/r0gue-io/pop-cli?tab=readme-ov-file#install) can be used to spin up a local development network with a parachain and relay chain setup, which can be configured in `devnet.toml` file inside `zombienet-config`.
