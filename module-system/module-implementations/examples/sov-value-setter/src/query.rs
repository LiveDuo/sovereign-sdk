#![allow(missing_docs)]
use jsonrpsee::core::RpcResult;
use sov_modules_api::macros::rpc_gen;
use sov_state::WorkingSet;

use super::ValueSetter;

/// Response returned from the valueSetter_queryValue endpoint.
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Response {
    /// Value saved in the module's state.
    pub value: Option<u32>,
}

#[rpc_gen(client, server, namespace = "valueSetter")]
impl<C: sov_modules_api::Context> ValueSetter<C> {
    /// Queries the state of the module.
    #[rpc_method(name = "queryValue")]
    pub fn query_value(&self, working_set: &mut WorkingSet<C::Storage>) -> RpcResult<Response> {
        Ok(Response {
            value: self.value.get(working_set),
        })
    }
}
