
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

*   **Unrecognized asset:** Asset location or format is incorrect. Use the correct `parents` and `interior` fields.
*   **Filtered:** If `XcmExecuteFilter` is set to `Nothing`, outbound transfers will fail.
*   **Reserve not recognized:** Ensure transfers come from Asset Hub (Parachain 1000), not directly from the relay chain.

## Testing Transfers

To test the configuration, you can use the `polkadotXcm.execute` extrinsic to send XCM messages.

### From EduChain to Asset Hub

To send PAS tokens from EduChain to Asset Hub, construct an XCM message with the following instructions:

| Instruction | Purpose | Parameters |
|-------------|---------|------------|
| `WithdrawAsset` | Withdraw PAS from sender's account | **Asset ID:** `{ parents: 1, interior: Here }` (relay chain token)<br>**Amount:** Transfer amount in plancks |
| `InitiateReserveWithdraw` | Send tokens to Asset Hub (the reserve) | **Assets:** `Wild(AllCounted(1))`<br>**Reserve:** `{ parents: 1, interior: X1(Parachain(1000)) }` (Asset Hub)<br>**XCM Program:** See sub-instructions below |
| `BuyExecution` | Pay for execution on Asset Hub | **Fees:** Same asset, ~50% of transfer amount<br>**Weight Limit:** `Unlimited` |
| `DepositAsset` | Deposit tokens to beneficiary | **Assets:** `Wild(AllCounted(1))`<br>**Beneficiary:** `{ parents: 0, interior: X1(AccountId32({ id: beneficiary_bytes })) }` |

**Parameter Details:**

*   `beneficiary_bytes` - The recipient's account ID as a 32-byte array. In PAPI Dev Console or Polkadot.JS UI, this is the decoded SS58 address.
*   Transfer amount should be in plancks (smallest unit). For example, 1 PAS = 10^10 plancks.

**Max Weight:** `{ ref_time: 2_000_000_000, proof_size: 200_000 }`

### From Asset Hub to EduChain

To send PAS tokens from Asset Hub to EduChain, construct an XCM message with the following instructions:

| Instruction | Purpose | Parameters |
|-------------|---------|------------|
| `WithdrawAsset` | Withdraw PAS from sender's account on Asset Hub | **Asset ID:** `{ parents: 1, interior: Here }` (relay chain token)<br>**Amount:** Transfer amount in plancks |
| `DepositReserveAsset` | Send tokens to EduChain (Asset Hub holds reserve) | **Assets:** `Wild(AllCounted(1))`<br>**Destination:** `{ parents: 1, interior: X1(Parachain(educhain_id)) }` (EduChain)<br>**XCM Program:** See sub-instructions below |
| `BuyExecution` | Pay for execution on EduChain | **Fees:** Same asset, ~50% of transfer amount<br>**Weight Limit:** `Unlimited` |
| `DepositAsset` | Deposit tokens to beneficiary | **Assets:** `Wild(AllCounted(1))`<br>**Beneficiary:** `{ parents: 0, interior: X1(AccountId32({ id: beneficiary_bytes })) }` |

**Parameter Details:**

*   `educhain_id` - EduChain's parachain ID on Paseo (e.g., 4883).
*   `beneficiary_bytes` - The recipient's account ID as a 32-byte array. In PAPI Dev Console or Polkadot.JS UI, this is the decoded SS58 address.
*   Transfer amount should be in plancks (smallest unit). For example, 1 PAS = 10^10 plancks.

**Max Weight:** `{ ref_time: 2_000_000_000, proof_size: 200_000 }`

!!! tip "Key Differences"
    - **From EduChain:** Use `InitiateReserveWithdraw` (tokens going to reserve)
    - **From Asset Hub:** Use `DepositReserveAsset` (tokens leaving reserve)
    - Both require `XcmExecuteFilter = Everything` on EduChain for execution

### Verification Steps

After sending transfers:

1.  **From EduChain to Asset Hub:**
    *   Check balance decreases on EduChain
    *   Check balance increases on Asset Hub

2.  **From Asset Hub to EduChain:**
    *   Check balance decreases on Asset Hub
    *   Check balance increases on EduChain

## Resources

- [Polkadot XCM Transfer Guide (from Polkadot Docs)](https://docs.polkadot.com/develop/interoperability/xcm-guides/from-apps/transfers/)
- [Configuring a parachain to use Relay Chain native token (Rustdocs Guide)](https://paritytech.github.io/polkadot-sdk/master/xcm_docs/cookbook/relay_token_transactor/index.html)
- [Changing the DOT reserve from Relay Chain to Asset Hub (HackMD Guide)](https://hackmd.io/@n9QBuDYOQXG-nWCBrwx8YQ/HkYVQFS8ke#Changing-the-DOT-reserve-from-Relay-Chain-to-Asset-Hub) - Source of the `PasFromAssetHub` implementation
