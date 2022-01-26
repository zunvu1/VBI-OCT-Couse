#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate log;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::{dispatch::DispatchResultWithPostInfo, ensure, pallet_prelude::*};
	use frame_system::{ensure_signed, pallet_prelude::*};
	// use parity_scale_codec::EncodeLike;
	use scale_info::TypeInfo;
	use sp_std::fmt::Debug;

	use sp_std::prelude::Vec;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Car<T: Config> {
		pub chassis_num: Option<u32>,
		pub brand_name: Vec<u8>,
		pub price: Option<u32>,
		pub owner: AccountOf<T>,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, u32, Vec<u8>, u32),

		RevokeSuccessful(T::AccountId, u32, Vec<u8>, u32),

		TransferSuccessful(T::AccountId, u32, Vec<u8>, u32),
	}

	impl<T: Config> Debug for Car<T> {
		fn fmt(
			&self,
			f: &mut frame_support::dispatch::fmt::Formatter<'_>,
		) -> frame_support::dispatch::fmt::Result {
			f.debug_struct("Car")
				.field("chassis_num", &self.chassis_num)
				.field("brand_name", &self.brand_name)
				.field("price", &self.price)
				.field("owner", &self.owner)
				.finish()
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		NoSuchCarOwner,

		CarProofAlreadyExists,

		InsertCarProofFails,

		NotCarOwner,

		SameOwnerTransfer,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type CarStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Car<T>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			chassis_num: Option<u32>,
			brand_name: Vec<u8>,
			price: Option<u32>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(!<CarStorage<T>>::contains_key(&sender), <Error<T>>::CarProofAlreadyExists);

			let car = Self::create_car(&sender, chassis_num, brand_name.clone(), price)?;

			info!(
				"chassis_num: {}, brand_name: {:?}, price: {}",
				chassis_num.unwrap(),
				brand_name,
				price.unwrap()
			);

			<CarStorage<T>>::insert(&sender, &car);

			Self::deposit_event(Event::<T>::ClaimCreated(
				sender,
				chassis_num.unwrap(),
				brand_name,
				price.unwrap(),
			));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(<CarStorage<T>>::contains_key(&sender), <Error<T>>::NoSuchCarOwner);

			let car = <CarStorage<T>>::get(&sender).unwrap();

			ensure!(car.owner == sender, Error::<T>::NotCarOwner);

			<CarStorage<T>>::remove(&sender);

			Self::deposit_event(<Event<T>>::RevokeSuccessful(
				sender,
				car.chassis_num.unwrap(),
				car.brand_name,
				car.price.unwrap(),
			));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			des: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(sender != des, Error::<T>::SameOwnerTransfer);

			ensure!(<CarStorage<T>>::contains_key(&sender), <Error<T>>::NoSuchCarOwner);

			// ensure des has no car ownership
			ensure!(!<CarStorage<T>>::contains_key(&des), <Error<T>>::NoSuchCarOwner);

			let car = <CarStorage<T>>::get(&sender).unwrap();

			<CarStorage<T>>::remove(&sender);

			<CarStorage<T>>::insert(&des, &car);

			Self::deposit_event(<Event<T>>::TransferSuccessful(
				sender,
				car.chassis_num.unwrap(),
				car.brand_name,
				car.price.unwrap(),
			));

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn create_car(
			owner: &T::AccountId,
			chassis_num: Option<u32>,
			brand_name: Vec<u8>,
			price: Option<u32>,
		) -> Result<Car<T>, Error<T>> {
			let car = Car::<T> { owner: owner.clone(), chassis_num, brand_name, price };

			Ok(car)
		}
	}
}
