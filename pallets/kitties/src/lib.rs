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
    // Vec<u8> không có giới hạn kích thước nên không thể tự động tạo thông tin lưu trữ (MaxEncodedLen)
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // Define Kitties
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        pub dna: Vec<u8>,
        pub price: u64,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    // Define Gender
    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Gender {
        Male,
        Female,
    }

    // TODO : Define KittyId storage 
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub(super) type KittyId<T: Config> =StorageValue<_, u32, ValueQuery>;

	//TODO : Define Kitties storage + OptionQuery
    // key is DNA => Vec<u8>
    // value is Kitty<T>
	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	//TODO : Define KittiesOwned storage + ValueQuery
	#[pallet::storage]
	#[pallet::getter(fn kitty_owned)]
	pub(super) type KittiesOwned<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;


    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { kitty: Vec<u8>, owner: T::AccountId },

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		OverFlow,

	}

   #[pallet::call]
	impl<T: Config> Pallet<T> {
       #[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;

			//TODO : generate gender 
            let gender = Self::gen_gender(&dna)?;

			// TODO: Check if the kitty does not already exist in our storage map
			// using ensure!
            ensure!(
                !Kitties::<T>::contains_key(&dna),
                Error::<T>::DuplicateKitty
            );
			// return DuplicateKitty if error
            // TODO: define new kitty 
            let new_kitty = Kitty::<T> {
                dna: dna.clone(),
                price: 0,
                gender,
                owner: owner.clone()
            };

			// TODO: Get current kitty id 
            let current_kitty_id = Self::kitty_id();
			
			// TODO: Increase kitty Id by 1 (if overflow return OverFlow)
            let new_kitty_id = current_kitty_id.checked_add(1).ok_or(Error::<T>::OverFlow)?;

			// TODO: Append new kitty to KittiesOwned
            // let mut dnas = KittiesOwned::<T>::get(&owner);
            // dnas.push(dna.clone());
            // KittiesOwned::<T>::insert(&owner, dnas);
            KittiesOwned::<T>::append(&owner, dna.clone());

			// TODO: Write new kitty to storage
            Kitties::<T>::insert(&dna, new_kitty);

			// TODO: Write new kitty id 
            KittyId::<T>::put(new_kitty_id);

			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone()});

			Ok(())
        }
    }
}

impl<T> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender,Error<T>>{
		if dna.len()%2 ==0 {
            return Ok(Gender::Male);
        }else {
            return Ok(Gender::Female);
        }
	}
}