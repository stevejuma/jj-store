//! Immutable workflow catalogs rooted only by the native manifest.
use super::*;
use jj_workflow as w;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowRoot {
    pub version: u32,
    pub catalogs: [Option<RecordRef>; 5],
}
impl WorkflowRoot {
    fn empty() -> Self {
        Self {
            version: 1,
            catalogs: [None; 5],
        }
    }
    fn validate(&self) -> Result<()> {
        if self.version != 1
            || self
                .catalogs
                .iter()
                .flatten()
                .any(|r| r.kind != RecordKind::Catalog || r.attempt.is_some())
        {
            return Err(Error::Corrupt("workflow root"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Object {
    Selection {
        workflow: w::WorkflowId,
        version: Option<w::VersionId>,
    },
    Definition(w::DefinitionVersion),
    Evaluation(w::Evaluation),
    Run(w::Run),
    Evidence(w::Evidence),
}
impl Object {
    fn key(&self) -> RecordDigest {
        let (kind, id): (&[u8], &[u8]) = match self {
            Self::Selection { workflow, .. } => (b"selection", workflow.as_bytes()),
            Self::Definition(v) => (b"definition", v.version.as_bytes()),
            Self::Evaluation(v) => (b"event", v.event.id.as_bytes()),
            Self::Run(v) => (b"run", v.id.as_bytes()),
            Self::Evidence(v) => (b"evidence", v.id.as_bytes()),
        };
        let mut hash = Sha256::new_with_prefix(b"jj-store/workflow-catalog/v1\0");
        hash.update(kind);
        hash.update(id);
        RecordDigest::new(hash.finalize().into())
    }
}
#[derive(Clone, Debug)]
pub struct WorkflowStoreSnapshot {
    reference: RecordRef,
    root: WorkflowRoot,
    state: w::State,
    pub(crate) records: BTreeSet<RecordRef>,
}
impl WorkflowStoreSnapshot {
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn state(&self) -> &w::State {
        &self.state
    }
}
impl Coordinator {
    /// Source identity derived from a selected record, never a receiver WAL number.
    pub async fn workflow_source_publications(
        &self,
        record: &SelectedRecord,
    ) -> Result<Vec<(String, BTreeSet<OperationId>)>> {
        let tx = record.transaction();
        if tx.repository != self.records.repository {
            return Err(Error::Denied);
        }
        if matches!(tx.kind, SemanticKind::ReplicationSeeded) {
            return Ok(Vec::new());
        }
        if tx.origins.is_empty() {
            if tx.published_operations.is_empty() {
                return Ok(Vec::new());
            }
            return Ok(vec![(
                format!("{}/{}/{}", tx.repository, tx.after.epoch, tx.receipt.digest),
                tx.published_operations.clone(),
            )]);
        }
        let mut result = Vec::new();
        for reference in &tx.origins {
            let receipt: OriginReceiptV1 = self
                .records
                .get(*reference, RecordKind::OriginReceipt)
                .await?;
            let origin = self.origin_publication(receipt).await?;
            if !origin.published_operations.is_empty() {
                result.push((
                    format!(
                        "origin/{}/{}",
                        origin.origin_key.stream.repository, receipt.origin_digest
                    ),
                    origin.published_operations,
                ));
            }
        }
        Ok(result)
    }
    pub async fn workflow_store(&self, reference: RecordRef) -> Result<WorkflowStoreSnapshot> {
        let root: WorkflowRoot = self
            .records
            .get(reference, RecordKind::WorkflowRoot)
            .await?;
        root.validate()?;
        let mut state = w::State::default();
        let mut records = BTreeSet::from([reference]);
        for (index, catalog) in root.catalogs.iter().enumerate() {
            let closure = self.catalog.closure(*catalog, w::MAX_ENTITIES).await?;
            records.extend(closure.pages);
            for (key, reference) in closure.entries {
                let object: Object = self
                    .records
                    .get(reference, RecordKind::WorkflowObject)
                    .await?;
                if reference.attempt.is_some() || object.key() != key {
                    return Err(Error::Corrupt("workflow object key"));
                }
                records.insert(reference);
                match (index, object) {
                    (0, Object::Selection { workflow, version }) => {
                        if let Some(version) = version {
                            state.workflows.insert(workflow, version);
                        }
                    }
                    (1, Object::Definition(v)) => {
                        state.definitions.insert(v.version, v);
                    }
                    (2, Object::Evaluation(v)) => {
                        state.evaluations.insert(v.event.id, v);
                    }
                    (3, Object::Run(v)) => {
                        state.runs.insert(v.id, v);
                    }
                    (4, Object::Evidence(v)) => {
                        state.evidence.insert(v.id, v);
                    }
                    _ => return Err(Error::Corrupt("workflow catalog kind")),
                }
            }
        }
        state.validate().map_err(workflow_error)?;
        Ok(WorkflowStoreSnapshot {
            reference,
            root,
            state,
            records,
        })
    }
    pub(super) async fn empty_workflow_store(&self) -> Result<WorkflowStoreSnapshot> {
        let reference = self
            .records
            .put(RecordKind::WorkflowRoot, None, &WorkflowRoot::empty())
            .await?;
        self.workflow_store(reference).await
    }
    pub(super) async fn stage_workflow_result(
        &self,
        pinned: &WorkflowStoreSnapshot,
        result: &w::CommandResult,
    ) -> Result<RecordRef> {
        let mut root = pinned.root.clone();
        let mut objects: [Vec<Object>; 5] = std::array::from_fn(|_| Vec::new());
        if let Some(workflow) = result.workflow {
            objects[0].push(Object::Selection {
                workflow,
                version: result.definition.as_ref().map(|v| v.version),
            });
        }
        if let Some(v) = &result.definition {
            objects[1].push(Object::Definition(v.clone()));
        }
        if let Some(v) = &result.evaluation {
            objects[2].push(Object::Evaluation(v.clone()));
        }
        objects[3].extend(result.runs.iter().cloned().map(Object::Run));
        if let Some(v) = &result.evidence {
            objects[4].push(Object::Evidence(v.clone()));
        }
        for (index, values) in objects.into_iter().enumerate() {
            let mut entries = BTreeMap::new();
            for object in values {
                entries.insert(
                    object.key(),
                    self.records
                        .put(RecordKind::WorkflowObject, None, &object)
                        .await?,
                );
            }
            if !entries.is_empty() {
                root.catalogs[index] = self.catalog.update(root.catalogs[index], entries).await?;
            }
        }
        let reference = self
            .records
            .put(RecordKind::WorkflowRoot, None, &root)
            .await?;
        self.workflow_store(reference).await?;
        Ok(reference)
    }
}
pub(super) fn workflow_error(error: w::Error) -> Error {
    match error {
        w::Error::Stale => Error::Stale,
        w::Error::Denied => Error::Denied,
        w::Error::Limit => Error::Limit("workflow"),
        _ => Error::Corrupt("workflow model"),
    }
}
