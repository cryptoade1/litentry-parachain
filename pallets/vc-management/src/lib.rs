// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! A pallet to serve as the interface for F/E for verifiable credentials (VC)
//! management. Similar to IMP pallet, the actual processing will be done within TEE.

// Note:
// the admin account can only be set by SetAdminOrigin, which will be bound at runtime.
// TODO: benchmark and weights: we need worst-case scenarios

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_variables)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use core_primitives::{AesOutput, SchemaIndex, ShardIdentifier, SCHEMA_CONTENT_LEN, SCHEMA_ID_LEN};
pub use pallet::*;
use sp_core::H256;
use sp_std::vec::Vec;

mod vc_context;
pub use vc_context::*;

mod assertion;
pub use assertion::*;

mod schema;
pub use schema::*;

pub type VCIndex = H256;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core_primitives::{ErrorString, VCMPError};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// some extrinsics should only be called by origins from TEE
		type TEECallOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The origin who can set the admin account
		type SetAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// a map VCIndex -> VC context
	#[pallet::storage]
	#[pallet::getter(fn vc_registry)]
	pub type VCRegistry<T: Config> = StorageMap<_, Blake2_128Concat, VCIndex, VCContext<T>>;

	// the Schema admin account
	#[pallet::storage]
	#[pallet::getter(fn schema_admin)]
	pub type SchemaAdmin<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn schema_index)]
	pub type SchemaRegistryIndex<T: Config> = StorageValue<_, SchemaIndex, ValueQuery>;

	// the VC Schema storage
	#[pallet::storage]
	#[pallet::getter(fn schema_registry)]
	pub type SchemaRegistry<T: Config> = StorageMap<_, Blake2_128Concat, SchemaIndex, VCSchema<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO: do we need account as event parameter? This needs to be decided by F/E
		VCRequested { shard: ShardIdentifier, assertion: Assertion },
		// a VC is disabled on chain
		VCDisabled { index: VCIndex },
		// a VC is revoked on chain
		VCRevoked { index: VCIndex },
		VCNotExist { index: VCIndex },
		// event that should be triggered by TEECallOrigin
		// a VC is just issued
		VCIssued { account: T::AccountId, index: VCIndex, vc: AesOutput },
		// Admin account was changed
		SchemaAdminChanged { old_admin: Option<T::AccountId>, new_admin: Option<T::AccountId> },
		// a Schema is issued
		SchemaIssued { account: T::AccountId, shard: ShardIdentifier, index: SchemaIndex },
		// a Schema is disabled
		SchemaDisabled { account: T::AccountId, shard: ShardIdentifier, index: SchemaIndex },
		// a Schema is activated
		SchemaActivated { account: T::AccountId, shard: ShardIdentifier, index: SchemaIndex },
		// a Schema is revoked
		SchemaRevoked { account: T::AccountId, shard: ShardIdentifier, index: SchemaIndex },
		// event errors caused by processing in TEE
		// copied from core_primitives::IMPError, we use events instead of pallet::errors,
		// see https://github.com/litentry/litentry-parachain/issues/1275
		HttpRequestFailed { reason: ErrorString },
		RequestVCHandlingFailed,
		ParseError,
		Assertion1Failed,
		Assertion2Failed,
		Assertion3Failed,
		Assertion4Failed,
		Assertion5Failed,
		Assertion6Failed,
		Assertion7Failed,
		Assertion8Failed,
		Assertion10Failed,
		Assertion11Failed,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the VC already exists
		VCAlreadyExists,
		/// the ID doesn't exist
		VCNotExist,
		/// The requester doesn't have the permission (because of subject mismatch)
		VCSubjectMismatch,
		/// The VC is already disabled
		VCAlreadyDisabled,
		/// Error when the caller account is not the admin
		RequireSchemaAdmin,
		/// Schema not exists
		SchemaNotExists,
		/// Schema is already disabled
		SchemaAlreadyDisabled,
		/// Schema is active
		SchemaAlreadyActivated,
		SchemaIndexOverFlow,
		LengthMismatch,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(195_000_000)]
		pub fn request_vc(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			assertion: Assertion,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::VCRequested { shard, assertion });
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(195_000_000)]
		pub fn disable_vc(origin: OriginFor<T>, index: VCIndex) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			VCRegistry::<T>::try_mutate(index, |context| {
				match context.take() {
					Some(mut c) => {
						ensure!(who == c.subject, Error::<T>::VCSubjectMismatch);
						ensure!(c.status == Status::Active, Error::<T>::VCAlreadyDisabled);
						c.status = Status::Disabled;
						*context = Some(c);
						Self::deposit_event(Event::VCDisabled { index });
					},
					None => {
						Self::deposit_event(Event::VCNotExist { index });
					},
				}

				Ok(().into())
			})
		}

		#[pallet::call_index(2)]
		#[pallet::weight(195_000_000)]
		pub fn revoke_vc(origin: OriginFor<T>, index: VCIndex) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let context = VCRegistry::<T>::get(index).ok_or(Error::<T>::VCNotExist)?;
			ensure!(who == context.subject, Error::<T>::VCSubjectMismatch);
			VCRegistry::<T>::remove(index);
			Self::deposit_event(Event::VCRevoked { index });
			Ok(().into())
		}

		/// ---------------------------------------------------
		/// The following extrinsics are supposed to be called by TEE only
		/// ---------------------------------------------------
		#[pallet::call_index(3)]
		#[pallet::weight(195_000_000)]
		pub fn vc_issued(
			origin: OriginFor<T>,
			account: T::AccountId,
			index: H256,
			hash: H256,
			vc: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(!VCRegistry::<T>::contains_key(index), Error::<T>::VCAlreadyExists);
			VCRegistry::<T>::insert(index, VCContext::<T>::new(account.clone(), hash));
			Self::deposit_event(Event::VCIssued { account, index, vc });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(195_000_000)]
		pub fn some_error(origin: OriginFor<T>, error: VCMPError) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			match error {
				VCMPError::HttpRequestFailed(s) =>
					Self::deposit_event(Event::HttpRequestFailed { reason: s }),
				VCMPError::RequestVCHandlingFailed =>
					Self::deposit_event(Event::RequestVCHandlingFailed),
				VCMPError::ParseError => Self::deposit_event(Event::ParseError),
				VCMPError::Assertion1Failed => Self::deposit_event(Event::Assertion1Failed),
				VCMPError::Assertion2Failed => Self::deposit_event(Event::Assertion2Failed),
				VCMPError::Assertion3Failed => Self::deposit_event(Event::Assertion3Failed),
				VCMPError::Assertion4Failed => Self::deposit_event(Event::Assertion4Failed),
				VCMPError::Assertion5Failed => Self::deposit_event(Event::Assertion5Failed),
				VCMPError::Assertion6Failed => Self::deposit_event(Event::Assertion6Failed),
				VCMPError::Assertion7Failed => Self::deposit_event(Event::Assertion7Failed),
				VCMPError::Assertion8Failed => Self::deposit_event(Event::Assertion8Failed),
				VCMPError::Assertion10Failed => Self::deposit_event(Event::Assertion10Failed),
				VCMPError::Assertion11Failed => Self::deposit_event(Event::Assertion11Failed),
			}
			Ok(Pays::No.into())
		}

		// Change the schema Admin account
		#[pallet::call_index(5)]
		#[pallet::weight(195_000_000)]
		pub fn set_schema_admin(
			origin: OriginFor<T>,
			new: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::SetAdminOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::SchemaAdminChanged {
				old_admin: Self::schema_admin(),
				new_admin: Some(new.clone()),
			});
			<SchemaAdmin<T>>::put(new);
			Ok(Pays::No.into())
		}

		// It requires the schema Admin account
		#[pallet::call_index(6)]
		#[pallet::weight(195_000_000)]
		pub fn add_schema(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			id: Vec<u8>,
			content: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender.clone()) == Self::schema_admin(), Error::<T>::RequireSchemaAdmin);
			ensure!((id.len() as u32) <= SCHEMA_ID_LEN, Error::<T>::LengthMismatch);
			ensure!((content.len() as u32) <= SCHEMA_CONTENT_LEN, Error::<T>::LengthMismatch);

			let index = Self::schema_index();
			<SchemaRegistryIndex<T>>::put(
				index.checked_add(1u64).ok_or(Error::<T>::SchemaIndexOverFlow)?,
			);

			SchemaRegistry::<T>::insert(
				index,
				VCSchema::<T>::new(id.clone(), sender.clone(), content.clone()),
			);
			Self::deposit_event(Event::SchemaIssued { account: sender, shard, index });
			Ok(().into())
		}

		// It requires the schema Admin account
		#[pallet::call_index(7)]
		#[pallet::weight(195_000_000)]
		pub fn disable_schema(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			index: SchemaIndex,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender.clone()) == Self::schema_admin(), Error::<T>::RequireSchemaAdmin);

			SchemaRegistry::<T>::try_mutate(index, |context| {
				let mut c = context.take().ok_or(Error::<T>::SchemaNotExists)?;
				ensure!(c.status == Status::Active, Error::<T>::SchemaAlreadyDisabled);
				c.status = Status::Disabled;
				*context = Some(c);
				Self::deposit_event(Event::SchemaDisabled { account: sender, shard, index });
				Ok(().into())
			})
		}

		// It requires the schema Admin account
		#[pallet::call_index(8)]
		#[pallet::weight(195_000_000)]
		pub fn activate_schema(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			index: SchemaIndex,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender.clone()) == Self::schema_admin(), Error::<T>::RequireSchemaAdmin);
			SchemaRegistry::<T>::try_mutate(index, |context| {
				let mut c = context.take().ok_or(Error::<T>::SchemaNotExists)?;
				ensure!(c.status == Status::Disabled, Error::<T>::SchemaAlreadyActivated);
				c.status = Status::Active;
				*context = Some(c);
				Self::deposit_event(Event::SchemaActivated { account: sender, shard, index });
				Ok(().into())
			})
		}

		// It requires the schema Admin account
		#[pallet::call_index(9)]
		#[pallet::weight(195_000_000)]
		pub fn revoke_schema(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			index: SchemaIndex,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender.clone()) == Self::schema_admin(), Error::<T>::RequireSchemaAdmin);

			let _ = SchemaRegistry::<T>::get(index).ok_or(Error::<T>::SchemaNotExists)?;
			SchemaRegistry::<T>::remove(index);
			Self::deposit_event(Event::SchemaRevoked { account: sender, shard, index });
			Ok(().into())
		}
	}
}
