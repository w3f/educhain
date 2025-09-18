//! # Pallet News Provenance
//!
//! Records and tracks provenance of news articles with simple, valuable utility:
//! - Immutable anchor hash per article (content_hash == anchor_hash)
//! - Compact version history (append-only list of hashes per anchor)
//! - Signature is stored (for off-chain verification), not verified on-chain
//!
//! ## Storage
//! - `ArticleByHash`: Content hash (any version) -> ArticleRecord
//! - `RootByItem`: (collection_id, item_id) -> anchor hash (or latest depending on your policy)
//! - `ArticlesByPublisher`: Publisher -> bounded list of their article anchor hashes (first versions)
//! - `AnchorOf`: Any version hash -> anchor hash (first version)
//! - `HistoryByAnchor`: Anchor hash -> bounded, chronological list of all version hashes (including anchor)
//! - `ArticleUpdateCount`: Anchor hash -> updates count (version number of the latest record)
//!
//! ## Calls
//! - `record_article`: Create a new article (first version). Its content_hash is the anchor.
//! - `update_article`: Append a new version (new hash) to the existing article lineage. Only publisher can update.
//!
//! ## Events
//! - ArticleRecorded, ArticleUpdated, ArticleHistoryExtended, ArticleVerified
//!
//! ## Notes
//! - Signatures are stored in `ArticleRecord.signature` for **off-chain** verification.
//! - On-chain verification of signatures is intentionally omitted to keep runtime cost low.

#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pallet::*;

// FRAME test scaffolding
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod weights;

#[frame::pallet]
pub mod pallet {
    use frame::{ deps::sp_runtime::{ AccountId32, MultiSignature }, prelude::*, traits::Verify };

    /// Unique identifier for a collection of articles (optional NFT linkage).
    pub type CollectionId = u128;
    /// Unique identifier for an item within a collection (optional NFT linkage).
    pub type ItemId = u128;
    /// Hash of the article content. For the first version, this is the **anchor hash**.
    pub type ContentHash = H256;

