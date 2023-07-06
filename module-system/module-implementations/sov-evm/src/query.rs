use crate::Evm;
use ethereum_types::U64;
use sov_modules_macros::rpc_gen;
use sov_state::WorkingSet;

#[rpc_gen(client, server, namespace = "eth")]
impl<C: sov_modules_api::Context> Evm<C> {
    #[rpc_method(name = "chainId")]
    pub fn chain_id(&self, _working_set: &mut WorkingSet<C::Storage>) -> Option<U64> {
        println!("eth_chainId!");
        Some(U64::from(1u64))
    }
}
