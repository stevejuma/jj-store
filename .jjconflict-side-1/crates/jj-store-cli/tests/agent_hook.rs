use jj_agent::{IntegrationId, SourcePositionV1};
use jj_store_cli::{
    agent_capture::AgentCaptureSpool,
    agent_commands::AgentCommand,
    agent_hook::ingest_hook,
    agent_integrations::AgentIntegrationRegistry,
    agent_sanitization::{KINGFISHER_POLICY, warm_kingfisher_cache},
    agent_setup::{SetupRequestV1, setup},
};
use jj_store_model::RepositoryId;
use std::{fs, path::Path, thread};

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: jj_store_cli::commands::Extension,
}

const SECRET: &str = "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";

#[cfg(unix)]
fn executable(path: &Path, version: &str) {
    use std::os::unix::fs::PermissionsExt as _;
    fs::write(path, format!("#!/bin/sh\nprintf '%s\\n' '{version}'\n")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
}

fn durable_bytes(path: &Path) -> Vec<u8> {
    let mut bytes = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            bytes.extend(durable_bytes(&path));
        } else {
            bytes.extend(fs::read(path).unwrap());
        }
    }
    bytes
}

#[test]
fn hook_command_selects_one_supported_integration() {
    use clap::Parser as _;
    let parsed = Cli::try_parse_from(["jj", "agent", "hook", "codex"]).unwrap();
    let jj_store_cli::commands::Extension::Agent(args) = parsed.command else {
        panic!("expected agent extension");
    };
    assert!(matches!(
        args.command,
        Some(AgentCommand::Hook {
            transcript_capture: false,
            agent: jj_store_cli::agent_commands::AgentName::Codex
        })
    ));
}

#[test]
fn hook_rejects_disabled_integrations_before_capture_state_exists() {
    let temporary = tempfile::tempdir().unwrap();
    let workspace = temporary.path().join("workspace");
    let state = temporary.path().join("state");
    fs::create_dir_all(&workspace).unwrap();
    let repository = RepositoryId::new([42; 16]);
    drop(AgentIntegrationRegistry::open(&state, &workspace, repository).unwrap());

    assert!(
        ingest_hook(
            b"not-json",
            &workspace,
            &state,
            repository,
            IntegrationId::Codex,
        )
        .is_err()
    );
    assert!(!state.join("capture").exists());
    assert!(!state.join("kingfisher-cache").exists());
}

#[cfg(unix)]
#[test]
fn hook_ingestion_survives_restart_orders_sequences_and_never_persists_raw_input() {
    let temporary = tempfile::tempdir().unwrap();
    let workspace = temporary.path().join("workspace");
    let state = temporary.path().join("state");
    let bin = temporary.path().join("bin");
    fs::create_dir_all(workspace.join(".codex")).unwrap();
    fs::create_dir_all(&bin).unwrap();
    executable(&bin.join("codex"), "codex-cli 0.156.1");
    let repository = RepositoryId::new([41; 16]);

    let mut registry = AgentIntegrationRegistry::open(&state, &workspace, repository).unwrap();
    registry.refresh(std::slice::from_ref(&bin)).unwrap();
    registry.enable(&[IntegrationId::Codex], false).unwrap();
    setup(
        &mut registry,
        SetupRequestV1 {
            integration: IntegrationId::Codex,
            executable: Path::new("/opt/jj-store/bin/jj").into(),
            dry_run: false,
        },
    )
    .unwrap();
    drop(registry);
    warm_kingfisher_cache(&state.join("kingfisher-cache")).unwrap();

    let raw = serde_json::to_vec(&serde_json::json!({
        "hook_event_name": "SessionStart",
        "session_id": "session-1",
        "message": format!("token={SECRET}"),
        "cwd": workspace,
    }))
    .unwrap();
    let first = ingest_hook(&raw, &workspace, &state, repository, IntegrationId::Codex).unwrap();
    let second = ingest_hook(&raw, &workspace, &state, repository, IntegrationId::Codex).unwrap();
    let omitted = ingest_hook(
        b"not-json",
        &workspace,
        &state,
        repository,
        IntegrationId::Codex,
    )
    .unwrap();
    assert_eq!(
        (first.sequence, second.sequence, omitted.sequence),
        (1, 2, 3)
    );
    assert!(!first.omitted && !second.omitted && omitted.omitted);

    let mut concurrent = thread::scope(|scope| {
        (0..8)
            .map(|_| {
                scope.spawn(|| {
                    ingest_hook(
                        b"not-json",
                        &workspace,
                        &state,
                        repository,
                        IntegrationId::Codex,
                    )
                    .unwrap()
                    .sequence
                })
            })
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>()
    });
    concurrent.sort_unstable();
    assert_eq!(concurrent, (4..=11).collect::<Vec<_>>());

    let spool_path = state.join("capture/spool");
    let spool = AgentCaptureSpool::open(&spool_path, 64 * 1024 * 1024).unwrap();
    let mut pending = spool.pending().unwrap();
    pending.sort_by_key(|entry| entry.batch.observations[0].source_sequence);
    assert_eq!(pending.len(), 11);
    let source = pending[0].batch.source_instance_id;
    for (index, entry) in pending.iter().enumerate() {
        let sequence = (index + 1) as u64;
        assert_eq!(entry.batch.source_instance_id, source);
        assert_eq!(entry.batch.sanitizer_version, KINGFISHER_POLICY);
        assert_eq!(entry.batch.observations[0].source_sequence, sequence);
        assert!(entry.batch.observations[0].occurred_at_millis.is_some());
        assert_eq!(
            entry.cursor.position,
            SourcePositionV1::NativeSequence { sequence }
        );
    }
    assert_eq!(
        pending[2].batch.observations[0].payload.bytes(),
        br#"{"status":"omitted"}"#
    );
    drop(spool);

    let durable = durable_bytes(&state);
    assert!(
        !durable
            .windows(SECRET.len())
            .any(|value| value == SECRET.as_bytes())
    );
}
