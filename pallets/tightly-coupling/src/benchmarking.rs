#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as TightlyCouplingPallet, *};
use frame::deps::frame_support::assert_ok;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

use pallet_parachain_template as template_pallet;

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as TightlyCouplingPallet;
    use frame_system::RawOrigin;

   #[benchmark]
    fn access_on_chain_pallet_template() {
        // SETUP
        let caller: T::AccountId = whitelisted_caller();

        // Put a known value vào pallet_parachain_template::Something
        template_pallet::Something::<T>::put(
            template_pallet::CompositeStruct {
                block_number: 123u32.into(), // Convert u32 -> BlockNumberFor<T> (U256)
            }
        );

        // CALL
        #[extrinsic_call]
        access_on_chain_pallet_template(RawOrigin::Signed(caller), 0u32);

        // VERIFY (không bắt buộc nhưng tốt)
        let got = template_pallet::Something::<T>::get().unwrap();
        assert_eq!(got.block_number, 123u32.into());
    }

	#[benchmark]
    fn update_on_chain_pallet_template() {
        let caller: T::AccountId = whitelisted_caller();

        // CALL
        #[extrinsic_call]
        update_on_chain_pallet_template(RawOrigin::Signed(caller.clone()), 555u32.into());

        // VERIFY — storage trong pallet template đã được update
        let stored = template_pallet::Something::<T>::get().unwrap();
        assert_eq!(stored.block_number,  555u32.into());
    }

	// If you want to run benchmarks as tests, provide a test suite
	impl_benchmark_test_suite!(
		Pallet,
		crate::mock::new_test_ext(), // nếu bạn có mock::new_test_ext()
		crate::tests::test_benchmarking // optional fn to call after
	);
}

