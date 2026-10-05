//! Independent descriptors and real Git retention for canonical collaboration graphs.
use super::{LocalError, LocalRepository, LocalResult, collab_external::ExternalV1};
use jj_collab::*;
use jj_store_jj::local_metadata::{GitOid, RetainedKind, Snapshot};
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) trait Root: Serialize + DeserializeOwned + Clone {
    fn dependencies(&self) -> Vec<CollabObjectRef>;
}
impl Root for ThreadRootV1 {
    fn dependencies(&self) -> Vec<CollabObjectRef> {
        self.dependencies()
    }
}
impl Root for ReactionRootV1 {
    fn dependencies(&self) -> Vec<CollabObjectRef> {
        self.dependencies()
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObjectEntryV1 {
    reference: CollabObjectRef,
    blob: GitOid,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DescriptorV1<R> {
    version: u32,
    repository: RepositoryId,
    root: R,
    objects: Vec<ObjectEntryV1>,
    external: ExternalV1,
}
pub(crate) async fn read<R: Root>(
    repo: &LocalRepository,
    pin: &Snapshot,
    bytes: &[u8],
) -> LocalResult<(R, CollabGraphV1, ExternalV1)> {
    let descriptor: DescriptorV1<R> =
        serde_json::from_slice(bytes).map_err(|_| LocalError::Corrupt)?;
    if descriptor.version != 1
        || descriptor.repository != pin.repository()
        || descriptor.objects.len() > MAX_COLLAB_OBJECTS
        || descriptor
            .objects
            .windows(2)
            .any(|w| w[0].reference >= w[1].reference)
        || serde_json::to_vec(&descriptor).map_err(|_| LocalError::Corrupt)? != bytes
    {
        return Err(LocalError::Corrupt);
    }
    let mut graph = CollabGraphV1::default();
    let mut size = 0usize;
    for entry in descriptor.objects {
        let bytes =
            repo.metadata
                .read_retained_blob(pin, entry.blob)
                .map_err(|error| match error {
                    jj_store_jj::local_metadata::Error::Stale => LocalError::Corrupt,
                    other => other.into(),
                })?;
        size = size.checked_add(bytes.len()).ok_or(LocalError::Limit)?;
        if size > 256 * 1024 * 1024 {
            return Err(LocalError::Limit);
        }
        let object = decode_object(&bytes).map_err(|_| LocalError::Corrupt)?;
        if graph.stage(object).map_err(|_| LocalError::Corrupt)? != entry.reference {
            return Err(LocalError::Corrupt);
        }
    }
    let closure = graph
        .closure(descriptor.root.dependencies())
        .map_err(|_| LocalError::Corrupt)?;
    if closure != graph.objects.keys().copied().collect::<BTreeSet<_>>() {
        return Err(LocalError::Corrupt);
    }
    descriptor.external.check_retained(repo, Some(pin))?;
    descriptor
        .external
        .complete(repo, Some(pin), pin.repository(), &graph, &closure, false)
        .await?;
    Ok((descriptor.root, graph, descriptor.external))
}
pub(crate) async fn stage<R: Root>(
    repo: &LocalRepository,
    pin: Option<&Snapshot>,
    repository: RepositoryId,
    root: R,
    graph: &CollabGraphV1,
    external: &ExternalV1,
) -> LocalResult<(Vec<u8>, BTreeMap<GitOid, RetainedKind>)> {
    let closure = graph.closure(root.dependencies())?;
    let external = external
        .complete(repo, pin, repository, graph, &closure, true)
        .await?;
    let mut retained = external.retained(repo, pin)?;
    let mut objects = Vec::new();
    for reference in closure {
        let blob = repo
            .metadata
            .stage_blob(&encode_object(graph.get(reference)?)?)?;
        retained.insert(blob, RetainedKind::Blob);
        objects.push(ObjectEntryV1 { reference, blob });
    }
    let descriptor = DescriptorV1 {
        version: 1,
        repository,
        root,
        objects,
        external,
    };
    let bytes = serde_json::to_vec(&descriptor).map_err(|_| LocalError::Corrupt)?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err(LocalError::Limit);
    }
    Ok((bytes, retained))
}
