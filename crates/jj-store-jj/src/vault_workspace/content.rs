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
    /// Resolve only a retained checkpoint ancestor, never an arbitrary Git object.
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
        let selected = CommitId::try_from_hex(checkpoint).ok_or(Error::Binding)?;
        let head = self
            .repo
            .view()
            .get_local_bookmark(RefName::new("lapis-backup"));
        let head = head.as_normal().ok_or(Error::Binding)?;
        if !self
            .repo
            .index()
            .is_ancestor(&selected, head)
            .await
            .map_err(jj)?
        {
            return Err(Error::Binding);
        }
        let path = RepoPathBuf::from_internal_string(path.to_owned()).map_err(jj)?;
        let commit = self.repo.store().get_commit(&selected).map_err(jj)?;
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
