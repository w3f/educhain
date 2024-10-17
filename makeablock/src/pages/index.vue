<template>
  <v-container class="order" align="center" justify="center">
    <h1>make blocks - paseo on demand</h1>
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
