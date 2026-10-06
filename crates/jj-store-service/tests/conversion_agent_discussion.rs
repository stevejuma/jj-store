use jj_collab::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use jj_store_service::conversion::{CollabGraphArchiveV1, transfer_collab};
use jj_store_wal::publication::{
    AgentEventId, AgentObjectId, AgentObjectKind, AgentObjectRef, AgentSessionId, MessageId,
};
use std::collections::BTreeMap;

#[test]
fn conversion_preserves_agent_discussion_identity_and_requires_exact_witness_mapping() {
    let batch = AgentObjectRef {
        kind: AgentObjectKind::EventBatch,
        id: AgentObjectId::new([1; 32]),
    };
    let content = AgentObjectRef {
        kind: AgentObjectKind::Content,
        id: AgentObjectId::new([2; 32]),
    };
    let next_batch = AgentObjectRef {
        id: AgentObjectId::new([3; 32]),
        ..batch
    };
    let next_content = AgentObjectRef {
        id: AgentObjectId::new([4; 32]),
        ..content
    };
    let session_id = AgentSessionId::new([5; 16]);
    let message_id = MessageId::new([6; 16]);
    let event_id = AgentEventId::new([7; 16]);
    let thread_id = ThreadId::new([8; 16]);
    let mut graph = CollabGraphV1::default();
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::CreateAgentDiscussion {
                thread_id,
                title: "Exact message feedback".into(),
                target: ReactionTargetV1::AgentMessage {
                    session_id,
                    message_id,
                    event_id,
                    batch,
                    content,
                },
            },
            PrincipalId::new([9; 16]),
            RequestId::new([10; 16]),
            11,
            &mut graph,
        )
        .unwrap();
    let archive = CollabGraphArchiveV1 {
        thread: Some(root),
        reaction: None,
        objects: graph.objects.into_iter().collect(),
    };
    let mappings = BTreeMap::from([(batch, next_batch), (content, next_content)]);
    let (converted, graph, remapped) = transfer_collab(
        &archive,
        RepositoryId::new([12; 16]),
        &mappings,
        &BTreeMap::new(),
    )
    .unwrap();
    let state = ThreadStateV1::read(converted.thread.unwrap(), &graph).unwrap();
    assert_eq!(state.threads[&thread_id], remapped[&created.version]);
    let CollabObjectV1::ThreadVersion(version) = graph.get(remapped[&created.version]).unwrap()
    else {
        panic!("expected a converted Thread version")
    };
    assert_eq!(version.thread_id, thread_id);
    assert_eq!(
        version.agent_message,
        Some(ReactionTargetV1::AgentMessage {
            session_id,
            message_id,
            event_id,
            batch: next_batch,
            content: next_content,
        })
    );
    // Conversion must not leave an unverified original repository witness behind.
    assert!(
        transfer_collab(
            &archive,
            RepositoryId::new([12; 16]),
            &BTreeMap::from([(batch, next_batch)]),
            &BTreeMap::new(),
        )
        .is_err()
    );
}
