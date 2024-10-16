<template>
  <v-app>
    <v-main>
      <div v-if="state.$state.extensionLoaded">
        <router-view />
      </div> 
      <div v-else>
        <h2>It appears that you don't have the Polkadot JS browser extension, install it <a
            href="https://polkadot.js.org/extension/" target="_blank">here</a></h2>
      </div>
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
  import { useAppStore } from '@/stores/app';
  const state = useAppStore();
</script>
