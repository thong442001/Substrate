#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
pub use pallet::*;

#[frame::pallet]
pub mod pallet {
	use frame::prelude::*;
	use polkadot_sdk::frame_support::{traits::fungible::Inspect, pallet_prelude::*};
	use polkadot_sdk::frame_system::pallet_prelude::*;
	use polkadot_sdk::pallet_balances;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FreeBalance { balance: <T as pallet_balances::Config>::Balance, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10000)]
		pub fn call_balances_pallet(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// Lấy balance từ pallet-balances của polkadot-sdk
			let balance = <pallet_balances::Pallet<T> as Inspect<_>>::balance(&who);

			Self::deposit_event(Event::FreeBalance { balance, who });

			Ok(().into())
		}
	}
}
