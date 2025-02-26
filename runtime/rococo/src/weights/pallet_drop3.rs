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

//! Autogenerated weights for `pallet_drop3`
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
// --pallet=pallet_drop3
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_drop3.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_drop3`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_drop3::WeightInfo for WeightInfo<T> {
	/// Storage: Drop3 Admin (r:1 w:1)
	/// Proof Skipped: Drop3 Admin (max_values: Some(1), max_size: None, mode: Measured)
	fn set_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `499`
		// Minimum execution time: 13_793 nanoseconds.
		Weight::from_ref_time(14_636_000)
			.saturating_add(Weight::from_proof_size(499))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Drop3 Admin (r:1 w:0)
	/// Proof Skipped: Drop3 Admin (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Drop3 RewardPools (r:1 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	fn approve_reward_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3562`
		// Minimum execution time: 23_493 nanoseconds.
		Weight::from_ref_time(24_215_000)
			.saturating_add(Weight::from_proof_size(3562))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Drop3 Admin (r:1 w:0)
	/// Proof Skipped: Drop3 Admin (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Drop3 RewardPools (r:1 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	/// Proof Skipped: Drop3 RewardPoolOwners (max_values: None, max_size: None, mode: Measured)
	fn reject_reward_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `6860`
		// Minimum execution time: 70_105 nanoseconds.
		Weight::from_ref_time(70_837_000)
			.saturating_add(Weight::from_proof_size(6860))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Drop3 RewardPools (r:1 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Drop3 Admin (r:1 w:0)
	/// Proof Skipped: Drop3 Admin (max_values: Some(1), max_size: None, mode: Measured)
	fn start_reward_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3562`
		// Minimum execution time: 23_546 nanoseconds.
		Weight::from_ref_time(24_053_000)
			.saturating_add(Weight::from_proof_size(3562))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Drop3 RewardPools (r:1 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Drop3 Admin (r:1 w:0)
	/// Proof Skipped: Drop3 Admin (max_values: Some(1), max_size: None, mode: Measured)
	fn stop_reward_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3562`
		// Minimum execution time: 23_606 nanoseconds.
		Weight::from_ref_time(24_478_000)
			.saturating_add(Weight::from_proof_size(3562))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Drop3 RewardPools (r:1 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Drop3 Admin (r:1 w:0)
	/// Proof Skipped: Drop3 Admin (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	/// Proof Skipped: Drop3 RewardPoolOwners (max_values: None, max_size: None, mode: Measured)
	fn close_reward_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `6860`
		// Minimum execution time: 43_258 nanoseconds.
		Weight::from_ref_time(44_337_000)
			.saturating_add(Weight::from_proof_size(6860))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Drop3 CurrentMaxPoolId (r:1 w:1)
	/// Proof Skipped: Drop3 CurrentMaxPoolId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	/// Proof Skipped: Drop3 RewardPoolOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Drop3 RewardPools (r:0 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 16]`.
	fn propose_reward_pool(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137`
		//  Estimated: `3509`
		// Minimum execution time: 39_107 nanoseconds.
		Weight::from_ref_time(40_981_281)
			.saturating_add(Weight::from_proof_size(3509))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Drop3 RewardPools (r:1 w:1)
	/// Proof Skipped: Drop3 RewardPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn send_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545`
		//  Estimated: `8226`
		// Minimum execution time: 45_149 nanoseconds.
		Weight::from_ref_time(45_799_000)
			.saturating_add(Weight::from_proof_size(8226))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
