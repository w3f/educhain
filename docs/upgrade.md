# Upgrading Your Runtime

!!! danger "Sudo Required"
    **You need access to a *sudo* account to authorize and apply upgrades!**

Every parachain runtime (the code in `src/runtime/*`) compiles to a single `.wasm` blob. The relay chain uses this blob to validate state transitions. "Forkless upgrades" simply mean replacing this WebAssembly blob with a new one.

## The Parachain Upgrade Process

On a solo chain, you just call `system.setCode`. On a parachain, it's a **two-step process** because the relay chain must be notified:

1.  **Authorize:** `system.authorizeUpgrade` - Notify the relay chain of the impending upgrade (requires the hash of the new runtime).
2.  **Apply:** `system.applyAuthorizedUpgrade` - Upload the new code.

## 1. Compile your WASM Blob

Compile your runtime in release mode:

```sh
cargo build --release
```

The artifact will be at: `target/release/wbuild/educhain-runtime/educhain_runtime.compressed.wasm`

## 2. Obtain Runtime Hash

You need the `Blake2b_256` hash of your new runtime file.

*   **Option A:** Use [this online tool](https://toolkitbay.com/tkb/tool/BLAKE2b_256).
*   **Option B:** Upload the file in PolkadotJS UI (under `system.authorizeUpgrade`) to see the hash, then copy it.

## 3. Perform the Upgrade

=== "PolkadotJS UI"

    1.  **Authorize:** Call `system.authorizeUpgrade` with your new runtime hash.
        *   *Note: This is done on the parachain.*
    2.  **Wait:** If using on-demand coretime, order a block to process the transaction. You should see the upgrade queued.
    3.  **Apply:** Call `system.applyAuthorizedUpgrade` and upload your **compressed** WASM blob (`educhain_runtime.compressed.wasm`).
        *   *Don't forget to order coretime if needed!*

=== "Substrate Frontend"

    !!! warning "Prerequisites"
        *   Clone the [modified frontend template](https://github.com/CrackTheCode016/substrate-front-end-template).
        *   Update `src/config/development.json` with your node details.

    You can use the frontend template's upgrade function, but remember to still use the pallet interactor to **authorize** the upgrade first.

