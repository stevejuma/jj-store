//! Typed WorkStore records and audited root-pinned catalogs.
use super::*;
use jj_work::MAX_ENTITIES;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRootV1 {
    pub schema_version: u32,
    pub projects: Option<RecordRef>,
    pub tasks: Option<RecordRef>,
    pub comments: Option<RecordRef>,
}
impl WorkRootV1 {
    pub fn empty() -> Self {
        Self {
            schema_version: 1,
            projects: None,
            tasks: None,
            comments: None,
        }
    }
    fn catalogs(&self) -> [Option<RecordRef>; 3] {
        [self.projects, self.tasks, self.comments]
    }
    fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self
                .catalogs()
                .into_iter()
                .flatten()
                .any(|r| r.kind != RecordKind::Catalog || r.attempt.is_some())
        {
            return Err(Error::Corrupt("work root"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum WorkObjectV1 {
    Project(ProjectV1),
    Task(TaskV1),
    Comment(WorkCommentV1),
}
impl WorkObjectV1 {
    fn key(&self) -> RecordDigest {
        match self {
            Self::Project(p) => work_key(b"project", p.id.as_bytes()),
            Self::Task(t) => work_key(b"task", t.id.as_bytes()),
            Self::Comment(c) => work_key(b"comment", c.id.as_bytes()),
        }
    }
}
fn work_key(kind: &[u8], id: &[u8; 16]) -> RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/work-key/v1\0");
    hash.update(kind);
    hash.update(id);
    RecordDigest::new(hash.finalize().into())
}
#[derive(Clone, Debug)]
pub struct WorkStoreSnapshot {
    pub(crate) reference: RecordRef,
    pub(crate) root: WorkRootV1,
    pub(crate) state: WorkStateV1,
    pub(crate) records: BTreeSet<RecordRef>,
}
impl WorkStoreSnapshot {
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn state(&self) -> &WorkStateV1 {
        &self.state
    }
    pub fn object_count(&self) -> usize {
        self.records.len()
    }
}
impl Coordinator {
    pub async fn work_store(&self, reference: RecordRef) -> Result<WorkStoreSnapshot> {
        let root: WorkRootV1 = self.records.get(reference, RecordKind::WorkRoot).await?;
        root.validate()?;
        let mut state = WorkStateV1::default();
        let mut records = BTreeSet::from([reference]);
        for (catalog, root) in root.catalogs().into_iter().enumerate() {
            let closure = self.catalog.closure(root, MAX_ENTITIES).await?;
            records.extend(closure.pages);
            for (key, reference) in closure.entries {
                if reference.attempt.is_some() {
                    return Err(Error::Corrupt("work object reference"));
                }
                let object: WorkObjectV1 =
                    self.records.get(reference, RecordKind::WorkObject).await?;
                if object.key() != key {
                    return Err(Error::Corrupt("work catalog key"));
                }
                records.insert(reference);
                match (catalog, object) {
                    (0, WorkObjectV1::Project(p)) => {
                        if state.projects.insert(p.id, p).is_some() {
                            return Err(Error::Corrupt("duplicate project"));
                        }
                    }
                    (1, WorkObjectV1::Task(t)) => {
                        if state.tasks.insert(t.id, t).is_some() {
                            return Err(Error::Corrupt("duplicate task"));
                        }
                    }
                    (2, WorkObjectV1::Comment(c)) => {
                        if state.comments.insert(c.id, c).is_some() {
                            return Err(Error::Corrupt("duplicate comment"));
                        }
                    }
                    _ => return Err(Error::Corrupt("work catalog kind")),
                }
            }
        }
        state.validate().map_err(work_model_error)?;
        Ok(WorkStoreSnapshot {
            reference,
            root,
            state,
            records,
        })
    }
    pub(super) async fn stage_work_result(
        &self,
        pinned: &WorkStoreSnapshot,
        result: &WorkCommandResultV1,
    ) -> Result<RecordRef> {
        let mut root = pinned.root.clone();
        let objects = [
            result.project.clone().map(WorkObjectV1::Project),
            result.task.clone().map(WorkObjectV1::Task),
            result.comment.clone().map(WorkObjectV1::Comment),
        ];
        for (catalog, object) in objects.into_iter().enumerate() {
            let Some(object) = object else {
                continue;
            };
            let key = object.key();
            let reference = self
                .records
                .put(RecordKind::WorkObject, None, &object)
                .await?;
            let previous = root.catalogs()[catalog];
            let selected = self
                .catalog
                .update(previous, BTreeMap::from([(key, reference)]))
                .await?;
            match catalog {
                0 => root.projects = selected,
                1 => root.tasks = selected,
                _ => root.comments = selected,
            }
        }
        root.validate()?;
        let reference = self.records.put(RecordKind::WorkRoot, None, &root).await?;
        self.work_store(reference).await?;
        Ok(reference)
    }
    pub(super) async fn empty_work_store(&self) -> Result<WorkStoreSnapshot> {
        let reference = self
            .records
            .put(RecordKind::WorkRoot, None, &WorkRootV1::empty())
            .await?;
        self.work_store(reference).await
    }
}
pub(super) fn work_model_error(error: jj_work::Error) -> Error {
    match error {
        jj_work::Error::Limit => Error::Limit("work model"),
        jj_work::Error::Stale => Error::Stale,
        _ => Error::Corrupt("work model"),
    }
}

impl Coordinator {
    /// Build a validated independent snapshot; staging publishes no repository state.
    pub async fn stage_conversion_work(&self, state: &WorkStateV1) -> Result<RecordRef> {
        state.validate().map_err(work_model_error)?;
        let mut root = WorkRootV1::empty();
        for (index, objects) in [
            state
                .projects
                .values()
                .cloned()
                .map(WorkObjectV1::Project)
                .collect::<Vec<_>>(),
            state
                .tasks
                .values()
                .cloned()
                .map(WorkObjectV1::Task)
                .collect(),
            state
                .comments
                .values()
                .cloned()
                .map(WorkObjectV1::Comment)
                .collect(),
        ]
        .into_iter()
        .enumerate()
        {
            let mut entries = BTreeMap::new();
            for object in objects {
                let key = object.key();
                let reference = self
                    .records
                    .put(RecordKind::WorkObject, None, &object)
                    .await?;
                entries.insert(key, reference);
            }
            let selected = self.catalog.update(None, entries).await?;
            match index {
                0 => root.projects = selected,
                1 => root.tasks = selected,
                _ => root.comments = selected,
            }
        }
        let reference = self.records.put(RecordKind::WorkRoot, None, &root).await?;
        if self.work_store(reference).await?.state() != state {
            return Err(Error::Corrupt("converted work snapshot"));
        }
        Ok(reference)
    }
}
