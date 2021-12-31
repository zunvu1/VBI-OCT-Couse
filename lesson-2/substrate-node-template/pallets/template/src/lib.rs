#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	/*traits::ReservableCurrency*/
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};

	use frame_system::pallet_prelude::*;

	// pallet configuration
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// type Currency: ReservableCurrency<Self::AccountId>;
	}

	// pallet event
	#[pallet::event]
	// #[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Deposited { who: T::AccountId, amount: u32 },

		Withdrawn { who: T::AccountId, amount: u32 },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidAmount,

		NotEnough,

		NoSuchOwner,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Todo Change storage parameters
	#[pallet::storage]
	pub(super) type DbankBalance<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

	// optional hooks
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn deposit_money(origin: OriginFor<T>, amount: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(amount > 0, Error::<T>::InvalidAmount);

			if <DbankBalance<T>>::contains_key(&sender) {
				<DbankBalance<T>>::mutate(&sender, |deposited| deposited.unwrap() + amount);
			} else {
				<DbankBalance<T>>::insert(&sender, &amount);
			}

			Self::deposit_event(Event::<T>::Deposited { who: sender, amount });

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn withdraw_money(origin: OriginFor<T>, amount: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(<DbankBalance<T>>::contains_key(&sender), Error::<T>::NoSuchOwner);

			ensure!(amount > 0, Error::<T>::InvalidAmount);

			ensure!(<DbankBalance<T>>::get(&sender).unwrap() >= amount, Error::<T>::InvalidAmount);

			<DbankBalance<T>>::mutate(&sender, |deposited| deposited.unwrap() - amount);

			Self::deposit_event(Event::<T>::Withdrawn { who: sender, amount });

			Ok(().into())
		}
	}
}
