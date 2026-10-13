use jj_store_model as m;
use jj_workflow as w;
const OWNER: m::PrincipalId = m::PrincipalId::new([1; 16]);
const HOST: m::PrincipalId = m::PrincipalId::new([2; 16]);
fn source() -> w::Source {
    w::Source {
        repository: m::RepositoryId::new([3; 16]),
        operation: Some(m::OperationId::new([4; 64])),
        commit: Some(m::CommitId::new([5; 64])),
        bookmark: None,
        conversation: None,
    }
}
fn definition() -> w::Definition {
    w::Definition {
        name: "Build".into(),
        description: String::new(),
        enabled: false,
        repository: source().repository,
        trigger: w::Trigger::CodePublished,
        labels: Default::default(),
        conditions: Default::default(),
        actions: vec![w::Action {
            id: "build".into(),
            executor: w::Executor::Command {
                name: "build".into(),
            },
            inputs: Default::default(),
            timeout_seconds: 30,
        }],
    }
}
fn apply(
    state: &mut w::State,
    actor: m::PrincipalId,
    byte: u8,
    command: w::Command,
) -> w::CommandResult {
    let (next, result) = state
        .apply(
            &command,
            actor,
            m::RequestId::new([byte; 16]),
            i64::from(byte),
        )
        .unwrap();
    *state = next;
    result
}
fn setup() -> (w::State, w::Run) {
    let mut state = w::State::default();
    let workflow = w::WorkflowId::new([6; 16]);
    let version = apply(
        &mut state,
        OWNER,
        10,
        w::Command::Put {
            workflow,
            expected: None,
            definition: definition(),
        },
    )
    .definition
    .unwrap()
    .version;
    let run = apply(
        &mut state,
        OWNER,
        11,
        w::Command::Run {
            workflow,
            expected: version,
            source: source(),
        },
    )
    .runs
    .remove(0);
    (state, run)
}
#[test]
fn no_match_is_frozen_and_a_new_event_pins_the_new_definition() {
    let mut state = w::State::default();
    let workflow = w::WorkflowId::new([6; 16]);
    let original = apply(
        &mut state,
        OWNER,
        10,
        w::Command::Put {
            workflow,
            expected: None,
            definition: definition(),
        },
    )
    .definition
    .unwrap();
    let event =
        w::SourceEvent::new("origin/first".into(), w::EventKind::CodePublished, source()).unwrap();
    assert!(
        apply(
            &mut state,
            HOST,
            11,
            w::Command::Evaluate {
                definitions: None,
                event: event.clone()
            }
        )
        .runs
        .is_empty()
    );
    let mut enabled = definition();
    enabled.enabled = true;
    let next = apply(
        &mut state,
        OWNER,
        12,
        w::Command::Put {
            workflow,
            expected: Some(original.version),
            definition: enabled,
        },
    )
    .definition
    .unwrap();
    assert!(
        apply(
            &mut state,
            HOST,
            13,
            w::Command::Evaluate {
                definitions: None,
                event
            }
        )
        .runs
        .is_empty()
    );
    let event = w::SourceEvent::new(
        "origin/second".into(),
        w::EventKind::CodePublished,
        source(),
    )
    .unwrap();
    let result = apply(
        &mut state,
        HOST,
        14,
        w::Command::Evaluate {
            definitions: None,
            event,
        },
    );
    assert_eq!(result.runs.len(), 1);
    assert_eq!(result.runs[0].version, next.version);
}
#[test]
fn claims_reports_and_sequences_remain_bound_to_one_attempt() {
    let (mut state, run) = setup();
    let attempt = run.current().id;
    apply(
        &mut state,
        HOST,
        12,
        w::Command::Claim {
            run: run.id,
            attempt,
            host: "host-a".into(),
        },
    );
    let report = w::Command::Report {
        run: run.id,
        attempt,
        sequence: 0,
        data: w::EvidenceData::Log {
            stream: "stdout".into(),
            text: "first\n".into(),
        },
    };
    assert!(matches!(
        state.apply(&report, OWNER, m::RequestId::new([13; 16]), 13),
        Err(w::Error::Denied)
    ));
    apply(&mut state, HOST, 14, report.clone());
    assert!(matches!(
        state.apply(&report, HOST, m::RequestId::new([15; 16]), 15),
        Err(w::Error::Stale)
    ));
    assert!(
        state
            .apply(
                &w::Command::Claim {
                    run: run.id,
                    attempt,
                    host: "host-b".into()
                },
                OWNER,
                m::RequestId::new([16; 16]),
                16
            )
            .is_err()
    );
    assert_eq!(state.evidence.len(), 1);
}
#[test]
fn unknown_never_becomes_available_for_automatic_reassignment() {
    let (mut state, run) = setup();
    let attempt = run.current().id;
    apply(
        &mut state,
        HOST,
        12,
        w::Command::Claim {
            run: run.id,
            attempt,
            host: "host-a".into(),
        },
    );
    apply(
        &mut state,
        HOST,
        13,
        w::Command::Interrupt {
            run: run.id,
            attempt,
        },
    );
    assert_eq!(state.runs[&run.id].current().status, w::RunStatus::Unknown);
    assert!(
        state
            .apply(
                &w::Command::Retry {
                    run: run.id,
                    expected: attempt
                },
                OWNER,
                m::RequestId::new([14; 16]),
                14
            )
            .is_err()
    );
    assert!(
        state
            .apply(
                &w::Command::Claim {
                    run: run.id,
                    attempt,
                    host: "host-b".into()
                },
                OWNER,
                m::RequestId::new([15; 16]),
                15
            )
            .is_err()
    );
}
#[test]
fn cancellation_requires_confirmation_and_retry_retains_original_pins() {
    let (mut state, run) = setup();
    let attempt = run.current().id;
    apply(
        &mut state,
        HOST,
        12,
        w::Command::Claim {
            run: run.id,
            attempt,
            host: "host-a".into(),
        },
    );
    apply(&mut state, OWNER, 13, w::Command::Cancel { run: run.id });
    assert_eq!(
        state.runs[&run.id].current().status,
        w::RunStatus::CancelRequested
    );
    assert!(
        state
            .apply(
                &w::Command::Finish {
                    run: run.id,
                    attempt,
                    status: w::RunStatus::Completed,
                    result: String::new()
                },
                HOST,
                m::RequestId::new([14; 16]),
                14
            )
            .is_err()
    );
    apply(
        &mut state,
        HOST,
        15,
        w::Command::Finish {
            run: run.id,
            attempt,
            status: w::RunStatus::Cancelled,
            result: "Stopped".into(),
        },
    );
    let retried = apply(
        &mut state,
        OWNER,
        16,
        w::Command::Retry {
            run: run.id,
            expected: attempt,
        },
    )
    .runs
    .remove(0);
    assert_ne!(retried.current().id, attempt);
    assert_eq!(retried.source, run.source);
    assert_eq!(retried.version, run.version);
    assert!(retried.current().owner.is_none());
}
#[test]
fn removal_retains_history_and_the_original_definition_owner() {
    let (mut state, run) = setup();
    apply(
        &mut state,
        OWNER,
        12,
        w::Command::Remove {
            workflow: run.workflow,
            expected: run.version,
        },
    );
    assert!(state.workflows.is_empty());
    assert_eq!(state.runs.len(), 1);
    assert_eq!(state.definitions.len(), 1);
    assert!(matches!(
        state.apply(
            &w::Command::Put {
                workflow: run.workflow,
                expected: None,
                definition: definition()
            },
            HOST,
            m::RequestId::new([13; 16]),
            13
        ),
        Err(w::Error::Denied)
    ));
}
#[test]
fn yaml_and_artifact_profiles_are_bounded_and_strict() {
    let mut d = definition();
    d.actions.push(d.actions[0].clone());
    assert!(d.validate().is_err());
    let mut yaml = serde_yaml::to_string(&definition()).unwrap();
    yaml += "executable: /bin/sh\n";
    assert!(w::Definition::parse_yaml(&yaml).is_err());
    assert!(
        w::EvidenceData::Artifact {
            name: "../outside".into(),
            media_type: "text/plain".into(),
            bytes: vec![]
        }
        .validate()
        .is_err()
    );
    assert!(
        w::EvidenceData::Artifact {
            name: "report".into(),
            media_type: "text/plain".into(),
            bytes: vec![0; w::MAX_EVIDENCE + 1]
        }
        .validate()
        .is_err()
    );
}
