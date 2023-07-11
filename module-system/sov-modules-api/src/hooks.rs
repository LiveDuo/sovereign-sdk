use crate::{transaction::Transaction, Context, Spec};
use sov_rollup_interface::da::BlobTransactionTrait;
use sov_state::{StateCheckpoint, WorkingSet};

/// Hooks that execute within the `StateTransitionFunction::apply_tx_blob` function for each processed transaction.
pub trait TxHooks {
    type Context: Context;

    /// Runs just before a transaction is dispatched to an appropriate module.
    /// TODO: Why does it return address?
    /// Does it implies that it should do signature verification.
    /// Can other code rely on that assumption?
    fn pre_dispatch_tx_hook(
        &self,
        tx: &Transaction<Self::Context>,
        working_set: &mut WorkingSet<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<<Self::Context as Spec>::Address>;

    /// Runs after the tx is dispatched to an appropriate module.
    /// IF this hook returns error rollup panics
    fn post_dispatch_tx_hook(
        &self,
        tx: &Transaction<Self::Context>,
        working_set: &mut WorkingSet<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<()>;
}

/// Hooks related to the Sequencer functionality.
/// In essence, the sequencer locks a bond at the beginning of the `StateTransitionFunction::apply_tx_blob`,
/// and is rewarded once a blob of transactions is processed.
pub trait ApplyBlobHooks {
    type Context: Context;
    type BlobResult;

    /// Runs at the beginning of apply_tx_blob, locks the sequencer bond.
    /// If this hook returns Err, batch is not applied
    fn begin_blob_hook(
        &self,
        blob: &mut impl BlobTransactionTrait,
        working_set: &mut WorkingSet<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<()>;

    /// Executes at the end of apply_tx_blob and rewards or slashed the sequencer
    /// If this hook returns Err rollup panics
    fn end_blob_hook(
        &self,
        result: Self::BlobResult,
        working_set: &mut WorkingSet<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<()>;
}

/// Hooks that execute during the `StateTransitionFunction::begin_slot` and `end_slot` functions.
pub trait SlotHooks {
    type Context: Context;

    fn begin_slot_hook(
        &self,
        blob: &mut impl BlobTransactionTrait,
        state_checkpoint: StateCheckpoint<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<<Self::Context as Spec>::Storage>;

    fn end_slot_hook(
        &self,
        new_state_root: [u8; 32],
        state_checkpoint: StateCheckpoint<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<()>;
}

/// Hooks that execute within the `StateTransitionFunction::apply_sync_blob` function for each processed transaction.
pub trait SyncHooks {
    type Context: Context;

    /// Runs just before a blob is deserialized and processed to an appropriate module.
    /// May be used to check that the blob sender is bonded
    fn pre_blob_hook(
        &self,
        blob: &mut impl BlobTransactionTrait,
        working_set: &mut WorkingSet<<Self::Context as Spec>::Storage>,
    ) -> anyhow::Result<<Self::Context as Spec>::Address>;
}
