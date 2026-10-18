#![cfg(feature = "store-capture")]
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef, NativeCaptureBindingV1,
};
use jj_collab::{CollabObjectId, CollabObjectKind, CollabObjectRef, ThreadBindingV1, ThreadId};
use jj_store_conversations::{capture::CaptureError, store_capture::*};
use jj_store_model::{PrincipalId, PublicationId, RecordDigest, RepositoryId};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};

fn entry() -> EntryV1 {
    let context = ContextV1 {
        repository: RepositoryId::new([1; 16]),
        thread: ThreadId::new([2; 16]),
        binding: NativeCaptureBindingV1 {
            scope: "vault".into(),
            worker: "worker".into(),
            request: "open".into(),
            target: "target".into(),
            window: "main".into(),
            admission: "generation".into(),
        },
        policy: "fixture/v1".into(),
    };
    let capture=serde_json::to_vec(&json!({"schema":1,"policy":context.policy,"binding":context.binding,"ordinal":u64::MAX,"publication":{"kind":"event","sequence":u64::MAX},"frames":[{"version":3,"type":"record","sessionId":"worker","sequence":u64::MAX,"record":{"status":"completed"}}]})).unwrap();
    let candidate = CandidateV1 {
        schema: 1,
        context,
        ordinal: u64::MAX,
        capture_digest: Sha256::new_with_prefix(b"jj-store/native-capture/v1\0")
            .chain_update(&capture)
            .finalize()
            .into(),
        capture,
    };
    let actor = PrincipalId::new([3; 16]);
    let prepared = PreparedV1 {
        schema: 1,
        capture_digest: candidate.capture_digest,
        request_id: candidate.request(),
        publication_id: Some(PublicationId::new([4; 16])),
        command: PreparedCommandV1::Native {
            actor,
            delta: AgentDeltaV1 {
                schema_version: 1,
                edits: vec![AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Sessions,
                    key: AgentCatalogKey::derive("session", &[5; 16]).unwrap(),
                    expected: None,
                    replacement: Some(AgentObjectRef::new(
                        AgentObjectKind::SessionSnapshot,
                        AgentObjectId::new([6; 32]),
                    )),
                }],
            },
            result: vec![0, 128, 255],
            binding: ThreadBindingV1 {
                thread_id: candidate.context.thread,
                observed_thread: CollabObjectRef {
                    kind: CollabObjectKind::ThreadVersion,
                    id: CollabObjectId::new([7; 32]),
                },
                observed_assignment: None,
            },
            accepted_at_millis: i64::MAX,
        },
    };
    let confirmation = ConfirmationV1 {
        schema: 1,
        request_id: candidate.request(),
        actor,
        repository: candidate.context.repository,
        version: u64::MAX,
        selection: vec![255, 0],
        provider_receipt: vec![128, 1],
        root: RecordDigest::new([8; 32]),
        batch: None,
    };
    EntryV1 {
        schema: 1,
        candidate,
        prepared: Some(prepared),
        confirmation: Some(confirmation),
    }
}
#[test]
fn private_native_preparation_and_original_receipts_roundtrip_without_number_or_byte_changes() {
    let entry = entry();
    let encoded = entry.encode().unwrap();
    assert_eq!(EntryV1::decode(&encoded).unwrap(), entry);
    let mut git = entry.clone();
    let PreparedCommandV1::Native { binding, .. } = git.prepared.as_ref().unwrap().command.clone()
    else {
        unreachable!()
    };
    git.prepared.as_mut().unwrap().command = PreparedCommandV1::Capture { binding };
    git.prepared.as_mut().unwrap().publication_id = None;
    git.confirmation.as_mut().unwrap().batch = Some(git.candidate.batch_id());
    assert_eq!(EntryV1::decode(&git.encode().unwrap()).unwrap(), git);
}
#[test]
fn private_records_reject_unknown_fields_tampering_and_mismatched_preparation_or_receipts() {
    for mutate in [
        |v: &mut Value| {
            v["schema"] = json!(2);
        },
        |v: &mut Value| {
            v["unknown"] = json!(true);
        },
        |v: &mut Value| {
            v["candidate"]["context"]["command"] = json!("foreign");
        },
        |v: &mut Value| {
            v["candidate"]["capture_digest"][0] = json!(0);
        },
        |v: &mut Value| {
            v["candidate"]["ordinal"] = json!(1);
        },
        |v: &mut Value| {
            v["prepared"] = Value::Null;
        },
        |v: &mut Value| {
            v["prepared"]["capture_digest"][0] = json!(0);
        },
        |v: &mut Value| {
            v["prepared"]["command"]["accepted_at_millis"] = json!(-1);
        },
        |v: &mut Value| {
            v["prepared"]["command"]["binding"]["thread_id"] = json!(ThreadId::new([99; 16]));
        },
        |v: &mut Value| {
            v["confirmation"]["actor"] = json!(PrincipalId::new([99; 16]));
        },
        |v: &mut Value| {
            v["confirmation"]["repository"] = json!(RepositoryId::new([99; 16]));
        },
        |v: &mut Value| {
            v["confirmation"]["version"] = json!(0);
        },
        |v: &mut Value| {
            v["confirmation"]["provider_receipt"] = json!([]);
        },
    ] {
        let mut value = serde_json::to_value(entry()).unwrap();
        mutate(&mut value);
        assert!(
            EntryV1::decode(&serde_json::to_vec(&value).unwrap()).is_err(),
            "{value}"
        );
    }
}
#[test]
fn private_record_aggregate_and_provider_fields_are_bounded_before_host_io() {
    assert_eq!(
        EntryV1::decode(&vec![b' '; MAX_JOURNAL_BYTES + 1]).err(),
        Some(CaptureError::Limit)
    );
    let mut value = entry();
    value.confirmation.as_mut().unwrap().provider_receipt = vec![255; 2 * 1024 * 1024 + 1];
    assert!(value.encode().is_err());
    let mut value = entry();
    value.confirmation.as_mut().unwrap().provider_receipt = vec![255; 2 * 1024 * 1024];
    value.confirmation.as_mut().unwrap().selection = vec![255; 1024 * 1024];
    assert_eq!(value.encode().err(), Some(CaptureError::Limit));
}
