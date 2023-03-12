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

use crate::{
	error::{Error, Result, VCMPError},
	executor::Executor,
	IndirectCallsExecutor,
};
use codec::Encode;
use ita_stf::{TrustedCall, TrustedOperation};
use itp_node_api::{
	api_client::{pallet_timestamp::PalletTimestampApi, ParentchainUncheckedExtrinsic},
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes,
		pallet_utility::UtilityCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata,
	},
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{RequestVCFn, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::ParentchainBlockNumber;
use log::*;
use sp_runtime::traits::{AccountIdLookup, StaticLookup};

pub(crate) struct RequestVC {
	pub(crate) block_number: ParentchainBlockNumber,
}

impl RequestVC {
	fn execute_internal<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
	>(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		extrinsic: ParentchainUncheckedExtrinsic<
			<Self as Executor<
				ShieldingKeyRepository,
				StfEnclaveSigner,
				TopPoolAuthor,
				NodeMetadataProvider,
			>>::Call,
		>,
	) -> Result<()>
	where
		ShieldingKeyRepository: AccessKey,
		<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
			+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
		StfEnclaveSigner: StfEnclaveSigning,
		TopPoolAuthor: AuthorApi<H256, H256> + Send + Sync + 'static,
		NodeMetadataProvider: AccessNodeMetadata,
		NodeMetadataProvider::MetadataType:
			IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
	{
		let (_, (shard, assertion)) = extrinsic.function;
		let shielding_key = context.shielding_key_repo.retrieve_key()?;

		if let Some((multiaddress_account, _, _)) = extrinsic.signature {
			let account = AccountIdLookup::lookup(multiaddress_account)?;
			debug!(
				"indirect call Requested VC, who:{:?}, assertion: {:?}",
				account_id_to_string(&account),
				assertion
			);

			let enclave_account_id = context.stf_enclave_signer.get_enclave_account()?;

			let trusted_call = TrustedCall::build_assertion(
				enclave_account_id,
				account,
				assertion,
				shard,
				self.block_number,
			);
			let signed_trusted_call =
				context.stf_enclave_signer.sign_call_with_self(&trusted_call, &shard)?;
			let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

			let encrypted_trusted_call = shielding_key.encrypt(&trusted_operation.encode())?;
			context.submit_trusted_call(shard, encrypted_trusted_call);
		}
		Ok(())
	}
}

impl<ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider>
	Executor<ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider>
	for RequestVC
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	StfEnclaveSigner: StfEnclaveSigning,
	TopPoolAuthor: AuthorApi<H256, H256> + Send + Sync + 'static,
	NodeMetadataProvider: AccessNodeMetadata,
	NodeMetadataProvider::MetadataType:
		IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
{
	type Call = RequestVCFn;

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(
		&self,
		metadata_type: &NodeMetadataProvider::MetadataType,
	) -> Result<[u8; 2]> {
		metadata_type.request_vc_call_indexes().map_err(|e| e.into())
	}

	fn execute(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()> {
		let (_, (shard, _)) = extrinsic.function;
		let e = Error::VCMPHandlingError(VCMPError::RequestVCHandlingFailed);
		if self.execute_internal(context, extrinsic).is_err() {
			// try to handle the error internally, if we get another error, log it and return the
			// original error
			if let Err(internal_e) = context.submit_trusted_call_from_error(shard, &e) {
				warn!("fail to handle internal errors in request_vc: {:?}", internal_e);
			}
			return Err(e)
		}
		Ok(())
	}
}
