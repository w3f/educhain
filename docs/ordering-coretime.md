# Ordering Coretime

Coretime is the time allocated for your parachain to execute blocks on the relay chain. There are two types:

*   **Bulk Coretime:** Rent computation for a fixed period (e.g., 28 days). Requires renewal.
*   **On-demand Coretime:** Buy computation per block. Requires sending an extrinsic to order a block.

Educhain primarily uses **on-demand coretime** for testing.

## Ordering On-demand Coretime

You can order coretime using the CLI or the PolkadotJS UI.

=== "CLI"

    You can use the [`polkadot-js-api`](https://www.npmjs.com/package/@polkadot/api-cli) to call the extrinsic.

    **1. Install the API CLI:**

    ```sh
    yarn global add @polkadot/api-cli
    # or
    npm -g install @polkadot/api-cli
    ```

    **2. Place an Order:**

    Replace `PARA_ID` with your parachain ID and provide a seed phrase with ROC/PAS ([faucet here](https://faucet.polkadot.io/)).

    ```sh
    polkadot-js-api tx.onDemand.placeOrderAllowDeath \
      1000000000000 \
      PARA_ID \
      --seed "your seed here" \
      --ws "wss://paseo.rpc.amforc.com"
    ```

    **3. Automate (Optional):**

    Run a loop to order blocks regularly:

    ```sh
    while :
    do
        polkadot-js-api tx.onDemand.placeOrderAllowDeath \
            1000000000000 \
            PARA_ID \
            --seed "your seed here" \
            --ws "wss://paseo.rpc.amforc.com"
        sleep 12
    done
    ```

=== "PolkadotJS UI"

    Follow the [official guide](https://wiki.polkadot.network/docs/learn-guides-coretime-parachains#run-a-parachain-with-on-demand-coretime) to order on-demand coretime via the UI.

## Ordering Bulk Coretime

Use the [following guides to order bulk coretime.](https://wiki.polkadot.network/docs/learn-guides-coretime-parachains#run-a-parachain-with-bulk-coretime)
