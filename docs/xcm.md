# XCM Configuration

EduChain is configured to use the **relay chain token (e.g., PAS)** as its native currency, leveraging Polkadot’s XCM (Cross-Consensus Messaging) for secure, reserve-backed asset transfers. This document explains the configuration and how reserve transfers work in practice.

!!!info
    The complete, working XCM configuration can be found in the EduChain repository: [`xcm_config.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/xcm_config.rs)

## Overview: Reserve Transfer Mechanism

In Polkadot, a parachain can use the relay chain’s token (such as DOT or PAS) as its native currency. This is achieved by treating the relay chain as the *reserve location* for the token, and using XCM to move balances between the relay chain and parachain.

A sovereign account represents the parachain on the relay chain, which is where the tokens are "reserved" while in use on the parachain. You can view EduChain's [sovereign account here.](https://paseo.subscan.io/account/13YMK2dtwE9kmdYK7XbBYiTJrVnTbSVFiYNqzNTAv3USGFWf)

### How it works

- When a user transfers tokens from the reserve chain to the parachain, the tokens are locked (“reserved”) on the reserve chain and minted on the parachain.
- When tokens are sent back, they are burned on the parachain and released on the reserve chain.

This mechanism ensures that the total supply remains consistent and that the reserve chain always holds the actual reserves.

- **From Reserve Chain to Parachain:**  
  The relay chain locks the user’s tokens and sends an XCM message to the parachain, which mints the equivalent amount for the user.
- **From Parachain to Reserve Chain:**  
  The parachain burns the user’s tokens and sends an XCM message to the reserve chain, which releases the equivalent amount to the user’s reserve chain account.

This ensures that the parachain’s native currency is always backed by actual reserves on the reserve chain.

## Reserve Chain and Location

Our specific parachain is configured to recognize the relay chain (Polkadot) as its parent and to use its network ID for account mapping. In theory, you can use another chain, such as Asset Hub, as your reserve:

```rust
parameter_types! {
    pub const RelayNetwork: Option<NetworkId> = Some(NetworkId::Polkadot);
    	pub UniversalLocation: InteriorLocation = [
        GlobalConsensus(NetworkId::Polkadot), 
        Parachain(ParachainInfo::parachain_id().into())
    ].into();
}
```

## Location to AccountId Mapping

This mapping allows the runtime to convert XCM locations (relay chain, sibling parachains, AccountId32) into local account IDs:

```rust
pub type LocationToAccountId = (
    ParentIsPreset<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);
```

Another option is to use `HashedDescription`, which is a newer, preferred alternative:

```rust
pub type LocationToAccountId = (
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
);
```

## Asset Transactor for Relay Tokens

The `AssetTransactor` is responsible for handling incoming assets. Notably, the `FungibleAdapter` is configured such that it handles DOT (or the native currency of the reserve chain): 
```rust
pub type AssetTransactor = FungibleAdapter<
    Balances,
    IsConcrete<ParentRelayLocation>,
    LocationToAccountId,
    AccountId,
    ()
>;
```

## Reserve Asset Definition

The relay chain is set as the reserve location for its native token:

```rust
parameter_types! {
    pub RelayTokenForRelay: (AssetFilter, Location) = (
        Wild(AllOf { id: AssetId(Parent.into()), fun: WildFungible }),
        Parent.into()
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
                // If the message is one that immediately attemps to pay for execution, then allow it.
                AllowTopLevelPaidExecutionFrom<ParentOrParentsExecutivePlurality>,
                // Subscriptions for version tracking are OK.
                AllowSubscriptionsFrom<Everything>,
            ),
            UniversalLocation,
            ConstU32<8>
        >,
    )
>;
```

## Enabling Reserve Transfers

Reserve transfers are enabled in the XCM pallet configuration:

```rust
impl pallet_xcm::Config for Runtime {
    // ...
    type XcmReserveTransferFilter = Everything;
    // ...
}
```

## Finalized Configuration

To view this configuration, please visit the EduChain repository and view [`xcm_config.rs`](https://github.com/w3f/educhain/blob/main/runtime/src/configs/xcm_config.rs).

## Example XCM Message

**Relay Chain ➔ Parachain:**
- Instruction: `transferAssets`
- Destination: Parachain (by ID)
- Beneficiary: Parachain user’s AccountId
- Asset: Relay chain token (e.g., PAS)
- Fee: Paid from transferred asset

> Sample Payload: `0x630b040001004d4c0400010100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d04040000000700e87648170000000000`

**Parachain ➔ Relay Chain:**
- Instruction: `transferAssets`
- Destination: Relay chain
- Beneficiary: Relay chain account
- Asset: Relay chain token
- Fee: Paid from transferred asset

> Sample Payload: `0x1f0b0401000400010100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d04040100000700e87648170000000000`

## Common Issues

- **TooExpensive:** Not enough fee or weight limit. Use `weightLimit: "Unlimited"` and ensure fees are covered.
- **Unrecognized asset:** Asset location or format is incorrect. Use the correct `parents` and `interior` fields.

## Resources

- [Polkadot Docs XCM Guides](https://docs.polkadot.com/develop/interoperability/xcm-guides/)
- [Configuring a parachain to use Relay Chain native token - rustdocs guide](https://paritytech.github.io/polkadot-sdk/master/xcm_docs/cookbook/relay_token_transactor/index.html)
- [Polkadot.js Apps](https://polkadot.js.org/apps/)
