use super::*;
fn script(path: &Path, text: &str) {
    fs::write(path, text).unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
}
fn npm(root: &Path, fail: bool) -> PathBuf {
    let path = root.join("npm-cli.sh");
    script(
        &path,
        &format!(
            r#"#!/bin/sh
while [ "$#" -gt 0 ]; do
  if [ "$1" = "--prefix" ]; then shift; prefix="$1"; fi
  shift
done
mkdir -p "$prefix/bin" "$prefix/lib/node_modules/@agentclientprotocol/codex-acp/dist" "$prefix/lib/node_modules/@openai/codex/bin"
printf '%s' '{{"name":"@agentclientprotocol/codex-acp","version":"{ADAPTER_VERSION}"}}' > "$prefix/lib/node_modules/@agentclientprotocol/codex-acp/package.json"
printf '%s' '{{"name":"@openai/codex","version":"{CODEX_VERSION}"}}' > "$prefix/lib/node_modules/@openai/codex/package.json"
printf '#!/bin/sh\necho {ADAPTER_VERSION}\n' > "$prefix/lib/node_modules/@agentclientprotocol/codex-acp/dist/index.js"
printf '#!/bin/sh\necho {CODEX_VERSION}\n' > "$prefix/lib/node_modules/@openai/codex/bin/codex.js"
chmod 700 "$prefix/lib/node_modules/@agentclientprotocol/codex-acp/dist/index.js" "$prefix/lib/node_modules/@openai/codex/bin/codex.js"
ln -s ../lib/node_modules/@agentclientprotocol/codex-acp/dist/index.js "$prefix/bin/codex-acp"
ln -s ../lib/node_modules/@openai/codex/bin/codex.js "$prefix/bin/codex"
exit {}
"#,
            if fail { 1 } else { 0 }
        ),
    );
    path
}
fn intent(state: &mut State, n: u8) -> RequestId {
    let id = RequestId::new([n; 16]);
    state.runtime_installs.insert(
        id,
        InstallIntent {
            digest: format!("intent-{n}"),
            outcome: dto::Outcome {
                version: 1,
                request_id: id,
                runtime_id: "codex".into(),
                state: "installing".into(),
                error: None,
            },
        },
    );
    id
}
#[tokio::test]
async fn private_install_verifies_before_selection_and_survives_cold_journal_restart() {
    let root = tempfile::tempdir().unwrap();
    let mut state = State::default();
    let id = intent(&mut state, 1);
    journal::persist(root.path(), &state).unwrap();
    let fake = npm(root.path(), false);
    // Controlled fixture npm runs under sh; production only resolves and probes Node.
    prepare(
        root.path(),
        id,
        Path::new("/bin/sh"),
        &fake,
        &["/usr/bin".into(), "/bin".into()],
    )
    .await
    .unwrap();
    assert!(state.runtime_selection.is_none());
    assert!(
        !paths::directories(root.path(), None)
            .contains(&paths::prefix(root.path(), id).join("bin"))
    );
    complete(&mut state, id, Ok(())).unwrap();
    journal::persist(root.path(), &state).unwrap();
    let reopened = journal::read(root.path()).unwrap();
    assert_eq!(reopened.runtime_installs[&id].outcome.state, "completed");
    let paths = paths::directories(root.path(), reopened.runtime_selection.as_ref());
    let adapter = paths::resolve(&paths, "codex-acp").unwrap();
    assert!(adapter.starts_with(root.path()));
    assert_eq!(
        paths::probe(&adapter, &paths).await.as_deref(),
        Some(ADAPTER_VERSION)
    );
    assert_eq!(reopened.setup_revision, None);
    assert!(reopened.sessions.is_empty() && reopened.intents.is_empty());
    assert_eq!(
        prepare(root.path(), id, Path::new("/bin/sh"), &fake, &paths)
            .await
            .unwrap_err(),
        "runtime_staging_unavailable"
    );
}
#[tokio::test]
async fn failed_update_and_interrupted_staging_preserve_previous_selection() {
    let root = tempfile::tempdir().unwrap();
    let mut state = State::default();
    let old = intent(&mut state, 1);
    complete(&mut state, old, Ok(())).unwrap();
    let failed = intent(&mut state, 2);
    let fake = npm(root.path(), true);
    let result = prepare(
        root.path(),
        failed,
        Path::new("/bin/sh"),
        &fake,
        &["/usr/bin".into(), "/bin".into()],
    )
    .await;
    assert_eq!(result, Err("runtime_install_failed_check_network"));
    complete(&mut state, failed, result).unwrap();
    let interrupted = intent(&mut state, 3);
    journal::persist(root.path(), &state).unwrap();
    let reopened = journal::read(root.path()).unwrap();
    assert_eq!(reopened.runtime_selection.unwrap().request_id, old);
    assert_eq!(reopened.runtime_installs[&failed].outcome.state, "failed");
    assert_eq!(
        reopened.runtime_installs[&interrupted].outcome.state,
        "interrupted"
    );
    assert!(!paths::prefix(root.path(), interrupted).exists());
}
#[tokio::test]
async fn aborted_installer_kills_its_process_group_without_late_side_effects() {
    let root = tempfile::tempdir().unwrap();
    let fake = root.path().join("slow.sh");
    script(
        &fake,
        r#"#!/bin/sh
touch started
(sleep 2; touch late-side-effect) &
wait
"#,
    );
    let id = RequestId::new([4; 16]);
    let path = root.path().to_owned();
    let task = tokio::spawn(async move {
        prepare(
            &path,
            id,
            Path::new("/bin/sh"),
            &fake,
            &["/bin".into(), "/usr/bin".into()],
        )
        .await
    });
    let stage = paths::prefix(root.path(), id);
    tokio::time::timeout(Duration::from_secs(5), async {
        while !stage.join("started").exists() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    task.abort();
    let _ = task.await;
    tokio::time::sleep(Duration::from_millis(2200)).await;
    assert!(!stage.join("late-side-effect").exists());
}
#[test]
fn fixed_install_command_never_inherits_npm_credentials_or_executes_a_shell() {
    let root = tempfile::tempdir().unwrap();
    let fake = npm(root.path(), false);
    let command = command(
        Path::new("/trusted/node"),
        &fake,
        root.path(),
        &["/usr/bin".into()],
    )
    .unwrap();
    let cmd = command.as_std();
    assert_eq!(cmd.get_program(), "/trusted/node");
    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    assert!(args.contains(&format!("@agentclientprotocol/codex-acp@{ADAPTER_VERSION}")));
    assert!(args.contains(&format!("@openai/codex@{CODEX_VERSION}")));
    assert!(args.contains(&"--ignore-scripts".into()));
    assert!(args.contains(&"--registry=https://registry.npmjs.org".into()));
    let env: Vec<_> = cmd
        .get_envs()
        .map(|(k, _)| k.to_string_lossy().into_owned())
        .collect();
    assert_eq!(env, vec!["HOME", "PATH", "TMPDIR"]);
}
#[test]
fn disk_budget_does_not_follow_symlinks_outside_staging() {
    let root = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink("/", root.path().join("outside")).unwrap();
    fs::write(root.path().join("small"), "abc").unwrap();
    assert_eq!(size(root.path()).unwrap(), 3);
}
#[tokio::test]
#[ignore = "downloads fixed npm packages into a disposable private directory"]
async fn real_private_codex_install_verifies_pinned_versions_and_cold_selection() {
    let root = tempfile::tempdir().unwrap();
    let paths = paths::directories(root.path(), None);
    let node = paths::resolve(&paths, "node")
        .unwrap()
        .canonicalize()
        .unwrap();
    let npm = paths::resolve(&paths, "npm").unwrap();
    let mut state = State::default();
    let id = intent(&mut state, 8);
    journal::persist(root.path(), &state).unwrap();
    prepare(root.path(), id, &node, &npm, &paths).await.unwrap();
    complete(&mut state, id, Ok(())).unwrap();
    journal::persist(root.path(), &state).unwrap();
    let reopened = journal::read(root.path()).unwrap();
    let paths = paths::directories(root.path(), reopened.runtime_selection.as_ref());
    let adapter = paths::resolve(&paths, "codex-acp").unwrap();
    let cli = paths::resolve(&paths, "codex").unwrap();
    assert!(adapter.starts_with(root.path()) && cli.starts_with(root.path()));
    assert_eq!(
        paths::probe(&adapter, &paths).await.as_deref(),
        Some(ADAPTER_VERSION)
    );
    assert_eq!(
        paths::probe(&cli, &paths).await.as_deref(),
        Some(CODEX_VERSION)
    );
    assert!(reopened.sessions.is_empty());
}
