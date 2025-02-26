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

//! Autogenerated weights for pallet_identity_management
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-19, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_identity_management
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output=./pallets/identity-management/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_identity_management.
pub trait WeightInfo {
	fn add_delegatee() -> Weight;
	fn remove_delegatee() -> Weight;
	fn link_identity() -> Weight;
	fn deactivate_identity() -> Weight;
	fn activate_identity() -> Weight;
	fn set_user_shielding_key() -> Weight;
	fn user_shielding_key_set() -> Weight;
	fn identity_linked() -> Weight;
	fn identity_deactivated() -> Weight;
	fn identity_activated() -> Weight;
	fn some_error() -> Weight;
}

/// Weights for pallet_identity_management using the Litentry node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	// Storage: IdentityManagement Delegatee (r:0 w:1)
	// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_delegatee() -> Weight {
		Weight::from_ref_time(6_324_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: IdentityManagement Delegatee (r:1 w:1)
	// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_delegatee() -> Weight {
		Weight::from_ref_time(10_160_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn link_identity() -> Weight {
		Weight::from_ref_time(11_618_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn deactivate_identity() -> Weight {
		Weight::from_ref_time(8_114_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_identity() -> Weight {
		Weight::from_ref_time(8_129_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn set_user_shielding_key() -> Weight {
		Weight::from_ref_time(8_250_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn user_shielding_key_set() -> Weight {
		Weight::from_ref_time(10_319_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_linked() -> Weight {
		Weight::from_ref_time(10_280_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_deactivated() -> Weight {
		Weight::from_ref_time(10_213_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_activated() -> Weight {
		Weight::from_ref_time(10_294_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn some_error() -> Weight {
		Weight::from_ref_time(10_322_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: IdentityManagement Delegatee (r:0 w:1)
	// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_delegatee() -> Weight {
		Weight::from_ref_time(6_324_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: IdentityManagement Delegatee (r:1 w:1)
	// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_delegatee() -> Weight {
		Weight::from_ref_time(10_160_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn link_identity() -> Weight {
		Weight::from_ref_time(11_618_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn deactivate_identity() -> Weight {
		Weight::from_ref_time(8_114_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_identity() -> Weight {
		Weight::from_ref_time(8_129_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn set_user_shielding_key() -> Weight {
		Weight::from_ref_time(8_250_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn user_shielding_key_set() -> Weight {
		Weight::from_ref_time(10_319_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_linked() -> Weight {
		Weight::from_ref_time(10_280_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_deactivated() -> Weight {
		Weight::from_ref_time(10_213_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_activated() -> Weight {
		Weight::from_ref_time(10_294_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn some_error() -> Weight {
		Weight::from_ref_time(10_322_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
}

