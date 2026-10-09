//! Shared bounded repository readers. No command-output parsing or snapshot capability.
use super::*;
use jj_store_jj::{local_browser as local, query::content as native};
use jj_store_model as m;
use jj_store_service::reads::{CommittedRead, OpenRead, ReadLimits};

#[derive(Clone)]
pub(in crate::agent_execution) struct Entry {
    pub path: String,
    pub file: Option<String>,
    pub kind: String,
}
enum Reader {
    Local {
        executor: local::Executor,
        access: Arc<Access>,
    },
    Native {
        reader: native::ContentReader,
        pin: Arc<CommittedRead>,
    },
}
pub(in crate::agent_execution) struct Source {
    reader: Reader,
    operation: OperationId,
    files: tokio::sync::Mutex<BTreeMap<String, Arc<Vec<Entry>>>>,
}
struct Access {
    supervisor: Supervisor,
    repository: RepositoryId,
    metadata: PathBuf,
    generation: u64,
}
#[async_trait::async_trait]
impl local::Access for Access {
    fn repository(&self) -> RepositoryId {
        self.repository
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &Path {
        &self.metadata
    }
    async fn authorize(&self) -> local::Result<()> {
        self.supervisor
            .authorized(self.repository, "local_jj_git", false)
            .await
            .map_err(|_| local::Error::Denied)?;
        let (folder, generation) = self
            .supervisor
            .0
            .resources
            .folders
            .as_ref()
            .ok_or(local::Error::Unavailable)?
            .folder(self.repository)
            .map_err(|_| local::Error::Denied)?;
        if Path::new(&folder.metadata) != self.metadata || generation != self.generation {
            return Err(local::Error::Denied);
        }
        Ok(())
    }
}
fn error(_: impl std::fmt::Debug) -> Error {
    Error::Unavailable
}
impl Supervisor {
    pub(in crate::agent_execution) async fn workspace_source(
        &self,
        repository: RepositoryId,
        profile: &str,
        operation: OperationId,
    ) -> Result<Source> {
        let reader = if profile == "local_jj_git" {
            let (folder, generation) = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .folder(repository)
                .map_err(error)?;
            Reader::Local {
                executor: local::Executor::new(4).map_err(error)?,
                access: Arc::new(Access {
                    supervisor: self.clone(),
                    repository,
                    metadata: folder.metadata.into(),
                    generation,
                }),
            }
        } else {
            let session = self
                .0
                .resources
                .host
                .as_ref()
                .ok_or(Error::Unavailable)?
                .endpoint(repository)
                .map_err(error)?
                .bind_local_trusted(
                    self.0.resources.actor,
                    jj_store_service::auth::Permission::Admin,
                )
                .map_err(error)?
                .into_session();
            let OpenRead::Pinned(pin) = session
                .open_read(Some(operation), ReadLimits::default())
                .await
                .map_err(error)?
            else {
                return Err(Error::Unavailable);
            };
            let limits = native::Limits {
                max_blob_bytes: 256 * 1024,
                max_text_bytes: 256 * 1024,
                ..Default::default()
            };
            Reader::Native {
                reader: native::ContentReader::new(limits, 4).map_err(error)?,
                pin: Arc::new(pin),
            }
        };
        Ok(Source {
            reader,
            operation,
            files: Default::default(),
        })
    }
}
impl Source {
    pub async fn parents(&self, commit: &str) -> Result<Vec<String>> {
        match &self.reader {
            Reader::Local { executor, access } => {
                let local::Response::CommitSummary(object) = executor
                    .read(
                        access.clone(),
                        self.operation,
                        local::Request::CommitSummary {
                            commit: commit.parse().map_err(error)?,
                        },
                    )
                    .await
                    .map_err(error)?
                else {
                    return Err(Error::Unavailable);
                };
                let local::ObjectData::Commit(value) = object.data else {
                    return Err(Error::Unavailable);
                };
                Ok(value.parents.iter().map(ToString::to_string).collect())
            }
            Reader::Native { reader, pin } => {
                let value = pin
                    .object(reader, m::ObjectKey::Commit(commit.parse().map_err(error)?))
                    .await
                    .map_err(error)?;
                let native::ObjectData::Commit(value) = value.data else {
                    return Err(Error::Unavailable);
                };
                Ok(value.parents.iter().map(ToString::to_string).collect())
            }
        }
    }
    pub async fn revision(
        &self,
        commit: &str,
        target: Option<&str>,
    ) -> Result<dto::WorkspaceRevision> {
        let mut references = vec![];
        let mut defaults = BTreeSet::new();
        // Native and local IDs keep their original widths. Only display strings are shared.
        macro_rules! refs {
            ($view:expr) => {{
                let view = $view;
                for (name, value) in view.local_bookmarks {
                    if let [Some(id)] = value.0.as_slice() {
                        references.push(dto::WorkspaceReference {
                            name,
                            commit: id.to_string(),
                        });
                    }
                }
                for (remote, view) in view.remote_views {
                    for (name, value) in view.bookmarks {
                        if let [Some(id)] = value.target.0.as_slice() {
                            references.push(dto::WorkspaceReference {
                                name: format!("{name}@{remote}"),
                                commit: id.to_string(),
                            });
                        }
                    }
                }
                for (name, value) in view.git_refs {
                    if name.ends_with("/HEAD")
                        && let [Some(id)] = value.0.as_slice()
                    {
                        defaults.insert(id.to_string());
                    }
                }
            }};
        }
        match &self.reader {
            Reader::Local { executor, access } => {
                let local::Response::Operation(value) = executor
                    .read(
                        access.clone(),
                        self.operation,
                        local::Request::Operation {
                            operation: self.operation,
                        },
                    )
                    .await
                    .map_err(error)?
                else {
                    return Err(Error::Unavailable);
                };
                let local::ObjectData::View(view) = value.view.data else {
                    return Err(Error::Unavailable);
                };
                refs!(view);
            }
            Reader::Native { reader, pin } => {
                let native::ObjectData::Operation(op) = pin
                    .object(reader, m::ObjectKey::Operation(self.operation))
                    .await
                    .map_err(error)?
                    .data
                else {
                    return Err(Error::Unavailable);
                };
                let native::ObjectData::View(view) = pin
                    .object(reader, m::ObjectKey::View(op.view_id))
                    .await
                    .map_err(error)?
                    .data
                else {
                    return Err(Error::Unavailable);
                };
                refs!(view);
            }
        }
        if references.len() > 4096 {
            return Err(Error::Limit);
        }
        references.sort_by(|a, b| a.name.cmp(&b.name));
        let default_base = if let Some(target) = target {
            references
                .iter()
                .find(|r| r.name == target)
                .map(|r| r.commit.clone())
        } else if defaults.len() == 1 {
            defaults.into_iter().next()
        } else {
            None
        };
        Ok(dto::WorkspaceRevision {
            commit: commit.into(),
            parents: self.parents(commit).await?,
            references,
            default_base,
        })
    }
    pub async fn files(&self, commit: &str) -> Result<Arc<Vec<Entry>>> {
        if let Some(entries) = self.files.lock().await.get(commit) {
            return Ok(entries.clone());
        }
        let mut entries = vec![];
        let mut anchor = None;
        loop {
            macro_rules! rows {
                ($page:expr, $value:path) => {{
                    let page = $page;
                    for entry in page.entries {
                        if !jj_store_jj::agent_workspaces::inspection::safe_path(&entry.path) {
                            continue;
                        }
                        let (file, kind) = if entry.conflicted {
                            (None, "conflict")
                        } else if let [Some($value { id, .. })] = entry.terms.as_slice() {
                            (Some(id.to_string()), "file")
                        } else {
                            (None, "unsupported")
                        };
                        entries.push(Entry {
                            path: entry.path,
                            file,
                            kind: kind.into(),
                        });
                    }
                    anchor = page.next;
                }};
            }
            match &self.reader {
                Reader::Local { executor, access } => {
                    let request = local::paths::Request {
                        commit: commit.parse().map_err(error)?,
                        path: String::new(),
                        recursive: true,
                        include_last_change: false,
                        last_change_mode: Default::default(),
                        limit: 500,
                        anchor,
                        direction: local::Direction::Forward,
                    };
                    let local::Response::Paths(page) = executor
                        .read(
                            access.clone(),
                            self.operation,
                            local::Request::Paths(request),
                        )
                        .await
                        .map_err(error)?
                    else {
                        return Err(Error::Unavailable);
                    };
                    rows!(page, local::dto::TreeValue::File);
                }
                Reader::Native { reader, pin } => {
                    let request = native::paths::Request {
                        commit: commit.parse().map_err(error)?,
                        path: String::new(),
                        recursive: true,
                        include_last_change: false,
                        last_change_mode: Default::default(),
                        limit: 500,
                        anchor,
                        direction: native::Direction::Forward,
                    };
                    rows!(
                        pin.paths(reader, request).await.map_err(error)?,
                        jj_store_jj::native_dto::TreeValue::File
                    );
                }
            }
            if entries.len() > 4096 {
                return Err(Error::Limit);
            }
            if anchor.is_none() {
                break;
            }
        }
        let entries = Arc::new(entries);
        self.files
            .lock()
            .await
            .insert(commit.into(), entries.clone());
        Ok(entries)
    }
    pub async fn file(&self, commit: &str, path: &str) -> Result<String> {
        let entries = self.files(commit).await?;
        let file = entries
            .iter()
            .find(|e| e.path == path)
            .and_then(|e| e.file.as_ref())
            .ok_or(Error::Unavailable)?;
        let text = match &self.reader {
            Reader::Local { executor, access } => {
                let request = local::Request::FileAt {
                    commit: commit.parse().map_err(error)?,
                    path: path.into(),
                    file: file.parse().map_err(error)?,
                };
                let local::Response::FileAt(content) = executor
                    .read(access.clone(), self.operation, request)
                    .await
                    .map_err(error)?
                else {
                    return Err(Error::Unavailable);
                };
                content.text.ok_or(Error::Unavailable)?
            }
            Reader::Native { reader, pin } => {
                let mut content = pin
                    .file(reader, file.parse().map_err(error)?, None)
                    .await
                    .map_err(error)?;
                if content.info().await.map_err(error)?.full_length > 256 * 1024 {
                    return Err(Error::Limit);
                }
                let mut bytes = vec![];
                while let Some(chunk) = content.next_chunk().await.map_err(error)? {
                    bytes.extend(chunk);
                }
                String::from_utf8(bytes).map_err(error)?
            }
        };
        if text.len() > 256 * 1024 || text.contains('\0') {
            return Err(Error::Limit);
        }
        Ok(text)
    }
    pub async fn common_ancestor(&self, current: &str, target: &str) -> Result<String> {
        let mut graph = BTreeMap::new();
        let mut sides = Vec::new();
        for head in [current, target] {
            let mut seen = BTreeSet::new();
            let mut pending = vec![head.to_owned()];
            while let Some(id) = pending.pop() {
                if !seen.insert(id.clone()) {
                    continue;
                }
                if graph.len() >= 4096 {
                    return Err(Error::Limit);
                }
                if !graph.contains_key(&id) {
                    graph.insert(id.clone(), self.parents(&id).await?);
                }
                pending.extend(graph[&id].iter().cloned());
            }
            sides.push(seen);
        }
        let common: BTreeSet<_> = sides[0].intersection(&sides[1]).cloned().collect();
        let ancestors: BTreeSet<_> = common
            .iter()
            .flat_map(|id| graph[id].iter().cloned())
            .collect();
        let heads: Vec<_> = common.difference(&ancestors).collect();
        if heads.len() != 1 {
            return Err(Error::Unavailable);
        }
        Ok(heads[0].clone())
    }
}
