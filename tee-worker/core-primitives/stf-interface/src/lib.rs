/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

//! Provides a state interface.
//! This allow to easily mock the stf and exchange it with another storage.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{sync::Arc, vec::Vec};
use itp_node_api_metadata::NodeMetadataTrait;
use itp_node_api_metadata_provider::AccessNodeMetadata;
use itp_types::{OpaqueCall, ShardIdentifier, H256};

#[cfg(feature = "mocks")]
pub mod mocks;
pub mod parentchain_pallet;
pub mod runtime_upgrade;
pub mod sudo_pallet;
pub mod system_pallet;

/// Interface to initialize a new state.
pub trait InitState<State, AccountId> {
	/// Initialize a new state for a given enclave account.
	fn init_state(enclave_account: AccountId) -> State;
}

/// Interface for all functions calls necessary to update an already
/// initialized state.
pub trait UpdateState<State, StateDiff> {
	/// Updates a given state for
	fn apply_state_diff(state: &mut State, state_diff: StateDiff);
	fn storage_hashes_to_update_on_block() -> Vec<Vec<u8>>;
}

/// Interface to execute state mutating calls on a state.
pub trait StateCallInterface<Call, State, NodeMetadataRepository>
where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: NodeMetadataTrait,
{
	type Error;

	/// Execute a call on a specific state. Callbacks are added as an `OpaqueCall`.
	///
	/// Litentry:
	/// 1. add a parameter to pass the top_hash around
	/// 2. returns the encoded rpc response value field that should be passed
	/// back to the requester when the call is triggered synchronously
	fn execute_call(
		state: &mut State,
		shard: &ShardIdentifier,
		call: Call,
		top_hash: H256,
		calls: &mut Vec<OpaqueCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<Vec<u8>, Self::Error>;
}

/// Interface to execute state reading getters on a state.
pub trait StateGetterInterface<Getter, State> {
	/// Execute a getter on a specific state.
	fn execute_getter(state: &mut State, getter: Getter) -> Option<Vec<u8>>;
}

/// Trait used to abstract the call execution.
pub trait ExecuteCall<NodeMetadataRepository>
where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: NodeMetadataTrait,
{
	type Error;

	/// Execute a call. Callbacks are added as an `OpaqueCall`.
	///
	/// Litentry: returns the encoded rpc response that should be passed back to
	/// the requester when the call is triggered synchronously
	fn execute(
		self,
		shard: &ShardIdentifier,
		top_hash: H256,
		calls: &mut Vec<OpaqueCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<Vec<u8>, Self::Error>;

	/// Get storages hashes that should be updated for a specific call.
	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>>;
}

/// Trait used to abstract the getter execution.
pub trait ExecuteGetter {
	/// Execute a getter.
	fn execute(self) -> Option<Vec<u8>>;
	/// Get storages hashes that should be updated for a specific getter.
	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>>;
}
