use super::*;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

fn files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            result.extend(files(&entry.path()));
        } else {
            result.insert(entry.path(), fs::read(entry.path()).unwrap());
        }
    }
    result
}
fn assert_repo_files_stable(root: &Path, mut before: BTreeMap<PathBuf, Vec<u8>>) {
    let marker = root.join(".jj/repo/workspace_store/type");
    let mut after = files(&root.join(".jj/repo"));
    let written = after.remove(&marker);
    let previous = before.remove(&marker);
    assert_eq!(after, before);
    if let Some(previous) = previous {
        assert_eq!(written.as_ref(), Some(&previous));
    } else {
        assert_eq!(
            written.as_deref(),
            Some(jj_lib::simple_workspace_store::SimpleWorkspaceStore::NAME.as_bytes())
        );
    }
}
#[tokio::test]
async fn historical_only_code_remains_reachable_after_gc_and_packed_ref_replay() {
    let fixture = Fixture::new().await;
    stock(&fixture.root(), &["new", "-m", "historical-only"]);
    let output = stock(
        &fixture.root(),
        &[
            "--ignore-working-copy",
            "log",
            "--no-graph",
            "-r",
            "@",
            "-T",
            "commit_id",
        ],
    );
    let historical: m::GitSha1Id = output
        .trim()
        .parse()
        .unwrap_or_else(|error| panic!("historical ID output {output:?}: {error}"));
    stock(
        &fixture.root(),
        &["--ignore-working-copy", "abandon", &historical.to_string()],
    );
    let operation = stock(
        &fixture.root(),
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    )
    .parse()
    .unwrap();
    let before = fixture.state();
    let source_before = files(&fixture.root().join(".jj/repo"));
    let archive = adversarial::capture(&fixture, operation).await;
    assert!(archive.commits.contains_key(&historical));
    assert!(
        !archive.capsule().unwrap().commits.contains_key(&historical),
        "test requires code used only by a historical view"
    );
    assert_repo_files_stable(&fixture.root(), source_before);
    let store = MetadataStore::open(&fixture.root().join(".git"), Limits::default()).unwrap();
    let key = adversarial::key(&fixture);
    let StackRetentionOutcome::Retained(selected) =
        store.retain_stack_execution(&key, &archive).unwrap()
    else {
        panic!()
    };
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(store.git_dir())
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let git = gix::open(store.git_dir()).unwrap();
    assert!(
        git.find_object(gix::ObjectId::from_bytes_or_panic(historical.as_bytes()))
            .is_ok()
    );
    let StackRetentionOutcome::Retained(replay) =
        store.retain_stack_execution(&key, &archive).unwrap()
    else {
        panic!()
    };
    assert_eq!(replay, selected);
    assert_eq!(fixture.state(), before);
}

#[tokio::test]
async fn archive_capture_never_imports_missing_extras_or_creates_nogc_refs() {
    let fixture = Fixture::new().await;
    let operation = fixture.witness().await.baseline.operation;
    // Drop only derived extra-table heads in this disposable fixture. Upstream
    // read_commit would implicitly import all now-missing records.
    let extras = fixture.root().join(".jj/repo/store/extra/heads");
    for entry in fs::read_dir(&extras).unwrap() {
        fs::remove_file(entry.unwrap().path()).unwrap();
    }
    let before = files(&fixture.root().join(".jj/repo"));
    let refs = files(&fixture.root().join(".git/refs"));
    let state = fixture.state();
    assert!(
        local_evidence::capture_operation_archive(
            &UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap(),
            &fixture.root().join(".jj/repo"),
            fixture.access().repository(),
            operation,
            OperationArchiveLimits::default(),
        )
        .await
        .is_err(),
        "recovery cannot infer lost original change IDs/extras"
    );
    assert_repo_files_stable(&fixture.root(), before);
    assert_eq!(files(&fixture.root().join(".git/refs")), refs);
    assert_eq!(fixture.state(), state);
}
