# Sub0 Reset 2024 Workshop

## Launch your Appchain to Production

The journey of any production grade application always starts on a test network. Polkadot is privileged to have a high quality testnet: 
[Paseo Network](https://github.com/paseo-network), maintained by the community. Paseo network mirrors Polkadot and its system chain
runtimes, making it a play ground to test apps as well as appchains! 

## Who is the audience?

Anyone who wants to understand the process of launching their own chain on Polkadot and to leverage the powerful system chain integration options available. Some pre-requisite skills that help:

- familiarity with Polkadot JS UI
- have Substrate runtime development experience
- tinkered with [minimal chain](https://github.com/paritytech/polkadot-sdk-minimal-template) and [solochain](https://github.com/paritytech/polkadot-sdk-solochain-template) templates

## Tasks

- (Easy) [Register a ParaID](#register-a-paraid) on Paseo Network
- (Easy) Build the [parachain template](#build-parachain-template)

- (Intermediate) Build Chainspec with custom collator keys
- (Intermediate) Register genesis state and wasm blob of the parachain ()
- (Intermediate) Run collator nodes and set collator keys

After the parachain is onboarded

- (Easy) Produce the first parachain block on-demand

Integrations with System chains

- (Easy) [Establish a communication channel](#establish-hrmp-channel-with-asset-hub) with Paseo Asset Hub
- (Intermediate) Register the parachain native token as a foreign asset on Paseo Asset Hub

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

### Establish HRMP Channel with Asset Hub

- Check Educhain < > Asset Hub channel setup instructions [here](asset-hub.md#asset-hub-channel-setup)
- If you are not familiar with sending XCM calls from your parachain, you can view a live demo on Asset Hub channel setup [here](https://www.youtube.com/watch?v=4vq12vY0uYs&t=1445s)

### Establish HRMP Channel with Asset Hub

- Check Educhain < > Asset Hub channel setup instructions [here](asset-hub.md#asset-hub-channel-setup)
- If you are not familiar with constructing/sending XCM calls from your parachain, you can view a live demo on Asset Hub channel setup [here](https://www.youtube.com/watch?v=4vq12vY0uYs&t=1445s)

### Register Foreign Asset

- Check the instructions for the registration of parachain native token as a foreign asset [here](asset-hub.md#foreign-asset-registry)
- If you are not familiar with constructing/sending XCM calls from your parachain, you can view a live demo on Foreign Asset Registration [here](https://youtu.be/4vq12vY0uYs?si=JwPMBHKz1_njIZBc&t=1373)


