use super::*;
use futures::AsyncReadExt as _;
use jj_lib::{
    backend::{CopyId, TreeValue},
    merge::Merge,
    merged_tree_builder::MergedTreeBuilder,
    repo_path::RepoPathBuf,
};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncConflict {
    pub path: String,
    pub terms: Vec<SyncTerm>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncTerm {
    pub id: usize,
    pub role: String,
    pub kind: String,
    pub text: Option<String>,
    pub executable: Option<bool>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SyncResolution {
    Term { id: usize },
    Text { text: String, executable: bool },
    Delete,
}
impl Session {
    pub async fn sync_conflict(&self, candidate: &str, path: &str) -> Result<SyncConflict> {
        restore_io::checked(path)?;
        let commit = self.sync_candidate(candidate).await?;
        let path_buf = RepoPathBuf::from_internal_string(path.to_owned()).map_err(jj)?;
        let value = commit.tree().path_value(&path_buf).await.map_err(jj)?;
        if value.is_resolved() {
            return Err(Error::Binding);
        }
        let mut terms = Vec::new();
        let mut total = 0usize;
        for (id, value) in value.iter().enumerate() {
            let (kind, text) = match value {
                None => ("deleted", None),
                Some(TreeValue::File { id, .. }) => {
                    let reader = self
                        .repo
                        .store()
                        .read_file(&path_buf, id)
                        .await
                        .map_err(jj)?;
                    let mut bytes = Vec::new();
                    reader.take(1024 * 1024 + 1).read_to_end(&mut bytes).await?;
                    total += bytes.len();
                    if total > 4 * 1024 * 1024 {
                        return Err(Error::Jj("Conflict exceeds the editor byte limit".into()));
                    }
                    let text = if bytes.len() <= 1024 * 1024 && !bytes.contains(&0) {
                        String::from_utf8(bytes).ok()
                    } else {
                        None
                    };
                    (if text.is_some() { "text" } else { "binary" }, text)
                }
                _ => ("unsupported", None),
            };
            let executable = match value {
                Some(TreeValue::File { executable, .. }) => Some(*executable),
                _ => None,
            };
            terms.push(SyncTerm {
                id,
                role: if id % 2 == 0 { "add" } else { "remove" }.into(),
                kind: kind.into(),
                text,
                executable,
            });
        }
        Ok(SyncConflict {
            path: path.into(),
            terms,
        })
    }
    pub async fn resolve_sync(
        &mut self,
        candidate: &str,
        path: &str,
        resolution: SyncResolution,
        exclusions: &[String],
    ) -> Result<Checkpoint> {
        restore_io::checked(path)?;
        if scan::excluded(path, exclusions) {
            return Err(Error::Binding);
        }
        let commit = self.sync_candidate(candidate).await?;
        self.sync_base(candidate)?;
        let path = RepoPathBuf::from_internal_string(path.to_owned()).map_err(jj)?;
        let conflict = commit.tree().path_value(&path).await.map_err(jj)?;
        if conflict.is_resolved() {
            return Err(Error::Binding);
        }
        let value = match resolution {
            SyncResolution::Term { id } => conflict.iter().nth(id).ok_or(Error::Binding)?.clone(),
            SyncResolution::Delete => None,
            SyncResolution::Text { text, executable } => {
                if text.len() > 1024 * 1024 || text.contains('\0') {
                    return Err(Error::Binding);
                }
                let id = self
                    .repo
                    .store()
                    .write_file(&path, &mut futures::io::Cursor::new(text.into_bytes()))
                    .await
                    .map_err(jj)?;
                Some(TreeValue::File {
                    id,
                    executable,
                    copy_id: CopyId::placeholder(),
                })
            }
        };
        let mut builder = MergedTreeBuilder::new(commit.tree());
        builder.set_or_remove(path, Merge::resolved(value));
        let tree = builder.write_tree().await.map_err(jj)?;
        let mut tx = self.repo.start_transaction();
        let resolved = tx
            .repo_mut()
            .new_commit(vec![commit.id().clone()], tree)
            .set_description("Resolve vault synchronization conflict")
            .write()
            .await
            .map_err(jj)?;
        tx.repo_mut().set_local_bookmark_target(
            RefName::new("lapis-sync"),
            RefTarget::normal(resolved.id().clone()),
        );
        let file = self.binding.state.join("sync-proposal.json");
        let mut saved: Saved = publication::read(&file)?;
        if saved.candidate != candidate {
            return Err(Error::Binding);
        }
        saved.candidate = resolved.id().hex();
        publication::save(&file, &saved)?;
        self.repo = tx
            .commit("resolve vault synchronization conflict")
            .await
            .map_err(jj)?;
        Ok(checkpoint(&resolved, &self.repo))
    }
}
