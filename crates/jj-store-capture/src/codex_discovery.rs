//! Read-only source acquisition; repository admission and Cursor binding stay here.
use crate::{CaptureResult, profile::CaptureProfile};
use std::path::{Path, PathBuf};

mod scanner;
pub(crate) use lapis_ai_capture::discovery::PhysicalId;
pub use lapis_ai_capture::discovery::{ApprovedCodexRoot, CodexSourceCandidate};
pub use scanner::{CodexDirectoryScanner, CodexDiscoveryStep};

pub fn resolve_codex_roots(
    profile: CaptureProfile,
    explicit: &[PathBuf],
    home: Option<&Path>,
    codex_home: Option<&Path>,
) -> CaptureResult<Vec<PathBuf>> {
    lapis_ai_capture::discovery::resolve_codex_roots(profile.into(), explicit, home, codex_home)
}

#[cfg(all(test, unix))]
mod binding_tests {
    use super::*;
    use std::{fs, os::unix::fs::symlink};

    #[test]
    fn current_binding_distinguishes_absence_from_unknown_io_outcomes() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("approved");
        fs::create_dir(&path).unwrap();
        let root = ApprovedCodexRoot::open(&path).unwrap().unwrap();
        assert!(root.current_binding().unwrap());
        fs::rename(&path, temporary.path().join("retained")).unwrap();
        assert!(!root.current_binding().unwrap());
        // A symlink loop is an unknown IO outcome, never retirement evidence.
        symlink(&path, &path).unwrap();
        assert!(root.current_binding().is_err());
        assert!(!root.revalidate().unwrap());
    }
}
