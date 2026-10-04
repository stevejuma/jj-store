//! Explicit code identity translation for independent local/native destinations.
//! These versioned archives retain source bytes; they are not native persistence.
use crate::{
    codec::{self, NativeCodec, NativeSymlink},
    local_evidence::{self, GitEvidenceCapsuleV1},
    objects::{NativeObjects, UploadStatus},
    roots,
};
use futures::{FutureExt as _, future::BoxFuture};
use gix::objs::Write as _;
use jj_lib::{
    backend::{self as b, Backend as _},
    config::StackedConfig,
    default_backend_factories::default_backend_factories,
    git_backend::GitBackend,
    object_id::ObjectId as _,
    op_store as o,
    repo::{Repo as _, RepoLoader},
    repo_path::RepoPath,
    settings::UserSettings,
    workspace::Workspace,
};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub const MAX_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_OBJECTS: usize = 100_000;
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawGitObject {
    pub oid: m::GitSha1Id,
    pub kind: String,
    pub body: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "profile", rename_all = "snake_case", deny_unknown_fields)]
pub enum CodeArchiveV1 {
    Git {
        capsule: Box<GitEvidenceCapsuleV1>,
        objects: Vec<RawGitObject>,
    },
    Native {
        operation: m::OperationId,
        objects: Vec<(m::ObjectKey, Vec<u8>)>,
    },
}
pub fn settings() -> Result<UserSettings> {
    Ok(UserSettings::from_config(StackedConfig::with_defaults())?)
}
pub async fn archive_local(
    path: &Path,
    repository: m::RepositoryId,
    operation: m::OperationId,
    metadata: Option<crate::local_metadata::GitOid>,
) -> Result<CodeArchiveV1> {
    let settings = settings()?;
    let loader = RepoLoader::init_from_file_system(&settings, path, &default_backend_factories())?;
    // Only explicit committed heads are accepted; no head reconciliation or filesystem snapshot.
    if !loader
        .op_heads_store()
        .get_op_heads()
        .await?
        .iter()
        .any(|v| v.as_bytes() == operation.as_bytes())
    {
        return Err("source operation is no longer a committed head".into());
    }
    let capsule = local_evidence::inspect(&settings, path, repository, operation).await?;
    let backend = loader
        .store()
        .backend()
        .downcast_ref::<GitBackend>()
        .ok_or("unsupported backend")?;
    let git = backend.git_repo();
    let mut pending: BTreeSet<_> = capsule
        .commits
        .keys()
        .filter(|id| id.as_bytes() != &[0; 20])
        .map(|id| id.to_string())
        .collect();
    pending.extend(
        capsule
            .commits
            .values()
            .flat_map(|c| c.root_tree.iter().map(ToString::to_string)),
    );
    if let Some(metadata) = metadata {
        pending.insert(metadata.to_string());
    }
    let mut seen = BTreeSet::new();
    let mut objects = Vec::new();
    let mut bytes = capsule.encode()?.len();
    while let Some(id) = pending.pop_first() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if seen.len() > MAX_OBJECTS {
            return Err("conversion object limit".into());
        }
        let id = gix::ObjectId::from_hex(id.as_bytes())?;
        let size = git.find_header(id)?.size();
        if size > MAX_BYTES as u64
            || bytes
                .checked_add(size as usize)
                .is_none_or(|v| v > MAX_BYTES)
        {
            return Err("conversion byte limit".into());
        }
        let object = git.find_object(id)?;
        if gix::objs::compute_hash(gix::hash::Kind::Sha1, object.kind, &object.data)? != id {
            return Err("Git archive integrity".into());
        }
        match object.kind {
            gix::objs::Kind::Blob => {}
            gix::objs::Kind::Tree => {
                for e in
                    gix::objs::TreeRef::from_bytes(&object.data, gix::hash::Kind::Sha1)?.entries
                {
                    if !e.mode.is_commit() {
                        pending.insert(e.oid.to_string());
                    }
                }
            }
            gix::objs::Kind::Commit => {
                let c = object.try_to_commit_ref()?;
                pending.insert(c.tree().to_string());
                pending.extend(c.parents().map(|v| v.to_string()));
            }
            _ => return Err("unsupported Git archive object".into()),
        }
        bytes += object.data.len();
        objects.push(RawGitObject {
            oid: m::GitSha1Id::from_slice(id.as_bytes())?,
            kind: object.kind.to_string(),
            body: object.data.clone(),
        });
    }
    Ok(CodeArchiveV1::Git {
        capsule: Box::new(capsule),
        objects,
    })
}
/// Archives only a manifest-selected, verified inventory closure supplied by the service.
/// Keys and immutable bytes are reverified; staged objects cannot enter this archive.
pub async fn archive_native(
    store: Arc<dyn jj_store_object::ObjectStore>,
    repository: m::RepositoryId,
    operation: m::OperationId,
    selected: &[jj_store_wal::inventory::VerifiedNode],
) -> Result<CodeArchiveV1> {
    use jj_store_wal::inventory::{DependencySource as _, Representation};
    let admitted: BTreeMap<_, _> = selected.iter().map(|n| (n.key, n)).collect();
    if admitted.len() != selected.len() || admitted.len() > MAX_OBJECTS {
        return Err("conversion inventory limit".into());
    }
    let source = NativeObjects::new(store.clone(), repository, MAX_BYTES as u64)?;
    let mut pending = BTreeSet::from([m::ObjectKey::Operation(operation)]);
    let mut seen = BTreeSet::new();
    let mut objects = Vec::new();
    let mut size = 0usize;
    while let Some(key) = pending.pop_first() {
        if !seen.insert(key) {
            continue;
        }
        let expected = admitted
            .get(&key)
            .ok_or("operation is not selected by source manifest")?;
        let loaded = source.load_verified(key, (MAX_BYTES - size) as u64).await?;
        if &loaded != *expected {
            return Err("source inventory identity mismatch".into());
        }
        pending.extend(loaded.dependencies);
        if let Representation::Stored { bytes, .. } = loaded.representation {
            let raw = store
                .get(
                    &jj_store_object::Key::new(key.storage_path(repository))?,
                    None,
                )
                .await?
                .read_bounded(bytes)
                .await?;
            size = size.checked_add(raw.len()).ok_or("conversion byte limit")?;
            if size > MAX_BYTES {
                return Err("conversion byte limit".into());
            }
            objects.push((key, raw));
        }
    }
    Ok(CodeArchiveV1::Native { operation, objects })
}

