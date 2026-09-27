//! Native metadata and explicitly configured source scopes, through owned reads.
use crate::*;
use jj_store_adapters::{Batch, Context, Identity, Snapshot};
use jj_store_service::{
    adapter::{ProjectionRead, ROOT_COMMIT, native_dto as dto},
    reads::query::{
        self,
        content::{self, ContentReader, ObjectData},
    },
};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Scope {
    Bookmark { name: String },
    Workspace { name: String },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub scopes: BTreeMap<String, Scope>,
    pub index_source: bool,
    pub include_prefixes: BTreeSet<String>,
    pub exclude_prefixes: BTreeSet<String>,
    pub max_text_bytes: usize,
    pub max_files: usize,
    pub max_work: usize,
    pub max_output_bytes: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            scopes: BTreeMap::new(),
            index_source: true,
            include_prefixes: BTreeSet::new(),
            exclude_prefixes: BTreeSet::new(),
            max_text_bytes: 1024 * 1024,
            max_files: 10_000,
            max_work: 1_000_000,
            max_output_bytes: 64 * 1024 * 1024,
        }
    }
}
impl Config {
    fn validate(&self) -> Result<()> {
        if self.scopes.len() > 128
            || self.max_text_bytes == 0
            || self.max_text_bytes > 1024 * 1024
            || !(1..=10_000).contains(&self.max_files)
            || !(1..=1_000_000).contains(&self.max_work)
            || self.max_output_bytes == 0
            || self.max_output_bytes > 256 * 1024 * 1024
            || self.include_prefixes.len() + self.exclude_prefixes.len() > 128
        {
            return Err(Error::Limit);
        }
        for (id, scope) in &self.scopes {
            DocumentKey::File {
                scope: id.clone(),
                path: "_".into(),
            }
            .exact()?;
            let name = match scope {
                Scope::Bookmark { name } | Scope::Workspace { name } => name,
            };
            if name.is_empty() || name.len() > 4096 || name.contains('\0') {
                return Err(Error::Invalid);
            }
        }
        for path in self.include_prefixes.iter().chain(&self.exclude_prefixes) {
            DocumentKey::File {
                scope: "_".into(),
                path: path.clone(),
            }
            .exact()?;
        }
        Ok(())
    }
    pub fn identity(&self) -> Result<Identity> {
        self.validate()?;
        Ok(Identity {
            name: "tantivy".into(),
            implementation: IMPLEMENTATION.into(),
            schema: SCHEMA_VERSION,
            config: digest(&bounded_json(&("native-extraction-v1", self), 1024 * 1024)?),
        })
    }
    fn includes(&self, path: &str) -> bool {
        let matches = |prefix: &String| {
            path == prefix
                || path
                    .strip_prefix(prefix)
                    .is_some_and(|rest| rest.starts_with('/'))
        };
        (self.include_prefixes.is_empty() || self.include_prefixes.iter().any(matches))
            && !self.exclude_prefixes.iter().any(matches)
    }
}
pub struct NativeExtractor {
    config: Config,
    reader: ContentReader,
}
struct Budget<'a> {
    config: &'a Config,
    context: &'a Context,
    work: usize,
    bytes: usize,
}
impl Budget<'_> {
    fn work(&mut self, n: usize) -> Result<()> {
        self.context.check()?;
        self.work = self.work.checked_add(n).ok_or(Error::Limit)?;
        if self.work > self.config.max_work {
            return Err(Error::Limit);
        }
        Ok(())
    }
    fn push(&mut self, update: &mut Update, doc: Document) -> Result<()> {
        self.work(1)?;
        self.bytes = self
            .bytes
            .checked_add(bounded_json(&doc, self.config.max_output_bytes)?.len())
            .ok_or(Error::Limit)?;
        if self.bytes > self.config.max_output_bytes {
            return Err(Error::Limit);
        }
        update.upsert.push(doc);
        Ok(())
    }
}
impl NativeExtractor {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let reader = ContentReader::new(
            content::Limits {
                max_changes: config.max_files,
                max_text_bytes: config.max_text_bytes,
                max_output_bytes: config.max_output_bytes,
                ..Default::default()
            },
            1,
        )
        .map_err(map_query)?;
        Ok(Self { config, reader })
    }
    async fn read(&self, snapshot: &Snapshot) -> Result<ProjectionRead> {
        let operation = *snapshot.heads.first().ok_or(Error::Corrupt)?;
        ProjectionRead::new(snapshot, operation, self.reader.clone()).await
    }
    async fn scopes(
        &self,
        snapshot: &Snapshot,
        read: &ProjectionRead,
    ) -> Result<BTreeMap<String, ScopeSnapshot>> {
        let operation = (snapshot.heads.len() == 1).then(|| *snapshot.heads.first().unwrap());
        let view = if self.config.index_source && !self.config.scopes.is_empty() {
            if let Some(id) = operation {
                let ObjectData::Operation(op) = read
                    .object(m::ObjectKey::Operation(id))
                    .await
                    .map_err(map_query)?
                    .data
                else {
                    return Err(Error::Corrupt);
                };
                let ObjectData::View(view) = read
                    .object(m::ObjectKey::View(op.view_id))
                    .await
                    .map_err(map_query)?
                    .data
                else {
                    return Err(Error::Corrupt);
                };
                Some(view)
            } else {
                None
            }
        } else {
            None
        };
        let mut out = BTreeMap::new();
        for (id, scope) in &self.config.scopes {
            let (state, commits) = if !self.config.index_source {
                (ScopeState::Disabled, BTreeSet::new())
            } else if let Some(view) = &view {
                match scope {
                    Scope::Workspace { name } => match view.wc_commit_ids.get(name) {
                        Some(id) => (ScopeState::Ready, BTreeSet::from([*id])),
                        None => (ScopeState::Missing, BTreeSet::new()),
                    },
                    Scope::Bookmark { name } => {
                        match view.local_bookmarks.get(name).map(|t| t.0.as_slice()) {
                            Some([Some(id)]) => (ScopeState::Ready, BTreeSet::from([*id])),
                            Some(terms) if terms.len() > 1 => (
                                ScopeState::Conflicted,
                                terms.iter().flatten().copied().collect(),
                            ),
                            _ => (ScopeState::Missing, BTreeSet::new()),
                        }
                    }
                }
            } else {
                (ScopeState::AmbiguousHeads, BTreeSet::new())
            };
            out.insert(
                id.clone(),
                ScopeSnapshot {
                    state,
                    operation,
                    commits,
                },
            );
        }
        Ok(out)
    }
    async fn extract(
        &self,
        snapshot: &Snapshot,
        previous: Option<&Snapshot>,
        context: &Context,
    ) -> Result<Update> {
        let mut budget = Budget {
            config: &self.config,
            context,
            work: 0,
            bytes: 0,
        };
        let read = self.read(snapshot).await?;
        let initial = snapshot.at(m::WalPosition(0)).await?;
        let mut prior = if let Some(previous) = previous {
            previous.at(previous.selected_position().position).await?
        } else {
            initial
        };
        let mut after = prior.selected_position().position;
        let mut actors = BTreeMap::new();
        let mut origins = BTreeMap::new();
        while after.0 < snapshot.selected_position().position.0 {
            budget.work(1)?;
            let records = snapshot
                .history(after, snapshot.history_page_size())
                .await?;
            if records.is_empty() {
                return Err(Error::Corrupt);
            }
            for record in records {
                let tx = record.transaction();
                for op in &tx.published_operations {
                    actors.insert(*op, record.principal());
                }
                let at = snapshot.at(tx.after.position).await?;
                budget.work(at.objects.entries().len())?;
                if tx.published_operations.len() == 1 {
                    let op = *tx.published_operations.first().unwrap();
                    for key in at
                        .objects
                        .entries()
                        .keys()
                        .filter(|k| !prior.objects.entries().contains_key(k))
                    {
                        if let m::ObjectKey::Commit(id) = key {
                            origins.insert(*id, op);
                        }
                    }
                }
                after = tx.after.position;
                prior = at;
            }
        }
        let mut update = Update {
            scopes: self.scopes(snapshot, &read).await?,
            ..Default::default()
        };
        for key in snapshot.objects.entries().keys().copied() {
            budget.work(1)?;
            if previous.is_some_and(|p| p.objects.entries().contains_key(&key)) {
                continue;
            }
            let key = match key {
                m::ObjectKey::Operation(_) | m::ObjectKey::Commit(_) => key,
                _ => continue,
            };
            let object = read.object(key).await.map_err(map_query)?;
            let doc = metadata(key, object.data, &actors, &origins)?;
            budget.push(&mut update, doc)?;
        }
        let before = if let Some(previous) = previous {
            Some(self.scopes(previous, &self.read(previous).await?).await?)
        } else {
            None
        };
        for (name, scope) in update.scopes.clone() {
            budget.work(1)?;
            let old = before.as_ref().and_then(|s| s.get(&name));
            if scope.state != ScopeState::Ready {
                update.replace_scopes.insert(name);
                continue;
            }
            let commit = *scope.commits.first().ok_or(Error::Corrupt)?;
            let from = if let Some(old) = old.filter(|s| s.state == ScopeState::Ready) {
                *old.commits.first().ok_or(Error::Corrupt)?
            } else {
                update.replace_scopes.insert(name.clone());
                ROOT_COMMIT
            };
            if from == commit {
                continue;
            }
            let diff = read
                .diff(content::diff::Request {
                    from,
                    to: commit,
                    path: String::new(),
                    content: false,
                })
                .await
                .map_err(map_query)?;
            for change in diff.changes {
                budget.work(1)?;
                let key = DocumentKey::File {
                    scope: name.clone(),
                    path: change.path.clone(),
                };
                let document = self
                    .file(&read, &scope, key.clone(), &change.path, &change.after)
                    .await?;
                if let Some(document) = document {
                    budget.push(&mut update, document)?;
                } else if previous.is_some() {
                    update.delete.insert(key);
                }
            }
        }
        bounded_json(&update, self.config.max_output_bytes)?;
        snapshot.objects.authorize().await?;
        Ok(update)
    }
    async fn file(
        &self,
        read: &ProjectionRead,
        scope: &ScopeSnapshot,
        key: DocumentKey,
        path: &str,
        terms: &[Option<dto::TreeValue>],
    ) -> Result<Option<Document>> {
        if !self.config.includes(path) {
            return Ok(None);
        }
        let [
            Some(dto::TreeValue::File {
                id,
                executable,
                copy,
            }),
        ] = terms
        else {
            return Ok(None);
        };
        if read.file_bytes(*id).await.map_err(map_query)? > self.config.max_text_bytes as u64 {
            return Ok(None);
        }
        let mut stream = read.file(*id).await.map_err(map_query)?;
        let mut bytes = Vec::new();
        while let Some(chunk) = stream.next_chunk().await.map_err(map_query)? {
            if bytes.len() + chunk.len() > self.config.max_text_bytes {
                return Err(Error::Limit);
            }
            bytes.extend(chunk);
        }
        if bytes.contains(&0) {
            return Ok(None);
        }
        let Ok(text) = String::from_utf8(bytes) else {
            return Ok(None);
        };
        let mut doc = blank(key);
        doc.title = path.into();
        doc.body = text;
        doc.file = Some(*id);
        doc.operation = scope.operation;
        doc.commit = scope.commits.first().copied();
        doc.language = path
            .rsplit('/')
            .next()
            .and_then(|name| name.rsplit_once('.'))
            .map(|(_, suffix)| suffix.to_ascii_lowercase());
        doc.attributes
            .insert("executable".into(), executable.to_string());
        doc.attributes.insert(
            "copy".into(),
            serde_json::to_string(copy).map_err(|_| Error::Corrupt)?,
        );
        Ok(Some(doc))
    }
}
#[async_trait::async_trait]
impl Extractor for NativeExtractor {
    fn validate_identity(&self, identity: &Identity) -> Result<()> {
        if *identity != self.config.identity()? {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    async fn bootstrap(&mut self, snapshot: &Snapshot, context: &Context) -> Result<Update> {
        self.extract(snapshot, None, context).await
    }
    async fn update(&mut self, batch: &Batch, context: &Context) -> Result<Update> {
        let previous = batch.snapshot.at(batch.before.position).await?;
        self.extract(&batch.snapshot, Some(&previous), context)
            .await
    }
}
fn map_query(error: query::Error) -> Error {
    match error {
        query::Error::Denied | query::Error::EpochChanged | query::Error::NotCommitted => {
            Error::Denied
        }
        query::Error::Corrupt | query::Error::Incomplete => Error::Corrupt,
        query::Error::Limit | query::Error::IndexBudget => Error::Limit,
        query::Error::Canceled => Error::Canceled,
        _ => Error::Unavailable,
    }
}
fn blank(key: DocumentKey) -> Document {
    Document {
        key,
        title: String::new(),
        body: String::new(),
        actor: None,
        author: String::new(),
        workspace: None,
        operation: None,
        commit: None,
        change: None,
        file: None,
        parents: vec![],
        predecessors: vec![],
        attributes: BTreeMap::new(),
        language: None,
        has_conflict: false,
        timestamp_millis: 0,
    }
}
fn metadata(
    key: m::ObjectKey,
    value: ObjectData,
    actors: &BTreeMap<m::OperationId, m::PrincipalId>,
    origins: &BTreeMap<m::CommitId, m::OperationId>,
) -> Result<Document> {
    Ok(match (key, value) {
        (m::ObjectKey::Operation(id), ObjectData::Operation(op)) => {
            let mut doc = blank(DocumentKey::Operation { id });
            doc.operation = Some(id);
            doc.title = op.metadata.description;
            doc.actor = actors.get(&id).map(ToString::to_string);
            doc.author = op.metadata.username;
            doc.workspace = op.metadata.workspace_name;
            doc.timestamp_millis = op.metadata.end.millis;
            doc.attributes.insert(
                "native.attributes".into(),
                serde_json::to_string(&op.metadata.attributes).map_err(|_| Error::Corrupt)?,
            );
            doc.attributes.insert(
                "native.start".into(),
                serde_json::to_string(&op.metadata.start).map_err(|_| Error::Corrupt)?,
            );
            doc.attributes.insert(
                "native.end".into(),
                serde_json::to_string(&op.metadata.end).map_err(|_| Error::Corrupt)?,
            );
            doc.attributes
                .insert("native.hostname".into(), op.metadata.hostname);
            doc.attributes.insert(
                "native.parents".into(),
                serde_json::to_string(&op.parents).map_err(|_| Error::Corrupt)?,
            );
            doc.attributes.insert(
                "native.predecessor_data_present".into(),
                op.commit_predecessors.is_some().to_string(),
            );
            if let Some(predecessors) = op.commit_predecessors {
                doc.predecessors = predecessors
                    .into_iter()
                    .flat_map(|(id, mut before)| {
                        before.push(id);
                        before
                    })
                    .collect();
                doc.predecessors.sort();
                doc.predecessors.dedup();
            }
            doc
        }
        (m::ObjectKey::Commit(id), ObjectData::Commit(commit)) => {
            let mut doc = blank(DocumentKey::Commit { id });
            doc.commit = Some(id);
            doc.operation = origins.get(&id).copied();
            doc.change = Some(commit.change_id);
            doc.title = commit.description;
            doc.author = format!("{} <{}>", commit.author.name, commit.author.email);
            doc.timestamp_millis = commit.committer.timestamp.millis;
            doc.parents = commit.parents;
            doc.predecessors = commit.predecessors;
            doc.has_conflict = commit.root_tree.len() > 1;
            doc.attributes.insert(
                "native.committer".into(),
                serde_json::to_string(&commit.committer).map_err(|_| Error::Corrupt)?,
            );
            doc.attributes.insert(
                "native.author".into(),
                serde_json::to_string(&commit.author).map_err(|_| Error::Corrupt)?,
            );
            doc.attributes.insert(
                "native.signed".into(),
                commit.secure_sig.is_some().to_string(),
            );
            doc
        }
        _ => return Err(Error::Corrupt),
    })
}
