use super::*;
use jj_store_capture::{codex_transcript::TranscriptPoll, reader_progress::ReaderSourceProgress};
use std::sync::Arc;

#[test]
fn aliases_restart_and_capacity_use_existing_identity_and_exclusive_leases() {
    let fixture = Fixture::new(1);
    let (probe, binding) = fixture.prepared();
    let source = fixture
        .registry
        .acquire_import(&probe, binding, || Ok(()))
        .unwrap()
        .source_instance_id();
    let alias = fixture.transcripts.join("alias.jsonl");
    fs::hard_link(&fixture.path, &alias).unwrap();
    let alias_source = qualify(&alias, CaptureProfile::CodexCliJsonlV1);
    let reopened = SourceRegistry::open(&fixture.state, [8; 16], 1).unwrap();
    let mut alias_probe = CodexImportProbe::reconcile(alias_source, binding).unwrap();
    assert_eq!(ready(&mut alias_probe), binding);
    let lease = reopened
        .acquire_import(&alias_probe, binding, || Ok(()))
        .unwrap();
    assert_eq!(lease.source_instance_id(), source);
    let before = fixture.catalog();
    assert!(
        fixture
            .registry
            .acquire_import(&probe, binding, || Ok(()))
            .is_err()
    );
    assert_eq!(fixture.catalog(), before);
    drop(lease);
    let other = fixture.transcripts.join("other.jsonl");
    write(&other, CaptureProfile::CodexCliJsonlV1);
    let mut preparation =
        CodexImportProbe::select(qualify(&other, CaptureProfile::CodexCliJsonlV1)).unwrap();
    let other_binding = ready(&mut preparation);
    assert!(
        reopened
            .acquire_import(&preparation, other_binding, || Ok(()))
            .is_err()
    );
    assert_eq!(fixture.catalog(), before);
    assert!(
        reopened
            .lookup_import_admitted(&preparation, other_binding, || Ok(()))
            .unwrap()
            .is_none()
    );
    assert_eq!(
        reopened
            .acquire_import(&probe, binding, || Ok(()))
            .unwrap()
            .source_instance_id(),
        source
    );
}

#[test]
fn imported_raw_frame_reserves_and_resumes_common_channel_without_reallocation() {
    let fixture = Fixture::new(4);
    let (probe, binding) = fixture.prepared();
    let mut reader = fixture
        .registry
        .acquire_import(&probe, binding, || Ok(()))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let TranscriptPoll::Ready(original) = reader.poll().unwrap() else {
        panic!("expected initial history")
    };
    let before = fixture.catalog();
    let mut parked = reader.park_automatic();
    assert!(parked.has_channel_reservation());
    append(&fixture.path);
    assert!(
        fixture
            .registry
            .acquire_import(&probe, binding, || Ok(()))
            .is_err()
    );
    assert!(
        parked
            .acquire_reserved(&fixture.registry)
            .unwrap()
            .is_none()
    );
    assert!(
        parked
            .acquire_reserved_import(&fixture.registry, &probe, binding, || Ok(()))
            .is_err()
    );
    let mut reconciled = CodexImportProbe::reconcile(fixture.qualified(), binding).unwrap();
    assert_eq!(ready(&mut reconciled), binding);
    assert!(
        parked
            .acquire_reserved_import(&fixture.registry, &reconciled, binding, || Err(
                "consent revoked".into()
            ))
            .is_err()
    );
    assert!(parked.has_channel_reservation());
    let reserved = parked
        .acquire_reserved_import(&fixture.registry, &reconciled, binding, || Ok(()))
        .unwrap()
        .unwrap();
    let bounds = ReaderSourceProgress {
        progress: None,
        generation_floor: 0,
        sequence_floor: 0,
    };
    let mut resumed = parked
        .resume_reserved_at(reserved, &bounds, false)
        .unwrap()
        .unwrap();
    let TranscriptPoll::Ready(retained) = resumed.poll().unwrap() else {
        panic!("expected retained history")
    };
    assert_eq!(original.batch_id, retained.batch_id);
    assert_eq!(original.cursor, retained.cursor);
    assert!(Arc::ptr_eq(&original.records, &retained.records));
    assert_eq!(fixture.catalog(), before);
    drop(resumed);
    assert!(
        fixture
            .registry
            .acquire_import(&reconciled, binding, || Ok(()))
            .is_ok()
    );
}
