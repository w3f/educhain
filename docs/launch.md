# Launching on Paseo

EduChain is modeled after the [Polkadot SDK Parachain Template](https://github.com/paritytech/polkadot-sdk-parachain-template). It is an ecosystem-standard template maintained by Parity and includes the necessary configuration to deploy to a relay chain.

!!! tip "Zero to Hero"
    The [Polkadot Developer Documentation offers an end-to-end tutorial](https://docs.polkadot.com/tutorials/polkadot-sdk/parachains/zero-to-hero/) for developing and launching a parachain on the [Paseo Testnet](https://github.com/paseo-network).

## Personalize Template

You can rebrand the runtime template to your own project by replacing occurrences of `parachain-runtime` and `parachain_runtime` with your project name.

### Adding Pallets

You can add, remove, or create new pallets. Adding new pallets involves:

1.  Configuring their associated types in [`runtime/src/configs/mod.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/mod.rs).
2.  Adding the pallet to the runtime in the [`#[frame_support::runtime]`](https://github.com/w3f/educhain/blob/main/runtime/src/lib.rs#L245) macro:

```rust
#[runtime::pallet_index(0)]
pub type System = frame_system;
```

For more details, see [how pallets work](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html#pallets) in the Polkadot SDK docs.

## Generating Build Artifacts

### Genesis Configuration

The genesis config sets the initial state of your pallets. See the [Polkadot SDK docs](https://docs.polkadot.com/develop/parachains/intro-polkadot-sdk/#frame) for more info.

!!! warning "Security Notice"
    Most tutorials use Alice or Bob keys for root, collator, and session keys. **Always replace these with custom keys for production.**
    
    *   **Root Account:** Use a cold wallet.
    *   **Collator Key:** Use a cold wallet.
    *   **Session Key:** Can be generated from a hot wallet (rotated often).

### Streamlining with pop! CLI

For an all-inclusive solution, use [pop! CLI](https://github.com/r0gue-io/pop-cli). It handles:
*   [Deployment](https://learn.onpop.io/chains/guides/launch-a-chain)
*   [Local networks](https://learn.onpop.io/chains/guides/launch-a-chain/running-your-parachain)
*   [Runtime upgrades](https://learn.onpop.io/chains/guides/test-runtime-upgrades)
*   [Generating chain specifications, WASM, and genesis state](https://learn.onpop.io/chains/guides/launch-a-chain/launch-a-chain-to-paseo#generate-the-chain-spec)

## Collator Node Setup 

You can spin up a collator on your local machine or a cloud instance. Educhain nodes run as [Digital Ocean droplets](https://www.digitalocean.com/pricing/droplets).

To interact with your collator via [Polkadot JS UI](https://polkadot.js.org/), you need a secure connection (SSL). This requires:
1.  A domain (e.g., `web3educhain.xyz`).
2.  SSL setup with a proxy for the RPC interfaces.

[See the Collator Setup Guide](./collator.md){ .md-button }

## Local Development

To run and test Educhain locally, use `pop-cli`. You can customize the included Zombienet configuration to spawn a relay chain node and parachain collator nodes:

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

