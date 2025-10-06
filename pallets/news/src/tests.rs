use crate::{ 
    mock::*, 
    ArticleByHash, 
    ArticlesByPublisher,
    AnchorOf,
    HistoryByAnchor,
    ArticleUpdateCount,
    RootByItem,
    Error, 
    HashAlgo 
};
use frame::testing_prelude::*;
use frame::deps::sp_core::{ sr25519, Pair, H256 };
use frame::deps::sp_runtime::{ AccountId32, MultiSignature };

fn make_test_signature(pair: &sr25519::Pair, hash: &H256) -> MultiSignature {
    // Wrap the hash in <Bytes></Bytes> tags for signature verification
    let mut wrapped_msg = b"<Bytes>".to_vec();
    wrapped_msg.extend_from_slice(hash.as_bytes());
    wrapped_msg.extend_from_slice(b"</Bytes>");
    
    let sig = pair.sign(&wrapped_msg[..]);
    MultiSignature::from(sig)
}

fn create_test_article_data() -> (
    H256, // content_hash
    u128, // collection_id
    u128, // item_id
    BoundedVec<u8, <Test as crate::Config>::MaxTitleLen>, // title
    BoundedVec<u8, <Test as crate::Config>::MaxUrlLen>, // canonical_url
    u32, // word_count
) {
    let content_hash = H256::repeat_byte(42);
    let collection_id = 1u128;
    let item_id = 2u128;
    let title = BoundedVec::<u8, <Test as crate::Config>::MaxTitleLen>::try_from(b"Test Title".to_vec()).unwrap();
    let canonical_url = BoundedVec::<u8, <Test as crate::Config>::MaxUrlLen>::try_from(b"https://example.com".to_vec()).unwrap();
    let word_count = 123;
    
    (content_hash, collection_id, item_id, title, canonical_url, word_count)
}

#[test]
fn record_article_works() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let (content_hash, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        let signature = make_test_signature(&pair, &content_hash);
        let publisher: AccountId32 = pair.public().into();

        // Record article
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                signature.clone(),
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Verify ArticleByHash storage
        let article = ArticleByHash::<Test>::get(content_hash).unwrap();
        assert_eq!(article.publisher, publisher);
        assert_eq!(article.content_hash, content_hash);
        assert_eq!(article.title, title);
        assert_eq!(article.canonical_url, canonical_url);
        assert_eq!(article.word_count, word_count);
        assert_eq!(article.updates, 0);

        // Verify RootByItem storage
        assert_eq!(RootByItem::<Test>::get(collection_id, item_id), Some(content_hash));

        // Verify ArticlesByPublisher storage
        let publisher_articles = ArticlesByPublisher::<Test>::get(&publisher);
        assert_eq!(publisher_articles.len(), 1);
        assert_eq!(publisher_articles[0], content_hash);

        // Verify AnchorOf storage (should point to itself for anchor)
        assert_eq!(AnchorOf::<Test>::get(content_hash), Some(content_hash));

        // Verify HistoryByAnchor storage
        let history = HistoryByAnchor::<Test>::get(content_hash);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], content_hash);

        // Verify ArticleUpdateCount storage
        assert_eq!(ArticleUpdateCount::<Test>::get(content_hash), 0);

        // Should not allow duplicate
        assert_noop!(
            News::record_article(
                RuntimeOrigin::signed(publisher),
                content_hash,
                collection_id,
                item_id,
                title,
                canonical_url,
                signature,
                HashAlgo::Sha256,
                word_count
            ),
            Error::<Test>::ArticleAlreadyExists
        );
    });
}

