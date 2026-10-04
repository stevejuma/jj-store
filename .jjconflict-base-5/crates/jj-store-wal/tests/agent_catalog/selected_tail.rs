use super::*;

#[tokio::test]
async fn selected_cached_agent_reader_retains_audited_append_witnesses() {
    let coordinator = fixture().await;
    publish_single_capture(&coordinator, &codex_batch()).await;
    let selected = coordinator.read().await.unwrap();
    let snapshot = coordinator
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let sessions = snapshot.list_sessions(None, 10).await.unwrap();
    let AgentObjectV1::SessionSnapshot(session) = snapshot
        .read_object(sessions.selections[0].1)
        .await
        .unwrap()
    else {
        panic!("session")
    };
    let AgentObjectV1::TranscriptManifest(manifest) =
        snapshot.read_object(session.transcripts[0]).await.unwrap()
    else {
        panic!("transcript")
    };
    let root = manifest.streams[0].ordered_chunk_root;
    assert!(snapshot.transcript_proofs().boundary(root).is_some());
    let cached = snapshot.selection();
    drop(snapshot);
    let attached = coordinator.selected_agent_reader(&cached).unwrap();
    assert!(attached.transcript_proofs().boundary(root).is_some());
    assert_eq!(attached.object_count(), cached.object_count());
}
