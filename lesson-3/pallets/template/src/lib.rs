#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[macro_use]
extern crate log;

#[frame_support::pallet]
pub mod pallet {

	use std::{convert::TryInto, fmt};

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, Imbalance, WithdrawReasons},
	};

	use frame_system::pallet_prelude::*;

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	type BalanceOf<T> = <<T as Config>::MyCurrency as Currency<AccountIdOf<T>>>::Balance;

	// pallet configuration
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type MyCurrency: Currency<Self::AccountId>;
	}

	// pallet event
	#[pallet::event]
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
	pub(super) type DbankBalance<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

	// optional hooks
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn deposit_money(origin: OriginFor<T>, amount: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(amount > 0, Error::<T>::InvalidAmount);

			let val = T::MyCurrency::deposit_creating(&sender, amount.into());

			if <DbankBalance<T>>::contains_key(&sender) {
				<DbankBalance<T>>::mutate(&sender, |deposited| deposited.unwrap() + val.peek());
			} else {
				<DbankBalance<T>>::insert(&sender, val.peek());
			}

			Self::deposit_event(Event::<T>::Deposited { who: sender, amount });

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn withdraw_money(origin: OriginFor<T>, amount: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(<DbankBalance<T>>::contains_key(&sender), Error::<T>::NoSuchOwner);

			ensure!(amount > 0, Error::<T>::InvalidAmount);

			let withdrawal_amount = T::MyCurrency::withdraw(
				&sender,
				amount.into(),
				WithdrawReasons::RESERVE,
				ExistenceRequirement::KeepAlive,
			)?;

			<DbankBalance<T>>::mutate(&sender, |deposited| {
				deposited.unwrap() - withdrawal_amount.peek()
			});

			info!(
				"balance after withdrawing: {}",
				TryInto::<u32>::try_into(<DbankBalance<T>>::get(&sender)).ok().unwrap()
			);

			Self::deposit_event(Event::<T>::Withdrawn { who: sender, amount });

			Ok(().into())
		}
	}
}
