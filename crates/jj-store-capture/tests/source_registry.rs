#![cfg(unix)]
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    codex_transcript::TranscriptPoll,
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::json;
use std::{fs, path::Path, process::Command};
const INSTALLATION: [u8; 16] = [8; 16];
fn header(desktop: bool) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&json!({"type":"session_meta","payload":{
        "id":"00000000-0000-0000-0000-000000000001","cwd":"/private/project",
        "cli_version":if desktop { "0.159.2" } else { "0.156.1" },
        "source":if desktop { "vscode" } else { "cli" },
        "originator":if desktop { "Codex Desktop" } else { "codex_cli_rs" },
    }}))
    .unwrap();
    bytes.push(b'\n');
    bytes
}
fn qualified(root: &Path, path: &Path, profile: CaptureProfile) -> QualifiedCodexSource {
    let path = fs::canonicalize(path).unwrap();
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root).unwrap().unwrap()).unwrap();
    let step = scanner.step(100, 100).unwrap();
    let candidate = step
        .candidates
        .into_iter()
        .find(|c| c.path() == path)
        .unwrap();
    QualifiedCodexSource::inspect(profile, candidate)
        .unwrap()
        .unwrap()
}
fn acquire(
    registry: &SourceRegistry,
    root: &Path,
    path: &Path,
) -> jj_store_capture::source_registry::RegisteredCodexSource {
    registry
        .acquire(qualified(root, path, CaptureProfile::CodexCliJsonlV1))
        .unwrap()
}

#[test]
fn restart_overlap_hardlinks_and_replacement_retain_source_identity() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let nested = root.path().join("nested");
    fs::create_dir(&nested).unwrap();
    let path = nested.join("original.jsonl");
    let alias = root.path().join("alias.jsonl");
    fs::write(&path, header(false)).unwrap();
    fs::hard_link(&path, &alias).unwrap();
    let registry = SourceRegistry::open(&state, INSTALLATION, 2).unwrap();
    let first = acquire(&registry, root.path(), &path);
    let source = first.source_instance_id();
    assert!(
        registry
            .acquire(qualified(&nested, &path, CaptureProfile::CodexCliJsonlV1))
            .is_err()
    );
    assert!(
        registry
            .acquire(qualified(
                root.path(),
                &alias,
                CaptureProfile::CodexCliJsonlV1
            ))
            .is_err()
    );
    drop(first);
    assert_eq!(
        acquire(&registry, &nested, &path).source_instance_id(),
        source
    );
    assert_eq!(
        acquire(&registry, root.path(), &alias).source_instance_id(),
        source
    );
    drop(registry);
    let reopened = SourceRegistry::open(&state, INSTALLATION, 2).unwrap();
    assert_eq!(
        acquire(&reopened, root.path(), &alias).source_instance_id(),
        source
    );
    fs::rename(&path, nested.join("old-file")).unwrap();
    fs::write(&path, header(false)).unwrap();
    assert_eq!(
        acquire(&reopened, root.path(), &path).source_instance_id(),
        source
    );
    let retained = acquire(&reopened, root.path(), &alias);
    assert!(
        reopened
            .acquire(qualified(
                root.path(),
                &path,
                CaptureProfile::CodexCliJsonlV1
            ))
            .is_err()
    );
    drop(retained);
    let durable = fs::read_to_string(state.join("sources.json")).unwrap();
    assert!(!durable.contains("original.jsonl"));
    assert!(!durable.contains("private/project"));
    assert!(!durable.contains("session_meta"));
}

#[test]
fn header_qualification_has_no_registry_side_effect_and_leased_reader_revalidates() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let source = qualified(root.path(), &path, CaptureProfile::CodexCliJsonlV1);
    assert_eq!(source.directory(), Path::new("/private/project"));
    assert!(fs::read_dir(private.path()).unwrap().next().is_none());
    let registry = SourceRegistry::open(&private.path().join("registry"), INSTALLATION, 4).unwrap();
    let registered = registry.acquire(source).unwrap();
    let mut reader = registered.open_reader(None).unwrap().unwrap();
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Ready(_)));
    assert!(
        registry
            .acquire(qualified(
                root.path(),
                &path,
                CaptureProfile::CodexCliJsonlV1
            ))
            .is_err()
    );
    fs::rename(&path, root.path().join("retained")).unwrap();
    fs::write(&path, header(false)).unwrap();
    assert!(reader.poll().is_err());
    drop(reader);
    assert!(
        registry
            .acquire(qualified(
                root.path(),
                &path,
                CaptureProfile::CodexCliJsonlV1
            ))
            .is_ok()
    );
}

