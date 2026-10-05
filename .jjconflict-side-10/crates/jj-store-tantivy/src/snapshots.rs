use crate::{disk, *};
use jj_store_adapters::{ArtifactBundle, Checkpoint, Context, Identity, Snapshot};
use std::{collections::BTreeMap, sync::Arc};

pub(crate) async fn publish(
    owner: Arc<disk::Owner>,
    limits: Limits,
    snapshot: &Snapshot,
    checkpoint: &Checkpoint,
    identity: &Identity,
    context: Context,
) -> Result<()> {
    let worker = context.clone();
    let disk::SnapshotFiles { generation, files } =
        tokio::task::spawn_blocking(move || disk::snapshot_files(owner, limits, &worker))
            .await
            .map_err(|_| Error::Unavailable)??;
    if &generation.checkpoint != checkpoint {
        return Err(Error::Corrupt);
    }
    let artifacts = snapshot.artifacts(identity)?;
    let mut blobs = BTreeMap::new();
    for (name, bytes) in files {
        context.check()?;
        let file = tokio::fs::File::open(generation.path.join(&name))
            .await
            .map_err(|_| Error::Unavailable)?;
        let blob = artifacts.upload(file, bytes).await?;
        if blob.bytes != bytes {
            return Err(Error::Corrupt);
        }
        blobs.insert(name, blob);
    }
    let bundle = ArtifactBundle {
        version: 1,
        checkpoint: checkpoint.clone(),
        files: blobs,
    };
    if artifacts.publish(&bundle).await? != bundle {
        return Err(Error::Corrupt);
    }
    Ok(())
}

pub(crate) async fn restore(
    owner: Arc<disk::Owner>,
    limits: Limits,
    snapshot: &Snapshot,
    identity: &Identity,
    context: Context,
) -> Result<Option<Arc<disk::Generation>>> {
    let artifacts = snapshot.artifacts(identity)?;
    let Some(bundle) = artifacts.selected().await? else {
        return Ok(None);
    };
    if !bundle.files.contains_key("generation.json") || bundle.files.len() > limits.index_files + 1
    {
        return Err(Error::Corrupt);
    }
    let root = owner.root.clone();
    let staging = tokio::task::spawn_blocking(move || {
        tempfile::Builder::new().prefix("restore-").tempdir_in(root)
    })
    .await
    .map_err(|_| Error::Unavailable)?
    .map_err(|_| Error::Unavailable)?;
    for (name, blob) in &bundle.files {
        context.check()?;
        let path = staging.path().join(name);
        let mut options = tokio::fs::OpenOptions::new();
        options.create_new(true).write(true);
        let mut file = options.open(path).await.map_err(|_| Error::Unavailable)?;
        artifacts.download(blob, &mut file).await?;
        file.sync_all().await.map_err(|_| Error::Unavailable)?;
    }
    let checkpoint = bundle.checkpoint;
    let worker = context.clone();
    let generation = tokio::task::spawn_blocking(move || {
        disk::install_snapshot(owner, staging, &checkpoint, limits, &worker)
    })
    .await
    .map_err(|_| Error::Unavailable)??;
    Ok(Some(generation))
}
