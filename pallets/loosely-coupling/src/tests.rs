use crate::{mock::*, Error, Event};
use frame::testing_prelude::*;
use pallet_parachain_template::Pallet as TemplatePallet;

#[test]
fn access_on_chain_pallet_template_works() {
	new_test_ext().execute_with(|| {

		// FIX: phải set block > 0
        System::set_block_number(1);

		let alice = 1; 
        let input_value = 42;

        // Gọi extrinsic đầu tiên
        assert_ok!(LooselycouplingPallet::access_on_chain_pallet_template(
            RuntimeOrigin::signed(alice),
            input_value
        ));

        // Kiểm tra event đã emit đúng
        System::assert_last_event(
            Event::SomethingAccess {
                something: 0, // vì pallet template chưa có giá trị → trả về 0
                who: alice,
            }
            .into(),
        );
	});
}

#[test]
fn update_on_chain_pallet_template_works() {
    new_test_ext().execute_with(|| {

		 // FIX: phải set block > 0
        System::set_block_number(1);

        let alice = 1;
        let new_value = 999;

        // Gọi extrinsic update
        assert_ok!(LooselycouplingPallet::update_on_chain_pallet_template(
            RuntimeOrigin::signed(alice),
            new_value
        ));

        // Kiểm tra storage đã được cập nhật đúng
        let stored = pallet_parachain_template::Something::<Test>::get()
            .expect("should have value after update");
        assert_eq!(stored.block_number, 999);

        // Kiểm tra event
        System::assert_last_event(
            Event::SomethingUpdate {
                something: new_value,
                who: alice,
            }
            .into(),
        );
    });
}