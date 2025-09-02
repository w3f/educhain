//! # Pallet News Provenance
//!
//! A pallet for recording and tracking provenance of news articles.
//!
//! ## Overview
//!
//! This pallet provides storage and dispatchables for:
//! - Recording article fingerprints and provenance metadata
//! - Mapping articles to publishers and collections
//! - Verifying articles
//!
//! ## Storage
//!
//! - `ArticleByHash`: Maps article content hash to its record
//! - `RootByItem`: Maps (collection_id, item_id) to article content hash
//! - `ArticlesByPublisher`: Maps publisher to a bounded list of their article hashes
//!
//! ## Dispatchables
//!
//! - `record_article`: Record a new article fingerprint
//! - `verify_article`: Verify an existing article
//!
//! ## Helper Functions
//!
//! - `verify_signature`: Verify article signature

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{ pallet_prelude::*, BoundedVec };
    use frame_system::{ pallet_prelude::*, WeightInfo };
    use scale_info::TypeInfo;
    use sp_core::{ sr25519, H256 };
    use sp_runtime::{ traits::Verify, AnySignature, MultiSignature };

    /// Unique identifier for a collection of articles.
    pub type CollectionId = u128;
    /// Unique identifier for an item within a collection.
    pub type ItemId = u128;
    /// Hash of the article content (serves as a fingerprint).
    pub type ContentHash = H256;
    /// Optional hash representing a root of article sections.
    pub type SectionRoot = H256;

    /// Supported hash algorithms for article content.
    #[derive(
        scale_info::TypeInfo,
        codec::Encode,
        codec::Decode,
        Clone,
        Copy,
        PartialEq,
        Eq,
        MaxEncodedLen,
        Debug
    )]
    pub enum HashAlgo {
        Sha256 = 1,
        Blake2b256 = 2,
    }

    /// Pallet configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Max number of articles per publisher.
        #[pallet::constant]
        type MaxArticlesPerPublisher: Get<u32>;
        /// The overarching runtime event type.
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
    }

    /// Bounded vector for storing a publisher's article hashes.
    pub type BoundedArticleList<T> = BoundedVec<
        ContentHash,
        <T as Config>::MaxArticlesPerPublisher
    >;

    /// Record representing a single article's provenance and metadata.
    #[derive(TypeInfo, codec::Encode, codec::Decode, Clone, PartialEq, Eq, MaxEncodedLen, Debug)]
    #[scale_info(skip_type_params(T))]
    pub struct ArticleRecord<T: Config> {
        /// AccountId of the publisher (author) of the article.
        pub publisher: T::AccountId,
        /// NFT Collection ID to which this article belongs.
        pub collection_id: CollectionId,
        /// Item ID within the NFT collection.
        pub item_id: ItemId,
        /// Hash of the article content.
        pub content_hash: ContentHash,
        /// Optional root hash for article sections.
        pub section_root: Option<SectionRoot>,
        /// Signature over the content hash.
        pub signature: MultiSignature,
        /// Hash algorithm used.
        pub hash_algo: HashAlgo,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Maps content hash to article record.
    #[pallet::storage]
    pub type ArticleByHash<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ContentHash,
        ArticleRecord<T>,
        OptionQuery
    >;

    /// Maps (collection_id, item_id) to content hash.
    #[pallet::storage]
    pub type RootByItem<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        CollectionId,
        Blake2_128Concat,
        ItemId,
        ContentHash,
        OptionQuery
    >;

    /// Maps publisher to bounded list of their article hashes.
    #[pallet::storage]
    pub type ArticlesByPublisher<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedArticleList<T>,
        ValueQuery
    >;

    /// Events emitted by the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emitted when a new article is recorded.
        ArticleRecorded {
            publisher: T::AccountId,
            content_hash: ContentHash,
        },
        /// Emitted when an article's signature is verified.
        ArticleVerified {
            content_hash: ContentHash,
            valid: bool,
        },
    }

    /// Errors returned by the pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Article with this content hash already exists.
        ArticleAlreadyExists,
        /// Article not found for the given hash.
        ArticleNotFound,
        /// Publisher has reached the max number of articles.
        PublisherArticleListFull,
        /// Signature is invalid or does not match publisher.
        SignatureInvalid,
        /// Unable to parse the provided signature.
        UnableToParseSignature,
        /// Publisher's AccountId is not 32 bytes (required for sr25519).
        AccountIdNot32Bytes,
    }

    /// Hooks for the pallet.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Record a new article fingerprint and provenance.
        ///
        /// Checks that the article does not already exist, and stores all provenance data.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn record_article(
            origin: OriginFor<T>,
            content_hash: ContentHash,
            section_root: Option<SectionRoot>,
            collection_id: CollectionId,
            item_id: ItemId,
            signature: MultiSignature,
            hash_algo: HashAlgo
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure article does not already exist
            ensure!(
                !ArticleByHash::<T>::contains_key(&content_hash),
                Error::<T>::ArticleAlreadyExists
            );

            // Create the article record
            let record = ArticleRecord::<T> {
                publisher: who.clone(),
                collection_id,
                item_id,
                content_hash,
                section_root,
                signature: signature.clone(),
                hash_algo,
            };

            // Insert into ArticleByHash storage
            ArticleByHash::<T>::insert(&content_hash, &record);

            // Map (collection_id, item_id) to content_hash
            RootByItem::<T>::insert(collection_id, item_id, content_hash);

            // Add article hash to publisher's bounded list
            ArticlesByPublisher::<T>::try_mutate(&who, |list| {
                if list.len() < (T::MaxArticlesPerPublisher::get() as usize) {
                    list.try_push(content_hash).map_err(|_| Error::<T>::PublisherArticleListFull)?;
                    Ok(())
                } else {
                    Err(Error::<T>::PublisherArticleListFull)
                }
            })?;

            // Emit event for successful recording
            Self::deposit_event(Event::ArticleRecorded { publisher: who, content_hash });

            Ok(())
        }

        /// Verify an article's signature.
        ///
        /// Checks that the signature stored for the article is valid for the publisher and content hash.
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn verify_article(
            origin: OriginFor<T>,
            content_hash: ContentHash,
            publisher: T::AccountId
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // Fetch the article record
            let record = ArticleByHash::<T>::get(&content_hash).ok_or(Error::<T>::ArticleNotFound)?;

            // Verify the signature for the article
            let valid = Self::verify_signature(
                &record.content_hash,
                &record.signature.encode(),
                &publisher
            )?;

            // Emit event with verification result
            Self::deposit_event(Event::ArticleVerified { content_hash, valid });

            // Fail if signature is not valid
            ensure!(valid, Error::<T>::SignatureInvalid);

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Verifies an sr25519 signature on a content hash, assuming AccountId is a 32-byte public key.
        ///
        /// Returns Ok(true) if the signature is valid, Ok(false) if not, or an error if parsing fails.
        fn verify_signature(
            hash: &ContentHash,
            sig: &[u8],
            publisher: &T::AccountId
        ) -> Result<bool, Error<T>> {
            // Try to parse the signature as sr25519
            let signature: AnySignature = sr25519::Signature
                ::try_from(sig)
                .map_err(|_| Error::<T>::UnableToParseSignature)?
                .into();

            // Decode the publisher's AccountId as a sr25519 public key
            let public: sp_core::sr25519::Public = Decode::decode(
                &mut &publisher.encode()[..]
            ).map_err(|_| Error::<T>::AccountIdNot32Bytes)?;

            // Verify the signature over the content hash bytes
            Ok(signature.verify(hash.as_bytes(), &public))
        }
    }
}