use crate::{document::Fields, *};
use jj_store_adapters::{Checkpoint, Context, Identity, Position};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, Term};

pub(crate) struct Owner {
    pub root: PathBuf,
    _lock: File,
}
impl Owner {
    pub fn acquire(root: PathBuf) -> Result<Arc<Self>> {
        if !root.is_absolute() {
            return Err(Error::Invalid);
        }
        io(fs::create_dir_all(&root))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            io(fs::set_permissions(
                &root,
                fs::Permissions::from_mode(0o700),
            ))?;
        }
        let mut options = OpenOptions::new();
        options.create(true).truncate(false).read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let lock = io(options.open(root.join("owner.lock")))?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        let owner = Arc::new(Self { root, _lock: lock });
        // Acquiring the exclusive root lock proves no reader in any surviving
        // process owns these crash leftovers. Never remove the selected one.
        if let Ok(Some(selected)) = active(&owner) {
            for (n, entry) in io(fs::read_dir(&owner.root))?.enumerate() {
                if n > 1024 {
                    return Err(Error::Limit);
                }
                let entry = io(entry)?;
                let name = entry.file_name();
                if name
                    .to_str()
                    .is_some_and(|s| generation(s) && s != selected)
                    && io(entry.file_type())?.is_dir()
                {
                    io(fs::remove_dir_all(entry.path()))?;
                }
            }
        }
        Ok(owner)
    }
}
pub(crate) struct Generation {
    pub payload: Payload,
    pub checkpoint: Checkpoint,
    pub reader: IndexReader,
    pub fields: Fields,
    pub path: PathBuf,
    pub retired: AtomicBool,
    _owner: Arc<Owner>,
}
impl Drop for Generation {
    fn drop(&mut self) {
        if self.retired.load(Ordering::SeqCst) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileInfo {
    bytes: u64,
    digest: m::RecordDigest,
}
fn transfer(
    source: &Path,
    target: Option<&Path>,
    limit: u64,
    context: &Context,
) -> Result<FileInfo> {
    let meta = io(fs::symlink_metadata(source))?;
    if !meta.is_file() || meta.len() > limit {
        return Err(Error::Corrupt);
    }
    let mut input = io(File::open(source))?;
    let mut output = if let Some(target) = target {
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        Some(io(options.open(target))?)
    } else {
        None
    };
    let mut buffer = vec![0; 64 * 1024];
    let mut bytes = 0u64;
    let mut hasher = Sha256::new();
    loop {
        context.check()?;
        let count = io(input.read(&mut buffer))?;
        if count == 0 {
            break;
        }
        bytes = bytes.checked_add(count as u64).ok_or(Error::Limit)?;
        if bytes > limit {
            return Err(Error::Limit);
        }
        hasher.update(&buffer[..count]);
        if let Some(output) = &mut output {
            io(output.write_all(&buffer[..count]))?;
        }
    }
    if bytes != meta.len() {
        return Err(Error::Corrupt);
    }
    if let Some(output) = output {
        io(output.sync_all())?;
    }
    Ok(FileInfo {
        bytes,
        digest: m::RecordDigest::new(hasher.finalize().into()),
    })
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Descriptor {
    version: u32,
    payload: Payload,
    files: BTreeMap<String, FileInfo>,
}

fn filename(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && name.len() <= 256
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}
fn generation(name: &str) -> bool {
    name.len() == 32
        && name
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn read(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let meta = io(fs::symlink_metadata(path))?;
    if !meta.is_file() || meta.len() > limit {
        return Err(Error::Corrupt);
    }
    let mut value = Vec::new();
    io(File::open(path))?
        .take(limit + 1)
        .read_to_end(&mut value)
        .map_err(|_| Error::Unavailable)?;
    if value.len() as u64 > limit {
        return Err(Error::Limit);
    }
    Ok(value)
}
fn write(path: &Path, value: &[u8]) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = io(options.open(path))?;
    io(file.write_all(value))?;
    io(file.sync_all())
}
fn sync_dir(path: &Path) -> Result<()> {
    io(io(File::open(path))?.sync_all())
}
fn active(owner: &Owner) -> Result<Option<String>> {
    let bytes = match read(&owner.root.join("active"), 32) {
        Ok(b) => b,
        Err(_) if !owner.root.join("active").exists() => return Ok(None),
        Err(e) => return Err(e),
    };
    let name = String::from_utf8(bytes).map_err(|_| Error::Corrupt)?;
    if !generation(&name) {
        return Err(Error::Corrupt);
    }
    Ok(Some(name))
}
pub(crate) fn recover(
    owner: Arc<Owner>,
    limits: Limits,
    context: &Context,
) -> Result<Option<Arc<Generation>>> {
    context.check()?;
    let name = match active(&owner) {
        Ok(Some(name)) => name,
        Ok(None) => return Ok(None),
        Err(_) => return Ok(None),
    };
    // Derived corruption is recoverable by a source rebuild, never by declaring
    // an advanced remote cursor applied to an empty local index.
    let result = load(owner, &name, limits, context);
    context.check()?;
    match result {
        Ok(generation) => Ok(Some(generation)),
        Err(Error::Limit | Error::Canceled) => result.map(Some),
        Err(_) => Ok(None),
    }
}
fn descriptor(path: &Path, limits: Limits, context: &Context) -> Result<Descriptor> {
    let bytes = read(&path.join("generation.json"), 4 * 1024 * 1024)?;
    let descriptor: Descriptor = serde_json::from_slice(&bytes).map_err(|_| Error::Corrupt)?;
    if descriptor.version != 1
        || descriptor.files.len() > limits.index_files
        || !descriptor.files.contains_key("meta.json")
        || !descriptor.files.contains_key(".managed.json")
        || descriptor.payload.version != 1
        || descriptor.payload.identity.schema != SCHEMA_VERSION
        || descriptor.payload.identity.implementation != IMPLEMENTATION
        || !generation(&descriptor.payload.generation)
        || serde_json::to_vec(&descriptor).map_err(|_| Error::Corrupt)? != bytes
    {
        return Err(Error::Corrupt);
    }
    descriptor.payload.identity.validate()?;
    if descriptor.payload.scopes.len() > 128 {
        return Err(Error::Corrupt);
    }
    for scope in descriptor.payload.scopes.values() {
        scope.validate()?;
    }
    let mut total = 0u64;
    for (name, expected) in &descriptor.files {
        context.check()?;
        total = total.checked_add(expected.bytes).ok_or(Error::Limit)?;
        if total > limits.index_bytes {
            return Err(Error::Limit);
        }
        if !filename(name) {
            return Err(Error::Corrupt);
        }
        if transfer(&path.join(name), None, limits.index_bytes, context)? != *expected {
            return Err(Error::Corrupt);
        }
    }
    Ok(descriptor)
}
fn load(
    owner: Arc<Owner>,
    name: &str,
    limits: Limits,
    context: &Context,
) -> Result<Arc<Generation>> {
    let path = owner.root.join(name);
    let generation = load_path(owner, path, limits, context)?;
    if generation.payload.generation != name {
        return Err(Error::Corrupt);
    }
    Ok(generation)
}
fn load_path(
    owner: Arc<Owner>,
    path: PathBuf,
    limits: Limits,
    context: &Context,
) -> Result<Arc<Generation>> {
    let descriptor = descriptor(&path, limits, context)?;
    let index = tv(Index::open_in_dir(&path))?;
    let fields = Fields::new();
    if index.schema() != fields.schema {
        return Err(Error::Corrupt);
    }
    let payload = serde_json::to_string(&descriptor.payload).map_err(|_| Error::Corrupt)?;
    if tv(index.load_metas())?.payload.as_deref() != Some(payload.as_str()) {
        return Err(Error::Corrupt);
    }
    let reader = tv(index
        .reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into())?;
    let reader: IndexReader = reader;
    if reader.searcher().num_docs() > limits.documents as u64 {
        return Err(Error::Limit);
    }
    context.check()?;
    Ok(Arc::new(Generation {
        checkpoint: descriptor.payload.checkpoint()?,
        payload: descriptor.payload,
        reader,
        fields,
        path,
        retired: AtomicBool::new(false),
        _owner: owner,
    }))
}
pub(crate) struct SnapshotFiles {
    pub generation: Arc<Generation>,
    pub files: Vec<(String, u64)>,
}
pub(crate) fn snapshot_files(
    owner: Arc<Owner>,
    limits: Limits,
    context: &Context,
) -> Result<SnapshotFiles> {
    let generation = recover(owner, limits, context)?.ok_or(Error::Corrupt)?;
    let descriptor = descriptor(&generation.path, limits, context)?;
    let mut files: Vec<_> = descriptor
        .files
        .into_iter()
        .map(|(name, info)| (name, info.bytes))
        .collect();
    let info = transfer(
        &generation.path.join("generation.json"),
        None,
        limits.index_bytes,
        context,
    )?;
    files.push(("generation.json".into(), info.bytes));
    if files.len() > limits.index_files + 1 {
        return Err(Error::Limit);
    }
    Ok(SnapshotFiles { generation, files })
}
pub(crate) fn install_snapshot(
    owner: Arc<Owner>,
    staging: tempfile::TempDir,
    expected: &Checkpoint,
    limits: Limits,
    context: &Context,
) -> Result<Arc<Generation>> {
    context.check()?;
    let candidate = load_path(owner.clone(), staging.path().to_owned(), limits, context)?;
    if &candidate.checkpoint != expected {
        return Err(Error::Corrupt);
    }
    let name = candidate.payload.generation.clone();
    drop(candidate);
    let destination = owner.root.join(&name);
    if destination.exists() {
        let existing = load(owner, &name, limits, context)?;
        if &existing.checkpoint != expected {
            return Err(Error::Corrupt);
        }
        return Ok(existing);
    }
    let staging = staging.keep();
    io(fs::rename(staging, &destination))?;
    sync_dir(&owner.root)?;
    let loaded = load(owner.clone(), &name, limits, context)?;
    if &loaded.checkpoint != expected {
        return Err(Error::Corrupt);
    }
    let mut pointer = io(tempfile::NamedTempFile::new_in(&owner.root))?;
    io(pointer.write_all(name.as_bytes()))?;
    io(pointer.as_file().sync_all())?;
    context.check()?;
    pointer
        .persist(owner.root.join("active"))
        .map_err(|_| Error::Unavailable)?;
    sync_dir(&owner.root)?;
    Ok(loaded)
}
fn copy_generation(source: &Path, target: &Path, limits: Limits, context: &Context) -> Result<()> {
    let descriptor = descriptor(source, limits, context)?;
    for (name, info) in &descriptor.files {
        context.check()?;
        if transfer(
            &source.join(name),
            Some(&target.join(name)),
            info.bytes,
            context,
        )? != *info
        {
            return Err(Error::Corrupt);
        }
    }
    Ok(())
}
pub(crate) struct CommitInput {
    pub previous: Option<Arc<Generation>>,
    pub identity: Identity,
    pub source: Position,
    pub update: Update,
}
pub(crate) fn commit(
    owner: Arc<Owner>,
    limits: Limits,
    context: &Context,
    input: CommitInput,
    faults: &crate::Faults,
) -> Result<Arc<Generation>> {
    let CommitInput {
        previous,
        identity,
        source,
        update,
    } = input;
    let previous = previous.as_ref();
    context.check()?;
    update.validate(limits)?;
    identity.validate()?;
    let mut generations = 0;
    for (n, entry) in io(fs::read_dir(&owner.root))?.enumerate() {
        context.check()?;
        if n > 1024 {
            return Err(Error::Limit);
        }
        let entry = io(entry)?;
        if io(entry.file_type())?.is_dir() {
            generations += 1;
        }
    }
    if generations >= limits.generations {
        return Err(Error::Limit);
    }
    if identity.implementation != IMPLEMENTATION || identity.schema != SCHEMA_VERSION {
        return Err(Error::Invalid);
    }
    if let Some(previous) = previous
        && (previous.payload.identity != identity
            || previous.payload.source.repository != source.repository
            || previous.payload.source.epoch != source.epoch
            || previous.payload.source.position.0 >= source.position.0)
    {
        return Err(Error::Corrupt);
    }
    let staging = io(tempfile::Builder::new()
        .prefix("stage-")
        .tempdir_in(&owner.root))?;
    let fields = Fields::new();
    let index = if let Some(previous) = previous {
        copy_generation(&previous.path, staging.path(), limits, context)?;
        tv(Index::open_in_dir(staging.path()))?
    } else {
        tv(Index::create_in_dir(staging.path(), fields.schema.clone()))?
    };
    let mut writer: IndexWriter = tv(index.writer_with_num_threads(1, limits.writer_bytes))?;
    writer.set_merge_policy(Box::new(tantivy::merge_policy::NoMergePolicy));
    if let Some(previous) = previous {
        for name in previous.payload.scopes.keys().chain(update.scopes.keys()) {
            if previous.payload.scopes.get(name) != update.scopes.get(name)
                && !update.replace_scopes.contains(name)
                && !matches!((previous.payload.scopes.get(name), update.scopes.get(name)), (Some(a), Some(b)) if a.state == ScopeState::Ready && b.state == ScopeState::Ready)
            {
                return Err(Error::Invalid);
            }
        }
    }
    for scope in &update.replace_scopes {
        writer.delete_term(Term::from_field_text(fields.scope, scope));
    }
    for key in &update.delete {
        context.check()?;
        writer.delete_term(Term::from_field_text(fields.key, &key.exact()?));
    }
    for document in &update.upsert {
        context.check()?;
        writer.delete_term(Term::from_field_text(fields.key, &document.key.exact()?));
        tv(writer.add_document(fields.document(document)?))?;
    }
    let name = uuid::Uuid::new_v4().simple().to_string();
    let payload = Payload {
        version: 1,
        identity,
        source,
        previous: previous.map(|g| g.checkpoint.artifact),
        generation: name.clone(),
        update: digest(&bounded_json(&update, limits.update_bytes)?),
        scopes: update.scopes,
    };
    faults.check(crate::Fault::BeforeCommit)?;
    context.check()?;
    let mut prepared = tv(writer.prepare_commit())?;
    prepared.set_payload(&serde_json::to_string(&payload).map_err(|_| Error::Invalid)?);
    tv(prepared.commit())?;
    tv(writer.wait_merging_threads())?;
    faults.check(crate::Fault::AfterCommit)?;
    context.check()?;
    let reader: IndexReader = tv(index.reader())?;
    if reader.searcher().num_docs() > limits.documents as u64 {
        return Err(Error::Limit);
    }
    drop(reader);
    let metas = tv(index.load_metas())?;
    let mut names = std::collections::BTreeSet::from([
        PathBuf::from("meta.json"),
        PathBuf::from(".managed.json"),
    ]);
    for segment in metas.segments {
        names.extend(
            segment
                .list_files()
                .into_iter()
                .filter(|p| staging.path().join(p).exists()),
        );
    }
    if names.len() > limits.index_files {
        return Err(Error::Limit);
    }
    let mut files = BTreeMap::new();
    let mut total = 0u64;
    for path in names {
        context.check()?;
        let info = transfer(
            &staging.path().join(&path),
            None,
            limits.index_bytes,
            context,
        )?;
        total = total.checked_add(info.bytes).ok_or(Error::Limit)?;
        if total > limits.index_bytes {
            return Err(Error::Limit);
        }
        io(io(File::open(staging.path().join(&path)))?.sync_all())?;
        files.insert(path.to_str().ok_or(Error::Corrupt)?.to_owned(), info);
    }
    drop(index);
    write(
        &staging.path().join("generation.json"),
        &serde_json::to_vec(&Descriptor {
            version: 1,
            payload,
            files,
        })
        .map_err(|_| Error::Invalid)?,
    )?;
    sync_dir(staging.path())?;
    context.check()?;
    let destination = owner.root.join(&name);
    io(fs::rename(staging.path(), &destination))?;
    sync_dir(&owner.root)?;
    // Verify the actual durable artifact before it becomes the active pointer.
    let loaded = load(owner.clone(), &name, limits, context)?;
    let mut pointer = io(tempfile::NamedTempFile::new_in(&owner.root))?;
    io(pointer.write_all(name.as_bytes()))?;
    io(pointer.as_file().sync_all())?;
    context.check()?;
    pointer
        .persist(owner.root.join("active"))
        .map_err(|_| Error::Unavailable)?;
    sync_dir(&owner.root)?;
    Ok(loaded)
}
