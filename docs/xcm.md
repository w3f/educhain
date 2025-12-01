
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

We use `HashedDescription` to convert XCM locations into local account IDs. This is flexible and future-proof.

```rust
pub type LocationToAccountId = (
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
    AccountId32Aliases<RelayNetwork, AccountId>
);
```

If your chain already uses the older converters, you may need to keep them for compatibility, but consider migrating to `HashedDescription` where possible.

## Asset Transactor for Relay Tokens

The `AssetTransactor` is responsible for handling incoming assets. The following configuration means: **only treat the relay chain’s native token (e.g., DOT or PAS) in this special way**. The reason for using `Parent` is that the asset ID for the relay chain token is the relay chain itself. If you receive the relay chain token from anywhere, it will be handled by this adapter:

```rust
pub type AssetTransactor = FungibleAdapter<
    Balances,
    IsConcrete<ParentRelayLocation>,
    LocationToAccountId,
    AccountId,
    (),
>;
```

## Reserve Asset Definition

The relay chain is set as the reserve location for its native token:

```rust
parameter_types! {
    pub RelayTokenForRelay: (AssetFilter, Location) = (
        Wild(AllOf { id: AssetId(Parent.into()), fun: WildFungible }),
        Parent.into(),
    );
}
pub type IsReserve = xcm_builder::Case<RelayTokenForRelay>;
```

## Barrier: Allow Paid Execution

The barrier configuration allows paid XCM execution and subscriptions, enabling reserve transfers:

```rust
pub type Barrier = TrailingSetTopicAsId<
    (
        // Weight that is paid for may be consumed.
        TakeWeightCredit,
        // Expected responses are OK.
        AllowKnownQueryResponses<PolkadotXcm>,
        WithComputedOrigin<
            (
                // If the message is one that immediately attempts to pay for execution, then allow it.
                AllowTopLevelPaidExecutionFrom<ParentOrParentsExecutivePlurality>,
                // Subscriptions for version tracking are OK.
                AllowSubscriptionsFrom<Everything>,
            ),
            UniversalLocation,
            ConstU32<8>,
        >,
    ),
>;
```

## Enabling Reserve Transfers

Reserve transfers are enabled in the XCM pallet configuration:

```rust
impl pallet_xcm::Config for Runtime {
    // ...existing code...
    type XcmReserveTransferFilter = Everything;
    // ...existing code...
}
```

## Finalized Configuration

To view this configuration, please visit the EduChain repository and view [`xcm_config.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/xcm_config.rs).

## Common Issues

- **TooExpensive:** Not enough fee or weight limit. Use `weightLimit: "Unlimited"` and ensure fees are covered.
- **Unrecognized asset:** Asset location or format is incorrect. Use the correct `parents` and `interior` fields.

## Resources

- [Polkadot XCM Transfer Guide (from Polkadot Docs)](https://docs.polkadot.com/develop/interoperability/xcm-guides/from-apps/transfers/)
- [Configuring a parachain to use Relay Chain native token (Rustdocs Guide)](https://paritytech.github.io/polkadot-sdk/master/xcm_docs/cookbook/relay_token_transactor/index.html)