#[test]
fn exclusive_source_lease_crosses_process_boundary_and_releases() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let state = private.path().join("registry");
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let lease = acquire(&registry, root.path(), &path);
    let probe = |busy: bool| {
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "source_registry_process_probe", "--nocapture"])
            .env("JJ_STORE_REGISTRY_PROBE", &state)
            .env("JJ_STORE_REGISTRY_SOURCE_ROOT", root.path())
            .env("JJ_STORE_REGISTRY_BUSY", if busy { "1" } else { "0" })
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    probe(true);
    drop(lease);
    probe(false);
}

#[test]
fn source_registry_process_probe() {
    let Some(state) = std::env::var_os("JJ_STORE_REGISTRY_PROBE") else {
        return;
    };
    let root = std::path::PathBuf::from(std::env::var_os("JJ_STORE_REGISTRY_SOURCE_ROOT").unwrap());
    let registry = SourceRegistry::open(Path::new(&state), INSTALLATION, 4).unwrap();
    let result = registry.acquire(qualified(
        &root,
        &root.join("session.jsonl"),
        CaptureProfile::CodexCliJsonlV1,
    ));
    if std::env::var_os("JJ_STORE_REGISTRY_HOLD").is_some() {
        use std::io::{Read as _, Write as _};
        let lease = result.unwrap();
        println!("source-instance: {}", lease.source_instance_id());
        std::io::stdout().flush().unwrap();
        let _ = std::io::stdin().read(&mut [0]);
        drop(lease);
        return;
    }
    assert_eq!(
        result.is_err(),
        std::env::var("JJ_STORE_REGISTRY_BUSY").unwrap() == "1"
    );
}

#[test]
fn killed_reader_releases_both_leases_and_keeps_durable_source_identity() {
    use std::io::{BufRead as _, BufReader};
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "source_registry_process_probe", "--nocapture"])
        .env("JJ_STORE_REGISTRY_PROBE", &state)
        .env("JJ_STORE_REGISTRY_SOURCE_ROOT", root.path())
        .env("JJ_STORE_REGISTRY_HOLD", "1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let output = BufReader::new(child.stdout.take().unwrap());
    let (send, ready) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for line in output.lines() {
            let Ok(line) = line else { break };
            // Serial libtest may print its test label before this marker.
            if let Some((_, source)) = line.split_once("source-instance: ") {
                let source = source.trim();
                if source.len() == 32 && source.bytes().all(|b| b.is_ascii_hexdigit()) {
                    let _ = send.send(source.to_owned());
                    return;
                }
            }
        }
    });
    let source = ready
        .recv_timeout(std::time::Duration::from_secs(10))
        .unwrap_or_else(|error| {
            let _ = child.kill();
            let _ = child.wait();
            panic!("source lease probe did not report readiness: {error}");
        });
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let busy = registry
        .acquire(qualified(
            root.path(),
            &path,
            CaptureProfile::CodexCliJsonlV1,
        ))
        .is_err();
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(busy);
    let resumed = acquire(&registry, root.path(), &path);
    assert_eq!(resumed.source_instance_id().to_string(), source.trim());
}

#[test]
fn current_binding_inspection_preserves_read_only_state_and_rejects_replacement_headers() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let _registry = SourceRegistry::open(&state, INSTALLATION, 2).unwrap();
    let before = fs::read(state.join("sources.json")).unwrap();
    let path = root.path().join("current.jsonl");
    fs::write(&path, header(false)).unwrap();
    let original = qualified(root.path(), &path, CaptureProfile::CodexCliJsonlV1);
    assert!(original.is_current().unwrap());
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    // The same physical file cannot retain a changed header witness.
    let changed = String::from_utf8(header(false))
        .unwrap()
        .replace("000000000001", "000000000002");
    fs::write(&path, changed).unwrap();
    assert!(!original.is_current().unwrap());
    let updated = qualified(root.path(), &path, CaptureProfile::CodexCliJsonlV1);
    assert!(updated.is_current().unwrap());
    fs::write(&path, b"not a qualified header\n").unwrap();
    assert!(updated.is_current().is_err());
    fs::write(&path, header(false)).unwrap();
    let current = qualified(root.path(), &path, CaptureProfile::CodexCliJsonlV1);
    let replacement = root.path().join("replacement");
    fs::write(&replacement, header(false)).unwrap();
    fs::rename(&replacement, &path).unwrap();
    assert!(!current.is_current().unwrap());
    assert!(
        qualified(root.path(), &path, CaptureProfile::CodexCliJsonlV1)
            .is_current()
            .unwrap()
    );
    fs::remove_file(&path).unwrap();
    assert!(!current.is_current().unwrap());
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
}
