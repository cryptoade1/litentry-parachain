// Copyright 2020-2023 Trust Computing GmbH.
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

//! Autogenerated weights for `pallet_asset_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-27, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_asset_manager
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_asset_manager.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_asset_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for WeightInfo<T> {
	/// Storage: AssetManager AssetTypeId (r:1 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager ForeignAssetTracker (r:1 w:1)
	/// Proof Skipped: AssetManager ForeignAssetTracker (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	fn register_foreign_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3538`
		// Minimum execution time: 34_706 nanoseconds.
		Weight::from_ref_time(35_688_000)
			.saturating_add(Weight::from_proof_size(3538))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: AssetManager AssetIdType (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	fn update_foreign_asset_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3079`
		// Minimum execution time: 21_197 nanoseconds.
		Weight::from_ref_time(21_776_000)
			.saturating_add(Weight::from_proof_size(3079))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: AssetManager AssetIdType (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn set_asset_units_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3079`
		// Minimum execution time: 20_082 nanoseconds.
		Weight::from_ref_time(20_721_000)
			.saturating_add(Weight::from_proof_size(3079))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:1 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	fn add_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `5600`
		// Minimum execution time: 26_389 nanoseconds.
		Weight::from_ref_time(27_376_000)
			.saturating_add(Weight::from_proof_size(5600))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: AssetManager AssetTypeId (r:2 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	fn remove_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482`
		//  Estimated: `5914`
		// Minimum execution time: 32_586 nanoseconds.
		Weight::from_ref_time(33_211_000)
			.saturating_add(Weight::from_proof_size(5914))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
