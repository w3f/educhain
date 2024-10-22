<template>
  <v-container class="order" align="center" justify="center">
    <h1>Make Blocks On-demand on Paseo Network</h1>
    <a href="https://web3educhain.xyz/" target="_blank">Learn how to launch an on-demand parachain!</a>
    <a href="https://faucet.polkadot.io/" target="_blank">NEED $PAS? GO TO THE FAUCET!</a>
    <br />
    <AccountSelector />
    <Suspense>
      <OrderCoretime />
    </Suspense>
  </v-container>
</template>

<style lang="css">
.order {
  background-color: #171717;
}

a {
  color: white;
}
</style>
