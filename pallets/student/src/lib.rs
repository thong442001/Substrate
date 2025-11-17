#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame::pallet]
pub mod pallet {
	use frame::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// A type representing the weights required by the dispatchables of this pallet.
		//type WeightInfo: crate::weights::WeightInfo;
	}

	// Định nghĩa struct Student
	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Default, Debug)]
	pub struct Student {
		name: [u8; 4],
		age: u16,
		grade: u8,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn map_person_slice)]
	pub type Students<T: Config> = StorageMap<_, Blake2_128, T::AccountId, Student, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// thong bao khi tao hoac cap nhat sinh vien
		CreatedStudent { account: T::AccountId },
		UpdatedStudent { account: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		// Error khi sinh vien da ton tai
		StudentExisted,
		NotFoundStudent,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn create_student(
			origin: OriginFor<T>,
			name: [u8; 4],
			age: u16,
			grade: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Kiểm tra sinh viên đã tồn tại chưa
			ensure!(!Students::<T>::contains_key(&who), Error::<T>::StudentExisted);

			// Tạo sinh viên mới
			let student = Student { name, age, grade };

			// Lưu vào storage
			Students::<T>::insert(&who, student);

			// Emit event
			Self::deposit_event(Event::CreatedStudent { account: who });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn update_student(origin: OriginFor<T>, age: u16, grade: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Students::<T>::try_mutate(&who, |maybe_student| -> DispatchResult {
				let student = maybe_student.as_mut().ok_or(Error::<T>::NotFoundStudent)?;

				student.age = age;
				student.grade = grade;

				Ok(())
			})?;

			Self::deposit_event(Event::UpdatedStudent { account: who });

			Ok(())
		}
	}
}
