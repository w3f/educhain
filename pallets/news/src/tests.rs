use crate::{ mock::*, ArticleByHash, Error, HashAlgo };
use frame::testing_prelude::*;
use frame::deps::sp_core::{ sr25519, Pair, H256 };
use frame::deps::sp_runtime::{ AccountId32, MultiSignature };

fn make_test_signature(pair: &sr25519::Pair, hash: &H256) -> MultiSignature {
    let sig = pair.sign(hash.as_bytes());
    MultiSignature::from(sig)
}

#[test]
fn record_article_works() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let content_hash = H256::repeat_byte(42);
        let section_root: Option<H256> = Some(H256::repeat_byte(99));
        let collection_id = 1u128;
        let item_id = 2u128;
        let signature = make_test_signature(&pair, &content_hash);

        // Record article
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(pair.public().into()),
                content_hash,
                section_root,
                collection_id,
                item_id,
                signature.clone(),
                HashAlgo::Blake2b256
            )
        );

        // Should not allow duplicate
        assert_noop!(
            News::record_article(
                RuntimeOrigin::signed(pair.public().into()),
                content_hash,
                section_root,
                collection_id,
                item_id,
                signature.clone(),
                HashAlgo::Sha256
            ),
            Error::<Test>::ArticleAlreadyExists
        );
        assert_eq!(ArticleByHash::<Test>::contains_key(content_hash), true);
    });
}

#[test]
fn verify_article_works() {
    new_test_ext().execute_with(|| {
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let content_hash = H256::repeat_byte(42);
        let section_root: Option<H256> = Some(H256::repeat_byte(99));
        let collection_id = 1u128;
        let item_id = 2u128;
        let signature = make_test_signature(&pair, &content_hash);
        let publisher: AccountId32 = pair.public().into();

        // Record article first
        assert_ok!(
            News::record_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                section_root,
                collection_id,
                item_id,
                signature.clone(),
                HashAlgo::Blake2b256
            )
        );

        // Now verify the article
        assert_ok!(
            News::verify_article(
                RuntimeOrigin::signed(publisher.clone()),
                content_hash,
                publisher.clone()
            )
        );
    });
}
