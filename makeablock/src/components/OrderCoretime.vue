<script setup lang="ts">
import { pas } from "@polkadot-api/descriptors"
import { createClient } from "polkadot-api"
import { getSmProvider } from "polkadot-api/sm-provider";
import { chainSpec } from "polkadot-api/chains/paseo";
import { startFromWorker } from "polkadot-api/smoldot/from-worker";

// Using vite
import SmWorker from "polkadot-api/smoldot/worker?worker";
import { useAppStore } from "@/stores/app";
const worker = new SmWorker();

const smoldot = startFromWorker(worker);
const chain = await smoldot.addChain({ chainSpec });

// Connect to the paseo relay chain.
const client = createClient(
    getSmProvider(chain)
);

const pasApi = client.getTypedApi(pas)
const paraId = ref(4414);
const disable = ref(false);
const buttonText = ref("MAKE A BLOCK!");

function buyBlock() {
    // Only for EduChain on Pas!
    const tx = pasApi.tx.OnDemand.place_order_keep_alive({
        max_amount: 1000000000000000n,
        para_id: paraId.value
    });

    tx.signSubmitAndWatch(useAppStore().$state.selectedAccount!.polkadotSigner).subscribe((status) => {
        console.log(status)
        disable.value = true;
        buttonText.value = "BLOCK ORDERED, WAITING FOR FINALIZATION...";
        if (status.type == "finalized") {
            disable.value = false;
            buttonText.value = "MAKE A BLOCK!";
        }
    })
}

</script>

<template>
    <v-container>
        <v-text-field v-model="paraId" hide-details="auto" type="number" label="Para ID"></v-text-field>
    </v-container>
    <v-container>
        <v-btn :disabled="disable" variant="outlined" size="x-large" @click="buyBlock">
            {{ buttonText }}
        </v-btn>
    </v-container>
</template>