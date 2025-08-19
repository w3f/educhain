
# XCM Configuration

EduChain is configured to use the **relay chain token (e.g., PAS)** as its native currency, leveraging Polkadot’s XCM (Cross-Consensus Messaging) for secure, reserve-backed asset transfers. This document explains the configuration and how reserve transfers work in practice.

!!!info
    The complete, working XCM configuration can be found in the EduChain repository: [`xcm_config.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/xcm_config.rs)

## Overview: Reserve Transfer Mechanism

In Polkadot, a parachain can use the relay chain’s token (such as DOT or PAS) as its native currency. This is achieved by treating the relay chain as the *reserve location* for the token, and using XCM to move balances between the relay chain and parachain.

> **Note:**
> While EduChain currently uses the relay chain as the reserve for its native token, it is now possible—and in many cases preferable—to use **Asset Hub** as the reserve location. Post-Asset Hub Migration (AHM), Asset Hub will be the source of truth for assets, and parachains are encouraged to use it as the reserve. The configuration is very similar, and you can already use Asset Hub as your reserve today.

A sovereign account represents the parachain on the reserve chain, which is where the tokens are "reserved" while in use on the parachain. You can view EduChain's [current sovereign account here.](https://paseo.subscan.io/account/13YMK2dtwE9kmdYK7XbBYiTJrVnTbSVFiYNqzNTAv3USGFWf)

### How it works

- When a user transfers tokens from the reserve chain to the parachain, the tokens are locked (“reserved”) on the reserve chain and minted on the parachain.
- When tokens are sent back, they are burned on the parachain and released on the reserve chain.

This mechanism ensures that the total supply remains consistent and that the reserve chain always holds the actual reserves.

- **From Reserve Chain to Parachain:**  
  The reserve chain locks the user’s tokens and sends an XCM message to the parachain, which mints the equivalent amount for the user.
- **From Parachain to Reserve Chain:**  
  The parachain burns the user’s tokens and sends an XCM message to the reserve chain, which releases the equivalent amount to the user’s reserve chain account.

This ensures that the parachain’s native currency is always backed by actual reserves on the reserve chain (i.e., Asset Hub, if configured).

## Reserve Chain and Location

EduChain is currently configured to recognize the relay chain (Polkadot) as its parent and to use its network ID for account mapping. However, you can use another chain, such as Asset Hub, as your reserve.

```rust
parameter_types! {
    pub const RelayNetwork: Option<NetworkId> = Some(NetworkId::Polkadot);
    pub UniversalLocation: InteriorLocation = [
        GlobalConsensus(NetworkId::Polkadot),
        Parachain(ParachainInfo::parachain_id().into()),
    ].into();
}
```

## Location to AccountId Mapping

This mapping allows the runtime to convert XCM locations (relay chain, sibling parachains, AccountId32) into local account IDs. The **preferred approach** is to use `HashedDescription`, which is more flexible and future-proof:

```rust
pub type LocationToAccountId = (
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
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
