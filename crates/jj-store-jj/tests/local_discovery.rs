use jj_store_jj::local_repository::{DiscoveryError, inspect_workspace};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn jj(root: &Path, args: &[&str]) {
    let program = std::env::var_os("JJ_STORE_TEST_STOCK_JJ").unwrap_or_else(|| "jj".into());
    let output = Command::new(program)
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
fn state(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn walk(root: &Path, path: &Path, result: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                walk(root, &path, result);
            } else {
                assert!(kind.is_file());
                result.insert(
                    path.strip_prefix(root).unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut result = BTreeMap::new();
    walk(root, root, &mut result);
    result
}
#[test]
fn discovery_and_default_carrier_open_preserve_dirty_files_identity_and_repository_config() {
    for colocate in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut args = vec!["git", "init"];
        if colocate {
            args.push("--colocate");
        } else {
            args.push("--no-colocate");
        }
        jj(root.path(), &args);
        fs::write(root.path().join("dirty.md"), "not recorded\n").unwrap();
        // Discovery must not parse or execute repository-owned settings.
        fs::write(root.path().join(".jj/repo/config.toml"), "invalid = [").unwrap();
        let before = state(root.path());
        let info = inspect_workspace(root.path()).unwrap();
        assert_eq!(info.path, root.path().canonicalize().unwrap());
        assert_eq!(info.metadata, info.path.join(".jj/repo"));
        assert!(info.repository.is_none());
        if colocate {
            assert_eq!(info.git, info.path.join(".git"));
        } else {
            assert!(info.git.starts_with(info.path.join(".jj")));
        }
        let carrier = info.open_metadata().unwrap();
        assert_eq!(carrier.git_dir().canonicalize().unwrap(), info.git);
        assert!(carrier.identity().unwrap().is_none());
        assert_eq!(state(root.path()), before);
    }
}
#[test]
fn linked_workspaces_resolve_shared_storage_without_reconciliation_or_initialization() {
    let fixture = tempfile::tempdir().unwrap();
    let source = fixture.path().join("source");
    jj(
        fixture.path(),
        &["git", "init", "--colocate", source.to_str().unwrap()],
    );
    let linked = fixture.path().join("linked");
    jj(&source, &["workspace", "add", linked.to_str().unwrap()]);
    fs::write(linked.join("dirty.md"), "retained\n").unwrap();
    let before = state(fixture.path());
    let original = inspect_workspace(&source).unwrap();
    let selected = inspect_workspace(&linked).unwrap();
    assert_eq!(selected.metadata, original.metadata);
    assert_eq!(selected.git, original.git);
    assert_ne!(selected.path, original.path);
    assert!(selected.repository.is_none());
    selected.open_metadata().unwrap();
    assert_eq!(state(fixture.path()), before);
}
#[test]
fn discovery_rejects_relative_roots_oversized_links_and_changed_observations() {
    assert_eq!(
        inspect_workspace(Path::new("relative")),
        Err(DiscoveryError::Invalid)
    );
    let fixture = tempfile::tempdir().unwrap();
    fs::create_dir_all(fixture.path().join(".jj/working_copy")).unwrap();
    fs::write(fixture.path().join(".jj/repo"), vec![b'x'; 4097]).unwrap();
    let before = state(fixture.path());
    assert_eq!(
        inspect_workspace(fixture.path()),
        Err(DiscoveryError::Limit)
    );
    assert_eq!(state(fixture.path()), before);
    let root = tempfile::tempdir().unwrap();
    jj(root.path(), &["git", "init", "--colocate"]);
    let mut selected = inspect_workspace(root.path()).unwrap();
    selected.git = root.path().join("invented");
    assert!(matches!(
        selected.open_metadata(),
        Err(DiscoveryError::IdentityChanged)
    ));
}
#[cfg(unix)]
#[test]
fn discovery_keeps_original_symlink_rejection() {
    let root = tempfile::tempdir().unwrap();
    jj(root.path(), &["git", "init", "--colocate"]);
    fs::rename(root.path().join(".jj"), root.path().join("saved")).unwrap();
    std::os::unix::fs::symlink(root.path().join("saved"), root.path().join(".jj")).unwrap();
    assert_eq!(inspect_workspace(root.path()), Err(DiscoveryError::Invalid));
}
