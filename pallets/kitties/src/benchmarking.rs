//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	// Ten cua benchmark
	create_kitty {
		// Khoi tao cac tham so cho extrinsic benchmark
		let dna: Vec<u8> = b"Tommy".to_vec();
		let caller: T::AccountId = whitelisted_caller();

	}: create_kitty(RawOrigin::Signed(caller), dna)

	// Kiem tra lai trang thai storage khi thuc hien extrinsic xem dung chua
	verify {
		assert_eq!(NumberKitties::<T>::get(), 1);
	}

	// Ten cua benchmark
	transfer_kitty {
		// Khoi tao cac tham so cho extrinsic benchmark
		let dna: Vec<u8> = b"Tommy".to_vec();
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin: <T as frame_system::Config>::Origin = RawOrigin::Signed(caller.clone()).into();

		Kitties::<T>::create_kitty(caller_origin, dna.clone());

		let kitty = Owner::<T>::get(&caller);

		let receiver: T::AccountId = account("receiver", 0, 0);

	}: transfer_kitty(RawOrigin::Signed(caller), receiver.clone(), dna)

	// Kiem tra lai trang thai storage khi thuc hien extrinsic xem dung chua
	verify {
		assert_eq!(Owner::<T>::get(&receiver), kitty);
	}

	// Thuc hien benchmark voi mock runtime, storage ban dau
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
