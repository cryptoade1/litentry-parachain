// Copyright 2020-2022 Litentry Technologies GmbH.
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
//! DATE: 2022-10-09, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_drop3
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_drop3.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_drop3`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_drop3::WeightInfo for WeightInfo<T> {
	// Storage: Drop3 Admin (r:1 w:1)
	fn set_admin() -> Weight {
		Weight::from_ref_time(15_811_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	fn approve_reward_pool() -> Weight {
		Weight::from_ref_time(22_410_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn reject_reward_pool() -> Weight {
		Weight::from_ref_time(59_561_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn start_reward_pool() -> Weight {
		Weight::from_ref_time(22_576_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn stop_reward_pool() -> Weight {
		Weight::from_ref_time(24_058_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn close_reward_pool() -> Weight {
		Weight::from_ref_time(40_787_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 CurrentMaxPoolId (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	// Storage: Drop3 RewardPools (r:0 w:1)
	/// The range of component `n` is `[0, 16]`.
	fn propose_reward_pool(n: u32, ) -> Weight {
		Weight::from_ref_time(35_633_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn send_reward() -> Weight {
		Weight::from_ref_time(41_949_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