#[test]
fn update_article_works() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let (content_hash, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        let signature = make_test_signature(&pair, &content_hash);
        let publisher: AccountId32 = pair.public().into();

        // First, record the initial article
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                signature,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Create update data
        let new_hash = H256::repeat_byte(43);
        let new_signature = make_test_signature(&pair, &new_hash);
        let new_title = BoundedVec::<u8, <Test as crate::Config>::MaxTitleLen>::try_from(b"Updated Title".to_vec()).unwrap();
        let new_canonical_url = BoundedVec::<u8, <Test as crate::Config>::MaxUrlLen>::try_from(b"https://updated.com".to_vec()).unwrap();
        let new_word_count = 456;

        // Update the article
        assert_ok!(
            News::update_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash, // old hash
                new_hash,     // new hash
                new_title.clone(),
                new_canonical_url.clone(),
                new_signature,
                HashAlgo::Blake2b256,
                new_word_count
            )
        );

        // Verify new article record exists
        let new_article = ArticleByHash::<Test>::get(new_hash).unwrap();
        assert_eq!(new_article.publisher, publisher);
        assert_eq!(new_article.content_hash, new_hash);
        assert_eq!(new_article.title, new_title);
        assert_eq!(new_article.canonical_url, new_canonical_url);
        assert_eq!(new_article.word_count, new_word_count);
        assert_eq!(new_article.updates, 1);

        // Verify original article still exists
        let original_article = ArticleByHash::<Test>::get(content_hash).unwrap();
        assert_eq!(original_article.updates, 0);

        // Verify RootByItem now points to latest version
        assert_eq!(RootByItem::<Test>::get(collection_id, item_id), Some(new_hash));

        // Verify ArticlesByPublisher unchanged (only anchors tracked)
        let publisher_articles = ArticlesByPublisher::<Test>::get(&publisher);
        assert_eq!(publisher_articles.len(), 1);
        assert_eq!(publisher_articles[0], content_hash); // Still the anchor

        // Verify AnchorOf for both versions
        assert_eq!(AnchorOf::<Test>::get(content_hash), Some(content_hash)); // anchor -> anchor
        assert_eq!(AnchorOf::<Test>::get(new_hash), Some(content_hash)); // new -> anchor

        // Verify HistoryByAnchor updated
        let history = HistoryByAnchor::<Test>::get(content_hash);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], content_hash); // anchor first
        assert_eq!(history[1], new_hash);     // new version second

        // Verify ArticleUpdateCount updated
        assert_eq!(ArticleUpdateCount::<Test>::get(content_hash), 1);
    });
}

#[test]
fn update_article_fails_for_non_publisher() {
    new_test_ext().execute_with(|| {
        let pair1 = sr25519::Pair::from_seed(&[1u8; 32]);
        let pair2 = sr25519::Pair::from_seed(&[2u8; 32]);
        let (content_hash, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        let signature = make_test_signature(&pair1, &content_hash);
        let publisher1: AccountId32 = pair1.public().into();
        let publisher2: AccountId32 = pair2.public().into();

        // Record article with publisher1
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher1),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                signature,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Try to update with publisher2 (should fail)
        let new_hash = H256::repeat_byte(43);
        let new_signature = make_test_signature(&pair2, &new_hash);
        
        assert_noop!(
            News::update_article(
                RuntimeOrigin::signed(publisher2),
                content_hash,
                new_hash,
                title,
                canonical_url,
                new_signature,
                HashAlgo::Blake2b256,
                456
            ),
            Error::<Test>::NotArticlePublisher
        );
    });
}

#[test]
fn multiple_updates_create_proper_history() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let (content_hash, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        let signature = make_test_signature(&pair, &content_hash);
        let publisher: AccountId32 = pair.public().into();

        // Record initial article
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                signature,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Create multiple updates
        let hash2 = H256::repeat_byte(43);
        let hash3 = H256::repeat_byte(44);
        let sig2 = make_test_signature(&pair, &hash2);
        let sig3 = make_test_signature(&pair, &hash3);

        // First update
        assert_ok!(
            News::update_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                hash2,
                title.clone(),
                canonical_url.clone(),
                sig2,
                HashAlgo::Blake2b256,
                200
            )
        );

        // Second update (from hash2 to hash3)
        assert_ok!(
            News::update_article(
                RuntimeOrigin::signed(publisher.clone()),
                hash2,
                hash3,
                title.clone(),
                canonical_url.clone(),
                sig3,
                HashAlgo::Blake2b256,
                300
            )
        );

        // Verify history chain
        let history = HistoryByAnchor::<Test>::get(content_hash);
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], content_hash); // anchor
        assert_eq!(history[1], hash2);        // first update
        assert_eq!(history[2], hash3);        // second update

        // Verify all versions point to same anchor
        assert_eq!(AnchorOf::<Test>::get(content_hash), Some(content_hash));
        assert_eq!(AnchorOf::<Test>::get(hash2), Some(content_hash));
        assert_eq!(AnchorOf::<Test>::get(hash3), Some(content_hash));

        // Verify update counts
        assert_eq!(ArticleByHash::<Test>::get(content_hash).unwrap().updates, 0);
        assert_eq!(ArticleByHash::<Test>::get(hash2).unwrap().updates, 1);
        assert_eq!(ArticleByHash::<Test>::get(hash3).unwrap().updates, 2);

        // Verify final update count
        assert_eq!(ArticleUpdateCount::<Test>::get(content_hash), 2);

        // Verify RootByItem points to latest
        assert_eq!(RootByItem::<Test>::get(collection_id, item_id), Some(hash3));
    });
}

