use crate::{context::Lease, *};
use async_trait::async_trait;
use std::{path::Path, sync::Arc};
use tokio::sync::{Semaphore, watch};

/// Implementations own transactional destination durability and replay safety.
/// Recover verifies actual local artifacts, rather than a remote cursor/cache.
/// Blocking workers must keep a Context clone until they exit, including after
/// timeout/cancellation, so they retain admission and the local writer lease.
#[async_trait]
pub trait Projection: Send {
    /// Reject incompatible implementation/configuration before any runner I/O.
    fn validate_identity(&self, identity: &Identity) -> Result<()> {
        identity.validate()
    }
    fn supports_snapshots(&self) -> bool {
        false
    }
    async fn restore_snapshot(
        &mut self,
        _snapshot: &Snapshot,
        _identity: &Identity,
        _context: Context,
    ) -> Result<Option<Checkpoint>> {
        Ok(None)
    }
    async fn publish_snapshot(
        &mut self,
        _snapshot: &Snapshot,
        _checkpoint: &Checkpoint,
        _identity: &Identity,
        _context: Context,
    ) -> Result<()> {
        Ok(())
    }
    async fn recover(&mut self, context: Context) -> Result<Option<Checkpoint>>;
    async fn bootstrap(
        &mut self,
        snapshot: Snapshot,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint>;
    async fn apply(
        &mut self,
        batch: Batch,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint>;
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Health {
    pub repository_position: Option<Position>,
    pub applied: Option<Checkpoint>,
    pub remote: Option<Checkpoint>,
    pub lag: Option<u64>,
    pub degraded: bool,
    pub last_error: Option<Error>,
}
pub struct Runner<P> {
    projection: P,
    source: Source,
    identity: Identity,
    limits: Limits,
    lease: Arc<Lease>,
    slots: Arc<Semaphore>,
    health: watch::Sender<Health>,
    snapshot_checked: bool,
    restore_snapshots: bool,
    snapshot_pending: Option<Checkpoint>,
    snapshot_error: Option<Error>,
}
impl<P: Projection> Runner<P> {
    pub fn new(
        source: Source,
        identity: Identity,
        projection: P,
        owner_root: &Path,
        limits: Limits,
    ) -> Result<Self> {
        identity.validate()?;
        projection.validate_identity(&identity)?;
        limits.validate()?;
        if limits.snapshot_every > 0 && !projection.supports_snapshots() {
            return Err(Error::Invalid);
        }
        let lease = Lease::acquire(owner_root, source.repository(), &identity.name)?;
        Ok(Self {
            source,
            identity,
            projection,
            limits,
            lease,
            slots: Arc::new(Semaphore::new(1)),
            health: watch::channel(Health::default()).0,
            snapshot_checked: false,
            restore_snapshots: true,
            snapshot_pending: None,
            snapshot_error: None,
        })
    }
    /// Wait for work retained by canceled/timed-out callbacks to release its
    /// context. The caller stops run/step first and supplies a shutdown deadline.
    pub async fn wait_idle(&self) -> Result<()> {
        let _permit = self.slots.acquire().await.map_err(|_| Error::Unavailable)?;
        Ok(())
    }
    /// Rebuild from selected native data instead of restoring an adapter snapshot.
    pub fn skip_snapshot_restore(&mut self) {
        self.restore_snapshots = false;
    }
    pub fn subscribe(&self) -> watch::Receiver<Health> {
        self.health.subscribe()
    }
    pub fn health(&self) -> Health {
        self.health.borrow().clone()
    }
    pub async fn step(&mut self, cancel: CancellationToken) -> Result<Health> {
        let context = Context::new(self.lease.clone(), self.slots.clone(), self.limits, cancel);
        let result = match context {
            Ok(context) => context.until(self.step_inner(&context)).await,
            Err(e) => Err(e),
        };
        self.health.send_modify(|health| match &result {
            Ok(()) => {
                health.degraded = self.snapshot_error.is_some();
                health.last_error = self.snapshot_error.clone();
            }
            Err(error) => {
                health.degraded = true;
                health.last_error = Some(error.clone());
            }
        });
        result.map(|()| self.health())
    }
    async fn step_inner(&mut self, context: &Context) -> Result<()> {
        let source = self.source.capture(context).await?;
        let current = source.position();
        self.health.send_modify(|health| {
            health.repository_position = Some(current.clone());
            health.lag = health
                .applied
                .as_ref()
                .filter(|a| a.source.epoch == current.epoch)
                .map(|a| current.position.0.saturating_sub(a.source.position.0));
        });
        let remote = progress::read(&self.source, &self.identity, context, current.epoch).await?;
        if let Some(checkpoint) = &remote.checkpoint
            && checkpoint.source.epoch == current.epoch
        {
            source.validate_position(&checkpoint.source).await?;
        }
        self.health
            .send_modify(|health| health.remote = remote.checkpoint.clone());
        let recovered = self.projection.recover(context.clone()).await?;
        let mut local = None;
        let mut selected_snapshot = None;
        if let Some(checkpoint) = recovered {
            checkpoint.validate()?;
            if checkpoint.source.repository != current.repository {
                return Err(Error::Corrupt);
            }
            if checkpoint.identity == self.identity && checkpoint.source.epoch == current.epoch {
                source.validate_position(&checkpoint.source).await?;
                local = Some(checkpoint);
            }
        }
        if local.is_none() && self.limits.snapshot_every > 0 && self.restore_snapshots {
            let snapshot = source.bootstrap().await?;
            if let Some(checkpoint) = self
                .projection
                .restore_snapshot(&snapshot, &self.identity, context.clone())
                .await?
            {
                checkpoint.validate()?;
                if checkpoint.identity != self.identity
                    || checkpoint.source.repository != current.repository
                    || checkpoint.source.epoch != current.epoch
                    || checkpoint.source.position.0 > current.position.0
                {
                    return Err(Error::Corrupt);
                }
                source.validate_position(&checkpoint.source).await?;
                if self.projection.recover(context.clone()).await?.as_ref() != Some(&checkpoint) {
                    return Err(Error::Corrupt);
                }
                local = Some(checkpoint);
                self.snapshot_checked = true;
                self.snapshot_error = None;
            }
            selected_snapshot = Some(snapshot);
        }
        self.set_applied(local.as_ref(), &current);
        let previous = local.clone();
        let (target, checkpoint, snapshot) = if let Some(local) = local {
            if let Some(batch) = source.batch(&local.source).await? {
                let snapshot = batch.snapshot.clone();
                let target = batch.snapshot.position.clone();
                let checkpoint = self
                    .projection
                    .apply(batch, &self.identity, context.clone())
                    .await?;
                (target, checkpoint, Some(snapshot))
            } else {
                (local.source.clone(), local, selected_snapshot)
            }
        } else {
            let snapshot = match selected_snapshot {
                Some(snapshot) => snapshot,
                None => source.bootstrap().await?,
            };
            let target = snapshot.position.clone();
            let checkpoint = self
                .projection
                .bootstrap(snapshot.clone(), &self.identity, context.clone())
                .await?;
            (target, checkpoint, Some(snapshot))
        };
        checkpoint.validate()?;
        if checkpoint.identity != self.identity || checkpoint.source != target {
            return Err(Error::Corrupt);
        }
        // A successful callback alone cannot advance the cursor: the projection
        // must already be able to recover the same durable local artifact.
        if self.projection.recover(context.clone()).await?.as_ref() != Some(&checkpoint) {
            return Err(Error::Corrupt);
        }
        if self.limits.snapshot_every > 0 {
            let interval = self.limits.snapshot_every;
            let crossed = previous.as_ref().is_none_or(|old| {
                old.source.position.0 / interval < checkpoint.source.position.0 / interval
            });
            if !self.snapshot_checked || crossed || self.snapshot_pending.is_some() {
                self.snapshot_pending = Some(checkpoint.clone());
            }
            self.snapshot_checked = true;
            if self.snapshot_pending.as_ref() == Some(&checkpoint) {
                let snapshot = match snapshot {
                    Some(snapshot) if snapshot.position == checkpoint.source => snapshot,
                    _ => {
                        let current = source.bootstrap().await?;
                        if current.position == checkpoint.source {
                            current
                        } else {
                            current.at(checkpoint.source.position).await?
                        }
                    }
                };
                match self
                    .projection
                    .publish_snapshot(&snapshot, &checkpoint, &self.identity, context.clone())
                    .await
                {
                    Ok(()) => {
                        self.snapshot_pending = None;
                        self.snapshot_error = None;
                    }
                    Err(error @ (Error::Denied | Error::Canceled)) => return Err(error),
                    Err(error) => self.snapshot_error = Some(error),
                }
            }
        }
        source.store.authorize().await?;
        self.set_applied(Some(&checkpoint), &current);
        let selected = progress::persist(&self.source, &remote, &checkpoint, context).await?;
        let final_remote =
            progress::read(&self.source, &self.identity, context, current.epoch).await?;
        if final_remote.checkpoint.as_ref() != Some(&selected) {
            return Err(Error::CursorConflict);
        }
        self.health
            .send_modify(|health| health.remote = final_remote.checkpoint);
        Ok(())
    }
    fn set_applied(&self, applied: Option<&Checkpoint>, current: &Position) {
        self.health.send_modify(|health| {
            health.applied = applied.cloned();
            health.lag = applied.map(|a| current.position.0.saturating_sub(a.source.position.0));
        });
    }
    /// Poll selected authority even without wake hints. Poison records are
    /// retried with bounded backoff; repository publication is never called.
    pub async fn run(&mut self, cancel: CancellationToken) -> Result<()> {
        let mut delay = self.limits.poll;
        loop {
            if cancel.is_cancelled() {
                return Ok(());
            }
            match self.step(cancel.clone()).await {
                Ok(_) => delay = self.limits.poll,
                Err(Error::Canceled) => return Ok(()),
                Err(_) => delay = delay.saturating_mul(2).min(self.limits.max_backoff),
            }
            tokio::select! { biased;
                _=cancel.cancelled()=>return Ok(()),
                _=tokio::time::sleep(delay)=>{}
            }
        }
    }
}
