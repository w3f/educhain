// Utilities
import { defineStore } from 'pinia'

import {
  getInjectedExtensions,
  connectInjectedExtension,
  InjectedExtension,
  InjectedPolkadotAccount,
} from "polkadot-api/pjs-signer"

const useAppStore = defineStore('app', {
  state: () => ({
    loadedAccounts: [] as InjectedPolkadotAccount[],
    selectedAccount: null as InjectedPolkadotAccount | null,
    extensionLoaded: false
  }),
});

// Get the list of installed extensions
const extensions: string[] = getInjectedExtensions()
console.log(extensions.length)
if (extensions.length >= 1) {
  // Connect to an extension
  const selectedExtension: InjectedExtension = await connectInjectedExtension(
    extensions[0],
  )
  useAppStore().$state.loadedAccounts = selectedExtension.getAccounts();
  useAppStore().$state.selectedAccount = selectedExtension.getAccounts()[0];
  useAppStore().$state.extensionLoaded = true;
  console.log(useAppStore().$state)
}

export { useAppStore };