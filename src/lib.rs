#![cfg_attr(not(feature = "std"), no_std)]

//! Struct Storage
//! This pallet demonstrates how to declare and store `strcuts` that contain types
//! that come from the pallet's configuration trait.

use frame_support::{
	decl_error, 
	decl_event, 
	decl_module, 
	decl_storage, 
	ensure, 
	dispatch::DispatchResult,
};
use sp_runtime::RuntimeDebug;
use frame_system::{
	self as system, 
	ensure_signed,
	ensure_root
};
use parity_scale_codec::{
	Decode, 
	Encode
};
use sp_std::prelude::*;


#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type AccountIdOf<T> = <T as system::Trait>::AccountId;
type ReferenceOf<T> = Reference<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct Reference<AccountId, BlockNumber> {
	user: AccountId,
	promoter: AccountId,
	when: BlockNumber

}


decl_storage! {
	trait Store for Module<T: Trait> as ReferralStore {
		Promoter get(fn promoter): map hasher(blake2_128_concat) Vec<u8> => AccountIdOf<T>;
		PromoterCode get(fn promoter_code): map hasher(blake2_128_concat) AccountIdOf<T>=> Vec<u8>;
		Registered get(fn registered): map hasher(blake2_128_concat) AccountIdOf<T> => bool;
		References get(fn references): map hasher(blake2_128_concat) (AccountIdOf<T>, u8) => Option<ReferenceOf<T>>;
		ReferenceCount get(fn reference_count): map hasher(blake2_128_concat) AccountIdOf<T> => u8;
	}
}


decl_event! {
	pub enum Event<T>
	where
	AccountId = <T as system::Trait>::AccountId,
	{
		/// Promoter created. \[account_id, code\]
		NewPromoter(AccountId, Vec<u8>),
		/// User created. \[promoter, user\]
		NewUser(AccountId, AccountId),		
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// LOL
		UserAlreadyRegistered		
	}
}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;	

		#[weight = 10_000]
		fn register_from_code(origin, code: Vec<u8>) -> DispatchResult {
			
			let caller = ensure_signed(origin)?;
			let is_caller_registered: bool = <Registered<T>>::get(&caller);
			ensure!(is_caller_registered == false, Error::<T>::UserAlreadyRegistered );
			let promoter = <Promoter<T>>::get(code);
			let when = <system::Module<T>>::block_number();

			<Registered<T>>::insert(&caller, true);
			let index = <ReferenceCount<T>>::get(&caller);

			let thing: ReferenceOf<T> = Reference {
				user: caller.clone(),
				promoter: promoter.clone(),
				when: when
			};
			<References<T>>::insert((caller.clone(), index), thing);
			<ReferenceCount<T>>::insert(caller.clone(), index + 1);
			
			Self::deposit_event(RawEvent::NewUser(promoter.clone(), caller.clone()));
			
			Ok(())
		}

		#[weight = 10_000]
		fn set_promoter(origin, account: AccountIdOf<T>, code: Vec<u8>) -> DispatchResult {
			
			let caller = ensure_signed(origin.clone())?;

			<Promoter<T>>::insert(code.clone(), account.clone());
			<PromoterCode<T>>::insert(account.clone(), code.clone());
			
			Self::deposit_event(RawEvent::NewPromoter(account, code.clone()));			
			
			Ok(())
		}		
	}
}