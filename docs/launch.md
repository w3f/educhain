# Launching on Paseo

[OpenZeppelin Polkadot Generic Runtime Template](https://github.com/OpenZeppelin/polkadot-runtime-templates) 
was chosen as a candidate for launching Educhain. They crafted the pallet list to be as minimalistic and 
preserved important pallets that are used in the Polkadot ecosystem. Their [docs](https://docs.openzeppelin.com/substrate-runtimes/1.0.0/)
along with [POP network docs](https://learn.onpop.io/v/appchains/guides/launching-your-parachain-on-polkadot/launching-on-paseo) serve as a good starting point to understand the launch process of a parachain on Polkadot testnet Paseo.

## Personalize Template

In a single shot, you can (almost) rebrand the runtime template to your own project by replacing 
the occurrences of `parachain-runtime` and `parachain_runtime` with your project name.

You may also add, remove, or create new pallets and add them to the runtime.  Adding new pallets involves correctly configuring their associated types and ensuring they are a part of the `construct_runtime!` macro.

You can view more about [how pallets work](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html#pallets) in the Polkadot SDK docs.

## Set Genesis Config

It is common that most of the tutorials and templates use Alice or Bob keys for the root, collator and session keys. It is obvious that these should be replaced by custom keys. It is 
recommended that you use an account created on a cold wallet for the root account. It is also important
for the collator key and its session key to be different. The collator key is recommended to be created on a 
cold wallet and the session key can be generated from a hot wallet, as you need to enter its seed/private key
into the collator's key store to start producing blocks. The session keys are rotated often for this reason.

The genesis config can be used to also configure the initial state of your pallets.  For more information on the genesis config, [see these Polkadot SDK docs.](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/chain_spec_genesis/index.html) For generating a chain spec, you can either view our process here or refer to the [chain spec buider docs.](https://paritytech.github.io/polkadot-sdk/master/staging_chain_spec_builder/index.html)

## Collator Node Setup 

You can spin up a collator on your local machine or on a cloud instance of your choice. Educhain node runs as 
a [Digital Ocean droplet](https://www.digitalocean.com/pricing/droplets). 

If you like to interact with your collator through the [Polkadot JS UI](https://polkadot.js.org/), the connection needs to be secured via SSL. This requires securing a domain (such as web3educhain.xyz) and then setting up SSL to access your server instance for the RPC address to be able to interact with it using Polkadot JS UI. Securing via SSL requires a proxy setup for the RPC interfaces of your node.

See [the collator section](./collator.md) to learn more about collator node setup and launch.

## Local Development

To run and test Educhain locally, `pop-cli` can be used (install it [here](https://github.com/r0gue-io/pop-cli)).  Once installed, you can customize the Zombienet configuration included. This will spawn a relay chain node and parachain collator nodes, which can be accessed through Polkadot JS UI:

```sh
pop up parachain -f ./zombienet-config/devnet.toml
```

## Block Production

Educhain is deployed on Rococo as an on-demand parachain. Transactions can be submitted to the collator node 
and they enter the transaction pool. When you like to execute these transactions, an order can be placed on 
Rococo relay chain for the parachain block production and validation through `onDemandAssignmentProvider` 
pallet call. If you intend to have a parachain that produces blocks continously, you can purchase bulk 
coretime through awesome interfaces from [Lastic](https://www.lastic.xyz/) or [RegionX](https://app.regionx.tech/).

For more details and tips on ordering coretime, see [the ordering coretime page](./ordering-coretime.md).