    /// Supported hash algorithms for article content.
    #[derive(
        TypeInfo,
        DecodeWithMemTracking,
        Encode,
        Decode,
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

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Max number of anchor articles per publisher.
        #[pallet::constant]
        type MaxArticlesPerPublisher: Get<u32>;
        /// Maximum length of a single article's version history (including the anchor).
        #[pallet::constant]
        type MaxHistoryLen: Get<u32>;
        /// Maximum length of article title
        #[pallet::constant]
        type MaxTitleLen: Get<u32>;
        /// Maximum length of canonical URL
        #[pallet::constant]
        type MaxUrlLen: Get<u32>;
        /// The overall runtime event type.
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weights.
        type WeightInfo: crate::weights::WeightInfo;
    }

    /// Bounded vector for storing a publisher's article **anchors** (first versions).
    pub type BoundedArticleList<T> = BoundedVec<
        ContentHash,
        <T as Config>::MaxArticlesPerPublisher
    >;

    /// Record representing a single article version (including anchor).
    #[derive(TypeInfo, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, Debug)]
    #[scale_info(skip_type_params(T))]
    pub struct ArticleRecord<T: Config> {
        /// Title of the article.
        pub title: BoundedVec<u8, T::MaxTitleLen>,
        /// Canonical URL of the article.
        pub canonical_url: BoundedVec<u8, T::MaxUrlLen>,
        /// AccountId of the publisher (author/owner).
        pub publisher: T::AccountId,
        /// Optional NFT Collection linkage.
        pub collection_id: CollectionId,
        /// Optional NFT Item linkage.
        pub item_id: ItemId,
        /// Hash of this version's content (anchor for first version).
        pub content_hash: ContentHash,
        /// Signature over the content hash (stored for off-chain verification; not verified on-chain).
        pub signature: MultiSignature,
        /// Hash algorithm used.
        pub hash_algo: HashAlgo,
        /// Word count of the article (this version).
        pub word_count: u32,
        /// Block number of last update for this version record.
        pub last_updated_at: BlockNumberFor<T>,
        /// How many updates have been made to the article up to and including this version.
        /// For anchor: 0, for next version: 1, etc.
        pub updates: u32,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Maps content hash (any version) to its record.
    #[pallet::storage]
    pub type ArticleByHash<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ContentHash,
        ArticleRecord<T>,
        OptionQuery
    >;

    /// Maps (collection_id, item_id) to anchor hash (you may update this to latest in `update_article` if desired).
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

    /// Maps publisher to a bounded list of their **anchor** article hashes.
    #[pallet::storage]
    pub type ArticlesByPublisher<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedArticleList<T>,
        ValueQuery
    >;

    /// For any article version hash, point to the lineage anchor (the first root).
    #[pallet::storage]
    pub type AnchorOf<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ContentHash, // any version hash
        ContentHash, // anchor hash (first version)
        OptionQuery
    >;

    /// For each anchor, keep the ordered list of version hashes (anchor first).
    #[pallet::storage]
    pub type HistoryByAnchor<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ContentHash, // anchor hash
        BoundedVec<ContentHash, T::MaxHistoryLen>,
        ValueQuery
    >;

    /// Updates count per anchor (latest version number for that lineage).
    #[pallet::storage]
    pub type ArticleUpdateCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ContentHash, // anchor hash
        u32, // updates count
        ValueQuery
    >;

    /// Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emitted when a new anchor article is recorded.
        ArticleRecorded {
            publisher: T::AccountId,
            content_hash: ContentHash, // also anchor
        },
        /// Emitted when an article's ownership is "verified" (publisher matches record).
        ArticleVerified {
            content_hash: ContentHash,
            publisher: T::AccountId,
            valid: bool,
        },
        /// Emitted on update (new version). `updates` is the new version count.
        ArticleUpdated {
            publisher: T::AccountId,
            old_hash: ContentHash,
            new_hash: ContentHash,
            updates: u32,
        },
        /// Emitted whenever history timeline grows (append-only).
        ArticleHistoryExtended {
            anchor: ContentHash,
            new_hash: ContentHash,
            index: u32, // 0-based index in HistoryByAnchor[anchor]
        },
    }

    /// Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Article with this content hash already exists.
        ArticleAlreadyExists,
        /// Article not found for the given hash.
        ArticleNotFound,
        /// Publisher's anchor list is full.
        PublisherArticleListFull,
        /// History too long for this anchor (MaxHistoryLen hit).
        HistoryTooLong,
        /// Caller is not the article's publisher / owner.
        NotArticlePublisher,
        /// Signature verification failed.
        AccountIdNot32Bytes,
        /// Signature verification failed.
        SignatureInvalid,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Record a new article (first version).
        ///
        /// - `content_hash` is the **anchor hash**.
        /// - **Verifies** the provided `signature` matches the extrinsic signer over `content_hash`.
        /// - Initializes history with the anchor.
        #[pallet::call_index(0)]
        #[pallet::weight(5000)]
        pub fn record_article(
            origin: OriginFor<T>,
            content_hash: ContentHash,
            collection_id: CollectionId,
            item_id: ItemId,
            title: BoundedVec<u8, T::MaxTitleLen>,
            canonical_url: BoundedVec<u8, T::MaxUrlLen>,
            signature: MultiSignature,
            hash_algo: HashAlgo,
            word_count: u32
        ) -> DispatchResult {
            let publisher = ensure_signed(origin)?;

            // Ensure content hash is unique
            Self::ensure_content_hash_unique(&content_hash)?;
            
            // Verify signature
            Self::verify_signature(&publisher, &content_hash, &signature)?;

            // Create and store article record
            let record = Self::create_article_record(
                publisher.clone(), 
                content_hash, 
                collection_id, 
                item_id, 
                title, 
                canonical_url, 
                signature, 
                hash_algo, 
                word_count, 
                0, // Initial updates count
            );
            Self::store_new_article(record, publisher, content_hash, collection_id, item_id)?;

            Ok(())
        }

        /// Update an existing article by adding a new version with a new `content_hash`.
        ///
        /// - Only the **original publisher** can update.
        /// - Appends the new hash to the anchor's history and increments update counter.
        #[pallet::call_index(1)]
        #[pallet::weight(5000)]
        pub fn update_article(
            origin: OriginFor<T>,
            old_hash: ContentHash,
            new_hash: ContentHash,
            title: BoundedVec<u8, T::MaxTitleLen>,
            canonical_url: BoundedVec<u8, T::MaxUrlLen>,
            new_signature: MultiSignature,
            hash_algo: HashAlgo,
            new_word_count: u32
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Old record must exist and be owned by caller
            let old = ArticleByHash::<T>::get(&old_hash).ok_or(Error::<T>::ArticleNotFound)?;
            ensure!(old.publisher == who, Error::<T>::NotArticlePublisher);

            // New must be unique
            ensure!(!ArticleByHash::<T>::contains_key(&new_hash), Error::<T>::ArticleAlreadyExists);

            // Determine anchor
            let anchor = AnchorOf::<T>::get(&old_hash).unwrap_or(old_hash);

            // Compute new updates = previous + 1
            let updates = old.updates.saturating_add(1);

            // Materialize new version record
            let new_rec = ArticleRecord::<T> {
                title,
                canonical_url,
                publisher: who.clone(),
                collection_id: old.collection_id,
                item_id: old.item_id,
                content_hash: new_hash,
                signature: new_signature, // stored; off-chain verify only
                hash_algo,
                word_count: new_word_count,
                last_updated_at: <frame_system::Pallet<T>>::block_number(),
                updates,
            };

            // Insert new record
            ArticleByHash::<T>::insert(&new_hash, &new_rec);

            // Map new version to anchor
            AnchorOf::<T>::insert(&new_hash, anchor);

            // Append to history (bounded)
            let index = HistoryByAnchor::<T>::try_mutate(anchor, |timeline| {
                let idx = timeline.len() as u32;
                timeline.try_push(new_hash).map_err(|_| Error::<T>::HistoryTooLong)?;
                Ok::<u32, Error<T>>(idx)
            })?;

            // Update lineage counter
            ArticleUpdateCount::<T>::insert(&anchor, updates);

            // (Optional policy) Update RootByItem to latest; comment out to keep pointing at anchor
            RootByItem::<T>::insert(old.collection_id, old.item_id, new_hash);

            // Events
            Self::deposit_event(Event::ArticleUpdated {
                publisher: who.clone(),
                old_hash,
                new_hash,
                updates,
            });
            Self::deposit_event(Event::ArticleHistoryExtended {
                anchor,
                new_hash,
                index,
            });

            Ok(())
        }
    }

    // Helper functions outside the dispatch section
    impl<T: Config> Pallet<T> {
        /// Ensures that an article with the given content hash does not already exist
        fn ensure_content_hash_unique(content_hash: &ContentHash) -> DispatchResult {
            ensure!(
                !ArticleByHash::<T>::contains_key(content_hash),
                Error::<T>::ArticleAlreadyExists
            );
            Ok(())
        }
        
        /// Verifies that the signature provided by the publisher is valid for the content hash
        fn verify_signature(
            publisher: &T::AccountId, 
            content_hash: &ContentHash, 
            signature: &MultiSignature
        ) -> DispatchResult {
            let bytes = publisher.encode();
            let account_id32 = AccountId32::try_from(&bytes[..])
                .map_err(|_| Error::<T>::AccountIdNot32Bytes)?;

            // Wrap content_hash in <Bytes></Bytes> tags for verification
            let mut wrapped_msg = b"<Bytes>".to_vec();
            wrapped_msg.extend_from_slice(content_hash.as_bytes());
            wrapped_msg.extend_from_slice(b"</Bytes>");
            
            let ok = signature.verify(&wrapped_msg[..], &account_id32);
            ensure!(ok, Error::<T>::SignatureInvalid);
            
            Ok(())
        }
        
        /// Creates a new ArticleRecord instance with the given parameters
        fn create_article_record(
            publisher: T::AccountId,
            content_hash: ContentHash,
            collection_id: CollectionId,
            item_id: ItemId,
            title: BoundedVec<u8, T::MaxTitleLen>,
            canonical_url: BoundedVec<u8, T::MaxUrlLen>,
            signature: MultiSignature,
            hash_algo: HashAlgo,
            word_count: u32,
            updates: u32,
        ) -> ArticleRecord<T> {
            ArticleRecord::<T> {
                title,
                canonical_url,
                publisher,
                collection_id,
                item_id,
                content_hash,
                signature,
                hash_algo,
                word_count,
                last_updated_at: <frame_system::Pallet<T>>::block_number(),
                updates,
            }
        }
        
        /// Stores a new article, updates necessary indices, and emits events
        fn store_new_article(
            record: ArticleRecord<T>, 
            publisher: T::AccountId,
            content_hash: ContentHash,
            collection_id: CollectionId,
            item_id: ItemId,
        ) -> DispatchResult {
            // Persist the article record
            ArticleByHash::<T>::insert(&content_hash, &record);

            // Link to NFT root
            RootByItem::<T>::insert(collection_id, item_id, content_hash);

            // Track publisher's anchors
            ArticlesByPublisher::<T>::try_mutate(&publisher, |list| {
                list.try_push(content_hash).map_err(|_| Error::<T>::PublisherArticleListFull)
            })?;

            // Initialize history tracking
            AnchorOf::<T>::insert(content_hash, content_hash);
            HistoryByAnchor::<T>::try_mutate(content_hash, |timeline| {
                timeline.try_push(content_hash).map_err(|_| Error::<T>::HistoryTooLong)
            })?;
            ArticleUpdateCount::<T>::insert(content_hash, 0);

            // Emit event
            Self::deposit_event(Event::ArticleRecorded {
                publisher,
                content_hash,
            });
            
            Ok(())
        }
    }
}
