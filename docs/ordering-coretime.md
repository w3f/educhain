# Ordering Coretime

There are two types of coretime:

- Bulk coretime - rent computation for an amount of time in advance (i.e., 28 days), requires renewal.
- On-demand coretime - buy computation on-demand on a per block basis, requires sending an extrinsic to order a block.

For EduChain, we mostly use on-demand coretime and place order when we need to test something.

## Ordering on-demand coretime

### Ordering via CLI

You can use the [`polkadot-js-api`](https://www.npmjs.com/package/@polkadot/api-cli) to call any extrinsic, including on-demand coretime for your parachain.

Ensure you have it installed:

```sh
yarn global add @polkadot/api-cli
# or
npm -g install @polkadot/api-cli
```

Once installed, you can send the extrinsic as follows. Be sure to supplement a seed phrase with ROC ([faucet here](https://faucet.polkadot.io/)) and also replace `PARA_ID` with your parachain's ID on Rococo:

```sh
polkadot-js-api tx.onDemandAssignmentProvider.placeOrderAllowDeath \
1000000000000 \
PARA_ID \
--seed "your seed here" \
--ws "wss://rococo-rpc.polkadot.io"
```

### Ordering via PolkadotJS

Use the [following guides to order on-demand coretime through PolkadotJS.](https://wiki.polkadot.network/docs/learn-guides-coretime-parachains#run-a-parachain-with-on-demand-coretime)

## Ordering Bulk Coretime

Use the [following guides to order bulk coretime.](https://wiki.polkadot.network/docs/learn-guides-coretime-parachains#run-a-parachain-with-bulk-coretime)