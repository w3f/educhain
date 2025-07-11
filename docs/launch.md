# Launching on Paseo

EduChain is modelled after the [Polkadot SDK Parachain Template](https://github.com/paritytech/polkadot-sdk-parachain-template). It is a ecosystem-standard template maintained by Parity and includes the necessary configuration to deploy to a relay chain.

The [Polkadot Developer Documentation offers an end-to-end tutorial](https://docs.polkadot.com/tutorials/polkadot-sdk/parachains/zero-to-hero/) for developing and launching a parachain on the [Paseo Testnet](https://github.com/paseo-network).

## Personalize Template

In a single shot, you can (almost) rebrand the runtime template to your own project by replacing 
the occurrences of `parachain-runtime` and `parachain_runtime` with your project name.

You may also add, remove, or create new pallets and add them to the runtime.  Adding new pallets involves correctly configuring their associated types in [`runtime/src/configs/lib.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/mod.rs), then adding the pallet as part of the runtime in the [`#[frame_support::runtime]`](https://github.com/w3f/educhain/blob/main/runtime/src/lib.rs#L245) macro, added in the following manner: 

```rust
#[runtime::pallet_index(0)]
pub type System = frame_system;
```

You can view more about [how pallets work](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html#pallets) in the Polkadot SDK Rust docs.

## Generating Build Artifacts

### Genesis Configuration

The genesis config can be used to also configure the initial state of your pallets.  For more information on the genesis config, [see these Polkadot SDK docs.](https://docs.polkadot.com/develop/parachains/intro-polkadot-sdk/#frame) For generating a chain spec, you can either view our process here or refer to the [chain spec builder docs.](https://docs.polkadot.com/develop/parachains/deployment/generate-chain-specs/).

It is common that most of the tutorials and templates use Alice or Bob keys for the root, collator and session keys. It is obvious that these should be replaced by custom keys. It is recommended that you use an account created on a cold wallet for the root account. 

It is also important for the collator key and its session key to be different. The collator key is recommended to be created on a cold wallet and the session key can be generated from a hot wallet, as you need to enter its seed/private key into the collator's key store to start producing blocks. The session keys are rotated often for this reason.

### Streamlining the process with pop! CLI

For an all-inclusive solution, you may also use [pop! CLI](https://github.com/r0gue-io/pop-cli), which takes care of [deployment](https://learn.onpop.io/chains/guides/launch-a-chain), [local networks](https://learn.onpop.io/chains/guides/launch-a-chain/running-your-parachain) and [runtime upgrades](https://learn.onpop.io/chains/guides/test-runtime-upgrades), [generating chain specifications, WASM, and genesis state](https://learn.onpop.io/chains/guides/launch-a-chain/launch-a-chain-to-paseo#generate-the-chain-spec) in one tool.

## Collator Node Setup 

You can spin up a collator on your local machine or on a cloud instance of your choice. Educhain node runs as 
a [Digital Ocean droplet](https://www.digitalocean.com/pricing/droplets). 

If you like to interact with your collator through the [Polkadot JS UI](https://polkadot.js.org/), the connection needs to be secured via SSL. This requires securing a domain (such as web3educhain.xyz) and then setting up SSL to access your server instance for the RPC address to be able to interact with it using Polkadot JS UI. Securing via SSL requires a proxy setup for the RPC interfaces of your node.

See [the collator section](./collator.md) to learn more about collator node setup and launch.

## Local Development

To run and test Educhain locally, `pop-cli` can be used (install it [here](https://github.com/r0gue-io/pop-cli)).  Once installed, you can customize the Zombienet configuration included. This will spawn a relay chain node and parachain collator nodes, which can be accessed through Polkadot JS UI:

```sh
pop up network -f ./pop-paseo-testnet-toml
```

## Block Production

Educhain is deployed on Rococo as an on-demand parachain. Transactions can be submitted to the collator node 
and they enter the transaction pool. When you like to execute these transactions, an order can be placed on 
Rococo relay chain for the parachain block production and validation through `onDemandAssignmentProvider` 
pallet call. 

- To streamline the on-demand ordering process, a tool like [`ondemand`](https://github.com/CrackTheCode016/ondemand) can be used.

- If you intend to have a parachain that produces blocks continuously, you can purchase bulk 
coretime through an interface like [RegionX](https://app.regionx.tech/).

For more details and tips on ordering coretime, see [the ordering coretime page](./ordering-coretime.md).

