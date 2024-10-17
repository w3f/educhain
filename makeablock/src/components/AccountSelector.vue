<template>
    <v-select 
        v-on:update:model-value="(ev) => selectAccount(ev)"
        v-model="store.$state.selectedAccount"
        :items="store.$state.loadedAccounts"
        return-object
        label="Select Account (PJS)"
        :item-title="(acc) => acc.name"
        variant="outlined"></v-select>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app';
import { InjectedPolkadotAccount } from 'polkadot-api/pjs-signer';
const store = useAppStore();
function selectAccount(ev: InjectedPolkadotAccount) {
    store.$state.selectedAccount = ev
}
</script>