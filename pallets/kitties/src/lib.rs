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
    
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // Define Kitties
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        pub dna: [u8;16],
        pub price: u64,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Gender {
        Male,
        Female,
    }

    // KittyId storage
    #[pallet::storage]
    #[pallet::getter(fn kitty_id)]
    pub(super) type KittyId<T: Config> = StorageValue<_, u32, ValueQuery>;

    // Kitties storage
    #[pallet::storage]
    #[pallet::getter(fn get_kitty)]
    pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, [u8;16], Kitty<T>, OptionQuery>;

    // KittiesOwned storage: mỗi user có tối đa 100 kitty
    #[pallet::storage]
    #[pallet::getter(fn kitty_owned)]
    pub(super) type KittiesOwned<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<[u8;16], ConstU32<100>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Created { kitty: [u8;16], owner: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        DuplicateKitty,
        OverFlow,
        TooManyKitties,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_kitty(origin: OriginFor<T>, dna: [u8;16]) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // generate gender
            let gender = Self::gen_gender(&dna)?;

            // check duplicate
            ensure!(
                !Kitties::<T>::contains_key(&dna),
                Error::<T>::DuplicateKitty
            );

            // define new kitty
            let new_kitty = Kitty::<T> {
                dna,
                price: 0,
                gender,
                owner: owner.clone(),
            };

            // get current kitty id
            let current_kitty_id = Self::kitty_id();

            // increase kitty id
            let new_kitty_id = current_kitty_id.checked_add(1).ok_or(Error::<T>::OverFlow)?;

            // append new kitty to KittiesOwned with bounded vec
            KittiesOwned::<T>::try_mutate(&owner, |kitty_list| {
                kitty_list.try_push(dna).map_err(|_| Error::<T>::TooManyKitties)
            })?;

            // write new kitty to storage
            Kitties::<T>::insert(&dna, new_kitty);

            // update kitty id
            KittyId::<T>::put(new_kitty_id);

            // deposit event
            Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

            Ok(())
        }
    }

    impl<T> Pallet<T> {
    fn gen_gender(dna: &[u8;16]) -> Result<Gender, Error<T>> {
        if dna.len() % 2 == 0 {
            Ok(Gender::Male)
        } else {
            Ok(Gender::Female)
        }
    }
    }

}

