#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame::pallet]
pub mod pallet {
	use frame::{prelude::*, token::currency};

	#[pallet::config]
	// Tương tác giữa các Pallets
	// Tightly Coupling: 2 Pallet phụ thuộc lẫn nhau.
	pub trait Config: frame_system::Config + pallet_parachain_template::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// #[derive(
	// 	Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
	// )]
	// #[scale_info(skip_type_params(T))]
	// pub struct CompositeStruct<T: Config> {
	// 	pub(crate) block_number: BlockNumberFor<T>,
	// }

	// #[pallet::storage]
	// pub type Something<T: Config> = StorageValue<_, CompositeStruct<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingAccess { something: u32, who: T::AccountId },
		SomethingUpdate { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	// #[pallet::hooks]
	// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn access_on_chain_pallet_template(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			
			let who = ensure_signed(origin)?;

			// Gọi pallet template để lưu block number
			let value_from_template = pallet_parachain_template::Pallet::<T>::number().unwrap_or_default();

			// Gửi sự kiện có block number của pallet template
			Self::deposit_event(Event::SomethingAccess { something: value_from_template, who });

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn update_on_chain_pallet_template(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			
			let who = ensure_signed(origin)?;

			// update storage from pallet template	
			pallet_parachain_template::Pallet::<T>::update_storage(something)?;
			
			// Gửi sự kiện có block number của pallet template
			Self::deposit_event(Event::SomethingUpdate { something: something, who });

			Ok(().into())
		}
	}
}

