// src/tests.rs
#![cfg(test)]

use crate::{mock::*, pallet::{Error, Event as StudentEvent}};
use frame::testing_prelude::*;

// Tạo sinh viên thành công
#[test]
fn create_student_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(StudentPallet::create_student(
            RuntimeOrigin::signed(1),
            *b"ABCD",
            20,
            10
        ));

        let student = StudentPallet::map_person_slice(1).unwrap();
        assert_eq!(student.name, *b"ABCD");
        assert_eq!(student.age, 20);
        assert_eq!(student.grade, 10);

        System::assert_last_event(StudentEvent::CreatedStudent { account: 1 }.into());
    });
}

// Không cho tạo trùng
#[test]
fn create_student_fails_when_already_exists() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(StudentPallet::create_student(RuntimeOrigin::signed(1), *b"ABCD", 20, 10));

        assert_noop!(
            StudentPallet::create_student(RuntimeOrigin::signed(1), *b"XYZ ", 21, 11),
            Error::<Test>::StudentExisted
        );
    });
}

// Cập nhật thành công (chính chủ update)
#[test]
fn update_student_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(StudentPallet::create_student(RuntimeOrigin::signed(1), *b"NGUY", 18, 8));
        assert_ok!(StudentPallet::update_student(RuntimeOrigin::signed(1), 25, 15));

        let student = StudentPallet::map_person_slice(1).unwrap();
        assert_eq!(student.age, 25);
        assert_eq!(student.grade, 15);
        assert_eq!(student.name, *b"NGUY");

        System::assert_last_event(StudentEvent::UpdatedStudent { account: 1 }.into());
    });
}

// Không cho cập nhật khi chưa tạo
#[test]
fn update_student_fails_when_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            StudentPallet::update_student(RuntimeOrigin::signed(999), 30, 12),
            Error::<Test>::NotFoundStudent
        );
    });
}

// Name không đổi
#[test]
fn update_keeps_original_name() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let name = *b"HOAN";

        assert_ok!(StudentPallet::create_student(RuntimeOrigin::signed(42), name, 19, 9));
        assert_ok!(StudentPallet::update_student(RuntimeOrigin::signed(42), 22, 10));

        assert_eq!(StudentPallet::map_person_slice(42).unwrap().name, name);
    });
}

// Origin::none bị từ chối
#[test]
fn create_student_fails_with_none_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            StudentPallet::create_student(RuntimeOrigin::none(), *b"FAIL", 20, 10),
            DispatchError::BadOrigin
        );
    });
}

// Giá trị max
#[test]
fn create_student_with_max_values() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(StudentPallet::create_student(
            RuntimeOrigin::signed(7),
            *b"MAXV",
            u16::MAX,
            u8::MAX
        ));

        let s = StudentPallet::map_person_slice(7).unwrap();
        assert_eq!(s.age, u16::MAX);
        assert_eq!(s.grade, u8::MAX);
    });
}

// Tên đặc biệt
#[test]
fn create_student_with_special_name() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let name = *b"OK!!";

        assert_ok!(StudentPallet::create_student(RuntimeOrigin::signed(100), name, 30, 15));
        assert_eq!(StudentPallet::map_person_slice(100).unwrap().name, name);
    });
}

// Storage giữ nguyên
#[test]
fn student_persists_after_creation() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(StudentPallet::create_student(RuntimeOrigin::signed(5), *b"KEEP", 18, 9));

        System::set_block_number(999);
        assert!(StudentPallet::map_person_slice(5).is_some());
    });
}

// Happy path đầy đủ — CHỈ CHÍNH CHỦ MỚI UPDATE ĐƯỢC
#[test]
fn full_happy_path() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(StudentPallet::create_student(RuntimeOrigin::signed(10), *b"TEST", 20, 10));
        System::assert_last_event(StudentEvent::CreatedStudent { account: 10 }.into());

        assert_ok!(StudentPallet::update_student(RuntimeOrigin::signed(10), 21, 11));
        System::assert_last_event(StudentEvent::UpdatedStudent { account: 10 }.into());

        assert_ok!(StudentPallet::update_student(RuntimeOrigin::signed(10), 99, 1));
        System::assert_last_event(StudentEvent::UpdatedStudent { account: 10 }.into());

        let s = StudentPallet::map_person_slice(10).unwrap();
        assert_eq!(s.name, *b"TEST");
        assert_eq!(s.age, 99);
        assert_eq!(s.grade, 1);
    });
}