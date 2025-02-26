#![allow(missing_docs)]
use jsonrpsee::core::RpcResult;
use sov_modules_api::macros::rpc_gen;
use sov_modules_api::ModuleInfo;
use sov_state::WorkingSet;

use super::BlobStorage;

/// Response returned from the blobStorage_getModuleAddress endpoint.
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Response {
    /// Address of the module.
    pub address: String,
}

/// TODO: https://github.com/Sovereign-Labs/sovereign-sdk/issues/626
#[rpc_gen(client, server, namespace = "blobStorage")]
impl<C: sov_modules_api::Context> BlobStorage<C> {
    /// Queries the address of the module.
    #[rpc_method(name = "getModuleAddress")]
    fn get_module_address(&self, _working_set: &mut WorkingSet<C::Storage>) -> RpcResult<Response> {
        Ok(Response {
            address: self.address().to_string(),
        })
    }
}
