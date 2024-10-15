// Utilities
import { defineStore } from 'pinia'

import {
  getInjectedExtensions,
  connectInjectedExtension,
  InjectedExtension,
} from "polkadot-api/pjs-signer"

// Get the list of installed extensions
const extensions: string[] = getInjectedExtensions()
// Connect to an extension
const selectedExtension: InjectedExtension = await connectInjectedExtension(
  extensions[0],
)

export const useAppStore = defineStore('app', {
  state: () => ({
    loadedAccounts: selectedExtension.getAccounts(),
    selectedAccount: selectedExtension.getAccounts()[0]
  }),
})
