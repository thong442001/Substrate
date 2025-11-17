// pallets/student/src/tests.rs

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RuntimeOrigin;

// Định nghĩa Pallet theo tên trong mock.rs để code rõ ràng hơn
type Pallet = crate::Pallet<Test>;

// Helper function để tạo mảng [u8; 4] cho test
const TEST_NAME: [u8; 4] = *b"JOHN"; 

// =========================================================================
// TEST CHO HÀM create_student
// =========================================================================

#[test]
fn create_student_works_success() {
    new_test_ext().execute_with(|| {
        let account: u64 = 1;
        let age: u16 = 20;
        let grade: u8 = 8;
        
        // 1. Gửi extrinsic tạo sinh viên
        assert_ok!(Pallet::create_student(
            RuntimeOrigin::signed(account),
            TEST_NAME,
            age,
            grade
        ));
        
        // 2. Kiểm tra sự kiện đã được phát ra
        System::assert_has_event(RuntimeEvent::StudentPallet(
            crate::Event::CreatedStudent { account: 1 }
        ));

        // 3. Kiểm tra Storage (sử dụng getter fn map_person_slice)
        let maybe_student = Pallet::map_person_slice(account); 
        
        assert!(maybe_student.is_some());
        let student = maybe_student.unwrap();

        // 4. Khẳng định các giá trị đúng
        assert_eq!(Student.name, TEST_NAME);
        assert_eq!(Student.age, age);
        assert_eq!(Student.grade, grade);
    });
}

#[test]
fn create_student_fails_if_already_existed() {
    new_test_ext().execute_with(|| {
        let account: u64 = 1;
        
        // Lần 1: Tạo thành công
        assert_ok!(Pallet::create_student(
            RuntimeOrigin::signed(account),
            TEST_NAME, 20, 8
        ));
        
        // Lần 2: Thất bại vì đã tồn tại (StudentExisted)
        assert_noop!(
            Pallet::create_student(RuntimeOrigin::signed(account), *b"ANNA", 21, 9),
            Error::<Test>::StudentExisted
        );
    });
}

// =========================================================================
// TEST CHO HÀM update_student
// =========================================================================

#[test]
fn update_student_works_success() {
    new_test_ext().execute_with(|| {
        let account: u64 = 1;
        let new_age: u16 = 25;
        let new_grade: u8 = 10;
        
        // 1. Chuẩn bị: Tạo sinh viên trước
        assert_ok!(Pallet::create_student(
            RuntimeOrigin::signed(account),
            TEST_NAME, 20, 8
        ));
        
        // 2. Cập nhật thông tin
        assert_ok!(Pallet::update_student(
            RuntimeOrigin::signed(account),
            new_age,
            new_grade
        ));

        // 3. Kiểm tra sự kiện đã được phát ra
        System::assert_has_event(RuntimeEvent::StudentPallet(
            crate::Event::UpdatedStudent { account: 1 }
        ));

        // 4. Kiểm tra Storage đã được cập nhật
        let student = Pallet::map_person_slice(account).unwrap();
        
        assert_eq!(Student.age, new_age);
        assert_eq!(Student.grade, new_grade);
    });
}

#[test]
fn update_student_fails_if_not_found() {
    new_test_ext().execute_with(|| {
        let account: u64 = 2; // Tài khoản chưa tồn tại

        // Thử cập nhật sinh viên chưa có
        assert_noop!(
            Pallet::update_student(RuntimeOrigin::signed(account), 25, 10),
            Error::<Test>::NotFoundStudent
        );
    });
}