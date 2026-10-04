use crate::*;
use jj_store_object::{Condition, Key, StagedUpload, VersionToken, WriteOutcome};
const MAX_CHECKPOINT: u64 = 16 * 1024;
pub(crate) struct Remote {
    pub checkpoint: Option<Checkpoint>,
    version: Option<VersionToken>,
}
fn key(repo: m::RepositoryId, name: &str) -> Key {
    Key::new(format!("repos/{repo}/adapters/v1/{name}/cursor.json"))
        .expect("validated adapter name")
}
async fn authorize(source: &Source, context: &Context, epoch: m::EpochId) -> Result<()> {
    if context.until(source.access.authorize()).await? != epoch {
        return Err(Error::Denied);
    }
    Ok(())
}
pub(crate) async fn read(
    source: &Source,
    identity: &Identity,
    context: &Context,
    epoch: m::EpochId,
) -> Result<Remote> {
    authorize(source, context, epoch).await?;
    context.request()?;
    let result = match source
        .store
        .get(&key(source.repository(), &identity.name), None)
        .await
    {
        Err(jj_store_object::Error::NotFound) => Remote {
            checkpoint: None,
            version: None,
        },
        Err(e) => return Err(e.into()),
        Ok(download) => {
            context.bytes(download.metadata.len)?;
            let version = download.metadata.version.clone();
            let bytes = download.read_bounded(MAX_CHECKPOINT).await?;
            let checkpoint: Checkpoint =
                serde_json::from_slice(&bytes).map_err(|_| Error::Corrupt)?;
            checkpoint.validate()?;
            if serde_json::to_vec(&checkpoint).map_err(|_| Error::Corrupt)? != bytes
                || checkpoint.source.repository != source.repository()
                || checkpoint.identity.name != identity.name
            {
                return Err(Error::Corrupt);
            }
            Remote {
                checkpoint: Some(checkpoint),
                version: Some(version),
            }
        }
    };
    authorize(source, context, epoch).await?;
    Ok(result)
}
pub(crate) async fn persist(
    source: &Source,
    remote: &Remote,
    checkpoint: &Checkpoint,
    context: &Context,
) -> Result<Checkpoint> {
    if remote.checkpoint.as_ref() == Some(checkpoint) {
        return Ok(checkpoint.clone());
    }
    if remote.checkpoint.as_ref().is_some_and(|old| {
        old.identity == checkpoint.identity
            && old.source.epoch == checkpoint.source.epoch
            && old.source.position.0 > checkpoint.source.position.0
    }) {
        // A compatible restored artifact catches up locally without moving the
        // remote cursor backwards. Remote position alone never skips local work.
        return Ok(remote.checkpoint.clone().expect("checked checkpoint"));
    }
    checkpoint.validate()?;
    let bytes = serde_json::to_vec(checkpoint).map_err(|_| Error::Corrupt)?;
    let upload = StagedUpload::from_bytes(&bytes, MAX_CHECKPOINT).await?;
    authorize(source, context, checkpoint.source.epoch).await?;
    context.request()?;
    context.bytes(bytes.len() as u64)?;
    let condition = remote
        .version
        .clone()
        .map_or(Condition::Absent, Condition::Matches);
    let outcome = source
        .store
        .put(
            &key(source.repository(), &checkpoint.identity.name),
            condition,
            &upload,
        )
        .await?;
    authorize(source, context, checkpoint.source.epoch).await?;
    if matches!(outcome, WriteOutcome::Applied(_)) {
        return Ok(checkpoint.clone());
    }
    let found = read(
        source,
        &checkpoint.identity,
        context,
        checkpoint.source.epoch,
    )
    .await?;
    if found.checkpoint.as_ref() == Some(checkpoint) {
        return Ok(checkpoint.clone());
    }
    match outcome {
        WriteOutcome::PreconditionFailed => Err(Error::CursorConflict),
        _ => Err(Error::Unavailable),
    }
}
