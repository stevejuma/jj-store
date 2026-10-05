//! Missing historical operations build privately, then retain verified memory.
use super::*;
use cache::resident::Data;
pub(super) struct Secondary {
    pub root: PathBuf,
    pub policy: cache::Policy,
    pub residents: Arc<cache::resident::Residents>,
    pub access: Arc<dyn ReadAccess>,
    pub budget: Arc<Budget>,
}
impl Secondary {
    pub async fn read(
        &self,
        operation: &Operation,
        store: &Arc<Store>,
    ) -> Result<Box<dyn ReadonlyIndex>> {
        self.budget.authorize(self.access.as_ref()).await?;
        let selected =
            m::OperationId::from_slice(operation.id().as_bytes()).map_err(|_| Error::Corrupt)?;
        if let Some(saved) = self.residents.get_memory_at(self.access.as_ref(), selected) {
            return saved.reader(self.budget.clone());
        }
        if let Some(data) = self
            .residents
            .get_additional(self.access.as_ref(), selected)
        {
            return data.reader(self.budget.clone());
        }
        let reserve = || {
            cache::Reservation::acquire_for_query(
                &self.root,
                self.access.repository(),
                self.policy,
                &self.budget,
            )
        };
        let reservation = match reserve() {
            Err(Error::Limit) if self.residents.reclaim_filtered() => reserve()?,
            result => result?,
        };
        let path = reservation.path().join("index");
        std::fs::create_dir(&path).map_err(|_| Error::Unavailable)?;
        let bytes = cache::construction_bytes(reservation.path(), self.policy, &self.budget)?
            .min(reservation.bytes());
        super::super::prepared::build(&path, operation, store, &self.budget, bytes, true, false)
            .await?;
        cache::constructed::verify(&path, selected, self.policy, &self.budget)?;
        let memory = Arc::new(memory::Indexes::default());
        memory.reserve(
            &path,
            store.commit_id_length(),
            store.change_id_length(),
            self.budget.limits.max_index_commits,
            || self.budget.work(1),
        )?;
        let loaded = DefaultIndexStore::load(&path)
            .get_index_at_op(operation, store)
            .await
            .map_err(|_| Error::Unavailable)?;
        let raw = loaded
            .downcast_ref::<DefaultReadonlyIndex>()
            .ok_or(Error::Unavailable)?;
        let view = operation.view().await.map_err(|_| Error::Incomplete)?;
        let index = projection::Prepared::build(
            raw.clone(),
            &view,
            store,
            self.access.as_ref(),
            &self.budget,
            reservation.path(),
            &path,
            selected,
        )
        .await?;
        self.budget.authorize(self.access.as_ref()).await?;
        let data = Data { index, memory };
        // JJ's readonly arrays are fully owned. No reader needs these files.
        drop(loaded);
        drop(reservation);
        self.residents.remember_additional(
            self.access.as_ref(),
            selected,
            data.clone(),
            self.policy,
        );
        data.reader(self.budget.clone())
    }
}
