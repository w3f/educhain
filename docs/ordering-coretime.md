# Ordering Coretime

There are two types of coretime:

- Bulk coretime
- On-demand coretime

## Ordering on-demand coretime

### Ordering via CLI

```sh
polkadot-js-api tx.onDemandAssignmentProvider.placeOrderAllowDeath \
1000000000000 \
4428 \
--seed "your seed here" \
--ws "wss://rococo-rpc.polkadot.io"
```

### Ordering via PolkadotJS

Use the [following guides to order on-demand coretime through PolkadotJS.](https://wiki.polkadot.network/docs/learn-guides-coretime-parachains#run-a-parachain-with-on-demand-coretime)

## Ordering Bulk Coretime

Use the [following guides to order bulk coretime.](https://wiki.polkadot.network/docs/learn-guides-coretime-parachains#run-a-parachain-with-bulk-coretime)