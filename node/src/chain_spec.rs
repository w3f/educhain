use cumulus_primitives_core::ParaId;
use educhain_runtime::{constants::currency::EXISTENTIAL_DEPOSIT, AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{bytes::from_hex, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

const ROCOCO_PARA_ID: u32 = 4428;

const PASEO_PARA_ID: u32 = 4012;

// Sudo privileges
pub const ROOT_ACCOUNT: &str = "0x6cfbd47775c5fa20eedf7275360885c5f77c64a426c4fd0d67272784ae5e346c";

// Collator accounts that produce blocks and earn rewards. Typically, private key is in cold storage

pub const COLLATOR1: &str = "0x38a2edbf7cd629e10700376f941122bf6c6a7b705bb70d6eb15359099055015b";
pub const COLLATOR2: &str = "0x3090de03bda721f91d4ea242c63c4220832194e63d2c5b61dbcbdd458224350f";

// The private key of these session keys needs to be inserted into the collator node for it to start
// producing blocks.

pub const SESSION1: &str = "0x1e0f4e48f26d802ce3699872c97e2ec7f8476a9b27a5d4307986ce0ddf0d8530";
pub const SESSION2: &str = "0x1e673715db64783eadc6ca927e493ded30f2447efff0f6d5d84578e823f86374";

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain
/// in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn pub_to_account_id(pubkey: &str) -> AccountId {
    let pubkey = sr25519::Public::from_raw(
        from_hex(pubkey)
            .expect("Unable to parse hex")
            .try_into()
            .expect("Unable to parse public key"),
    );
    //dbg!(pubkey.clone().into_account().to_string());
    pubkey.into_account().into()
}

pub fn pub_to_collator_key(pubkey: &str) -> AuraId {
    let pubkey = sr25519::Public::from_raw(
        from_hex(pubkey)
            .expect("Unable to parse hex")
            .try_into()
            .expect("Unable to parse public key"),
    );

    //dbg!(pubkey);

    AuraId::from(pubkey)
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we
/// have just one key).
pub fn template_session_keys(keys: AuraId) -> educhain_runtime::SessionKeys {
    educhain_runtime::SessionKeys { aura: keys }
}

pub fn live_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "EDU".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        educhain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo".into(),
            // You MUST set this to the correct network!
            para_id: ROCOCO_PARA_ID,
        },
    )
    .with_name("Educhain Rococo")
    .with_id("live")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(testnet_genesis(
        // initial collators.
        vec![
            (pub_to_account_id(COLLATOR1), pub_to_collator_key(SESSION1)),
            (pub_to_account_id(COLLATOR2), pub_to_collator_key(SESSION2)),
        ],
        vec![
            pub_to_account_id(COLLATOR1),
            pub_to_account_id(COLLATOR2),
            pub_to_account_id(ROOT_ACCOUNT),
        ],
        pub_to_account_id(ROOT_ACCOUNT),
        ROCOCO_PARA_ID.into(),
    ))
    .with_protocol_id("educhain-live")
    .with_properties(properties)
    .build()
}

pub fn paseo_live_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "EDU".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        educhain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "paseo".into(),
            // You MUST set this to the correct network!
            para_id: PASEO_PARA_ID,
        },
    )
    .with_name("Educhain Paseo")
    .with_id("live")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(testnet_genesis(
        // initial collators.
        vec![
            (pub_to_account_id(COLLATOR1), pub_to_collator_key(SESSION1)),
            (pub_to_account_id(COLLATOR2), pub_to_collator_key(SESSION2)),
        ],
        vec![
            pub_to_account_id(COLLATOR1),
            pub_to_account_id(COLLATOR2),
            pub_to_account_id(ROOT_ACCOUNT),
        ],
        pub_to_account_id(ROOT_ACCOUNT),
        ROCOCO_PARA_ID.into(),
    ))
    .with_protocol_id("educhain-live")
    .with_properties(properties)
    .build()
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "EDU".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::builder(
        educhain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(),
            // You MUST set this to the correct network!
            para_id: 1000,
        },
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis(
        // initial collators.
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        1000.into(),
    ))
    .build()
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "EDU".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        educhain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(),
            // You MUST set this to the correct network!
            para_id: 1000,
        },
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(testnet_genesis(
        // initial collators.
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        1000.into(),
    ))
    .with_protocol_id("template-local")
    .with_properties(properties)
    .build()
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    root: AccountId,
    id: ParaId,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
        },
        "parachainInfo": {
            "parachainId": id,
        },
        "collatorSelection": {
            "invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
            "candidacyBond": EXISTENTIAL_DEPOSIT * 16,
        },
        "session": {
            "keys": invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura), // session keys
                    )
                })
            .collect::<Vec<_>>(),
        },
        "treasury": {},
        "polkadotXcm": {
            "safeXcmVersion": Some(SAFE_XCM_VERSION),
        },
        "sudo": { "key": Some(root) }
    })
}
