use super::*;
use jj_store_server::stack_command_journal::Error as J;
struct Denied;
#[async_trait::async_trait]
impl Authority for Denied {
    async fn authorize(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<()> {
        Err(stack_preparation::Error::Forbidden)
    }
    async fn context(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<Context> {
        panic!("denied commands cannot obtain live context")
    }
}
#[tokio::test]
async fn authorization_stale_physical_selection_and_conflicting_identity_do_not_prepare() {
    let f = Fixture::new().await;
    let state = f.witness().await;
    let req = request(&state);
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope(&state)).unwrap();
    let before = f.state();
    assert!(matches!(
        stack_preparation::prepare_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &Denied,
            actor(),
            req.clone()
        )
        .await,
        Err(stack_preparation::Error::Forbidden)
    ));
    assert!(journal.pending(actor()).unwrap().is_empty());
    assert_eq!(f.state(), before);
    let auth = Allowed {
        access: f.access(),
        root: f.root(),
        state,
    };
    let mut stale = req.clone();
    stale.selection.working_tree = m::RecordDigest::new([99; 32]);
    stale.intent_digest = stale.intent().unwrap();
    let stale_result = stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &auth,
        actor(),
        stale,
    )
    .await;
    assert!(
        matches!(stale_result, Err(stack_preparation::Error::Stale)),
        "{stale_result:?}"
    );
    assert!(journal.pending(actor()).unwrap().is_empty());
    assert_eq!(f.state(), before);
    let mut stale_combination = req.clone();
    stale_combination
        .selection
        .combination
        .as_mut()
        .unwrap()
        .generation += 1;
    stale_combination.intent_digest = stale_combination.intent().unwrap();
    assert!(matches!(
        stack_preparation::prepare_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &auth,
            actor(),
            stale_combination,
        )
        .await,
        Err(stack_preparation::Error::Stale)
    ));
    assert!(journal.pending(actor()).unwrap().is_empty());
    assert_eq!(f.state(), before);
    stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &auth,
        actor(),
        req.clone(),
    )
    .await
    .unwrap();
    let mut changed = req.clone();
    changed.command = s::Command::Reword {
        commit: s::CommitIdentity::GitSha1(auth.state.baseline.commit),
        message: "Substituted intent".into(),
    };
    changed.intent_digest = changed.intent().unwrap();
    assert!(matches!(
        stack_preparation::prepare_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &NoLiveContext,
            actor(),
            changed
        )
        .await,
        Err(stack_preparation::Error::Journal(J::IntentMismatch))
    ));
    assert!(matches!(
        stack_preparation::prepare_reword(
            &journal,
            &Coordinator::new(1).unwrap(),
            &NoLiveContext,
            m::PrincipalId::new([99; 16]),
            req
        )
        .await,
        Err(stack_preparation::Error::Journal(J::IntentMismatch))
    ));
    assert_eq!(f.state(), before);
}
#[tokio::test]
async fn local_execution_is_immutable_legacy_cannot_be_upgraded_and_terminal_replay_stays_terminal()
{
    let f = Fixture::new().await;
    let state = f.witness().await;
    let req = request(&state);
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope(&state)).unwrap();
    let auth = Allowed {
        access: f.access(),
        root: f.root(),
        state,
    };
    let saved = stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &auth,
        actor(),
        req.clone(),
    )
    .await
    .unwrap();
    let record = journal
        .status(actor(), req.request_id, req.intent_digest)
        .unwrap()
        .unwrap();
    let execution = record.execution.unwrap();
    let legacy_path = private.path().join("legacy");
    let legacy = StackCommandJournal::create(&legacy_path, scope(&auth.state)).unwrap();
    legacy.record_prepared(actor(), &saved.preview).unwrap();
    let bytes = std::fs::read(legacy_path.join("state.json")).unwrap();
    assert_eq!(
        legacy.record_local_prepared(actor(), &saved.preview, &execution),
        Err(J::IntentMismatch)
    );
    assert_eq!(
        std::fs::read(legacy_path.join("state.json")).unwrap(),
        bytes
    );
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["version"], 1);
    assert!(value["records"][0].get("execution").is_none());
    let mut tampered = execution.clone();
    tampered.archive = m::RecordDigest::new([98; 32]);
    assert_eq!(
        journal.record_local_prepared(actor(), &saved.preview, &tampered),
        Err(J::IntentMismatch)
    );
    let apply = s::ApplyRequest {
        version: 1,
        request_id: req.request_id,
        intent_digest: req.intent_digest,
        prepared: saved.preview.prepared,
        selection: req.selection.clone(),
    };
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let terminal = s::ReceiptState::Retired {
        operation: saved.preview.plan.operation,
        reason: s::Retirement::Abandoned,
    };
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::PublicationUnknown,
            &terminal,
        )
        .unwrap();
    let replay = stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &NoLiveContext,
        actor(),
        req,
    )
    .await
    .unwrap();
    assert_eq!(replay.receipt.state, terminal);
    assert!(!replay.receipt.may_execute());
}
