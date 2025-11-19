// pallets/kitties/src/tests.rs
#![cfg(test)]

use crate::{mock::*, pallet::*};
use frame::testing_prelude::*;

type DNA = [u8; 16];

// Helper tạo DNA nhanh
fn dna_from_seed(seed: u8) -> DNA {
    [seed; 16]
}

#[test]
fn can_create_kitty() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1); // Đặt block > 0
        let dna = dna_from_seed(1);

        assert_ok!(KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna));

        // Kiểm tra kitty tồn tại
        assert!(KittiesPallet::get_kitty(&dna).is_some());
        let kitty = KittiesPallet::get_kitty(&dna).unwrap();
        assert_eq!(kitty.owner, 1);
        assert_eq!(kitty.price, 0);
        assert_eq!(kitty.gender, Gender::Male); // vì length = 16 (chẵn)

        // Kiểm tra KittiesOwned
        let owned = KittiesPallet::kitty_owned(1);
        assert_eq!(owned.len(), 1);
        assert_eq!(owned[0], dna);

        // Kiểm tra ID tăng
        assert_eq!(KittiesPallet::kitty_id(), 1);

        // Kiểm tra event
        System::assert_last_event(
            Event::Created { kitty: dna, owner: 1 }.into()
        );
    });
}

#[test]
fn reject_duplicate_kitty() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1); // Đặt block > 0
        let dna = dna_from_seed(42);

        assert_ok!(KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna));
        assert_noop!(
            KittiesPallet::create_kitty(RuntimeOrigin::signed(2), dna),
            Error::<Test>::DuplicateKitty
        );
    });
}

#[test]
fn reject_when_too_many_kitties() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1); // Đặt block > 0
        // Tạo đủ 100 con cho Alice (account 1)
        for i in 0..100u8 {
            let dna = dna_from_seed(i);
            assert_ok!(KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna));
        }

        // Con thứ 101 → bị từ chối
        let dna = dna_from_seed(255);
        assert_noop!(
            KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna),
            Error::<Test>::TooManyKitties
        );

        // Bob vẫn tạo được bình thường
        assert_ok!(KittiesPallet::create_kitty(RuntimeOrigin::signed(2), dna));
    });
}

#[test]
fn kitty_id_overflow_protected() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1); // Đặt block > 0
        // Đặt ID hiện tại = u32::MAX
        <KittyId<Test>>::put(u32::MAX);

        let dna = dna_from_seed(99);
        assert_noop!(
            KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna),
            Error::<Test>::OverFlow
        );
    });
}

#[test]
fn gender_is_correct() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1); // Đặt block > 0
        let dna_even = [0u8; 16]; // length = 16 → chẵn → Male
        assert_ok!(KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna_even));
        let kitty = KittiesPallet::get_kitty(&dna_even).unwrap();
        assert_eq!(kitty.gender, Gender::Male);

        // Dù thay đổi nội dung, length vẫn 16 → vẫn Male
        let mut dna2 = [1u8; 16];
        dna2[0] = 255;
        assert_ok!(KittiesPallet::create_kitty(RuntimeOrigin::signed(1), dna2));
        let kitty2 = KittiesPallet::get_kitty(&dna2).unwrap();
        assert_eq!(kitty2.gender, Gender::Male);
    });
}