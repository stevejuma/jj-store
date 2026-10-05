//! Foreground local-JJ coordination and explicit retained head publication.
//! Physical checkout and runtime command enablement remain separate gates.
mod guard;
mod physical_inventory;
pub use physical_inventory::{PhysicalDirectoryV1, PhysicalFileV1, PhysicalInventoryV1};
mod recovery;
mod reword;
use crate::{
    local_browser::{self, Access, Error, Result, working_tree::CurrentState},
    query::{content::Limits, workers::Workers},
};
pub use recovery::{
    CheckoutWitnessV1, PhysicalGuard, PublishedHead, RecoveredHistory, RecoveryBinding,
    RecoveryWitnessV1, RetainedIdentity, TreeCheckoutPlanV1, VerifiedUnchangedCheckout,
};
pub use reword::{BookmarkEffect, PreparedCode, Reword, Rewrite};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::OwnedSemaphorePermit;

#[derive(Clone)]
pub struct Coordinator {
    reader: local_browser::Executor,
    workers: Workers,
}
impl Coordinator {
    pub fn new(concurrency: usize) -> Result<Self> {
        Self::with_workers(Workers::new(concurrency)?, Limits::default())
    }
    /// Composition shares admission with its read/preparation providers.
    pub fn with_workers(workers: Workers, limits: Limits) -> Result<Self> {
        Ok(Self {
            reader: local_browser::Executor::with_workers(workers.clone(), limits),
            workers,
        })
    }
    /// Trusted composition supplies the registered root and exact current
    /// witness, never a client path. Acquisition does not wait on another CLI.
    pub async fn acquire(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        expected: CurrentState,
    ) -> Result<Foreground> {
        let permit = self.workers.acquire_preparation()?;
        let before = self
            .reader
            .working_tree_current(access.clone(), root.clone(), expected.baseline.operation)
            .await?;
        if before != expected {
            return Err(Error::EpochChanged);
        }
        let guard = guard::Guard::acquire(&root)?;
        let lease = Foreground {
            guard,
            access,
            root,
            expected,
            reader: self.reader.clone(),
            _permit: permit,
        };
        lease.validate_metadata().await?;
        Ok(lease)
    }
}

/// Owns the actual foreground lock through subsequent composition work.
/// Dropping releases it without finishing or changing checkout metadata.
pub struct Foreground {
    guard: guard::Guard,
    access: Arc<dyn Access>,
    root: PathBuf,
    expected: CurrentState,
    reader: local_browser::Executor,
    _permit: OwnedSemaphorePermit,
}
impl Foreground {
    pub fn witness(&self) -> &CurrentState {
        &self.expected
    }
    /// Exact metadata/policy check only. Disk file bytes need their separate
    /// physical observation and checkout guard; editors do not take this lock.
    pub async fn validate_metadata(&self) -> Result<()> {
        self.guard.check(&self.root)?;
        let current = self
            .reader
            .working_tree_current(
                self.access.clone(),
                self.root.clone(),
                self.expected.baseline.operation,
            )
            .await?;
        if current != self.expected {
            return Err(Error::EpochChanged);
        }
        self.guard.check(&self.root)
    }
}
