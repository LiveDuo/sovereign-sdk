use reth_primitives::{
    Signature as RethSignature, TransactionSigned as RethTransactionSigned,
    TransactionSignedEcRecovered as RethTransactionSignedEcRecovered,
    TransactionSignedNoHash as RethTransactionSignedNoHash, H160, H256,
};

use super::{Bytes32, EthAddress};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub(crate) struct BlockEnv {
    pub(crate) number: u64,
    pub(crate) coinbase: EthAddress,
    pub(crate) timestamp: Bytes32,
    /// Prevrandao is used after Paris (aka TheMerge) instead of the difficulty value.
    pub(crate) prevrandao: Option<Bytes32>,
    /// basefee is added in EIP1559 London upgrade
    pub(crate) basefee: Bytes32,
    pub(crate) gas_limit: Bytes32,
}

impl Default for BlockEnv {
    fn default() -> Self {
        Self {
            number: Default::default(),
            coinbase: Default::default(),
            timestamp: Default::default(),
            prevrandao: Some(Default::default()),
            basefee: Default::default(),
            gas_limit: [u8::MAX; 32],
        }
    }
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(schemars::JsonSchema)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct AccessListItem {
    pub address: EthAddress,
    pub storage_keys: Vec<Bytes32>,
}

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(schemars::JsonSchema)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq, Clone)]
pub struct RawEvmTransaction {
    pub tx: Vec<u8>,
}

pub struct EvmTransactionSignedEcRecovered {
    tx: RethTransactionSignedEcRecovered,
}

impl EvmTransactionSignedEcRecovered {
    pub fn new(tx: RethTransactionSignedEcRecovered) -> Self {
        Self { tx }
    }
    pub fn hash(&self) -> H256 {
        self.tx.hash()
    }

    pub fn signer(&self) -> H160 {
        self.tx.signer()
    }

    pub fn to(&self) -> Option<EthAddress> {
        self.tx.to().map(|to| to.into())
    }
}

impl From<EvmTransactionSignedEcRecovered> for RethTransactionSignedEcRecovered {
    fn from(tx: EvmTransactionSignedEcRecovered) -> Self {
        tx.tx
    }
}
