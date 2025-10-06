//! Benchmarking setup for pallet-news

use super::*;
use frame::{ deps::frame_benchmarking::v2::*, prelude::* };
use frame::deps::sp_core::H256;
use frame::deps::sp_keyring::Sr25519Keyring;
use frame::deps::sp_runtime::{ MultiSignature, MultiSigner, traits::IdentifyAccount, AccountId32 };
use frame::deps::sp_io::crypto::{ sr25519_generate, sr25519_sign };

/// Benchmark helper trait for signature creation
pub trait BenchmarkHelper<Signature, AccountId> {
    fn create_signature(content_hash: &H256) -> (Signature, AccountId);
}

impl BenchmarkHelper<MultiSignature, AccountId32> for () {
    fn create_signature(content_hash: &H256) -> (MultiSignature, AccountId32) {
        // Wrap content_hash in <Bytes></Bytes> tags for signature verification
        let mut wrapped_msg = b"<Bytes>".to_vec();
        wrapped_msg.extend_from_slice(content_hash.as_bytes());
        wrapped_msg.extend_from_slice(b"</Bytes>");
        
        let public = sr25519_generate(0.into(), None);
        let who_account: AccountId32 = MultiSigner::Sr25519(public).into_account().into();
        let signature = MultiSignature::Sr25519(sr25519_sign(0.into(), &public, &wrapped_msg).unwrap());
        (signature, who_account)
    }
}

#[benchmarks(where
    T: Config + Send + Sync,
    T::AccountId: From<AccountId32>,
)]
mod benchmarks {
    use super::*;
    use crate::pallet::Pallet as News;
    use frame_system::RawOrigin;

    #[benchmark]
    fn record_article() {
        let content_hash = H256::repeat_byte(42);
        let collection_id = 1u128;
        let item_id = 2u128;
        let title = BoundedVec::<u8, T::MaxTitleLen>
            ::try_from(b"Benchmark Title".to_vec())
            .unwrap();
        let canonical_url = BoundedVec::<u8, T::MaxUrlLen>
            ::try_from(b"https://benchmark.com".to_vec())
            .unwrap();

        let (signature, signer_account32) = T::BenchmarkHelper::create_signature(&content_hash);
        let caller: T::AccountId = signer_account32.into();
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

    #[benchmark]
    fn update_article() {
        let content_hash = H256::repeat_byte(42);
        let new_hash = H256::repeat_byte(43);
        let collection_id = 1u128;
        let item_id = 2u128;
        let title = BoundedVec::<u8, T::MaxTitleLen>
            ::try_from(b"Benchmark Title".to_vec())
            .unwrap();
        let canonical_url = BoundedVec::<u8, T::MaxUrlLen>
            ::try_from(b"https://benchmark.com".to_vec())
            .unwrap();

        let (signature, signer_account32) = T::BenchmarkHelper::create_signature(&content_hash);
        let (new_signature, _) = T::BenchmarkHelper::create_signature(&new_hash);
        let caller: T::AccountId = signer_account32.into();
        let word_count = 123u32;
        let new_word_count = 456u32;

        // First, record the initial article
        let _ = News::<T>::record_article(
            RawOrigin::Signed(caller.clone()).into(),
            content_hash,
            collection_id,
            item_id,
            title.clone(),
            canonical_url.clone(),
            signature,
            HashAlgo::Blake2b256,
            word_count
        );

        // Now, benchmark the update
        #[extrinsic_call]
        update_article(
            RawOrigin::Signed(caller),
            content_hash,
            new_hash,
            title.clone(),
            canonical_url.clone(),
            new_signature,
            HashAlgo::Blake2b256,
            new_word_count
        );

        assert!(ArticleByHash::<T>::get(new_hash).is_some());
    }

    impl_benchmark_test_suite!(News, crate::mock::new_test_ext(), crate::mock::Test);
}