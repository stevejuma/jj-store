//! An explicit task owns its checked root independently of a short page session.
use super::*;

pub(super) struct Access {
    session: Arc<NativeSession>,
    pin: ReadPin,
    members: Mutex<BTreeSet<m::ObjectKey>>,
}

#[cfg(test)]
mod tests;

impl Access {
    pub(super) async fn capture(read: &CommittedRead) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            session: read.session.clone(),
            pin: read.pin().await?,
            members: Default::default(),
        }))
    }
}
#[async_trait::async_trait]
impl query::ReadAccess for Access {
    async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        limits: query::copy_index::LookupLimits,
        deadline: Instant,
    ) -> query::Result<Option<query::copy_index::Family>> {
        self.session
            .copy_family_at(&self.pin.base, seeds, required, limits, deadline)
            .await
            .map(Some)
    }
    fn repository(&self) -> m::RepositoryId {
        self.pin.snapshot.authority.repo_id
    }
    fn epoch(&self) -> m::EpochId {
        self.pin.snapshot.authority.epoch
    }
    fn operation(&self) -> m::OperationId {
        self.pin.snapshot.operation_id
    }
    async fn authorize(&self) -> query::Result<()> {
        within_deadline(Instant::now() + Duration::from_secs(10), async {
            let current = self.session.snapshot().await?;
            if current.manifest().epoch != self.pin.snapshot.authority.epoch {
                return Err(Error::EpochChanged);
            }
            Ok(())
        })
        .await
        .map_err(query_error)
    }
    async fn contains(&self, key: m::ObjectKey) -> query::Result<bool> {
        self.authorize().await?;
        let found = within_deadline(Instant::now() + Duration::from_secs(10), async {
            let mut members = self.members.lock().await;
            if members.contains(&key) {
                return Ok(true);
            }
            let found = self
                .pin
                .base
                .inventory_member(self.session.object_store.clone(), key)
                .await?
                .is_some();
            if found {
                // Cache eviction cannot turn a previously admitted object into
                // authority: a miss repeats verified point membership at the pin.
                if members.len() >= 1024 {
                    members.clear();
                }
                members.insert(key);
            }
            Ok(found)
        })
        .await
        .map_err(query_error)?;
        self.authorize().await?;
        Ok(found)
    }
    async fn inventory(&self) -> query::Result<Arc<BTreeSet<m::ObjectKey>>> {
        Err(query::Error::IndexBudget)
    }
    async fn inventory_kind(
        &self,
        kind: query::InventoryKind,
        limit: usize,
        deadline: Instant,
    ) -> query::Result<BTreeSet<m::ObjectKey>> {
        self.session
            .inventory_ids_at(
                &self.pin.base,
                matches!(kind, query::InventoryKind::Copies),
                limit,
                deadline,
            )
            .await
            .map_err(kind_error)
    }
}
