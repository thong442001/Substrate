use crate::{mock::*, Error};
use frame::testing_prelude::*;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TightlycouplingPallet::access_on_chain_pallet_template(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		//assert_eq!(Something::<Test>::get().map(|v| v.block_number), Some(42));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TightlycouplingPallet::update_on_chain_pallet_template(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
