# Collator Setup

A collator is a type of node that produces blocks for a parachain.  These blocks are not finalized; that's the job of the relay chain.  As such, collation is not a secure task, a parachain only really needs one honest collator (honest as in, providing the correct information regarding its the state transitions within) to finalize with the relay chain.

A collator could also hold the role of an RPC node, which may be fine for testing, but is not ideal for production.  Ideally, one would have a set of dedicated collators (often acting as bootnodes), and a set of RPC nodes *separately*.

> In production, it is recommended to setup your collators and RPCs on separate machines, however for testing, it is entirely possible to allow for a single collator to also provide RPC capabilities.

If you wish to test an IP-only setup, then either Polkadot JS or the [Substrate frontend template](https://github.com/substrate-developer-hub/substrate-front-end-template) can be run locally, allowing insecure connections to be accessed.

## Setting up collators in the chain spec

If you are using the parachain template, you can immediately add collators into [`src/node/chain_spec.rs`](https://github.com/w3f/educhain/blob/main/node/src/chain_spec.rs).  You may configure:

- The initial *public* session key(s).
- The collator(s) public keys which are used for rewards.

As an example, EduChain sets **two** initial collation and session public keys, allowing the chain to hit the ground running with two collators other than Bob or Alice:

```rust
// Collator accounts that produce blocks and earn rewards.
pub const COLLATOR1: &str = "0x38a2edbf7cd629e10700376f941122bf6c6a7b705bb70d6eb15359099055015b";
pub const COLLATOR2: &str = "0x3090de03bda721f91d4ea242c63c4220832194e63d2c5b61dbcbdd458224350f";

// The private key of these session keys needs to be inserted into the collator node for it to start
// producing blocks.
pub const SESSION1: &str = "0x1e0f4e48f26d802ce3699872c97e2ec7f8476a9b27a5d4307986ce0ddf0d8530";
pub const SESSION2: &str = "0x1e673715db64783eadc6ca927e493ded30f2447efff0f6d5d84578e823f86374";
```

> Please note that the session keys are initial, and should ideally be rotated every once in a while for security purposes (so no one can impersonate your collator and make blocks on your behalf).

Assuming you are using the [`collator-section`](https://paritytech.github.io/polkadot-sdk/master/pallet_collator_selection/index.html) pallet (which is the default in most templates), there are two concepts you will see: 

- **Candidates** - Candidates for collation, which may or may not be selected.  A bond is required to participate.
- **Invulnerables** - An account that is *guaranteed* to be participating in block production, bond or not.  They will participate round-robin style in accordance to Aura.

Both invulnerables and candidates can be added to a running chain.  **Invulnerables**, however, are usually specified in the chain spec as the "bootnodes".  It is wise to add at least one collator in your chain_spec - and one that you can start easily, that way you can always gurantee a collator that can produce good, honest blocks.

> For testing, one may use a well-known account, such as `Alice` or `Bob`, however this is not good for production. If one does use `Alice`, for example, then they can run their collator with `--alice` as a start-up flag.

## Running a "bootnode" collator

To run a bootnode collator, one just needs to make sure that: 

- The collator is synced with the relay chain (a local copy is needed, pruning is *highly* recommended)
- The corresponding private key of the session key (for aura) in the chain spec is inserted, either through rpc or through (`polkadot-parachain key insert`)

Check out this sample command, which runs a parachain collator using the `polkadot-parachain` binary:

```sh
polkadot-parachain --name COLLATOR_NAME \
--collator \
--chain plain-parachain-chainspec.json \
--base-path ./educhain \
--rpc-cors=all \
--port 30333 \
--rpc-port 8844 \
-- \
--chain rococo \
--sync fast-unsafe \
--blocks-pruning 256
--state-pruning 256
```

> Note that a few of the arguments, such as `--name`, `--chain`, `--base-path` should be substituted with your own collator name, chain spec, and base path accordingly.

Once your collator has synced with its respective relay-chain, and as long as you have coretime (either bulk or on-demand) then your collator should be making blocks.

### `systemd` and Collators

Once you've achieved a stable setup, you can look into automating the launch of your collator upon startup. Here is an example of a service which automatically starts a shell script using `systemd` on Ubuntu Linux:

- `start.node.sh`:
```sh
polkadot-parachain --name C2_EDU \
--collator \
--chain plain-parachain-chainspec.json \
--base-path ./educhain \
--rpc-cors=all \
--port 30333 \
--rpc-port 8844 \
-- \
--chain rococo \
--sync fast-unsafe \
--blocks-pruning 256
```

- `collator.service`:
```ini
[Unit]
Description=Collator for EduChain
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/bin/sh start.node.sh
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

This way, now we can manage our elegantly collator with `systemctl`:

- `systemctl restart collator ` - Restarts the collator
- `systemctl stop collator ` - Stops the collator
- `systemctl start collator ` - Starts the collator
- `systemctl status collator ` - Retrieves and displays the status of the collator
  
Here is an example of how a functioning collator looks in action, via the status command: 

```sh
root@ubuntu-s-2vcpu-4gb-amd-nyc3-01:~# systemctl status collator
● collator.service - Collator for EduChain
     Loaded: loaded (/usr/lib/systemd/system/collator.service; disabled; preset: enabled)
     Active: active (running) since Wed 2024-06-05 18:02:23 UTC; 5 days ago
   Main PID: 132465 (sh)
      Tasks: 46 (limit: 4658)
     Memory: 2.1G (peak: 2.5G)
        CPU: 8h 23min 34.532s
     CGroup: /system.slice/collator.service
             ├─132465 /bin/sh start.node.sh
             └─132466 polkadot-parachain --name C2_EDU --collator --chain plain-parachain-chainspec.json --base-path ./educhain --rpc-cors=all>

Jun 10 20:48:00 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:00 [Relaychain] ♻️  Reorg on #10834776,0xf566…ab31 to #10834776,0x1>
Jun 10 20:48:00 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:00 [Relaychain] ✨ Imported #10834776 (0x1583…c83f)
Jun 10 20:48:05 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:05 [Parachain] 💤 Idle (0 peers), best: #153 (0x998c…e0d8), finali>
Jun 10 20:48:05 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:05 [Relaychain] 💤 Idle (15 peers), best: #10834776 (0x1583…c83f),>
Jun 10 20:48:06 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:06 [Relaychain] ✨ Imported #10834777 (0x705d…63cb)
Jun 10 20:48:10 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:10 [Parachain] 💤 Idle (0 peers), best: #153 (0x998c…e0d8), finali>
Jun 10 20:48:10 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:10 [Relaychain] 💤 Idle (15 peers), best: #10834777 (0x705d…63cb),>
Jun 10 20:48:12 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:12 [Relaychain] ✨ Imported #10834778 (0x7a52…93f6)
Jun 10 20:48:15 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:15 [Parachain] 💤 Idle (0 peers), best: #153 (0x998c…e0d8), finali>
Jun 10 20:48:15 ubuntu-s-2vcpu-4gb-amd-nyc3-01 sh[132466]: 2024-06-10 20:48:15 [Relaychain] 💤 Idle (15 peers), best: #10834778 (0x7a52…93f6),>
```
