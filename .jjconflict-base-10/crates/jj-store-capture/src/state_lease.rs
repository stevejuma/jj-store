//! Explicit lock release for private state across subprocess creation.
use crate::CaptureResult as AuthorityResult;
use std::{fs::File, path::Path};

pub struct StateLease(File);

impl Drop for StateLease {
    fn drop(&mut self) {
        // Closing alone can leave the lock held by a child's inherited file
        // description until exec. Release this lease before closing our handle.
        let _ = self.0.unlock();
    }
}

impl StateLease {
    pub(crate) fn from_locked(file: File) -> Self {
        Self(file)
    }

    pub fn exclusive(path: &Path) -> AuthorityResult<Self> {
        Ok(Self(crate::path_policy::exclusive_lock(path)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_lease_releases_with_an_inherited_file_description() {
        for shared in [true, false] {
            let directory = tempfile::tempdir().unwrap();
            let path = directory.path().join("capture.lock");
            let lease = if shared {
                crate::agent_capture_files::compatibility_lock(&path).unwrap()
            } else {
                StateLease::exclusive(&path).unwrap()
            };
            // Forked children briefly retain the same open file description until exec.
            let inherited = lease.0.try_clone().unwrap();
            assert!(crate::path_policy::exclusive_lock(&path).is_err());
            drop(lease);
            let replacement = crate::path_policy::exclusive_lock(&path).unwrap();
            drop(replacement);
            drop(inherited);
        }
    }
}