#[test]
fn multiple_publishers_tracked_separately() {
    new_test_ext().execute_with(|| {
        let pair1 = sr25519::Pair::from_seed(&[1u8; 32]);
        let pair2 = sr25519::Pair::from_seed(&[2u8; 32]);
        let publisher1: AccountId32 = pair1.public().into();
        let publisher2: AccountId32 = pair2.public().into();

        // Publisher 1 creates article
        let hash1 = H256::repeat_byte(41);
        let sig1 = make_test_signature(&pair1, &hash1);
        let (_, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher1.clone()),
                hash1,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                sig1,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Publisher 2 creates different article
        let hash2 = H256::repeat_byte(42);
        let sig2 = make_test_signature(&pair2, &hash2);
        
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher2.clone()),
                hash2,
                collection_id + 1,
                item_id + 1,
                title.clone(),
                canonical_url.clone(),
                sig2,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Publisher 1 creates another article
        let hash3 = H256::repeat_byte(43);
        let sig3 = make_test_signature(&pair1, &hash3);
        
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher1.clone()),
                hash3,
                collection_id + 2,
                item_id + 2,
                title.clone(),
                canonical_url.clone(),
                sig3,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Verify publisher tracking
        let publisher1_articles = ArticlesByPublisher::<Test>::get(&publisher1);
        let publisher2_articles = ArticlesByPublisher::<Test>::get(&publisher2);

        assert_eq!(publisher1_articles.len(), 2);
        assert!(publisher1_articles.contains(&hash1));
        assert!(publisher1_articles.contains(&hash3));

        assert_eq!(publisher2_articles.len(), 1);
        assert!(publisher2_articles.contains(&hash2));
    });
}

#[test]
fn signature_verification_works() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let wrong_pair = sr25519::Pair::from_seed(&[2u8; 32]);
        let (content_hash, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        
        // Create signature with wrong key
        let wrong_signature = make_test_signature(&wrong_pair, &content_hash);
        let publisher: AccountId32 = pair.public().into();

        // Should fail with wrong signature
        assert_noop!(
            News::record_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                wrong_signature,
                HashAlgo::Blake2b256,
                word_count
            ),
            Error::<Test>::SignatureInvalid
        );

        // Should work with correct signature
        let correct_signature = make_test_signature(&pair, &content_hash);
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher),
                content_hash,
                collection_id,
                item_id,
                title,
                canonical_url,
                correct_signature,
                HashAlgo::Blake2b256,
                word_count
            )
        );
    });
}

#[test]
fn update_nonexistent_article_fails() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let publisher: AccountId32 = pair.public().into();
        let nonexistent_hash = H256::repeat_byte(99);
        let new_hash = H256::repeat_byte(100);
        let new_signature = make_test_signature(&pair, &new_hash);
        let (_, _, _, title, canonical_url, _) = create_test_article_data();

        assert_noop!(
            News::update_article(
                RuntimeOrigin::signed(publisher),
                nonexistent_hash,
                new_hash,
                title,
                canonical_url,
                new_signature,
                HashAlgo::Blake2b256,
                456
            ),
            Error::<Test>::ArticleNotFound
        );
    });
}

#[test]
fn update_with_existing_hash_fails() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let (content_hash, collection_id, item_id, title, canonical_url, word_count) = create_test_article_data();
        let signature = make_test_signature(&pair, &content_hash);
        let publisher: AccountId32 = pair.public().into();

        // Record initial article
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                signature,
                HashAlgo::Blake2b256,
                word_count
            )
        );

        // Try to update with the same hash (should fail)
        let same_signature = make_test_signature(&pair, &content_hash);
        assert_noop!(
            News::update_article(
                RuntimeOrigin::signed(publisher),
                content_hash,
                content_hash, // same hash
                title,
                canonical_url,
                same_signature,
                HashAlgo::Blake2b256,
                456
            ),
            Error::<Test>::ArticleAlreadyExists
        );
    });
}
