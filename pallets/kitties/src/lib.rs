#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::dispatch::fmt;
use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;
use frame_support::traits::Get;
use frame_support::traits::Time;
use frame_system::pallet_prelude::*;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type TimeStamp<T> = <<T as Config>::Time as Time>::Moment;

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	#[derive(TypeInfo, Default, Decode, Encode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: BalanceOf<T>,
		gender: Gender,
		created_date: TimeStamp<T>,
	}

	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("dna", &self.dna)
				.field("owner", &self.owner)
				.field("price", &self.price)
				.field("gender", &self.gender)
				.field("created_date", &self.created_date)
				.finish()
		}
	}

	pub type Id = u32;

	#[derive(TypeInfo, Decode, Encode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		type Time: Time;

		#[pallet::constant]
		type KittiesLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn number_kitties)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type NumberKitties<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub(super) type Owner<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<Vec<u8>, T::KittiesLimit>,
		OptionQuery,
	>;

	// #[pallet::storage]
	// #[pallet::getter(fn limit_kitties)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// pub type LimitKitties<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters.
		KittyStored(Vec<u8>),

		KittyTransferedTo(T::AccountId, Vec<u8>),

		KittiesLimitSet(u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidPrice,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		KittyNotOwned,

		NoKitty,

		InvalidAccount,

		Overflow,

		ExceedLimit,

		DuplicateKitty,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, Vec<u8>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { kitties: Vec::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			NumberKitties::<T>::put(self.kitties.len() as u32);
			for (owner, kitty) in self.kitties.iter() {
				let mut kitties_owned = Owner::<T>::get(owner).unwrap_or_default();
				kitties_owned.try_push(kitty.clone()).unwrap();
				Owner::<T>::insert(owner, kitties_owned);
				let item = Kitty::<T> {
					dna: kitty.clone(),
					owner: owner.clone(),
					price: 0u32.into(),
					gender: Pallet::<T>::get_gender(kitty.clone()).unwrap(),
					created_date: T::Time::now(),
				};
				Kitties::<T>::insert(kitty, item);
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(29_336_000 + T::DbWeight::get().reads_writes(4, 3))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins

			let who = ensure_signed(origin)?;

			let mut kitties_owned = Owner::<T>::get(&who).unwrap_or_default();

			ensure!(
				((kitties_owned.clone()).len() as u32) < T::KittiesLimit::get(),
				Error::<T>::ExceedLimit
			);

			log::info!("Total balance: {:?}", T::Currency::total_balance(&who));

			let gender = Self::get_gender(dna.clone())?;

			log::info!("Created time {:?}", T::Time::now());

			let kitty = Kitty::<T> {
				dna: dna.clone(),
				owner: who.clone(),
				price: 0u32.into(),
				gender,
				created_date: T::Time::now(),
			};

			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			log::info!("Kitty: {:?}", kitty);

			// let mut vec_kitties = <Owner<T>>::get(who.clone());

			// Store kitty into owner's list
			// match <Owner<T>>::get(who.clone()) {
			// 	Some(mut x) => {
			// 		x.push(dna.clone());
			// 		<Owner<T>>::insert(who, x);
			// 	},
			// 	None => {
			// 		let mut vec_kitties = Vec::new();
			// 		vec_kitties.push(dna.clone());
			// 		<Owner<T>>::insert(who, vec_kitties);
			// 	},
			// }

			kitties_owned.try_push(kitty.dna.clone()).map_err(|_| Error::<T>::NoKitty)?;
			Owner::<T>::insert(&who, kitties_owned);

			// Store information of kitty

			<Kitties<T>>::insert(dna.clone(), kitty);

			// Update number of kitties

			let mut current_id = <NumberKitties<T>>::get();

			current_id += 1;

			<NumberKitties<T>>::put(current_id);

			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			account: T::AccountId,
			dna: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			// Get kitties of sender
			let mut sender_kitties = <Owner<T>>::get(who.clone()).unwrap();
			// Check ownership of kitty
			ensure!(sender_kitties.contains(&dna), Error::<T>::KittyNotOwned);
			// Get kitties of receiver
			let mut receiver_kitties = <Owner<T>>::get(account.clone()).unwrap_or_default();

			ensure!(
				((receiver_kitties.clone()).len() as u32) < T::KittiesLimit::get(),
				Error::<T>::ExceedLimit
			);

			// Transfer kitty from sender to receiver
			receiver_kitties.try_push(dna.clone()).map_err(|_| Error::<T>::ExceedLimit)?;
			sender_kitties.retain(|x| x != &dna);
			<Owner<T>>::insert(who, sender_kitties);
			<Owner<T>>::insert(account.clone(), receiver_kitties);

			// Update information of kitty
			let mut kitty_update = <Kitties<T>>::get(dna.clone()).unwrap();
			kitty_update.owner = account.clone();
			<Kitties<T>>::insert(dna.clone(), kitty_update);

			// Emit an event.
			Self::deposit_event(Event::KittyTransferedTo(account, dna));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn set_limit(origin: OriginFor<T>, limit: u32) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/v3/runtime/origins
		// 	let _ = ensure_signed(origin)?;

		// 	// Update storage.
		// 	<LimitKitties<T>>::put(limit);
		// 	// Emit an event.
		// 	Self::deposit_event(Event::KittiesLimitSet(limit));
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }
	}
}

//helper function
impl<T> Pallet<T> {
	fn get_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 != 0 {
			res = Gender::Female;
		}
		Ok(res)
	}
}
