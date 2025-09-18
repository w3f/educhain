use crate::{ mock::*, ArticleByHash, Error, HashAlgo };
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

#[test]
fn record_article_works() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let content_hash = H256::repeat_byte(42);
        let collection_id = 1u128;
        let item_id = 2u128;
        let signature = make_test_signature(&pair, &content_hash);
        let word_count = 123;
        let title = BoundedVec::<u8, <Test as crate::Config>::MaxTitleLen>::try_from(b"Test Title".to_vec()).unwrap();
        let canonical_url = BoundedVec::<u8, <Test as crate::Config>::MaxUrlLen>::try_from(b"https://example.com".to_vec()).unwrap();

        // Record article
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(pair.public().into()),
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

        // Should not allow duplicate
        assert_noop!(
            News::record_article(
                RuntimeOrigin::signed(pair.public().into()),
                content_hash,
                collection_id,
                item_id,
                title.clone(),
                canonical_url.clone(),
                signature.clone(),
                HashAlgo::Sha256,
                word_count
            ),
            Error::<Test>::ArticleAlreadyExists
        );
        assert_eq!(ArticleByHash::<Test>::contains_key(content_hash), true);
    });
}
