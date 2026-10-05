use super::*;
use jj_store_jj::conversion::CodeArchiveV1;
use jj_store_wal::records::RecordRef;
/// Explicit archive format, independent of either provider's persistent Work root.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkSnapshotArchiveV1 {
    pub version: u32,
    pub projects: BTreeMap<ProjectId, ProjectV1>,
    pub tasks: BTreeMap<TaskId, TaskV1>,
    pub comments: BTreeMap<WorkCommentId, WorkCommentV1>,
}
impl WorkSnapshotArchiveV1 {
    pub fn from_state(state: WorkStateV1) -> Result<Self> {
        state.validate()?;
        Ok(Self {
            version: 1,
            projects: state.projects,
            tasks: state.tasks,
            comments: state.comments,
        })
    }
    pub fn state(&self) -> Result<WorkStateV1> {
        if self.version != 1 {
            return Err("unsupported conversion work snapshot".into());
        }
        let state = WorkStateV1 {
            projects: self.projects.clone(),
            tasks: self.tasks.clone(),
            comments: self.comments.clone(),
        };
        state.validate()?;
        Ok(state)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollaborationSelectionV1 {
    pub source_pin: String,
    pub versions: BTreeMap<String, u64>,
    pub work: Option<WorkSnapshotArchiveV1>,
    pub agent: AgentGraphV1,
    pub review: ReviewGraphV1,
    pub collab: CollabGraphArchiveV1,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryArchiveV1 {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub source_pin: String,
    pub code: CodeArchiveV1,
    pub collaboration: Vec<CollaborationSelectionV1>,
    pub native_records: Vec<(RecordRef, Vec<u8>)>,
    pub native_receipts: Vec<(RecordRef, Vec<u8>)>,
}
impl RepositoryArchiveV1 {
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.version != 1 || self.collaboration.len() > 4096 {
            return Err("conversion archive format or history limit".into());
        }
        for selection in &self.collaboration {
            if let Some(work) = &selection.work {
                work.state()?;
            }
            selection.agent.root.validate()?;
            selection.review.root.validate()?;
        }
        let bytes = serde_json::to_vec(self)?;
        if bytes.len() > 16 * 1024 * 1024 {
            return Err("conversion archive byte limit".into());
        }
        Ok(bytes)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err("conversion archive byte limit".into());
        }
        let value: Self = serde_json::from_slice(bytes)?;
        if value.encode()?.as_slice() != bytes {
            return Err("noncanonical conversion archive".into());
        }
        Ok(value)
    }
}
impl crate::native::NativeSession {
    pub async fn conversion_archive(
        &self,
        operation: m::OperationId,
    ) -> Result<RepositoryArchiveV1> {
        self.principal(crate::auth::Permission::Admin).await?;
        let pin = self.snapshot().await?;
        if !pin.manifest().heads.contains(&operation) {
            return Err("conversion requires a selected source operation head".into());
        }
        let inventory = self
            .coordinator
            .inventory(&pin, self.limits.inventory_nodes)
            .await?;
        let code = jj_store_jj::conversion::archive_native(
            self.object_store.clone(),
            self.repository,
            operation,
            &inventory,
        )
        .await?;
        let mut size = serde_json::to_vec(&code)?.len();
        if size > 16 * 1024 * 1024 {
            return Err("conversion archive byte limit".into());
        }
        let mut states = Vec::new();
        let mut receipts = BTreeMap::new();
        let mut after = m::WalPosition(0);
        while after < pin.manifest().published {
            for transaction in self.coordinator.committed(&pin, after, 1024).await? {
                if states.is_empty() {
                    states.push((transaction.before.position, transaction.before_domains));
                }
                if states
                    .last()
                    .is_none_or(|(_, state)| *state != transaction.after_domains)
                {
                    states.push((transaction.after.position, transaction.after_domains));
                }
                let bytes = self
                    .coordinator
                    .conversion_receipt_archive(transaction.receipt)
                    .await?;
                size = size
                    .checked_add(serde_json::to_vec(&(transaction.receipt, &bytes))?.len())
                    .ok_or("conversion archive byte limit")?;
                if size > 16 * 1024 * 1024 {
                    return Err("conversion archive byte limit".into());
                }
                receipts.insert(transaction.receipt, bytes);
                after = transaction.after.position;
                if receipts.len() > 4096 {
                    return Err("conversion receipt history limit".into());
                }
            }
        }
        if states.is_empty() {
            states.push((pin.manifest().published, pin.manifest().domains()));
        }
        let mut collaboration = Vec::new();
        let mut native_records = BTreeMap::new();
        for (position, domains) in states {
            for (reference, bytes) in self.coordinator.conversion_domain_archive(domains).await? {
                if !native_records.contains_key(&reference) {
                    size = size
                        .checked_add(serde_json::to_vec(&(reference, &bytes))?.len())
                        .ok_or("conversion archive byte limit")?;
                    if size > 16 * 1024 * 1024 {
                        return Err("conversion archive byte limit".into());
                    }
                }
                native_records.insert(reference, bytes);
            }
            let mut versions = BTreeMap::new();
            let (work, agent, review, thread, reaction) = if let Some(d) = domains {
                versions.insert("agent".into(), d.agent.version);
                versions.insert("review".into(), d.review.version);
                for (name, state) in [
                    ("work", d.work),
                    ("thread", d.thread),
                    ("reaction", d.reaction),
                ] {
                    if let Some(state) = state {
                        versions.insert(name.into(), state.version);
                    }
                }
                let work = match d.work {
                    Some(v) => Some(WorkSnapshotArchiveV1::from_state(
                        self.coordinator.work_store(v.root).await?.state().clone(),
                    )?),
                    None => None,
                };
                let agent =
                    archive_agent(&self.coordinator.agent_store(d.agent.root).await?).await?;
                let review =
                    archive_review(&self.coordinator.review_store(d.review.root).await?).await?;
                let thread = match d.thread {
                    Some(v) => Some(self.coordinator.native_thread_store(v.root).await?),
                    None => None,
                };
                let reaction = match d.reaction {
                    Some(v) => Some(self.coordinator.native_reaction_store(v.root).await?),
                    None => None,
                };
                (work, agent, review, thread, reaction)
            } else {
                (
                    None,
                    AgentGraphV1 {
                        root: AgentRootV1::empty(),
                        objects: vec![],
                    },
                    ReviewGraphV1 {
                        root: ReviewRootV1::empty(),
                        objects: vec![],
                    },
                    None,
                    None,
                )
            };
            let mut objects = BTreeMap::new();
            if let Some(t) = &thread {
                objects.extend(t.graph.objects.clone());
            }
            if let Some(r) = &reaction {
                objects.extend(r.graph.objects.clone());
            }
            // Exact external closures can retain an older session/review than the current roots.
            let mut agent = agent;
            let mut review = review;
            include_external(
                &mut agent,
                &mut review,
                objects.values(),
                &self.coordinator,
                &self.coordinator,
            )
            .await?;
            let collab = CollabGraphArchiveV1 {
                thread: thread.map(|v| v.state.root),
                reaction: reaction.map(|v| v.root),
                objects: objects.into_iter().collect(),
            };
            let state = CollaborationSelectionV1 {
                source_pin: format!("native:{}:{}", pin.manifest().epoch, position.0),
                versions,
                work,
                agent,
                review,
                collab,
            };
            size = size
                .checked_add(serde_json::to_vec(&state)?.len())
                .ok_or("conversion archive byte limit")?;
            if size > 16 * 1024 * 1024 {
                return Err("conversion archive byte limit".into());
            }
            collaboration.push(state);
        }
        self.principal(crate::auth::Permission::Admin).await?;
        let value = RepositoryArchiveV1 {
            version: 1,
            repository: self.repository,
            source_pin: format!(
                "native:{}:{}:{operation}",
                pin.manifest().epoch,
                pin.digest()
            ),
            code,
            collaboration,
            native_records: native_records.into_iter().collect(),
            native_receipts: receipts.into_iter().collect(),
        };
        value.encode()?;
        Ok(value)
    }
}
pub(crate) async fn include_external<'a>(
    agent: &mut AgentGraphV1,
    review: &mut ReviewGraphV1,
    objects: impl IntoIterator<Item = &'a jj_collab::CollabObjectV1>,
    agent_io: &dyn AgentObjectIo,
    review_io: &dyn ReviewObjectIo,
) -> Result<()> {
    let mut agents: BTreeMap<_, _> = agent.objects.iter().cloned().collect();
    let mut reviews: BTreeMap<_, _> = review.objects.iter().cloned().collect();
    let mut ap = Vec::new();
    let mut rp = Vec::new();
    for value in objects {
        ap.extend(value.agent_dependencies());
        rp.extend(value.review_dependencies());
    }
    while let Some(reference) = rp.pop() {
        if reviews.contains_key(&reference) {
            continue;
        }
        if reviews.len() > 100_000 {
            return Err("conversion external review limit".into());
        }
        let value = review_io.read_review_object(reference).await?;
        super::review::admit(&value)?;
        rp.extend(value.review_dependencies()?);
        ap.extend(value.agent_dependencies()?);
        reviews.insert(reference, value);
    }
    while let Some(reference) = ap.pop() {
        if agents.contains_key(&reference) {
            continue;
        }
        if agents.len() > 4096 {
            return Err("conversion external agent limit".into());
        }
        let value = agent_io.read_agent_object(reference).await?;
        super::agent::admit(&value)?;
        ap.extend(value.dependencies()?);
        agents.insert(reference, value);
    }
    agent.objects = agents.into_iter().collect();
    review.objects = reviews.into_iter().collect();
    Ok(())
}
