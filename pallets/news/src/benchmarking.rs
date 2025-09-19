//! Benchmarking setup for pallet-news


use super::*;
use frame::{ deps::frame_benchmarking::v2::*, prelude::* };
use frame::deps::sp_core::{ sr25519, Pair, H256, H512 };
use frame::deps::sp_runtime::MultiSignature;

#[benchmarks]
mod benchmarks {
    use super::*;
    use crate::pallet::Pallet as News;
    use frame_system::RawOrigin;
    use frame::deps::sp_core::{ sr25519, H256 };
    use frame::deps::sp_runtime::MultiSignature;

    fn make_test_signature(pair: &sr25519::Pair, hash: &H256) -> MultiSignature {
        // Wrap the hash in <Bytes></Bytes> tags for signature verification
        let mut wrapped_msg = b"<Bytes>".to_vec();
        wrapped_msg.extend_from_slice(hash.as_bytes());
        wrapped_msg.extend_from_slice(b"</Bytes>");

        // Use the signing method available in this version
        let sig = sr25519::Signature::from(pair.sign(&wrapped_msg));
        MultiSignature::from(sig)
    }

    #[benchmark]
    fn record_article() {
        let caller: T::AccountId = whitelisted_caller();
        let pair = sr25519::Pair::from_seed(&[1u8; 32]);
        let content_hash = H256::repeat_byte(42);
        let collection_id = 1u128;
        let item_id = 2u128;
        let title = BoundedVec::<u8, T::MaxTitleLen>
            ::try_from(b"Benchmark Title".to_vec())
            .unwrap();
        let canonical_url = BoundedVec::<u8, T::MaxUrlLen>
            ::try_from(b"https://benchmark.com".to_vec())
            .unwrap();
        let signature = make_test_signature(&pair, &content_hash);
        let word_count = 123u32;

        #[extrinsic_call]
        record_article(
            RawOrigin::Signed(caller),
            content_hash,
            collection_id,
            item_id,
            title.clone(),
            canonical_url.clone(),
            signature,
            HashAlgo::Blake2b256,
            word_count
        );
        assert!(ArticleByHash::<T>::get(content_hash).is_some());
    }

    // #[benchmark]
    // fn update_article() {
    //     let caller: T::AccountId = whitelisted_caller();
    //     let pair = sr25519::Pair::from_seed(&[1u8; 32]);
    //     let content_hash = H256::repeat_byte(42);
    //     let new_hash = H256::repeat_byte(43);
    //     let collection_id = 1u128;
    //     let item_id = 2u128;
    //     let title = BoundedVec::<u8, T::MaxTitleLen>
    //         ::try_from(b"Benchmark Title".to_vec())
    //         .unwrap();
    //     let canonical_url = BoundedVec::<u8, T::MaxUrlLen>
    //         ::try_from(b"https://benchmark.com".to_vec())
    //         .unwrap();
    //     let signature = make_test_signature(&pair, &content_hash);
    //     let new_signature = make_test_signature(&pair, &new_hash);
    //     let word_count = 123u32;
    //     let new_word_count = 456u32;

    //     // First, record the initial article
    //     record_article(
    //         RawOrigin::Signed(caller.clone()),
    //         content_hash,
    //         collection_id,
    //         item_id,
    //         title.clone(),
    //         canonical_url.clone(),
    //         signature.clone(),
    //         HashAlgo::Blake2b256,
    //         word_count
    //     );

    //     // Now, benchmark the update
    //     #[extrinsic_call]
    //     update_article(
    //         RawOrigin::Signed(caller),
    //         content_hash,
    //         new_hash,
    //         title.clone(),
    //         canonical_url.clone(),
    //         new_signature.clone(),
    //         HashAlgo::Blake2b256,
    //         new_word_count
    //     );

    //     assert!(ArticleByHash::<T>::get(new_hash).is_some());
    // }

    impl_benchmark_test_suite!(News, crate::mock::new_test_ext(), crate::mock::Test);
}
