//! Exact graph differences against a retained fetched checkpoint, at one operation.
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheckpointDivergence {
    pub local_commit: String,
    pub remote_commit: String,
    pub incoming: u64,
    pub outgoing: u64,
}

impl HistoryReader {
    /// Compares saved checkpoints, excluding the mutable working-copy child.
    /// A missing local checkpoint is unknown, never an invented zero.
    pub async fn divergence(&self, incoming: &str) -> Result<Option<CheckpointDivergence>> {
        self.binding.verify_saved()?;
        let Some(local) = self.checkpoint()? else {
            return Ok(None);
        };
        let remote = self.retained_commit(incoming).await?;
        let local_commit = self.retained_commit(&local.commit).await?;
        let left = RevsetExpression::commits(vec![local_commit.id().clone()]).ancestors();
        let right = RevsetExpression::commits(vec![remote.id().clone()]).ancestors();
        let deadline = Instant::now() + Duration::from_secs(2);
        let incoming_count = count(self.repo.as_ref(), right.minus(&left), deadline).await?;
        let outgoing = count(self.repo.as_ref(), left.minus(&right), deadline).await?;
        self.binding.verify_saved()?;
        Ok(Some(CheckpointDivergence {
            local_commit: local.commit,
            remote_commit: incoming.to_owned(),
            incoming: incoming_count,
            outgoing,
        }))
    }
}

async fn count(
    repo: &ReadonlyRepo,
    expression: Arc<jj_lib::revset::ResolvedRevsetExpression>,
    deadline: Instant,
) -> Result<u64> {
    let evaluated = expression.evaluate(repo).map_err(jj)?;
    let mut ids = evaluated.stream();
    let mut count = 0;
    while let Some(id) = ids.next().await {
        id.map_err(jj)?;
        if count >= 100_000 || Instant::now() > deadline {
            return Err(Error::Jj(
                "Checkpoint divergence exceeds the read budget".into(),
            ));
        }
        count += 1;
    }
    if Instant::now() > deadline {
        return Err(Error::Jj(
            "Checkpoint divergence exceeds the read budget".into(),
        ));
    }
    Ok(count)
}
