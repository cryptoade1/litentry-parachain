use crate::ApiResult;
use sp_core::{Pair, H256 as Hash};
use sp_runtime::MultiSignature;
use substrate_api_client::{Api, ExtrinsicParams, RpcClient};

pub const TIMESTAMP: &str = "Timestamp";

/// ApiClient extension that enables communication with the `timestamp` pallet.
pub trait PalletTimestampApi {
	fn timestamp(&self, at_block: Option<Hash>) -> ApiResult<u64>;
}

impl<P: Pair, Client: RpcClient, Params: ExtrinsicParams> PalletTimestampApi
	for Api<P, Client, Params>
where
	MultiSignature: From<P::Signature>,
{
	fn timestamp(&self, at_block: Option<Hash>) -> ApiResult<u64> {
		Ok(self.get_storage_value(TIMESTAMP, "Now", at_block)?.unwrap_or_default())
	}
}
