#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
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
	pub trait Config: frame_system::Config{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(
		Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct CompositeStruct<T: Config> {
		pub block_number: BlockNumberFor<T>,
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T: Config> = StorageValue<_, CompositeStruct<T>>;

	#[pallet::storage]
	#[pallet::getter(fn number)]
	pub type Number<T: Config> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { block_number: BlockNumberFor<T>, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, bn: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let block_number: BlockNumberFor<T> = bn.into();

			<Something<T>>::put(CompositeStruct { block_number });

			Self::deposit_event(Event::SomethingStored { block_number, who });

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			match <Something<T>>::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(mut old) => {
					old.block_number = old
						.block_number
						.checked_add(&One::one())
						.ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(old);
					Ok(().into())
				},
			}
		}

	}
}

// Helper functions
impl<T: Config> Pallet<T> {
	pub fn update_storage(new_value: u32) -> DispatchResult {
		Something::<T>::put(CompositeStruct { block_number: new_value.into() });
		Ok(())
	}
}

pub trait ConfigHelper {
	fn get_something() -> Option<u32>;
	fn set_something(new_value: u32) -> DispatchResult;
}

impl<T: Config> ConfigHelper for Pallet<T> {
	fn get_something() -> Option<u32> {
		Something::<T>::get().map(|s| s.block_number.into().as_u32())
	}

	fn set_something(new_value: u32) -> DispatchResult {
		Something::<T>::put(CompositeStruct { block_number: new_value.into() });
		Ok(())
	}
}
