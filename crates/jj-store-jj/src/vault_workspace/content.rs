use super::*;
use futures::AsyncReadExt as _;
use jj_lib::{
    backend::{CommitId, TreeValue},
    ref_name::RefName,
    repo_path::RepoPathBuf,
};

/// Private bytes for native restore. This type is intentionally not a wire DTO.
#[derive(Debug, PartialEq, Eq)]
pub enum FileContents {
    Regular { data: Vec<u8>, executable: bool },
    Symlink { target: String },
}
impl Session {
    pub(super) async fn retained_commit(&self, checkpoint: &str) -> Result<jj_lib::commit::Commit> {
        self.binding.verify_saved()?;
        let selected = CommitId::try_from_hex(checkpoint).ok_or(Error::Binding)?;
        for name in ["lapis-backup", "lapis-incoming"] {
            let head = self.repo.view().get_local_bookmark(RefName::new(name));
            if let Some(head) = head.as_normal()
                && self
                    .repo
                    .index()
                    .is_ancestor(&selected, head)
                    .await
                    .map_err(jj)?
            {
                return self.repo.store().get_commit(&selected).map_err(jj);
            }
        }
        Err(Error::Binding)
    }
    /// Resolve only retained checkpoint/imported-head ancestors, never arbitrary objects.
    pub async fn file_at(&self, checkpoint: &str, path: &str) -> Result<Option<FileContents>> {
        self.binding.verify_saved()?;
        if path.is_empty()
            || path.len() > 4096
            || path.contains('\\')
            || path
                .split('/')
                .any(|p| matches!(p, "" | "." | ".." | ".git" | ".jj"))
        {
            return Err(Error::Binding);
        }
        let commit = self.retained_commit(checkpoint).await?;
        let path = RepoPathBuf::from_internal_string(path.to_owned()).map_err(jj)?;
        let value = commit.tree().path_value(&path).await.map_err(jj)?;
        let value = value
            .as_resolved()
            .ok_or_else(|| Error::Jj("The selected version contains a file conflict".into()))?;
        Ok(match value {
            None => None,
            Some(TreeValue::File { id, executable, .. }) => {
                let reader = self.repo.store().read_file(&path, id).await.map_err(jj)?;
                let mut data = Vec::new();
                reader
                    .take(64 * 1024 * 1024 + 1)
                    .read_to_end(&mut data)
                    .await
                    .map_err(jj)?;
                if data.len() > 64 * 1024 * 1024 {
                    return Err(Error::Binding);
                }
                Some(FileContents::Regular {
                    data,
                    executable: *executable,
                })
            }
            Some(TreeValue::Symlink(id)) => Some(FileContents::Symlink {
                target: self
                    .repo
                    .store()
                    .read_symlink(&path, id)
                    .await
                    .map_err(jj)?,
            }),
            Some(_) => return Err(Error::Binding),
        })
    }
}
