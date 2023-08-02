#[cfg(feature = "native")]
pub use sov_modules_api::default_context::DefaultContext;
pub use sov_modules_api::default_context::ZkDefaultContext;
#[cfg(feature = "native")]
pub use sov_modules_api::default_signature::private_key::DefaultPrivateKey;
#[cfg(feature = "native")]
use sov_modules_api::Spec;
use sov_modules_stf_template::AppTemplate;
pub use sov_modules_stf_template::Batch;
use sov_rollup_interface::da::BlobReaderTrait;
#[cfg(feature = "native")]
use sov_rollup_interface::zk::Zkvm;
#[cfg(feature = "native")]
use sov_state::ProverStorage;
use sov_state::Storage;
use sov_stf_runner::batch_builder::FiFoStrictBatchBuilder;
#[cfg(feature = "native")]
use sov_stf_runner::runner_config::StorageConfig;

use crate::runtime::Runtime;

#[cfg(feature = "native")]
pub struct App<Vm: Zkvm, B: BlobReaderTrait> {
    pub stf: AppTemplate<DefaultContext, Runtime<DefaultContext>, Vm, B>,
    pub batch_builder: Option<FiFoStrictBatchBuilder<Runtime<DefaultContext>, DefaultContext>>,
}

#[cfg(feature = "native")]
impl<Vm: Zkvm, B: BlobReaderTrait> App<Vm, B> {
    pub fn new(storage_config: StorageConfig) -> Self {
        let storage =
            ProverStorage::with_config(storage_config).expect("Failed to open prover storage");
        let app = AppTemplate::new(storage.clone(), Runtime::default());
        let batch_size_bytes = 1024 * 100; // 100 KB
        let batch_builder = FiFoStrictBatchBuilder::new(
            batch_size_bytes,
            u32::MAX as usize,
            Runtime::default(),
            storage,
        );
        Self {
            stf: app,
            batch_builder: Some(batch_builder),
        }
    }

    pub fn get_storage(&self) -> <DefaultContext as Spec>::Storage {
        self.stf.current_storage.clone()
    }
}
