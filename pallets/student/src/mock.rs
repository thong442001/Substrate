// pallets/student/src/mock.rs

use super::*;
use frame::prelude::*;
use frame_support::{traits::{ConstU16, ConstU64}, parameter_types};

// Cấu hình tài khoản và Block cơ bản
type Block = frame_system::mocking::MockBlock<Test>;

// Cấu hình cho pallet system (Tối thiểu cần thiết)
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64; // Sử dụng u64 cho các tài khoản test (1, 2, 3...)
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type RuntimeTask = (); // Tạm thời dùng () nếu không cần task cụ thể
    type ExtensionsWeightInfo = (); // Tạm thời dùng ()
    type SingleBlockMigrations = (); 
    type MultiBlockMigrator = ();
    type PreInherents = (); 
    type PostInherents = (); 
    type PostTransactions = ();
}

// Cần Pallet Balances để tài khoản có thể tồn tại (Existential Deposit)
impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxHolds = ();
    type RuntimeHoldReason = ();
}

// TRIỂN KHAI CONFIG CHO PALLET STUDENT CỦA BẠN
impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
}

// Khởi tạo Runtime cho việc Testing.
frame_system::construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        StudentPallet: crate, // Tên Pallet trong Runtime
    }
);

// Hàm khởi tạo môi trường test
pub fn new_test_ext() -> sp_io::TestExternalities {
    // Thêm Genesis Config cho Balances để tài khoản có tiền
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // Khởi tạo Balances với tài khoản 1 (Alice) có 100 đơn vị tiền
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 100)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let ext = sp_io::TestExternalities::new(t);
    ext
}