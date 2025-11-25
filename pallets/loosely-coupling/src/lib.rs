#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

pub use pallet::*;
use pallet_parachain_template::ConfigHelper;

#[frame::pallet]
pub mod pallet {
	use super::*;
	//use frame::{prelude::*, token::currency};

	#[pallet::config]
	// Tương tác giữa các Pallets
	// Tightly Coupling: 2 Pallet phụ thuộc lẫn nhau.
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type TemplateConfigHelper: ConfigHelper;

	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingAccess { something: u32, who: T::AccountId },
		SomethingUpdate { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn access_on_chain_pallet_template(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			
			let who = ensure_signed(origin)?;

			// Gọi pallet template để lưu block number
			let value_from_template = <<T as Config>::TemplateConfigHelper>::get_something().unwrap_or_default();

			// Gửi sự kiện có block number của pallet template
			Self::deposit_event(Event::SomethingAccess { something: value_from_template, who });

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn update_on_chain_pallet_template(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			
			let who = ensure_signed(origin)?;

			// update storage from pallet template	
			<<T as Config>::TemplateConfigHelper>::set_something(something)?;
			
			// Gửi sự kiện có block number của pallet template
			Self::deposit_event(Event::SomethingUpdate { something: something, who });

			Ok(().into())
		}
	}
}
