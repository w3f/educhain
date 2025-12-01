
# XCM Configuration

EduChain uses the **relay chain token (PAS)** as its native currency. This leverages Polkadot’s XCM (Cross-Consensus Messaging) for secure, reserve-backed asset transfers.

!!! info "Configuration File"
    The complete XCM configuration is in [`xcm_config.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/xcm_config.rs).

## Reserve Transfer Mechanism

In this model, the relay chain (or Asset Hub) acts as the **reserve location**.

*   **Reserve Chain:** Holds the actual tokens (locked).
*   **Parachain:** Mints/burns wrapped representations.

### How it works

1.  **Inbound (Reserve -> Parachain):**
    *   User sends tokens from Reserve.
    *   Reserve locks tokens.
    *   XCM message sent to Parachain.
    *   Parachain mints equivalent amount.

2.  **Outbound (Parachain -> Reserve):**
    *   User sends tokens from Parachain.
    *   Parachain burns tokens.
    *   XCM message sent to Reserve.
    *   Reserve releases tokens to user.

!!! note "Asset Hub Migration"
    While EduChain currently uses the relay chain as the reserve, **Asset Hub** is the recommended reserve location for the future. The configuration is similar.

## Sovereign Account

The sovereign account represents the parachain on the reserve chain.
[View EduChain's Sovereign Account](https://paseo.subscan.io/account/13YMK2dtwE9kmdYK7XbBYiTJrVnTbSVFiYNqzNTAv3USGFWf)

## Configuration Details

### Reserve Chain Location

EduChain recognizes the relay chain as its parent.

```rust
parameter_types! {
    pub const RelayNetwork: Option<NetworkId> = Some(NetworkId::Polkadot);
    pub UniversalLocation: InteriorLocation = [
        GlobalConsensus(NetworkId::Polkadot),
        Parachain(ParachainInfo::parachain_id().into()),
    ].into();
}
```

### Location to AccountId Mapping

EduChain uses standard converters to map XCM locations to local account IDs:

```rust
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the parent `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to `AccountId` via the `ParaId::into`.
    SiblingParachainConvertsVia<Sibling, AccountId>,
    // Straight up local `AccountId32` origins just alias directly to `AccountId`.
    AccountId32Aliases<RelayNetwork, AccountId>,
);
```

This configuration handles conversions for:
*   Parent relay chain accounts
*   Sibling parachain accounts
*   Direct `AccountId32` mappings

## Asset Transactor for PAS Tokens

The `AssetTransactor` handles incoming PAS tokens. The configuration recognizes the network's native token by its location identifier:

```rust
parameter_types! {
    pub ParentRelayLocation: Location = Location::parent();
}

pub type FungibleTransactor = xcm_builder::FungibleAdapter<
    Balances,
    xcm_builder::IsConcrete<ParentRelayLocation>,
    LocationToAccountId,
    AccountId,
    ()
>;
```

Even though PAS is identified by the relay chain's location (`Parent`), the actual reserve is Asset Hub.

## Reserve Asset Definition

Asset Hub is set as the reserve location for PAS tokens:

```rust
pub struct PasFromAssetHub;
impl ContainsPair<Asset, Location> for PasFromAssetHub {
    fn contains(asset: &Asset, location: &Location) -> bool {
        let is_pas = match asset {
            Asset {
                id: AssetId(asset_id),
                fun: Fungible(_),
            } => {
                // The identifier of PAS (Paseo native token).
                // The relative location from this parachain to the Relay Chain.
                // Even though PAS is identified by the location of the Relay Chain,
                // the reserve is Asset Hub.
                matches!(asset_id.unpack(), (1, []))
            },
            _ => false,
        };
        let is_from_asset_hub = matches!(
            location.unpack(),
            // The relative location of Asset Hub on Paseo (Parachain 1000).
            (1, [Parachain(1000)])
        );
        is_pas && is_from_asset_hub
    }
}

pub type IsReserve = PasFromAssetHub;
```

This configuration ensures that:
*   PAS is identified by the relay chain location `(1, [])`
*   But the **reserve is Asset Hub** at `(1, [Parachain(1000)])`
*   Transfers must come from Asset Hub to be recognized as reserve transfers

## Barrier: Allow Paid Execution

The barrier configuration controls which XCM messages are allowed to execute:

```rust
pub type Barrier = TrailingSetTopicAsId<
    DenyThenTry<
        DenyReserveTransferToRelayChain,
        (
            TakeWeightCredit,
            WithComputedOrigin<
                (
                    AllowTopLevelPaidExecutionFrom<Everything>,
                    AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
                    // Parent and its exec plurality get free execution
                ),
                UniversalLocation,
                ConstU32<8>,
            ),
        ),
    >,
>;
```

Key features:

*   **Blocks reserve transfers to the relay chain** (use Asset Hub instead)
*   **Allows paid execution** from any location
*   **Free execution** for the parent relay chain and its executive body

## Enabling XCM Execution and Transfers

**Critical Update:** XCM execution is now enabled, which is necessary for sending tokens back to Paseo or Asset Hub:

```rust
impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Everything;
    // ^ Enable XCM execution for transfers (changed from Nothing)
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Everything;
    type XcmReserveTransferFilter = Everything;
    // ...remaining config...
}
```

!!! warning "Important Change"
    `XcmExecuteFilter` is now set to `Everything` instead of `Nothing`. This allows:
    
    *   Users to send PAS tokens back to Paseo
    *   Users to send PAS tokens to Asset Hub
    *   Bidirectional token transfers between EduChain and other chains
    
    Without this setting, outbound transfers would fail, as it required a local XCM to be executed on your chain.

## Finalized Configuration

To view this configuration, please visit the EduChain repository and view [`xcm_config.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/xcm_config.rs).

## Common Issues

*   **TooExpensive:** Not enough fee or weight limit. Use `weightLimit: "Unlimited"` and ensure fees are covered.
*   **Unrecognized asset:** Asset location or format is incorrect. Use the correct `parents` and `interior` fields.
*   **Filtered:** If `XcmExecuteFilter` is set to `Nothing`, outbound transfers will fail. Ensure it's set to `Everything` for bidirectional transfers.
*   **Reserve not recognized:** Ensure transfers come from Asset Hub (Parachain 1000), not directly from the relay chain.

## Testing Transfers

To test the configuration:

1.  **Receive PAS from Asset Hub:**
    *   Send PAS from Asset Hub to your EduChain account
    *   Verify balance increases on EduChain

2.  **Send PAS back to Paseo:**
    *   Use the `limitedReserveTransferAssets` extrinsic
    *   Destination: Paseo relay chain
    *   Asset: PAS (identified as `Parent`)
    *   Verify balance decreases on EduChain and increases on Paseo

3.  **Send PAS to Asset Hub:**
    *   Use the `limitedReserveTransferAssets` extrinsic
    *   Destination: Asset Hub (Parachain 1000)
    *   Asset: PAS
    *   Verify bidirectional transfers work

## Resources

- [Polkadot XCM Transfer Guide (from Polkadot Docs)](https://docs.polkadot.com/develop/interoperability/xcm-guides/from-apps/transfers/)
- [Configuring a parachain to use Relay Chain native token (Rustdocs Guide)](https://paritytech.github.io/polkadot-sdk/master/xcm_docs/cookbook/relay_token_transactor/index.html)
- [Changing the DOT reserve from Relay Chain to Asset Hub (HackMD Guide)](https://hackmd.io/@n9QBuDYOQXG-nWCBrwx8YQ/HkYVQFS8ke#Changing-the-DOT-reserve-from-Relay-Chain-to-Asset-Hub) - Source of the `PasFromAssetHub` implementation
