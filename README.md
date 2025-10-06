<div align="center">

# Web3 Educhain


<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only"/>

> EduChain is based on the [parachain template](https://github.com/paritytech/polkadot-sdk-parachain-template).
>
> This template is automatically updated after releases in the main [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk).

</div>

Parachain developed and maintained by Technical Education team at Web3 Foundation. To be used for creating
tutorials on a wide range of [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) enabled features.

## Getting Started

* 🦀 The repository is using the Rust language.

* 👉 Check the
[Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

* 🛠️ Depending on your operating system and Rust version, there might be additional
packages required to compile this repository - please take note of the Rust compiler output.

### Build

🔨 Use the following command to build the node without launching it:

```sh
cargo build --release
```

🐳 Alternatively, build the docker image:

```sh
docker build . -t polkadot-sdk-parachain-template
```


### Generate Chain Artifacts

We provide two scripts for generating chain specifications:

#### Development Chain Spec

Use `deploy/gen_base_chain_dev_spec.sh` to generate a development chain specification for local testing:

```sh
./deploy/gen_base_chain_dev_spec.sh
```

This script builds the project and generates a development chain spec with basic parameters suitable for local testing environments. The output file will be placed in `./artifacts/dev/dev_plain_balances.json`.

When using the development chain spec, you typically don't need to modify it manually, as it comes preconfigured with development defaults.

#### Production Chain Spec

Use `deploy/gen_base_chain_spec.sh` to generate a production-ready chain specification:

```sh
# Generate plain chain spec
./deploy/gen_base_chain_spec.sh

# Generate raw chain spec (must run the plain version first)
./deploy/gen_base_chain_spec.sh raw
```

This script generates a more configurable chain spec that applies patches from `educhain.patch.json`. The output file will be placed in `./artifacts/latest_plain_chain_spec.json`. When run with the `raw` parameter, it converts the plain chain spec to a raw format suitable for validators.

After generating the chain spec, you may need to modify certain metadata fields to match your deployment requirements:

**For production (live) networks**, ensure the chain spec includes the following metadata:

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
  },
  // ...remaining configuration...
}
```

**For local testing networks**, modify the metadata to reflect local settings:

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
  },
  // ...remaining configuration...
}
```

If you make runtime changes and want to generate a new chainspec, first run the appropriate script, then manually verify and edit these metadata fields to ensure they match your deployment target.

### Local Development Chain

This project uses [pop! CLI](https://github.com/r0gue-io/pop-cli) to run a complete local setup:

```sh
pop up network pop-paseo-testnet-toml
```

### Example Clients / Solutions

#### EduNews - Demo Article Verification System

EduNews is an educational prototype that illustrates a potential solution for verifying article authenticity and provenance using blockchain technology. This learning project combines multiple specialized chains within the Polkadot ecosystem (including EduChain):

  - [EduNews SubXT CLI](https://github.com/w3f/edunews-subxt)
  - [EduNews Vue App](https://github.com/w3f/edunews)

### Connect with the Polkadot-JS Apps Front-End

* 🌐 You can interact with your local node using the
hosted version of the Polkadot/Substrate Portal:
[relay chain](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
and [parachain](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9988).

Please note that if running locally, you must use the URLs given in the terminal when running `pop` (or Zombienet).

* 🪐 A hosted version is also
available on [IPFS](https://dotapps.io/).

* 🧑‍🔧 You can also find the source code and instructions for hosting your own instance in the
[`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository.

## Contributing

* 🔄 This template is automatically updated after releases in the main [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk).

* ➡️ Any pull requests should be directed to this [source](https://github.com/paritytech/polkadot-sdk/tree/master/templates/parachain).

* 😇 Please refer to the monorepo's
[contribution guidelines](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CONTRIBUTING.md) and
[Code of Conduct](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CODE_OF_CONDUCT.md).

## Getting Help

* 🧑‍🏫 To learn about Polkadot in general, [Polkadot.network](https://polkadot.network/) website is a good starting point.

* 🧑‍🔧 For technical introduction, [here](https://github.com/paritytech/polkadot-sdk#-documentation) are
the Polkadot SDK documentation resources.

* 👥 Additionally, there are [GitHub issues](https://github.com/paritytech/polkadot-sdk/issues) and
[Substrate StackExchange](https://substrate.stackexchange.com/).