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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CommitDivergence {
    pub local_commit: String,
    pub remote_commit: Option<String>,
    pub incoming: u64,
    pub outgoing: u64,
}

impl HistoryReader {
    /// Compares saved checkpoints, excluding the mutable working-copy child.
    /// A missing local checkpoint is unknown, never an invented zero.
    pub async fn divergence(&self, incoming: &str) -> Result<Option<CheckpointDivergence>> {
        Ok(self
            .checkpoint_divergence(Some(incoming))
            .await?
            .map(|counts| CheckpointDivergence {
                local_commit: counts.local_commit,
                remote_commit: incoming.into(),
                incoming: counts.incoming,
                outgoing: counts.outgoing,
            }))
    }
    /// None is a known empty remote branch. An absent local checkpoint remains unknown.
    pub async fn checkpoint_divergence(
        &self,
        incoming: Option<&str>,
    ) -> Result<Option<CommitDivergence>> {
        self.binding.verify_saved()?;
        let Some(local) = self.checkpoint()? else {
            return Ok(None);
        };
        let remote = match incoming {
            Some(id) => Some(self.retained_commit(id).await?),
            None => None,
        };
        let local_commit = self.retained_commit(&local.commit).await?;
        let counts = graph_divergence(
            self.repo.as_ref(),
            local_commit.id(),
            remote.as_ref().map(|c| c.id()),
        )
        .await?;
        self.binding.verify_saved()?;
        Ok(Some(counts))
    }
}

pub(crate) async fn graph_divergence(
    repo: &ReadonlyRepo,
    local: &jj_lib::backend::CommitId,
    remote: Option<&jj_lib::backend::CommitId>,
) -> Result<CommitDivergence> {
    let left = RevsetExpression::commits(vec![local.clone()]).ancestors();
    let right = RevsetExpression::commits(vec![
        remote.unwrap_or(repo.store().root_commit_id()).clone(),
    ])
    .ancestors();
    let deadline = Instant::now() + Duration::from_secs(2);
    Ok(CommitDivergence {
        local_commit: local.hex(),
        remote_commit: remote.map(|id| id.hex()),
        incoming: count(repo, right.minus(&left), deadline).await?,
        outgoing: count(repo, left.minus(&right), deadline).await?,
    })
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
