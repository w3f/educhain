# Web3 Educhain

Educating Web3 through [Polkadot SDK.](https://github.com/paritytech/polkadot-sdk)

!!! success "Live on Paseo"
    **Educhain On-demand Parachain is now live on Paseo!**  
    [View on Polkadot.js Apps](https://polkadot.js.org/apps/?rpc=wss://rpc.web3educhain.xyz:443#/explorer)

## Why Educhain?

When we started the [Web3 Educhain](https://github.com/w3f/educhain) project, there were around 50 parachains on the Polkadot network, but concrete guides on launching and maintaining a production-grade parachain were scarce. Tutorials demonstrating the [Polkadot SDK](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/index.html) features shipped with the [FRAME](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html) library were also limited.

Web3 Educhain addresses these gaps by providing insights, examples, and tutorials *alongside* documentation on how to launch and maintain a parachain.

!!! note "Unique Feature"
    Web3 EduChain utilizes the relay chain's token as its own, leveraging Polkadot's Cross-Consensus Messaging (XCM). If EduChain is deployed on Paseo, it uses **PAS**, bypassing the need to maintain a separate token.

## Why Polkadot?

Beyond efficient coretime and blockspace utility, a parachain deployed on Polkadot gains access to:

*   **Shared Security:** Multi-billion dollar crypto-economic security and finality from block one.
*   **Interoperability:** Secure communication with multiple chains within and outside the Polkadot ecosystem.
*   **Wasm Runtimes:** Customizable blockchain runtimes written in Rust and executed as Wasm code.
*   **Decentralization:** A robust network and networking stack designed with light clients in mind.
*   **Community:** A visionary and intellectual developer community.
*   **OpenGov:** A treasury that funds development initiatives and pays [OECD-grade salaries](https://polkadot-fellows.github.io/dashboard/#/membership) to the Technical Fellowship.

With **Agile Coretime**, parachains can produce blocks continuously or on-demand. By activating **Async Backing**, parachains can achieve 6-second block times with 20-60 second finality and [around 800 TPS](https://polkadot.network/blog/the-way-to-a-10x-throughput-lift-on-parachains/).

With [elastic scaling](https://github.com/paritytech/polkadot-sdk/issues/1829), parachain throughput will improve further by utilizing multiple cores simultaneously. In the near future, parachains can subscribe to the [JAM chain](https://graypaper.com/) service, enabling advanced features that make blockchain technology feasible for a wider range of use cases.

## Launch a Parachain

With the available templates, you can launch a production-grade blockchain in a day! 

[Get Started with Installation](./install.md){ .md-button .md-button--primary }
