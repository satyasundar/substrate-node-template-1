//All pallets must be configred for no_std
#![cfg_attr(not(feature = "std"), no_std)]

//Re-export pallet items so that they can be accessed from the crate namespace
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emits events, it depends on runtime's defination of an event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		//pallets use weights to measure the complexity of the callable functions
		type WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// event emitted when the claim is created
		ClaimCreated { who: T::AccountId, claim: T::Hash },
		// event emitted when the claim is revoked
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}


	#[pallet::error]
	pub enum Error<T> {
		AlreadyClaimed,
		NoSuchClaim,
		NotClaimOwner,
	}


	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, BlockNumberFor<T>)>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::default())]
		#[pallet::call_index(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			//check if the extrinsic or transaction is signed
			let sender = ensure_signed(origin)?;

			//verify that specified claim is not yet stored
			ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();
			Claims::<T>::insert(&claim, (&sender, current_block));
			Self::deposit_event(Event::ClaimCreated { who: sender, claim });

			Ok(())
		}

		#[pallet::weight(Weight::default())]
		#[pallet::call_index(1)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;
			ensure!(sender == owner, Error::<T>::NotClaimOwner);
			Claims::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
			Ok(())
		}
	}

}

pub mod weights {
	//placeholder for pallet weights
	pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
}

