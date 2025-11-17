#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[derive(Encode, Decode, MaxEncodedLen, Debug, Clone, PartialEq, Eq, TypeInfo)]
    pub struct Student {
        pub name: [u8; 4],
        pub age: u16,
        pub grade: u8,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn map_person_slice)]
    pub type Students<T: Config> = StorageMap<_, Blake2_128, T::AccountId, Student, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CreatedStudent { account: T::AccountId },
        UpdatedStudent { account: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
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
            ensure!(!Students::<T>::contains_key(&who), Error::<T>::StudentExisted);
            Students::<T>::insert(&who, Student { name, age, grade });
            Self::deposit_event(Event::CreatedStudent { account: who });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn update_student(origin: OriginFor<T>, age: u16, grade: u8) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Students::<T>::try_mutate(&who, |opt| -> DispatchResult {
                let s = opt.as_mut().ok_or(Error::<T>::NotFoundStudent)?;
                s.age = age;
                s.grade = grade;
                Ok(())
            })?;
            Self::deposit_event(Event::UpdatedStudent { account: who });
            Ok(())
        }
    }
}