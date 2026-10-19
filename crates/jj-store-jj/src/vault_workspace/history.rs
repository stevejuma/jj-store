//! Bounded immutable checkpoint history for a rebuildable search projection.
mod divergence;
use super::*;
pub use divergence::CheckpointDivergence;
use futures::{AsyncReadExt as _, StreamExt as _, TryStreamExt as _};
use jj_lib::{backend::TreeValue, revset::RevsetExpression};
use std::time::{Duration, Instant};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryRevision {
    pub checkpoint: Checkpoint,
    pub author: String,
    pub email: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryPage {
    pub anchor: Option<String>,
    pub revision: Option<HistoryRevision>,
    pub more: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryFile {
    pub path: String,
    pub text: Option<String>,
    /// Binary, symlink, conflicted or over-one-MiB files remain path-searchable.
    pub unindexed: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryFiles {
    pub commit: String,
    pub after: Option<String>,
    pub files: Vec<HistoryFile>,
    pub next: Option<String>,
}
/// Immutable object reader. It takes no working-copy lease and never initializes
/// a source; admitted capture must have created the binding already.
pub struct HistoryReader {
    binding: Binding,
    repo: Arc<ReadonlyRepo>,
}
impl HistoryReader {
    pub async fn open(binding: Binding) -> Result<Self> {
        binding.verify_saved()?;
        let loader = RepoLoader::init_from_file_system(
            &settings()?,
            &binding.metadata(),
            &default_backend_factories(),
        )
        .map_err(jj)?;
        let heads = loader.op_heads_store().get_op_heads().await.map_err(jj)?;
        let [id] = heads.as_slice() else {
            return Err(Error::Binding);
        };
        let operation = loader.load_operation(id).await.map_err(jj)?;
        let repo = loader.load_at(&operation).await.map_err(jj)?;
        Ok(Self { binding, repo })
    }
    async fn retained_commit(&self, checkpoint: &str) -> Result<jj_lib::commit::Commit> {
        content::retained_commit(&self.binding, &self.repo, checkpoint).await
    }
    fn checkpoint(&self) -> Result<Option<Checkpoint>> {
        self.repo
            .view()
            .get_local_bookmark(jj_lib::ref_name::RefName::new("lapis-backup"))
            .as_normal()
            .map(|id| {
                self.repo
                    .store()
                    .get_commit(id)
                    .map(|c| checkpoint(&c, &self.repo))
                    .map_err(jj)
            })
            .transpose()
    }

    /// A fixed retained anchor prevents concurrent captures from shifting pages.
    /// Reads neither snapshot disk files nor reconcile operation heads.
    pub async fn history_revision(&self, anchor: Option<&str>, offset: u32) -> Result<HistoryPage> {
        self.binding.verify_saved()?;
        if offset > 1_000_000 {
            return Err(Error::Binding);
        }
        let anchor = anchor
            .map(str::to_owned)
            .or(self.checkpoint()?.map(|c| c.commit));
        let Some(id) = &anchor else {
            return Ok(HistoryPage {
                anchor: None,
                revision: None,
                more: false,
            });
        };
        let selected = self.retained_commit(id).await?;
        let expression = RevsetExpression::commits(vec![selected.id().clone()]).ancestors();
        let root = self.repo.store().root_commit_id();
        let ids = expression
            .evaluate(self.repo.as_ref())
            .map_err(jj)?
            .stream()
            .try_filter(|id| std::future::ready(id != root))
            .skip(offset as usize)
            .take(2)
            .try_collect::<Vec<_>>()
            .await
            .map_err(jj)?;
        let revision = ids
            .first()
            .map(|id| self.repo.store().get_commit(id).map_err(jj))
            .transpose()?
            .map(|c| HistoryRevision {
                checkpoint: checkpoint(&c, &self.repo),
                author: c.author().name.clone(),
                email: c.author().email.clone(),
            });
        Ok(HistoryPage {
            anchor,
            revision,
            more: ids.len() > 1,
        })
    }
    /// Present changed-file versions relative to the first parent. Earlier bytes
    /// of deletions remain available in their own preceding checkpoint record.
    pub async fn history_files(
        &self,
        commit: &str,
        after: Option<&str>,
        exclusions: &[String],
    ) -> Result<HistoryFiles> {
        if let Some(after) = after {
            restore_io::checked(after)?;
        }
        let selected = self.retained_commit(commit).await?;
        let before = selected
            .parent_ids()
            .first()
            .map(|id| self.repo.store().get_commit(id).map_err(jj))
            .transpose()?;
        let parent = before.as_ref().map(|c| c.tree());
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut files = Vec::new();
        let mut next = None;
        for (path, value) in selected.tree().entries() {
            if Instant::now() > deadline {
                return Err(Error::Jj(
                    "History text scan exceeded its work limit".into(),
                ));
            }
            let name = path.as_internal_file_string();
            if after.is_some_and(|a| name <= a) || scan::excluded(name, exclusions) {
                continue;
            }
            restore_io::checked(name)?;
            let value = value.map_err(jj)?;
            if let Some(parent) = &parent
                && parent.path_value(&path).await.map_err(jj)? == value
            {
                continue;
            }
            if files.len() == 8 {
                next = files.last().map(|f: &HistoryFile| f.path.clone());
                break;
            }
            let mut text = None;
            if let Some(Some(TreeValue::File { id, .. })) = value.as_resolved() {
                let reader = self.repo.store().read_file(&path, id).await.map_err(jj)?;
                let mut data = Vec::new();
                reader
                    .take(1024 * 1024 + 1)
                    .read_to_end(&mut data)
                    .await
                    .map_err(jj)?;
                if data.len() <= 1024 * 1024 && !data.contains(&0) {
                    text = String::from_utf8(data).ok();
                }
            }
            files.push(HistoryFile {
                path: name.to_owned(),
                unindexed: text.is_none(),
                text,
            });
        }
        Ok(HistoryFiles {
            commit: commit.into(),
            after: after.map(str::to_owned),
            files,
            next,
        })
    }
}

impl Session {
    pub async fn history_revision(&self, anchor: Option<&str>, offset: u32) -> Result<HistoryPage> {
        HistoryReader {
            binding: self.binding.clone(),
            repo: self.repo.clone(),
        }
        .history_revision(anchor, offset)
        .await
    }
    pub async fn history_files(
        &self,
        commit: &str,
        after: Option<&str>,
        exclusions: &[String],
    ) -> Result<HistoryFiles> {
        HistoryReader {
            binding: self.binding.clone(),
            repo: self.repo.clone(),
        }
        .history_files(commit, after, exclusions)
        .await
    }
}
