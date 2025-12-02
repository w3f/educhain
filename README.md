<div align="center">

# Web3 Educhain


<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only"/>

> EduChain is based on the [parachain template](https://github.com/paritytech/polkadot-sdk-parachain-template).
>
> This template is automatically updated after releases in the main [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk).

</div>

Parachain developed and maintained by Technical Education team at Web3 Foundation. To be used for creating
tutorials on a wide range of [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) enabled features.

📚 **[View the full documentation at web3educhain.xyz](https://web3educhain.xyz/)**

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


### Building Chain Specifications

Generate chain specifications using `chain-spec-builder` and `polkadot-omni-node`.

**Quick Start:**

```bash
# Development with funded accounts (no need for reserve transfers)
./scripts/build-chain-spec.sh dev with-balances

# Production with educhain.patch.json
./scripts/build-chain-spec.sh live
```

**Outputs** (in `./artifacts`):

- `dev_plain_balances.json` - Dev spec with pre-funded accounts
- `latest_plain_chain_spec.json` - Production plain spec
- `latest_raw_chain_spec.json` - Production raw spec
- `para-genesis-state`, `para-genesis-wasm` - Genesis artifacts for relay registration

#### Chain Spec Metadata

The `educhain.patch.json` file contains production configuration for Paseo testnet (ParaID 4883).

**For production (live) networks:**

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

**For local testing networks:**

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

For detailed instructions, see [Building Chain Specs](./docs/build-chain-spec.md).

### Local Testing

Test locally using pop CLI:

```sh
pop up network -f ./pop-paseo-testnet-toml
```

See [Using pop CLI](./docs/pop-cli.md) for more details.

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

* 📚 **[EduChain Documentation](https://web3educhain.xyz/)** - Complete guides and tutorials

* 🧑‍🏫 To learn about Polkadot in general, [Polkadot.network](https://polkadot.network/) website is a good starting point.

* 🧑‍🔧 For technical introduction, [here](https://github.com/paritytech/polkadot-sdk#-documentation) are
the Polkadot SDK documentation resources.

* 👥 Additionally, there are [GitHub issues](https://github.com/paritytech/polkadot-sdk/issues) and
[Substrate StackExchange](https://substrate.stackexchange.com/).