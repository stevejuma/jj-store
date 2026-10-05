use super::super::objects as git;
use super::*;
impl MetadataStore {
    pub(super) fn stack_selected(
        &self,
        repo: &gix::Repository,
        key: &StackPreparationKey,
    ) -> Result<Option<GitOid>> {
        let Some(reference) = repo
            .try_find_reference(&key.reference())
            .map_err(Error::other)?
        else {
            return Ok(None);
        };
        reference
            .try_id()
            .map(|id| GitOid::from_git(id.detach()))
            .transpose()?
            .ok_or(Error::Invalid("symbolic stack preparation ref"))
            .map(Some)
    }
    pub(super) fn stack_acknowledge(
        &self,
        repo: &gix::Repository,
        selected: StackRetention,
    ) -> Result<StackRetentionOutcome> {
        let sync = || -> Result<()> {
            if self.stack_selected(repo, &selected.key)? != Some(selected.commit) {
                return Err(Error::Stale);
            }
            let path = repo.common_dir().join(selected.key.reference());
            match std::fs::symlink_metadata(&path) {
                Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                    std::fs::File::open(&path)?.sync_all()?;
                    let mut dir = path.parent();
                    while let Some(parent) = dir {
                        git::sync_dir(parent)?;
                        if parent == repo.common_dir() {
                            break;
                        }
                        dir = parent.parent();
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    let packed = repo.common_dir().join("packed-refs");
                    let metadata = std::fs::symlink_metadata(&packed)?;
                    if !metadata.is_file() || metadata.file_type().is_symlink() {
                        return Err(Error::Invalid("stack packed ref file"));
                    }
                    std::fs::File::open(packed)?.sync_all()?;
                    git::sync_dir(repo.common_dir())?;
                }
                _ => return Err(Error::Invalid("stack selected ref file")),
            }
            if self.stack_selected(repo, &selected.key)? != Some(selected.commit) {
                return Err(Error::Stale);
            }
            Ok(())
        };
        Ok(if sync().is_ok() {
            StackRetentionOutcome::Retained(Box::new(selected))
        } else {
            StackRetentionOutcome::Unknown {
                commit: selected.commit,
            }
        })
    }
}
