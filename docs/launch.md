# Launching on Rococo

[OpenZepppelin Polkadot Generic Runtime Template](https://github.com/OpenZeppelin/polkadot-runtime-templates) 
was chosen as a candidate for launching Educhain. They crafted the pallet list to be as minimalistic and 
preserved important pallets that are used in the Polkadot ecosystem. Their [docs](https://docs.openzeppelin.com/substrate-runtimes/1.0.0/)
serve as a good starting point to understand the launch process of a parachain on Polkadot testnet Rococo.

## Personalize Template

In a single shot, you can (almost) rebrand the runtime template to your own project by replacing 
the occurrences of `parachain-runtime` and `parachain_runtime` with your project name.

## Set Genesis Config

It is common that most of the tutorials and templates use Alice or Bob keys for the root account, and for
collator's keys and session keys. It is obvious that these should be replaced by custom keys. It is 
recommended that you use an account created on a cold wallet for the root account. It is also important
for the collator key and its session key to be different. The collator key is recommended to be created on a 
cold wallet and the session key can be generated from a hot wallet, as you need to enter its seed/private key
into the collator's key store to start producing blocks. The session keys are rotated often for this reason.

## Collator Node Setup 

You can spin up a collator on your local machine or on a cloud instance of your choice. Educhain node runs as 
a [Digital Ocean droplet](https://www.digitalocean.com/pricing/droplets). If you like to interact with your
collator through [Polkadot JS UI](https://polkadot.js.org/), the connection needs to be secure and hence not
possible to just use the ip address. We had to secure a domain for Educhain and then setup a custom domain 
for the rpc address to be able to interact with it using Polkadot JS UI. The 
[Substrate Front-end template](https://github.com/substrate-developer-hub/substrate-front-end-template) does
not have this restriction, if you like to try it out.

## Block Production

Educhain is deployed on Rococo as an on-demand parachain. Transactions can be submitted to the collator node 
and they enter the transaction pool. When you like to execute these transactions, an order can be placed on 
Rococo relay chain for the parachain block production and validation through `onDemandAssignmentProvider` 
pallet call. If you intend to have a parachain that produces blocks continously, you can purchase bulk 
coretime through awesome interfaces from [Lastic](https://www.lastic.xyz/) or [RegionX](https://app.regionx.tech/).

