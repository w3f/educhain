# Sub0 Reset 2024 Workshop

## Launch your Appchain to Production

The journey of any production grade application always starts on a test network. Polkadot is privileged to have a high quality testnet: 
[Paseo Network](https://github.com/paseo-network), maintained by the community. Paseo network mirrors Polkadot and its system chain
runtimes, making it an ideal play ground for testing apps as well as appchains! 

## Who is the audience?

Anyone who wants to understand the process of launching their own chain on Polkadot and to leverage the powerful system chain integration options available. Some pre-requisite skills that help:

- familiarity with Polkadot JS UI
- have Substrate runtime development experience
- tinkered with [minimal chain](https://github.com/paritytech/polkadot-sdk-minimal-template) and [solochain](https://github.com/paritytech/polkadot-sdk-solochain-template) templates

## Having Trouble following the instructions?

As there are several moving parts, it is undertsandable that first time learners of Polkadot SDK can get a bit  overwhelmed. For
the code changes that need to be made to successfully complete this workshop,  you can find the repository 
[here](https://github.com/DrW3RK/polkadot-sdk-parachain-template/commit/7be19b80afa5c5df8c284f02ca50e22b76088510)

## Tasks

### Before attending the workshop

- (Easy) [Register a ParaID](#register-a-paraid) on Paseo Network
- (Easy) Build the [parachain template](#build-parachain-template)

- (Intermediate) Build [Chainspec](#chain-spec-with-genesis-state-and-wasm) with custom collator keys
- (Easy) Register genesis state and wasm blob of the parachain
- (Easy) Run collator nodes and set collator keys

### During the Workshop

After the parachain is onboarded

- (Easy) Produce the first parachain block on-demand

Integrations with System chains

- (Intermediate) [Establish a communication channel](#establish-hrmp-channel-with-asset-hub) with Paseo Asset Hub
- (Intermediate) Register the parachain native token as a foreign asset on Paseo Asset Hub

### After the Workshop

More tinkering to do!

- (Intermediate) Add a new pallet to the parachain runtime
- (Advanced) Perform a runtime upgrade

### Register a ParaID

- If you have not already, Create an account using any of the awesome [Polkadot wallets](https://polkadot.com/get-started/wallets) for browsers.
- Get the PAS tokens from the [Polkadot Faucet](https://faucet.polkadot.io/).
- Navigate to [Parachains tab on Polkadot JS UI](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpaseo.rpc.amforc.com#/parachains/parathreads) on Paseo Network and click on ParaID and do the needful to register a unique `ParaID`

### Build Parachain Template

- Setup the [Polkadot Parachain template](https://github.com/paritytech/polkadot-sdk-parachain-template) repository
- Follow the installation instructions and build the parachain template
- Explore all the commands available `./target/release/parachain-template-node --help`
- Start the parachain node locally with `./target/release/parachain-template-node --dev` and connect to it at the endpoint `ws://127.0.0.1:9944` using Polkadot JS UI. No blocks are produced in this mode.

![Connect to Parachain Template Node](./img/template/parachain-template-dev.png)

### Chain Spec with Genesis State and Wasm

The Genesis state paired with Wasm (State Transition function) contains all the info that is needed for a node to start producing blocks. It includes the information that uniquely identifies the Parachain (ParaID), relay chain, collator keys etc.

Although, dev keys like Alice, Bob etc. help with tinkering with the node, it is important to remove them and work with the keys generated
by you. The genesis state can be used include accounts with pre-minted balance, accounts with root (sudo) privileges, collator identity
and session keys etc. In case of chain migrations, the genesis could be the snapshot of the entire chain state. 

For the workshop you can work on

- Adding an account or multiple accounts with funds
- Adding at least one account with root privileges to issue sudo calls
- [Adding at least one pair of collator keys (identity and session)](collator.md#setting-up-collators-in-the-chain-spec)

Do not make the mistake of reusing the collator identity keys for session keys. The session keys are placed on a hot wallet and are 
[rotated often](collator.md#changing--rotating-session-keys). The collator identity key remains the same. That key is the identifier for 
the node that produces a block. 

After defining a live chain config in `chain_spec.rs` file, generate plain and raw parachain specs using the commands below, followed by
commands to generate genesis state and wasm. 4540 was the ParaID used to create a parachain for Sub0 Reset following the instructions on
this document.

`./target/release/parachain-template-node build-spec --disable-default-bootnode --chain live  > plain-parachain-chainspec.json`

`./target/release/parachain-template-node build-spec --chain plain-parachain-chainspec.json --disable-default-bootnode --raw > raw-parachain-chainspec.json`

`./target/release/parachain-template-node export-genesis-state --chain raw-parachain-chainspec.json para-4540-genesis-state`

`./target/release/parachain-template-node export-genesis-wasm --chain raw-parachain-chainspec.json para-4540-wasm`


### Register Genesis State and Wasm

Navigate to [Parachains tab on Polkadot JS UI](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpaseo.rpc.amforc.com#/parachains/parathreads) on Paseo Network and click on ParaID and do the needful to register Genesis state and wasm.

It takes two hours for the parachain to be on-boarded.

![Parachain Onboarding](./img/template/parachain-template-dev.png)


### Set Collator Session Keys

If you try to start are a collator without ssetting the keys in the storage, you will be presented with the following options:

```
Starting an authorithy without network key in ./sub0-reset/chains/live/network/secret_ed25519.
      
       This is not a safe operation because other authorities in the network may depend on your node having a stable identity.
      
       Otherwise these other authorities may not being able to reach you.
      
       If it is the first time running your node you could use one of the following methods:
      
       1. [Preferred] Separately generate the key with: <NODE_BINARY> key generate-node-key --base-path <YOUR_BASE_PATH>
      
       2. [Preferred] Separately generate the key with: <NODE_BINARY> key generate-node-key --file <YOUR_PATH_TO_NODE_KEY>
      
       3. [Preferred] Separately generate the key with: <NODE_BINARY> key generate-node-key --default-base-path
      
       4. [Unsafe] Pass --unsafe-force-node-key-generation and make sure you remove it for subsequent node restarts
```

Below is a hacky way to start the collator without a session key in its key store, with the `--unsafe-force-node-key-generation` flag and 
then use Polkadot JS UI to set the collator keys via an RPC call to the collator node. Follow the instructions on configuring the collator keys [here](collator.md#configuring-and-running-your-collator)

```
./target/release/parachain-template-node \
    --collator \
    --force-authoring \
    --chain raw-parachain-chainspec.json \
    --base-path ./data \
    --port 40333 \
    --rpc-port 8844 \
    --unsafe-force-node-key-generation\
    -- \
    --execution wasm \
    --chain paseo.raw.json \
    --port 30343 \
    --rpc-port 9977 \
    --sync fast-unsafe
```

This should start syncing Paseo relaychain which can take several hours.


### Establish HRMP Channel with Asset Hub

- Check Educhain < > Asset Hub channel setup instructions [here](asset-hub.md#asset-hub-channel-setup)
- If you are not familiar with sending XCM calls from your parachain, you can view a live demo on Asset Hub channel setup [here](https://www.youtube.com/watch?v=4vq12vY0uYs&t=1445s)

### Register Foreign Asset

- Check the instructions for the registration of parachain native token as a foreign asset [here](asset-hub.md#foreign-asset-registry)
- If you are not familiar with constructing/sending XCM calls from your parachain, you can view a live demo on Foreign Asset Registration [here](https://youtu.be/4vq12vY0uYs?si=JwPMBHKz1_njIZBc&t=1373)