pub enum Reader {
    Git {
        capsule: Box<GitEvidenceCapsuleV1>,
        backend: Box<GitBackend>,
    },
    Native {
        operation: m::OperationId,
        objects: BTreeMap<m::ObjectKey, Vec<u8>>,
    },
}
impl Reader {
    pub fn open(archive: &CodeArchiveV1, scratch: &Path) -> Result<Self> {
        match archive {
            CodeArchiveV1::Native { operation, objects } => {
                let map: BTreeMap<_, _> = objects.iter().cloned().collect();
                if map.len() != objects.len() {
                    return Err("duplicate code archive object".into());
                }
                Ok(Self::Native {
                    operation: *operation,
                    objects: map,
                })
            }
            CodeArchiveV1::Git { capsule, objects } => {
                capsule.validate()?;
                std::fs::create_dir_all(scratch)?;
                let bare = scratch.join("git");
                let git = if bare.exists() {
                    gix::open(&bare)?
                } else {
                    gix::init_bare(&bare)?
                };
                for object in objects {
                    let kind = match object.kind.as_str() {
                        "blob" => gix::objs::Kind::Blob,
                        "tree" => gix::objs::Kind::Tree,
                        "commit" => gix::objs::Kind::Commit,
                        _ => return Err("Git archive kind".into()),
                    };
                    let id = git.objects.write_buf(kind, &object.body)?;
                    if id.as_bytes() != object.oid.as_bytes() {
                        return Err("Git archive identity".into());
                    }
                }
                let state = scratch.join("backend");
                std::fs::create_dir_all(&state)?;
                let backend = if state.join("git_target").exists() {
                    GitBackend::load(&settings()?, &state)?
                } else {
                    GitBackend::init_external(&settings()?, &state, &bare)?
                };
                Ok(Self::Git {
                    capsule: capsule.clone(),
                    backend: Box::new(backend),
                })
            }
        }
    }
    fn width(&self) -> usize {
        if matches!(self, Self::Git { .. }) {
            20
        } else {
            64
        }
    }
    fn root(&self) -> b::CommitId {
        b::CommitId::from_bytes(&vec![0; self.width()])
    }
    fn native<T: NativeCodec>(&self, key: m::ObjectKey) -> Result<T> {
        let Self::Native { objects, .. } = self else {
            return Err("native source profile".into());
        };
        Ok(codec::decode(
            key,
            objects.get(&key).ok_or("incomplete code archive")?,
            codec::MAX_METADATA_BYTES,
        )?)
    }
    fn operation(&self) -> Result<o::Operation> {
        match self {
            Self::Git { capsule, .. } => Ok(capsule.operation.clone().try_into()?),
            Self::Native { operation, .. } if *operation == roots::ROOT_OPERATION => {
                Ok(roots::operation())
            }
            Self::Native { operation, .. } => self.native(m::ObjectKey::Operation(*operation)),
        }
    }
    fn view(&self) -> Result<o::View> {
        match self {
            Self::Git { capsule, .. } => Ok(capsule.view.clone().try_into()?),
            Self::Native { .. } => {
                let op = self.operation()?;
                let id = m::ViewId::from_slice(op.view_id.as_bytes())?;
                if id == roots::ROOT_VIEW {
                    Ok(roots::view())
                } else {
                    self.native(m::ObjectKey::View(id))
                }
            }
        }
    }
    fn commit(&self, id: &b::CommitId) -> Result<b::Commit> {
        if *id == self.root() {
            return Ok(match self {
                Self::Git { backend, .. } => b::make_root_commit(
                    backend.root_change_id().clone(),
                    backend.empty_tree_id().clone(),
                ),
                _ => roots::commit(),
            });
        }
        match self {
            Self::Git { capsule, .. } => Ok(capsule
                .commits
                .get(&m::GitSha1Id::from_slice(id.as_bytes())?)
                .ok_or("incomplete source commit")?
                .clone()
                .try_into()?),
            _ => self.native(m::ObjectKey::Commit(m::CommitId::from_slice(
                id.as_bytes(),
            )?)),
        }
    }
    async fn tree(&self, id: &b::TreeId) -> Result<b::Tree> {
        match self {
            Self::Git { backend, .. } => Ok(backend.read_tree(RepoPath::root(), id).await?),
            _ if *id == roots::empty_tree_id() => Ok(b::Tree::default()),
            _ => self.native(m::ObjectKey::Tree(m::TreeId::from_slice(id.as_bytes())?)),
        }
    }
    async fn file(&self, id: &b::FileId) -> Result<Vec<u8>> {
        match self {
            Self::Git { backend, .. } => {
                use futures::AsyncReadExt as _;
                let mut data = Vec::new();
                backend
                    .read_file(RepoPath::root(), id)
                    .await?
                    .take((MAX_BYTES + 1) as u64)
                    .read_to_end(&mut data)
                    .await?;
                if data.len() > MAX_BYTES {
                    return Err("conversion file limit".into());
                }
                Ok(data)
            }
            Self::Native { objects, .. } => {
                let key = m::ObjectKey::File(m::FileId::from_slice(id.as_bytes())?);
                let bytes = objects.get(&key).ok_or("missing native file")?;
                let payload = m::envelope::decode(bytes, key, MAX_BYTES as u64)?;
                use blake2::{Blake2b512, Digest as _};
                if Blake2b512::digest(payload).as_slice() != id.as_bytes() {
                    return Err("native file identity".into());
                }
                Ok(payload.to_vec())
            }
        }
    }
    async fn symlink(&self, id: &b::SymlinkId) -> Result<String> {
        match self {
            Self::Git { backend, .. } => Ok(backend.read_symlink(RepoPath::root(), id).await?),
            _ => Ok(self
                .native::<NativeSymlink>(m::ObjectKey::Symlink(m::SymlinkId::from_slice(
                    id.as_bytes(),
                )?))?
                .0),
        }
    }
}
pub enum Writer {
    Native(Arc<NativeObjects>),
    Git(Arc<dyn b::Backend>),
}
impl Writer {
    fn root(&self) -> b::CommitId {
        match self {
            Self::Native(_) => b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes()),
            Self::Git(b) => b.root_commit_id().clone(),
        }
    }
    async fn commit(&self, commit: &b::Commit) -> Result<b::CommitId> {
        match self {
            Self::Git(backend) => {
                let (id, written) = backend.write_commit(commit.clone(), None).await?;
                if written != *commit {
                    return Err("lossy destination commit".into());
                }
                Ok(id)
            }
            Self::Native(objects) => {
                let (key, status) = objects.put(commit).await?;
                ack(status)?;
                Ok(b::CommitId::from_bytes(key.as_bytes()))
            }
        }
    }
    async fn tree(&self, tree: &b::Tree) -> Result<b::TreeId> {
        match self {
            Self::Git(backend) => Ok(backend.write_tree(RepoPath::root(), tree).await?),
            Self::Native(objects) => {
                let (key, status) = objects.put(tree).await?;
                ack(status)?;
                Ok(b::TreeId::from_bytes(key.as_bytes()))
            }
        }
    }
    async fn file(&self, bytes: &[u8]) -> Result<b::FileId> {
        match self {
            Self::Git(backend) => Ok(backend
                .write_file(
                    RepoPath::root(),
                    &mut TokioAsyncReadCompatExt::compat(bytes),
                )
                .await?),
            Self::Native(objects) => {
                let (id, status) = objects.put_file(bytes).await?;
                ack(status)?;
                Ok(b::FileId::from_bytes(id.as_bytes()))
            }
        }
    }
    async fn symlink(&self, value: &str) -> Result<b::SymlinkId> {
        match self {
            Self::Git(backend) => Ok(backend.write_symlink(RepoPath::root(), value).await?),
            Self::Native(objects) => {
                let (key, status) = objects.put(&NativeSymlink(value.into())).await?;
                ack(status)?;
                Ok(b::SymlinkId::from_bytes(key.as_bytes()))
            }
        }
    }
}
fn ack(status: UploadStatus) -> Result<()> {
    if status == UploadStatus::Unknown {
        Err("conversion storage outcome unknown".into())
    } else {
        Ok(())
    }
}
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityMappingsV1 {
    pub operations: BTreeMap<String, String>,
    pub views: BTreeMap<String, String>,
    pub commits: BTreeMap<String, String>,
    pub trees: BTreeMap<String, String>,
    pub files: BTreeMap<String, String>,
    pub symlinks: BTreeMap<String, String>,
}
pub struct Transfer {
    reader: Reader,
    writer: Writer,
    pub mappings: IdentityMappingsV1,
    active: BTreeSet<String>,
    bytes: usize,
    predecessors: Option<BTreeMap<b::CommitId, Vec<b::CommitId>>>,
}
impl Transfer {
    pub fn new(reader: Reader, writer: Writer) -> Self {
        Self {
            reader,
            writer,
            mappings: Default::default(),
            active: Default::default(),
            bytes: 0,
            predecessors: None,
        }
    }
    fn budget(&mut self, bytes: usize) -> Result<()> {
        self.bytes = self
            .bytes
            .checked_add(bytes)
            .ok_or("conversion byte limit")?;
        if self.bytes > MAX_BYTES
            || self.mappings.commits.len() + self.mappings.trees.len() + self.mappings.files.len()
                > MAX_OBJECTS
        {
            return Err("conversion limit".into());
        }
        Ok(())
    }
    fn commit(&mut self, id: b::CommitId, depth: usize) -> BoxFuture<'_, Result<b::CommitId>> {
        async move {
            if id == self.reader.root() {
                let root = self.writer.root();
                self.mappings.commits.insert(id.hex(), root.hex());
                return Ok(root);
            }
            if depth > 512 {
                return Err("conversion depth limit".into());
            }
            if let Some(mapped) = self.mappings.commits.get(&id.hex()) {
                return Ok(
                    b::CommitId::try_from_hex(mapped.as_bytes()).ok_or("conversion identity")?
                );
            }
            let marker = format!("commit:{}", id.hex());
            if !self.active.insert(marker.clone()) {
                return Err("cyclic source commit".into());
            }
            let mut value = self.reader.commit(&id)?;
            // Signing/copy-conflict profiles require separate roundtrip qualification.
            if value.secure_sig.is_some()
                || !value.root_tree.is_resolved()
                || !value.conflict_labels.is_resolved()
            {
                return Err("signed or conflicted code is unsupported by conversion".into());
            }
            let mut parents = Vec::new();
            for parent in &value.parents {
                parents.push(self.commit(parent.clone(), depth + 1).await?)
            }
            let mut predecessors = Vec::new();
            for prior in &value.predecessors {
                predecessors.push(self.commit(prior.clone(), depth + 1).await?)
            }
            let tree = self
                .tree(value.root_tree.as_resolved().unwrap().clone(), depth + 1)
                .await?;
            value.parents = parents;
            value.predecessors = predecessors;
            value.root_tree = jj_lib::merge::Merge::resolved(tree);
            self.budget(value.description.len() + 1024)?;
            let mapped = self.writer.commit(&value).await?;
            self.mappings.commits.insert(id.hex(), mapped.hex());
            self.active.remove(&marker);
            Ok(mapped)
        }
        .boxed()
    }
    fn tree(&mut self, id: b::TreeId, depth: usize) -> BoxFuture<'_, Result<b::TreeId>> {
        async move {
            if depth > 512 {
                return Err("conversion tree depth limit".into());
            }
            if let Some(mapped) = self.mappings.trees.get(&id.hex()) {
                return Ok(
                    b::TreeId::try_from_hex(mapped.as_bytes()).ok_or("conversion tree identity")?
                );
            }
            let marker = format!("tree:{}", id.hex());
            if !self.active.insert(marker.clone()) {
                return Err("cyclic source tree".into());
            }
            let source = self.reader.tree(&id).await?;
            let mut entries = Vec::new();
            for entry in source.entries() {
                let value = match entry.value() {
                    b::TreeValue::File {
                        id,
                        executable,
                        copy_id,
                    } => {
                        if !copy_id.as_bytes().is_empty() {
                            return Err("non-placeholder copy identity unsupported".into());
                        }
                        let mapped = if let Some(mapped) = self.mappings.files.get(&id.hex()) {
                            b::FileId::try_from_hex(mapped.as_bytes()).ok_or("file identity")?
                        } else {
                            let bytes = self.reader.file(id).await?;
                            self.budget(bytes.len())?;
                            let mapped = self.writer.file(&bytes).await?;
                            self.mappings.files.insert(id.hex(), mapped.hex());
                            mapped
                        };
                        b::TreeValue::File {
                            id: mapped,
                            executable: *executable,
                            copy_id: copy_id.clone(),
                        }
                    }
                    b::TreeValue::Symlink(id) => {
                        let value = self.reader.symlink(id).await?;
                        self.budget(value.len())?;
                        let mapped = self.writer.symlink(&value).await?;
                        self.mappings.symlinks.insert(id.hex(), mapped.hex());
                        b::TreeValue::Symlink(mapped)
                    }
                    b::TreeValue::Tree(id) => {
                        b::TreeValue::Tree(self.tree(id.clone(), depth + 1).await?)
                    }
                    b::TreeValue::GitSubmodule(id) if id.as_bytes().len() == 20 => {
                        b::TreeValue::GitSubmodule(id.clone())
                    }
                    _ => return Err("submodule profile unsupported".into()),
                };
                entries.push((entry.name().to_owned(), value));
            }
            let mapped = self
                .writer
                .tree(&b::Tree::from_sorted_entries(entries))
                .await?;
            self.mappings.trees.insert(id.hex(), mapped.hex());
            self.active.remove(&marker);
            Ok(mapped)
        }
        .boxed()
    }
    async fn target(&mut self, target: &o::RefTarget) -> Result<o::RefTarget> {
        let mut terms = Vec::new();
        for term in target.as_merge().iter() {
            terms.push(match term {
                Some(id) => Some(self.commit(id.clone(), 0).await?),
                None => None,
            })
        }
        Ok(o::RefTarget::from_merge(jj_lib::merge::Merge::from_vec(
            terms,
        )))
    }
    pub async fn run(&mut self) -> Result<o::View> {
        let operation = self.reader.operation()?;
        let mut view = self.reader.view()?;
        let mut heads = std::collections::HashSet::new();
        for id in &view.head_ids {
            heads.insert(self.commit(id.clone(), 0).await?);
        }
        view.head_ids = heads;
        for value in view.wc_commit_ids.values_mut() {
            *value = self.commit(value.clone(), 0).await?;
        }
        for target in view
            .local_bookmarks
            .values_mut()
            .chain(view.local_tags.values_mut())
            .chain(view.git_refs.values_mut())
            .chain(view.git_heads.values_mut())
        {
            *target = self.target(target).await?;
        }
        for remote in view.remote_views.values_mut() {
            for value in remote
                .bookmarks
                .values_mut()
                .chain(remote.tags.values_mut())
            {
                value.target = self.target(&value.target).await?;
            }
        }
        if let Some(predecessors) = operation.commit_predecessors {
            let mut mapped = BTreeMap::new();
            for (commit, prior) in predecessors {
                let commit = self.commit(commit, 0).await?;
                let mut values = Vec::new();
                for id in prior {
                    values.push(self.commit(id, 0).await?);
                }
                mapped.insert(commit, values);
            }
            self.predecessors = Some(mapped);
        }
        Ok(view)
    }
}
fn source_ids(archive: &CodeArchiveV1) -> Result<(String, String)> {
    match archive {
        CodeArchiveV1::Git { capsule, .. } => Ok((
            capsule.operation_id.to_string(),
            capsule.view_id.to_string(),
        )),
        CodeArchiveV1::Native { operation, .. } => {
            let reader = Reader::open(archive, Path::new("."))?;
            Ok((operation.to_string(), reader.operation()?.view_id.hex()))
        }
    }
}
fn fresh_operation(view_id: o::ViewId) -> o::Operation {
    let mut op = roots::operation();
    op.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    op.view_id = view_id;
    op.metadata.description = "Convert an independent repository".into();
    op
}
pub async fn to_native(
    archive: &CodeArchiveV1,
    scratch: &Path,
    objects: Arc<NativeObjects>,
) -> Result<(m::OperationId, IdentityMappingsV1)> {
    let (old_operation, old_view) = source_ids(archive)?;
    let reader = Reader::open(archive, scratch)?;
    let mut transfer = Transfer::new(reader, Writer::Native(objects.clone()));
    let view = transfer.run().await?;
    let (view_key, status) = objects.put(&view).await?;
    ack(status)?;
    let mut op = fresh_operation(o::ViewId::from_bytes(view_key.as_bytes()));
    op.commit_predecessors = transfer.predecessors;
    let (key, status) = objects.put(&op).await?;
    ack(status)?;
    transfer.mappings.operations.insert(
        old_operation,
        key.as_bytes().iter().map(|b| format!("{b:02x}")).collect(),
    );
    transfer.mappings.views.insert(
        old_view,
        view_key
            .as_bytes()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect(),
    );
    Ok((
        m::OperationId::from_slice(key.as_bytes())?,
        transfer.mappings,
    ))
}
pub async fn to_local(
    archive: &CodeArchiveV1,
    scratch: &Path,
    destination: &Path,
) -> Result<(m::OperationId, IdentityMappingsV1)> {
    match std::fs::symlink_metadata(destination) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        _ => return Err("occupied destination".into()),
    }
    let (old_operation, old_view) = source_ids(archive)?;
    std::fs::create_dir(destination)?;
    let settings = settings()?;
    let (mut workspace, initial) =
        Workspace::init_colocated_git(&settings, destination, gix::hash::Kind::Sha1).await?;
    let backend = Arc::new(GitBackend::load(
        &settings,
        &destination.join(".jj/repo/store"),
    )?);
    let reader = Reader::open(archive, scratch)?;
    let mut transfer = Transfer::new(reader, Writer::Git(backend));
    let mut view = transfer.run().await?;
    let workspace_name = workspace.workspace_name().to_owned();
    if !view.wc_commit_ids.contains_key(&workspace_name) {
        let head = match view
            .head_ids
            .iter()
            .find(|v| **v != initial.store().root_commit_id().clone())
        {
            Some(head) => head.clone(),
            None => {
                let id = initial
                    .view()
                    .get_wc_commit_id(&workspace_name)
                    .ok_or("destination working-copy identity")?
                    .clone();
                view.head_ids.insert(id.clone());
                id
            }
        };
        view.wc_commit_ids.insert(workspace_name.clone(), head);
    }
    let store = initial.op_store();
    let view_id = store.write_view(&view).await?;
    let mut op = fresh_operation(view_id.clone());
    op.commit_predecessors = transfer.predecessors;
    let id = store.write_operation(&op).await?;
    let selected = workspace.repo_loader().load_operation(&id).await?;
    let repo = workspace.repo_loader().load_at(&selected).await?;
    let commit = repo
        .store()
        .get_commit(view.wc_commit_ids.get(&workspace_name).unwrap())?;
    workspace.check_out(id.clone(), None, &commit).await?;
    initial
        .op_heads_store()
        .update_op_heads(&[initial.op_id().clone()], &id)
        .await?;
    transfer.mappings.operations.insert(old_operation, id.hex());
    transfer.mappings.views.insert(old_view, view_id.hex());
    Ok((
        m::OperationId::from_slice(id.as_bytes())?,
        transfer.mappings,
    ))
}
