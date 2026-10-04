use jj_store_model::{PrincipalId, RequestId};
use jj_store_server::{
    capture_import_state::{CaptureImportIntent, CaptureImportJournal},
    opencode_import::OpenCodeImportProbe,
};
#[path = "opencode_source/fixture.rs"]
#[allow(dead_code)]
mod fixture;
use fixture::{Api, message};

async fn prepared(probe: &mut OpenCodeImportProbe) {
    for _ in 0..100 {
        if probe.step().await.unwrap() {
            return;
        }
    }
    panic!("bounded fixture never settled");
}
#[tokio::test]
async fn idle_api_history_selects_two_verified_prefix_passes_and_preserves_exact_intent() {
    let api = Api::new(
        "/repo".into(),
        (0..5)
            .map(|n| message(&format!("msg_{n}"), Some("/repo"), "native history"))
            .collect(),
        false,
    )
    .await;
    let mut probe = OpenCodeImportProbe::select(api.api.clone(), "ses_first")
        .await
        .unwrap();
    assert!(probe.boundary().is_err());
    for _ in 0..3 {
        assert!(!probe.step().await.unwrap());
        assert!(probe.boundary().is_err());
    }
    prepared(&mut probe).await;
    let accepted = probe.boundary().unwrap();
    assert_eq!(accepted.complete_records, 5);
    assert_eq!(accepted.complete_byte_offset, 0);
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("imports");
    let journal = CaptureImportJournal::create(&path, [1; 16], [2; 16]).unwrap();
    let intent = CaptureImportIntent {
        version: 1,
        generation: [2; 16],
        principal: PrincipalId::new([3; 16]),
        request: RequestId::new([4; 16]),
        expected_revision: 0,
        selector: format!("{}:0000000000000001", "05".repeat(16)),
    };
    let receipt = journal.record(&intent, &accepted).unwrap();
    let original = std::fs::read(path.join("state.json")).unwrap();
    api.data
        .lock()
        .unwrap()
        .messages
        .push(message("msg_tail", Some("/repo"), "continuing tail"));
    let mut replay = OpenCodeImportProbe::reconcile(api.api.clone(), "ses_first", accepted.clone())
        .await
        .unwrap();
    prepared(&mut replay).await;
    assert_eq!(replay.boundary().unwrap(), accepted);
    let reopened = CaptureImportJournal::open(&path, [1; 16], [2; 16]).unwrap();
    assert_eq!(
        reopened.resolve(&intent).unwrap(),
        Some(receipt.receipt.clone())
    );
    assert!(reopened.record(&intent, &accepted).unwrap().replayed);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), original);
}
#[tokio::test]
async fn selected_api_prefix_refuses_rewrite_truncation_directory_and_endpoint_rebinding() {
    let api = Api::new(
        "/repo".into(),
        vec![message("msg_first", Some("/repo"), "original")],
        false,
    )
    .await;
    let mut probe = OpenCodeImportProbe::select(api.api.clone(), "ses_first")
        .await
        .unwrap();
    prepared(&mut probe).await;
    let accepted = probe.boundary().unwrap();
    api.data.lock().unwrap().messages[0] = message("msg_first", Some("/repo"), "rewritten");
    let mut changed =
        OpenCodeImportProbe::reconcile(api.api.clone(), "ses_first", accepted.clone())
            .await
            .unwrap();
    assert!(changed.step().await.is_err());
    assert!(changed.boundary().is_err());
    api.data.lock().unwrap().messages.clear();
    let mut short = OpenCodeImportProbe::reconcile(api.api.clone(), "ses_first", accepted.clone())
        .await
        .unwrap();
    assert!(short.step().await.is_err());
    api.data.lock().unwrap().directory = "/rebound".into();
    assert!(
        OpenCodeImportProbe::reconcile(api.api.clone(), "ses_first", accepted.clone())
            .await
            .is_err()
    );
    let other = Api::new("/repo".into(), vec![], false).await;
    assert!(
        OpenCodeImportProbe::reconcile(other.api.clone(), "ses_first", accepted)
            .await
            .is_err()
    );
}
#[tokio::test]
async fn empty_api_history_has_a_qualified_zero_count_and_refuses_fake_byte_positions() {
    let api = Api::new("/repo".into(), vec![], false).await;
    let mut probe = OpenCodeImportProbe::select(api.api.clone(), "ses_first")
        .await
        .unwrap();
    prepared(&mut probe).await;
    let accepted = probe.boundary().unwrap();
    assert_eq!(accepted.complete_records, 0);
    assert_eq!(accepted.complete_byte_offset, 0);
    let root = tempfile::tempdir().unwrap();
    let journal =
        CaptureImportJournal::create(&root.path().join("imports"), [1; 16], [2; 16]).unwrap();
    let intent = CaptureImportIntent {
        version: 1,
        generation: [2; 16],
        principal: PrincipalId::new([3; 16]),
        request: RequestId::new([4; 16]),
        expected_revision: 0,
        selector: format!("{}:0000000000000001", "05".repeat(16)),
    };
    let mut fake = accepted.clone();
    fake.complete_byte_offset = 1;
    assert!(journal.record(&intent, &fake).is_err());
    fake = accepted.clone();
    fake.prefix_digest = [9; 32];
    assert!(journal.record(&intent, &fake).is_err());
    fake = accepted.clone();
    fake.complete_records = 131073;
    assert!(journal.record(&intent, &fake).is_err());
    assert!(journal.entries().unwrap().is_empty());
    assert!(journal.record(&intent, &accepted).is_ok());
}

#[tokio::test]
async fn selection_verifies_again_and_never_exposes_a_boundary_after_prefix_change() {
    let api = Api::new(
        "/repo".into(),
        vec![message("msg_first", Some("/repo"), "first pass")],
        false,
    )
    .await;
    let mut probe = OpenCodeImportProbe::select(api.api.clone(), "ses_first")
        .await
        .unwrap();
    assert!(!probe.step().await.unwrap());
    assert!(probe.boundary().is_err());
    api.data.lock().unwrap().messages[0] = message("msg_first", Some("/repo"), "second pass");
    assert!(probe.step().await.is_err());
    api.data.lock().unwrap().messages[0] = message("msg_first", Some("/repo"), "first pass");
    assert!(probe.step().await.is_err());
    assert!(probe.boundary().is_err());
}
