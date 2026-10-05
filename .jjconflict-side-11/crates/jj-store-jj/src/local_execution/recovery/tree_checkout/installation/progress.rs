use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::local_execution::recovery::tree_checkout) enum Intent {
    Directory(usize),
    Backup(usize),
    Install(usize),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::local_execution::recovery::tree_checkout) struct Progress {
    pub version: u32,
    pub plan: m::RecordDigest,
    pub inventory: m::RecordDigest,
    pub private: [u64; 2],
    pub originals: Vec<Option<Snapshot>>,
    pub candidates: Vec<Option<Snapshot>>,
    pub directories: Vec<Option<[u64; 8]>>,
    pub dirs_done: usize,
    // 0 original/absent, 1 backed up/absent, 2 candidate installed/deleted.
    pub phases: Vec<u8>,
    pub intent: Option<Intent>,
}
pub(in crate::local_execution::recovery::tree_checkout) const MAX_PROGRESS: usize =
    32 * 1024 * 1024;
impl Progress {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid)?;
        if bytes.len() > MAX_PROGRESS {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn validate(
        &self,
        plan: &TreeCheckoutPlanV1,
        inventory: &PhysicalInventoryV1,
        destination: &Destination,
    ) -> Result<()> {
        if self.version != 1
            || self.plan != plan.id()?
            || self.inventory != inventory.id()?
            || self.private != directory_stamp(&destination.directory)?[..2]
            || self.originals.len() != plan.changes.len()
            || self.candidates.len() != plan.changes.len()
            || self.phases.len() != plan.changes.len()
            || self.directories.len() != plan.directories.len()
            || self.dirs_done > plan.directories.len()
        {
            return Err(Error::Corrupt);
        }
        for (i, change) in plan.changes.iter().enumerate() {
            for (saved, value) in [
                (&self.originals[i], &change.before),
                (&self.candidates[i], &change.after),
            ] {
                match (saved, value) {
                    (Some(saved), Some(value)) => saved.value(value)?,
                    (None, None) => {}
                    _ => return Err(Error::Corrupt),
                }
            }
            if let Some(original) = &self.originals[i] {
                if inventory
                    .files()
                    .find(|f| f.path == change.path)
                    .map(|f| f.fingerprint)
                    != Some(original.fingerprint())
                {
                    return Err(Error::Corrupt);
                }
            } else if inventory.files().any(|f| f.path == change.path) {
                return Err(Error::Corrupt);
            }
            if self.phases[i] > 2 {
                return Err(Error::Corrupt);
            }
        }
        for (i, dir) in plan.directories.iter().enumerate() {
            if self.directories[i].is_some() != dir.identity.is_none() {
                return Err(Error::Corrupt);
            }
            if let Some(stamp) = self.directories[i]
                && (stamp[0] != self.private[0] || stamp[1] == 0 || stamp[3] & 0o170000 != 0o040000)
            {
                return Err(Error::Corrupt);
            }
        }
        if self
            .intent
            .is_some_and(|intent| Some(intent) != self.next(plan))
        {
            return Err(Error::Corrupt);
        }
        self.encode()?;
        Ok(())
    }
    pub fn next(&self, plan: &TreeCheckoutPlanV1) -> Option<Intent> {
        if let Some(i) = (self.dirs_done..plan.directories.len())
            .find(|&i| plan.directories[i].identity.is_none())
        {
            return Some(Intent::Directory(i));
        }
        for (i, change) in plan.changes.iter().enumerate() {
            match self.phases[i] {
                0 if change.before.is_some() => return Some(Intent::Backup(i)),
                0 | 1 if change.after.is_some() => return Some(Intent::Install(i)),
                _ => {}
            }
        }
        None
    }
    pub fn advance(&mut self, plan: &TreeCheckoutPlanV1) -> Result<()> {
        match self.intent.take().ok_or(Error::Invalid)? {
            Intent::Directory(i) => self.dirs_done = i + 1,
            Intent::Backup(i) => {
                self.phases[i] = if plan.changes[i].after.is_none() {
                    2
                } else {
                    1
                }
            }
            Intent::Install(i) => self.phases[i] = 2,
        }
        Ok(())
    }
    pub fn prepare(
        recovery: &RecoveredHistory,
        plan: &TreeCheckoutPlanV1,
        inventory: &PhysicalInventoryV1,
        destination: &Destination,
        budget: &Budget,
    ) -> Result<Self> {
        let root = physical::directory(&recovery.root)?;
        let private = directory_stamp(&destination.directory)?;
        if private[0] != directory_stamp(&root)?[0] {
            return Err(Error::Invalid);
        }
        let mut state = Self {
            version: 1,
            plan: plan.id()?,
            inventory: inventory.id()?,
            private: [private[0], private[1]],
            originals: vec![],
            candidates: vec![],
            directories: vec![],
            dirs_done: 0,
            phases: vec![0; plan.changes.len()],
            intent: None,
        };
        // Bound both original and final namespace before any physical write.
        let records = inventory.files().count() + inventory.directories().count();
        let additions = plan.changes.iter().filter(|c| c.before.is_none()).count()
            + plan
                .directories
                .iter()
                .filter(|d| d.identity.is_none())
                .count();
        if records.checked_add(additions).ok_or(Error::Limit)? > 4097 {
            return Err(Error::Limit);
        }
        let git = gix::open(recovery.witness.git_path()).map_err(|_| Error::Unavailable)?;
        for (i, change) in plan.changes.iter().enumerate() {
            let original = parent(&root, &change.path)?
                .map(|(dir, name)| files::read(&dir, &name, budget))
                .transpose()?
                .flatten();
            if original.as_ref().map(|v| v.1.fingerprint) != change.observed {
                return Err(Error::EpochChanged);
            }
            if original
                .as_ref()
                .is_some_and(|v| v.0.stamp[0] != private[0])
            {
                return Err(Error::Invalid);
            }
            state.originals.push(original.map(|v| v.0));
            state.candidates.push(if let Some(value) = &change.after {
                let bytes = effects::object(
                    &git,
                    value.object,
                    gix::objs::Kind::Blob,
                    tree::MAX_FILE_BYTES,
                    budget,
                )?;
                if tree::content_digest(&bytes) != value.content || bytes.len() as u64 != value.size
                {
                    return Err(Error::Corrupt);
                }
                Some(files::prepare_file(
                    &destination.directory,
                    &format!("f-{i}"),
                    &bytes,
                    value.executable,
                    budget,
                )?)
            } else {
                None
            });
        }
        for (i, dir) in plan.directories.iter().enumerate() {
            state.directories.push(if dir.identity.is_none() {
                let name = format!("d-{i}");
                if records::destination::optional(&destination.directory, &name)?.is_none() {
                    fs::mkdirat(
                        &destination.directory,
                        &name,
                        fs::Mode::from_raw_mode(0o755),
                    )
                    .map_err(|_| Error::Unavailable)?;
                }
                let staged = physical::child(&destination.directory, &name)?;
                if !files::entries(&staged, 1)?.is_empty() {
                    return Err(Error::EpochChanged);
                }
                staged.sync_all().map_err(|_| Error::Unavailable)?;
                Some(directory_stamp(&staged)?)
            } else {
                None
            });
        }
        destination
            .directory
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
        state.validate(plan, inventory, destination)?;
        Ok(state)
    }
}
