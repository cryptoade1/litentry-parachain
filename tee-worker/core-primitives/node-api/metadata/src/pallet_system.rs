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
use crate::{error::Result, Error, NodeMetadata};
use codec::Decode;
use sp_core::storage::StorageKey;

/// Pallet' name:
const SYSTEM: &str = "System";

pub trait SystemStorageIndexes {
	fn system_account_storage_key(&self) -> Result<StorageKey>;

	fn system_account_storage_map_key(&self, index: u64) -> Result<StorageKey>;
}

impl SystemStorageIndexes for NodeMetadata {
	fn system_account_storage_key(&self) -> Result<StorageKey> {
		self.storage_value_key(SYSTEM, "Account")
	}

	fn system_account_storage_map_key(&self, index: u64) -> Result<StorageKey> {
		self.storage_map_key(SYSTEM, "Account", index)
	}
}

// litentry
pub trait SystemSs58Prefix {
	fn system_ss58_prefix(&self) -> Result<u16>;
}

impl SystemSs58Prefix for NodeMetadata {
	fn system_ss58_prefix(&self) -> Result<u16> {
		match &self.node_metadata {
			None => Err(Error::MetadataNotSet),
			Some(meta_data) => {
				let pallet = meta_data.pallets.get(SYSTEM).ok_or(Error::MetadataNotSet)?;
				let mut raw = pallet.constants.get("SS58Prefix").unwrap().value.as_slice();
				u16::decode(&mut raw).map_err(|_| Error::InvalidMetadata)
			},
		}
	}
}
