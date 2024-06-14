# Upgrading Your Runtime

> **⚠️ You need access to a *sudo* account to authorize and apply upgrades!**

Every parachain runtime (the code inside of `src/runtime/*`) can be boiled down to a single `.wasm` blob.  This blob is what the relay chain uses to validate state transitions from the parachain's collators.  Part of the reason behind this design was to enable [**forkless upgrades**](https://wiki.polkadot.network/docs/learn-runtime-upgrades), where essentially we can replace the WebAssembly blob with an upgraded blob, which contains the upgraded business logic of our runtime.

On solo chains, it's usually as easy as calling `system.setCode(new_wasm)`, which quite literally replaces the WebAssembly runtime with another within the storage layer of the node(s) and will be utilized after the extrinsic is executed.

On a parachain, the process is essentially *two* steps instead of *one* due to the involvement of the relay chain, which must be notified before you upgrade your parachain's code using the `system.authorizeUpgrade` extrinsic, then you can apply that upgrade using `system.applyAuthorizedUpgrade`.  This lets the relay chain know that:

1. An upgrade is going to commence, and it expects a new state transition function for validation
2. The upgrade / new code gets applied.


## Compiling your WASM Blob

Getting your WASM blob is as simple as compiling your runtime:

```sh
# Assuming you are in the root of your node
cargo build --release
```

Post compilation, your blob should be located within: `target/release/wbuild/educhain-runtime/educhain_runtime.compressed.wasm`, where `*_runtime` is the name of your chain.

## Obtaining your runtime hash

Since `system.authorizeUpgrade` requires a `Blake2b_256` hash of the runtime, [which you can get via this tool.](https://toolkitbay.com/tkb/tool/BLAKE2b_256).  You can also get the hash of the file via the `system.authorizeUpgrade` extrinsic in the PolkadotJS UI, and hash the file there.

## Upgrade Via PolkadotJS

1. With your hash, authorize the upgrade (`system.authorizeUpgrade`) (replace `HASH_HERE` with your new hash and `YOUR_RPC_HERE` with your RPC URL). Keep in mind this done on the parachain:

2. If you're using on-demand coretime, ensure you order a block accordingly.  You should see the upgrade being queued.

3. Call `system.applyAuthorizedUpgrade` and upload your *compressed* (i.e., `educhain_runtime.compressed.wasm`) WASM blob.  *Be sure to also order coretime if needed!*

## Upgrade Via Substrate Frontend

> **⚠️ You will need to [clone a modified version of the template](https://github.com/CrackTheCode016/substrate-front-end-template) in order to perform this upgrade!**
> **Change `src/config/development.json` to feature your node as well!**

This requires a few more steps, but you could also use the frontend template's upgrade function.  Remember to still use the pallet interactor to authorize the upgrade.

