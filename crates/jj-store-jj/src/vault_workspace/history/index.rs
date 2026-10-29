//! Immutable graph/delta reads for the disposable historical text index.
use super::*;
use jj_lib::{matchers::EverythingMatcher, repo_path::RepoPathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexRevision {
    pub revision: HistoryRevision,
    pub parents: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexFile {
    pub path: String,
    pub content_id: Option<String>,
    pub text: Option<String>,
    pub deleted: bool,
    pub previous: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexFiles {
    pub files: Vec<IndexFile>,
    pub next: Option<String>,
}

impl HistoryReader {
    pub async fn index_revision(&self, id: &str) -> Result<IndexRevision> {
        let commit = self.retained_commit(id).await?;
        let root = self.repo.store().root_commit_id();
        Ok(IndexRevision {
            revision: HistoryRevision {
                checkpoint: checkpoint(&commit, &self.repo),
                author: commit.author().name.clone(),
                email: commit.author().email.clone(),
            },
            parents: commit
                .parent_ids()
                .iter()
                .filter(|id| *id != root)
                .map(|id| id.hex())
                .collect(),
        })
    }

    pub async fn index_files(
        &self,
        id: &str,
        after: Option<&str>,
        exclusions: &[String],
    ) -> Result<IndexFiles> {
        let commit = self.retained_commit(id).await?;
        let parent_id = commit
            .parent_ids()
            .first()
            .unwrap_or(self.repo.store().root_commit_id());
        let parent = self.repo.store().get_commit(parent_id).map_err(jj)?;
        let before = parent.tree();
        let after_tree = commit.tree();
        let mut stream = before.diff_stream(&after_tree, &EverythingMatcher);
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut files: Vec<IndexFile> = Vec::new();
        let mut next = None;
        while let Some(entry) = stream.next().await {
            if Instant::now() > deadline {
                return Err(Error::Jj("History delta exceeded its read budget".into()));
            }
            let name = entry.path.as_internal_file_string();
            if after.is_some_and(|a| name <= a) || scan::excluded(name, exclusions) {
                continue;
            }
            restore_io::checked(name)?;
            if files.len() == 8 {
                next = files.last().map(|file| file.path.clone());
                break;
            }
            let values = entry.values.map_err(jj)?;
            let deleted = values.after.as_resolved().is_some_and(Option::is_none);
            let content_id = match values.after.as_resolved() {
                Some(Some(TreeValue::File { id, .. })) => Some(id.hex()),
                _ => None,
            };
            let text = self.index_text(id, name, exclusions).await?;
            files.push(IndexFile {
                path: name.into(),
                content_id,
                text,
                deleted,
                previous: deleted.then(|| parent_id.hex()),
            });
        }
        Ok(IndexFiles { files, next })
    }

    pub async fn index_text(
        &self,
        id: &str,
        path: &str,
        exclusions: &[String],
    ) -> Result<Option<String>> {
        restore_io::checked(path)?;
        if scan::excluded(path, exclusions) {
            return Err(Error::Binding);
        }
        let commit = self.retained_commit(id).await?;
        let path = RepoPathBuf::from_internal_string(path).map_err(jj)?;
        let value = commit.tree().path_value(&path).await.map_err(jj)?;
        let Some(Some(TreeValue::File { id, .. })) = value.as_resolved() else {
            return Ok(None);
        };
        let mut reader = self
            .repo
            .store()
            .read_file(&path, id)
            .await
            .map_err(jj)?
            .take(1024 * 1024 + 1);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await.map_err(jj)?;
        Ok((bytes.len() <= 1024 * 1024 && !bytes.contains(&0))
            .then(|| String::from_utf8(bytes).ok())
            .flatten())
    }
}
