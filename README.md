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


### Generate Artifacts for Deployment

We use [pop! CLI](https://github.com/r0gue-io/pop-cli) to generate the artifacts: 

```sh
pop build spec \
  --output ./artifacts/latest_raw_chain_spec.json \
  --chain ./artifacts/latest_plain_chain_spec.json \
  --genesis-state \
  --genesis-code \
  --type live
```

If you make runtime changes and want to generate a new chainspec + the patch, use the `deploy/gen_base_chain_spec.sh` script, then run the above to generate the artifacts. Keep in mind you may have to change some details, such as the name, protocol ID, etc to its original state: 

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
  ...
}
```

And for the local configuration: 

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
  ...
}
```

### Local Development Chain

This project uses [pop! CLI](https://github.com/r0gue-io/pop-cli) to run a complete local setup:

```sh
pop up network pop-paseo-testnet-toml
```

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