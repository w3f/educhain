## Chain Specifications

The `chain-specs/` folder contains all useful/related chain specs for Paseo, Rococo, Educhain, and patches. A patch is used with the `chain-spec-builder` to create

```sh
chain-spec-builder -c ./chain-artifacts/chain-specs/educhain-spec.json create \
-r ./target/release/wbuild/educhain-runtime/educhain_runtime.wasm patch \
./chain-artifacts/chain-specs/educhain-patch.json
```

### Adding necessary parachain parameters

The above command won't add all the parameters needed for a parachain. Copy and paste the following at the top level to ensure it is parachain compatible: 

```json
{
  "name": "Educhain Rococo",
  "id": "live",
  "chainType": "Live",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "educhain-live",
  "properties": {
    "ss58Format": 42,
    "tokenDecimals": 12,
    "tokenSymbol": "EDU"
  },
  "relay_chain": "rococo",
  "para_id": 4428,
  "codeSubstitutes": {},
  ...
}
```

This ensures a **Rococo** compatible configuration, which you can start syncing with `polkadot-parachain`.