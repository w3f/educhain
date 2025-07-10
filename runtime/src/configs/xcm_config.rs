use crate::{
    AccountId,
    AllPalletsWithSystem,
    Balances,
    ParachainInfo,
    ParachainSystem,
    PolkadotXcm,
    Runtime,
    RuntimeCall,
    RuntimeEvent,
    RuntimeOrigin,
    WeightToFee,
    XcmpQueue,
};
use frame_support::{
    parameter_types,
    traits::{ ConstU32, Contains, Everything, Nothing },
    weights::Weight,
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use polkadot_parachain_primitives::primitives::Sibling;
use polkadot_runtime_common::impls::ToAuthor;
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases,
    AllowTopLevelPaidExecutionFrom,
    EnsureXcmOrigin,
    FixedWeightBounds,
    FrameTransactionalProcessor,
    FungibleAdapter,
    IsConcrete,
    ParentIsPreset,
    RelayChainAsNative,
    SiblingParachainAsNative,
    SiblingParachainConvertsVia,
    SignedAccountId32AsNative,
    SignedToAccountId32,
    SovereignSignedViaLocation,
    TakeWeightCredit,
    TrailingSetTopicAsId,
    UsingComponents,
    WithComputedOrigin,
    WithUniqueTopic,
    AllowKnownQueryResponses,
    AllowSubscriptionsFrom
};
use xcm_executor::XcmExecutor;

parameter_types! {
	pub const RelayLocation: Location = Location::parent();
	pub const RelayNetwork: Option<NetworkId> = Some(NetworkId::Polkadot);
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	// For the real deployment, it is recommended to set `RelayNetwork` according to the relay chain
	// and prepend `UniversalLocation` with `GlobalConsensus(RelayNetwork::get())`.
	pub UniversalLocation: InteriorLocation = [
        GlobalConsensus(NetworkId::Polkadot), 
        Parachain(ParachainInfo::parachain_id().into())
    ].into();}

/// Type for specifying how a `Location` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the parent `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<Sibling, AccountId>,
    // Straight up local `AccountId32` origins just alias directly to `AccountId`.
    AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Configuration related to asset transactors
mod asset_transactor {
    use super::*;

    parameter_types! {
		pub ParentRelayLocation: Location = Location::parent();
	}

    /// AssetTransactor for handling the relay chain token
    pub type FungibleTransactor = FungibleAdapter<
        // Use this implementation of the `fungible::*` traits.
        // `Balances` is the name given to the balances pallet in this particular recipe.
        // Any implementation of the traits would suffice.
        Balances,
        // This transactor deals with the native token of the Relay Chain.
        // This token is referenced by the Location of the Relay Chain relative to this chain
        // -- Location::parent().
        IsConcrete<ParentRelayLocation>,
        // How to convert an XCM Location into a local account id.
        // This is also something that's configured in the XCM executor.
        LocationToAccountId,
        // The type for account ids, only needed because `fungible` is generic over it.
        AccountId,
        // Not tracking teleports.
        // This recipe only uses reserve asset transfers to handle the Relay Chain token.
        ()
    >;

    /// Actual configuration item that'll be set in the XCM config.
    /// A tuple could be used here to have multiple transactors, each (potentially) handling
    /// different assets.
    /// In this recipe, we only have one.
    pub type AssetTransactor = FungibleTransactor;
}

/// Configuration related to token reserves
mod is_reserve {
    use super::*;

    parameter_types! {
		/// Reserves are specified using a pair `(AssetFilter, Location)`.
		/// Each pair means that the specified Location is a reserve for all the assets in AssetsFilter.
		/// Here, we are specifying that the Relay Chain is the reserve location for its native token.
		pub RelayTokenForRelay: (AssetFilter, Location) =
		  (Wild(AllOf { id: AssetId(Parent.into()), fun: WildFungible }), Parent.into());
	}

    /// The wrapper type xcm_builder::Case is needed in order to use this in the configuration.
    pub type IsReserve = xcm_builder::Case<RelayTokenForRelay>;
}

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    // Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
    // recognized.
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognized.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `RuntimeOrigin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

pub struct ParentOrParentsExecutivePlurality;
impl Contains<Location> for ParentOrParentsExecutivePlurality {
    fn contains(location: &Location) -> bool {
        matches!(location.unpack(), (1, []) | (1, [Plurality { id: BodyId::Executive, .. }]))
    }
}

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

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = ();
	type AssetTransactor = asset_transactor::AssetTransactor;
	type OriginConverter = ();
	// The declaration of which Locations are reserves for which Assets.
	type IsReserve = is_reserve::IsReserve;
	type IsTeleporter = ();
	type UniversalLocation = UniversalLocation;
	// This is not safe, you should use `xcm_builder::AllowTopLevelPaidExecutionFrom<T>` in a
	// production chain
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader =
		UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ToAuthor<Runtime>>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
	type XcmRecorder = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = WithUniqueTopic<
    (
        // Two routers - use UMP to communicate with the relay chain:
        cumulus_primitives_utility::ParentAsUmp<ParachainSystem, (), ()>,
        // ..and XCMP to communicate with the sibling chains.
        XcmpQueue,
    )
>;

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Nothing;
    // ^ Disable dispatchable execute on the XCM pallet.
    // Needs to be `Everything` for local testing.
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Everything;
    type XcmReserveTransferFilter = Everything;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;

    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    // ^ Override for AdvertisedXcmVersion default
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = LocationToAccountId;
    type MaxLockers = ConstU32<8>;
    type WeightInfo = pallet_xcm::TestWeightInfo;
    type AdminOrigin = EnsureRoot<AccountId>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}
