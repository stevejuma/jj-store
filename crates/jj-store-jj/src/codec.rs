//! Complete, explicitly owned V1 schemas. Native Serde implementations are not
//! persistence contracts. Native hashes are computed over decoded JJ values.
use std::collections::{BTreeMap, HashSet};

use blake2::{Blake2b512, Digest as _};
use jj_lib::content_hash::blake2b_hash;
use jj_lib::merge::Merge;
use jj_lib::object_id::ObjectId as _;
use jj_lib::repo_path::{RepoPathBuf, RepoPathComponentBuf};
use jj_lib::{backend as b, op_store as o};
use jj_store_model::{self as m, ObjectKey, ObjectKind, envelope};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Independent metadata budget; blobs use the streaming object transport.
pub const MAX_METADATA_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error(transparent)]
    Envelope(#[from] envelope::EnvelopeError),
    #[error(transparent)]
    Id(#[from] m::InvalidId),
    #[error("invalid V1 native payload: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid native structure: {0}")]
    Structure(&'static str),
    #[error("noncanonical native encoding")]
    Noncanonical,
    #[error("native content does not match requested identity")]
    Identity,
}
type Result<T> = std::result::Result<T, CodecError>;

mod sealed {
    pub trait Sealed {}
}
/// Implemented only by the audited native object types and raw content wrappers.
pub trait NativeCodec: sealed::Sealed + Sized + PartialEq {
    const KIND: ObjectKind;
    fn key(&self) -> ObjectKey;
    fn encode_payload(&self) -> Result<Vec<u8>>;
    fn decode_payload(bytes: &[u8]) -> Result<Self>;
}

pub fn encode<T: NativeCodec>(value: &T, limit: u64) -> Result<(ObjectKey, Vec<u8>)> {
    let payload = value.encode_payload()?;
    let key = value.key();
    Ok((key, envelope::encode(key, &payload, limit)?))
}
pub fn decode<T: NativeCodec>(expected: ObjectKey, bytes: &[u8], limit: u64) -> Result<T> {
    if expected.kind() != T::KIND {
        return Err(CodecError::Identity);
    }
    let payload = envelope::decode(bytes, expected, limit)?;
    let value = T::decode_payload(payload)?;
    if value.key() != expected {
        return Err(CodecError::Identity);
    }
    Ok(value)
}
fn canonical<W: Serialize + DeserializeOwned>(bytes: &[u8]) -> Result<W> {
    if bytes.len() as u64 > MAX_METADATA_BYTES {
        return Err(envelope::EnvelopeError::Limit.into());
    }
    let wire: W = serde_json::from_slice(bytes)?;
    if serde_json::to_vec(&wire)? != bytes {
        return Err(CodecError::Noncanonical);
    }
    Ok(wire)
}
macro_rules! metadata_codec {
    ($native:ty, $wire:ty, $kind:ident, $id:ident) => {
        impl sealed::Sealed for $native {}
        impl NativeCodec for $native {
            const KIND: ObjectKind = ObjectKind::$kind;
            fn key(&self) -> ObjectKey {
                ObjectKey::$kind(m::$id::new(blake2b_hash(self).into()))
            }
            fn encode_payload(&self) -> Result<Vec<u8>> {
                let wire = <$wire>::try_from(self)?;
                let restored = <$native>::try_from(wire.clone())?;
                if restored != *self {
                    return Err(CodecError::Structure("lossy conversion"));
                }
                let bytes = serde_json::to_vec(&wire)?;
                if bytes.len() as u64 > MAX_METADATA_BYTES {
                    return Err(envelope::EnvelopeError::Limit.into());
                }
                Ok(bytes)
            }
            fn decode_payload(bytes: &[u8]) -> Result<Self> {
                Self::try_from(canonical::<$wire>(bytes)?)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeFile(pub Vec<u8>);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSymlink(pub String);
impl sealed::Sealed for NativeFile {}
impl sealed::Sealed for NativeSymlink {}
impl NativeCodec for NativeFile {
    const KIND: ObjectKind = ObjectKind::File;
    fn key(&self) -> ObjectKey {
        ObjectKey::File(m::FileId::new(Blake2b512::digest(&self.0).into()))
    }
    fn encode_payload(&self) -> Result<Vec<u8>> {
        Ok(self.0.clone())
    }
    fn decode_payload(bytes: &[u8]) -> Result<Self> {
        Ok(Self(bytes.to_vec()))
    }
}
impl NativeCodec for NativeSymlink {
    const KIND: ObjectKind = ObjectKind::Symlink;
    fn key(&self) -> ObjectKey {
        ObjectKey::Symlink(m::SymlinkId::new(
            Blake2b512::digest(self.0.as_bytes()).into(),
        ))
    }
    fn encode_payload(&self) -> Result<Vec<u8>> {
        Ok(self.0.as_bytes().to_vec())
    }
    fn decode_payload(bytes: &[u8]) -> Result<Self> {
        Ok(Self(
            std::str::from_utf8(bytes)
                .map_err(|_| CodecError::Structure("symlink UTF-8"))?
                .to_owned(),
        ))
    }
}

// Conversions make width enforcement visible at the JJ boundary.
macro_rules! id_conversion {
    ($to:ident, $from:ident, $native:ty, $owned:ty) => {
        fn $to(id: &$native) -> Result<$owned> {
            Ok(<$owned>::from_slice(id.as_bytes())?)
        }
        fn $from(id: $owned) -> $native {
            <$native>::from_bytes(id.as_bytes())
        }
    };
}
id_conversion!(commit_id, native_commit_id, b::CommitId, m::CommitId);
id_conversion!(tree_id, native_tree_id, b::TreeId, m::TreeId);
id_conversion!(file_id, native_file_id, b::FileId, m::FileId);
id_conversion!(symlink_id, native_symlink_id, b::SymlinkId, m::SymlinkId);
id_conversion!(copy_id, native_copy_id, b::CopyId, m::CopyId);
id_conversion!(view_id, native_view_id, o::ViewId, m::ViewId);
id_conversion!(
    operation_id,
    native_operation_id,
    o::OperationId,
    m::OperationId
);
id_conversion!(change_id, native_change_id, b::ChangeId, m::ChangeId);
fn copies(id: &b::CopyId) -> Result<m::CopyReference> {
    if id.as_bytes().is_empty() {
        Ok(m::CopyReference::Placeholder)
    } else {
        Ok(m::CopyReference::Object(copy_id(id)?))
    }
}
fn native_copies(id: m::CopyReference) -> b::CopyId {
    match id {
        m::CopyReference::Placeholder => b::CopyId::placeholder(),
        m::CopyReference::Object(id) => native_copy_id(id),
    }
}
fn merge<T>(terms: Vec<T>) -> Result<Merge<T>> {
    if terms.is_empty() || terms.len().is_multiple_of(2) {
        return Err(CodecError::Structure("merge requires odd nonzero arity"));
    }
    Ok(Merge::from_vec(terms))
}
fn sorted_unique<T: Ord>(items: &[T]) -> Result<()> {
    if items.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(CodecError::Structure("unsorted or duplicate keys"));
    }
    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Timestamp {
    millis: i64,
    offset_minutes: i32,
}
impl From<b::Timestamp> for Timestamp {
    fn from(t: b::Timestamp) -> Self {
        Self {
            millis: t.timestamp.0,
            offset_minutes: t.tz_offset,
        }
    }
}
impl From<Timestamp> for b::Timestamp {
    fn from(t: Timestamp) -> Self {
        Self {
            timestamp: b::MillisSinceEpoch(t.millis),
            tz_offset: t.offset_minutes,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Signature {
    name: String,
    email: String,
    timestamp: Timestamp,
}
impl From<&b::Signature> for Signature {
    fn from(s: &b::Signature) -> Self {
        Self {
            name: s.name.clone(),
            email: s.email.clone(),
            timestamp: s.timestamp.into(),
        }
    }
}
impl From<Signature> for b::Signature {
    fn from(s: Signature) -> Self {
        Self {
            name: s.name,
            email: s.email,
            timestamp: s.timestamp.into(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SecureSig {
    data: Vec<u8>,
    sig: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Commit {
    parents: Vec<m::CommitId>,
    predecessors: Vec<m::CommitId>,
    root_tree: Vec<m::TreeId>,
    conflict_labels: Vec<String>,
    change_id: m::ChangeId,
    description: String,
    author: Signature,
    committer: Signature,
    secure_sig: Option<SecureSig>,
}
impl TryFrom<&b::Commit> for Commit {
    type Error = CodecError;
    fn try_from(c: &b::Commit) -> Result<Self> {
        Ok(Self {
            parents: c.parents.iter().map(commit_id).collect::<Result<_>>()?,
            predecessors: c
                .predecessors
                .iter()
                .map(commit_id)
                .collect::<Result<_>>()?,
            root_tree: c.root_tree.iter().map(tree_id).collect::<Result<_>>()?,
            conflict_labels: c.conflict_labels.iter().cloned().collect(),
            change_id: change_id(&c.change_id)?,
            description: c.description.clone(),
            author: (&c.author).into(),
            committer: (&c.committer).into(),
            secure_sig: c.secure_sig.as_ref().map(|s| SecureSig {
                data: s.data.clone(),
                sig: s.sig.clone(),
            }),
        })
    }
}
impl TryFrom<Commit> for b::Commit {
    type Error = CodecError;
    fn try_from(c: Commit) -> Result<Self> {
        if c.root_tree.len() != c.conflict_labels.len()
            || (c.root_tree.len() == 1 && !c.conflict_labels[0].is_empty())
        {
            return Err(CodecError::Structure(
                "conflict labels must match tree arity; resolved label must be empty",
            ));
        }
        Ok(Self {
            parents: c.parents.into_iter().map(native_commit_id).collect(),
            predecessors: c.predecessors.into_iter().map(native_commit_id).collect(),
            root_tree: merge(c.root_tree.into_iter().map(native_tree_id).collect())?,
            conflict_labels: merge(c.conflict_labels)?,
            change_id: native_change_id(c.change_id),
            description: c.description,
            author: c.author.into(),
            committer: c.committer.into(),
            secure_sig: c.secure_sig.map(|s| b::SecureSig {
                data: s.data,
                sig: s.sig,
            }),
        })
    }
}
metadata_codec!(b::Commit, Commit, Commit, CommitId);

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum TreeValue {
    File {
        id: m::FileId,
        executable: bool,
        copy: m::CopyReference,
    },
    Symlink {
        id: m::SymlinkId,
    },
    Tree {
        id: m::TreeId,
    },
    // External IDs (e.g. Git SHA-1) do not have the repository's native width.
    Submodule {
        id: Vec<u8>,
    },
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Tree {
    entries: Vec<(String, TreeValue)>,
}
impl TryFrom<&b::Tree> for Tree {
    type Error = CodecError;
    fn try_from(tree: &b::Tree) -> Result<Self> {
        Ok(Self {
            entries: tree
                .entries()
                .map(|e| {
                    let value = match e.value() {
                        b::TreeValue::File {
                            id,
                            executable,
                            copy_id,
                        } => TreeValue::File {
                            id: file_id(id)?,
                            executable: *executable,
                            copy: copies(copy_id)?,
                        },
                        b::TreeValue::Symlink(id) => TreeValue::Symlink {
                            id: symlink_id(id)?,
                        },
                        b::TreeValue::Tree(id) => TreeValue::Tree { id: tree_id(id)? },
                        b::TreeValue::GitSubmodule(id) => TreeValue::Submodule {
                            id: id.as_bytes().to_vec(),
                        },
                    };
                    Ok((e.name().as_internal_str().to_owned(), value))
                })
                .collect::<Result<_>>()?,
        })
    }
}
impl TryFrom<Tree> for b::Tree {
    type Error = CodecError;
    fn try_from(tree: Tree) -> Result<Self> {
        sorted_unique(&tree.entries.iter().map(|(n, _)| n).collect::<Vec<_>>())?;
        let entries = tree
            .entries
            .into_iter()
            .map(|(name, value)| {
                let name = RepoPathComponentBuf::new(name)
                    .map_err(|_| CodecError::Structure("tree path component"))?;
                let value = match value {
                    TreeValue::File {
                        id,
                        executable,
                        copy,
                    } => b::TreeValue::File {
                        id: native_file_id(id),
                        executable,
                        copy_id: native_copies(copy),
                    },
                    TreeValue::Symlink { id } => b::TreeValue::Symlink(native_symlink_id(id)),
                    TreeValue::Tree { id } => b::TreeValue::Tree(native_tree_id(id)),
                    TreeValue::Submodule { id } => {
                        if id.is_empty() || id.len() > 64 {
                            return Err(CodecError::Structure("external submodule ID width"));
                        }
                        b::TreeValue::GitSubmodule(b::CommitId::new(id))
                    }
                };
                Ok((name, value))
            })
            .collect::<Result<_>>()?;
        Ok(Self::from_sorted_entries(entries))
    }
}
metadata_codec!(b::Tree, Tree, Tree, TreeId);

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CopyHistory {
    current_path: String,
    parents: Vec<m::CopyReference>,
    salt: Vec<u8>,
}
impl TryFrom<&b::CopyHistory> for CopyHistory {
    type Error = CodecError;
    fn try_from(c: &b::CopyHistory) -> Result<Self> {
        Ok(Self {
            current_path: c.current_path.as_internal_file_string().to_owned(),
            parents: c.parents.iter().map(copies).collect::<Result<_>>()?,
            salt: c.salt.clone(),
        })
    }
}
impl TryFrom<CopyHistory> for b::CopyHistory {
    type Error = CodecError;
    fn try_from(c: CopyHistory) -> Result<Self> {
        let current_path = RepoPathBuf::from_internal_string(c.current_path)
            .map_err(|_| CodecError::Structure("copy path"))?;
        if current_path.as_internal_file_string().is_empty() {
            return Err(CodecError::Structure("copy path cannot be repository root"));
        }
        Ok(Self {
            current_path,
            parents: c.parents.into_iter().map(native_copies).collect(),
            salt: c.salt,
        })
    }
}
metadata_codec!(b::CopyHistory, CopyHistory, Copy, CopyId);

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct RefTarget(Vec<Option<m::CommitId>>);
impl TryFrom<&o::RefTarget> for RefTarget {
    type Error = CodecError;
    fn try_from(r: &o::RefTarget) -> Result<Self> {
        Ok(Self(
            r.as_merge()
                .iter()
                .map(|id| id.as_ref().map(commit_id).transpose())
                .collect::<Result<_>>()?,
        ))
    }
}
impl TryFrom<RefTarget> for o::RefTarget {
    type Error = CodecError;
    fn try_from(r: RefTarget) -> Result<Self> {
        Ok(Self::from_merge(merge(
            r.0.into_iter().map(|id| id.map(native_commit_id)).collect(),
        )?))
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RemoteState {
    New,
    Tracked,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RemoteRef {
    target: RefTarget,
    state: RemoteState,
}
impl TryFrom<&o::RemoteRef> for RemoteRef {
    type Error = CodecError;
    fn try_from(r: &o::RemoteRef) -> Result<Self> {
        Ok(Self {
            target: (&r.target).try_into()?,
            state: match r.state {
                o::RemoteRefState::New => RemoteState::New,
                o::RemoteRefState::Tracked => RemoteState::Tracked,
            },
        })
    }
}
impl TryFrom<RemoteRef> for o::RemoteRef {
    type Error = CodecError;
    fn try_from(r: RemoteRef) -> Result<Self> {
        Ok(Self {
            target: r.target.try_into()?,
            state: match r.state {
                RemoteState::New => o::RemoteRefState::New,
                RemoteState::Tracked => o::RemoteRefState::Tracked,
            },
        })
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RemoteView {
    bookmarks: BTreeMap<String, RemoteRef>,
    tags: BTreeMap<String, RemoteRef>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct View {
    head_ids: Vec<m::CommitId>,
    local_bookmarks: BTreeMap<String, RefTarget>,
    local_tags: BTreeMap<String, RefTarget>,
    remote_views: BTreeMap<String, RemoteView>,
    git_refs: BTreeMap<String, RefTarget>,
    git_heads: BTreeMap<String, RefTarget>,
    wc_commit_ids: BTreeMap<String, m::CommitId>,
}
// Map keys are native string wrappers. Convert explicitly; ordering is preserved.
macro_rules! wire_map {
    ($map:expr, $convert:expr) => {
        $map.iter()
            .map(|(k, v)| Ok((k.as_str().to_owned(), ($convert)(v)?)))
            .collect::<Result<BTreeMap<_, _>>>()?
    };
}
macro_rules! native_map {
    ($map:expr, $convert:expr) => {
        $map.into_iter()
            .map(|(k, v)| Ok((k.into(), ($convert)(v)?)))
            .collect::<Result<BTreeMap<_, _>>>()?
    };
}
impl TryFrom<&o::View> for View {
    type Error = CodecError;
    fn try_from(v: &o::View) -> Result<Self> {
        let mut head_ids = v
            .head_ids
            .iter()
            .map(commit_id)
            .collect::<Result<Vec<_>>>()?;
        head_ids.sort();
        Ok(Self {
            head_ids,
            local_bookmarks: wire_map!(v.local_bookmarks, RefTarget::try_from),
            local_tags: wire_map!(v.local_tags, RefTarget::try_from),
            remote_views: v
                .remote_views
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.as_str().to_owned(),
                        RemoteView {
                            bookmarks: wire_map!(v.bookmarks, RemoteRef::try_from),
                            tags: wire_map!(v.tags, RemoteRef::try_from),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
            git_refs: wire_map!(v.git_refs, RefTarget::try_from),
            git_heads: wire_map!(v.git_heads, RefTarget::try_from),
            wc_commit_ids: wire_map!(v.wc_commit_ids, commit_id),
        })
    }
}
impl TryFrom<View> for o::View {
    type Error = CodecError;
    fn try_from(v: View) -> Result<Self> {
        sorted_unique(&v.head_ids)?;
        if v.head_ids.is_empty() {
            return Err(CodecError::Structure("view must have heads"));
        }
        Ok(Self {
            head_ids: v
                .head_ids
                .into_iter()
                .map(native_commit_id)
                .collect::<HashSet<_>>(),
            local_bookmarks: native_map!(v.local_bookmarks, o::RefTarget::try_from),
            local_tags: native_map!(v.local_tags, o::RefTarget::try_from),
            remote_views: v
                .remote_views
                .into_iter()
                .map(|(k, v)| {
                    Ok((
                        k.into(),
                        o::RemoteView {
                            bookmarks: native_map!(v.bookmarks, o::RemoteRef::try_from),
                            tags: native_map!(v.tags, o::RemoteRef::try_from),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
            git_refs: native_map!(v.git_refs, o::RefTarget::try_from),
            git_heads: native_map!(v.git_heads, o::RefTarget::try_from),
            wc_commit_ids: v
                .wc_commit_ids
                .into_iter()
                .map(|(k, v)| (k.into(), native_commit_id(v)))
                .collect(),
        })
    }
}
metadata_codec!(o::View, View, View, ViewId);

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    start: Timestamp,
    end: Timestamp,
    description: String,
    hostname: String,
    username: String,
    is_snapshot: bool,
    workspace_name: Option<String>,
    attributes: BTreeMap<String, String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Operation {
    view_id: m::ViewId,
    parents: Vec<m::OperationId>,
    metadata: Metadata,
    commit_predecessors: Option<BTreeMap<m::CommitId, Vec<m::CommitId>>>,
}
impl TryFrom<&o::Operation> for Operation {
    type Error = CodecError;
    fn try_from(op: &o::Operation) -> Result<Self> {
        let m = &op.metadata;
        Ok(Self {
            view_id: view_id(&op.view_id)?,
            parents: op.parents.iter().map(operation_id).collect::<Result<_>>()?,
            metadata: Metadata {
                start: m.time.start.into(),
                end: m.time.end.into(),
                description: m.description.clone(),
                hostname: m.hostname.clone(),
                username: m.username.clone(),
                is_snapshot: m.is_snapshot,
                workspace_name: m.workspace_name.as_ref().map(|n| n.as_str().to_owned()),
                attributes: m.attributes.clone(),
            },
            commit_predecessors: op
                .commit_predecessors
                .as_ref()
                .map(|map| {
                    map.iter()
                        .map(|(k, v)| {
                            Ok((
                                commit_id(k)?,
                                v.iter().map(commit_id).collect::<Result<_>>()?,
                            ))
                        })
                        .collect::<Result<_>>()
                })
                .transpose()?,
        })
    }
}
impl TryFrom<Operation> for o::Operation {
    type Error = CodecError;
    fn try_from(op: Operation) -> Result<Self> {
        let m = op.metadata;
        Ok(Self {
            view_id: native_view_id(op.view_id),
            parents: op.parents.into_iter().map(native_operation_id).collect(),
            metadata: o::OperationMetadata {
                time: o::TimestampRange {
                    start: m.start.into(),
                    end: m.end.into(),
                },
                description: m.description,
                hostname: m.hostname,
                username: m.username,
                is_snapshot: m.is_snapshot,
                workspace_name: m.workspace_name.map(Into::into),
                attributes: m.attributes,
            },
            commit_predecessors: op.commit_predecessors.map(|map| {
                map.into_iter()
                    .map(|(k, v)| {
                        (
                            native_commit_id(k),
                            v.into_iter().map(native_commit_id).collect(),
                        )
                    })
                    .collect()
            }),
        })
    }
}
metadata_codec!(o::Operation, Operation, Operation, OperationId);
