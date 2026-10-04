use super::*;
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef,
};
use jj_store_service::collab_model::{
    CollabObjectId, CollabObjectKind, CollabObjectRef, ThreadBindingV1, ThreadId,
};
#[test]
fn delta_codec_retains_explicit_binding_and_unlinked_prepare_rejects_it() {
    let request = AgentDeltaRequest::new(
        m::PrincipalId::new([1; 16]),
        m::RequestId::new([2; 16]),
        m::PublicationId::new([3; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key: AgentCatalogKey::derive("session", &[4; 16]).unwrap(),
                expected: None,
                replacement: Some(AgentObjectRef::new(
                    AgentObjectKind::SessionSnapshot,
                    AgentObjectId::new([5; 32]),
                )),
            }],
        },
        vec![6],
    )
    .unwrap()
    .with_thread_binding(
        ThreadBindingV1 {
            thread_id: ThreadId::new([7; 16]),
            observed_thread: CollabObjectRef {
                kind: CollabObjectKind::ThreadVersion,
                id: CollabObjectId::new([8; 32]),
            },
            observed_assignment: None,
        },
        123,
    )
    .unwrap();
    let bytes = encode(&request, DELTA_BYTES).unwrap();
    assert_eq!(delta_request(&bytes).unwrap(), request);
    let binding = &request.thread_binding.as_ref().unwrap().binding;
    assert_eq!(
        prepared(
            wire_prepared(&request).unwrap(),
            request.request_id,
            request.publication_id,
            &request.result,
            Some(binding)
        )
        .unwrap(),
        request
    );
    let mut unexpected = binding.clone();
    unexpected.thread_id = ThreadId::new([9; 16]);
    assert_eq!(
        prepared(
            wire_prepared(&request).unwrap(),
            request.request_id,
            request.publication_id,
            &request.result,
            Some(&unexpected)
        )
        .unwrap_err()
        .code(),
        tonic::Code::DataLoss
    );
    assert_eq!(
        prepared(
            wire_prepared(&request).unwrap(),
            request.request_id,
            request.publication_id,
            &request.result,
            None
        )
        .unwrap_err()
        .code(),
        tonic::Code::DataLoss
    );
    let mut invalid = request.clone();
    invalid.thread_binding.as_mut().unwrap().accepted_at_millis = -1;
    assert_eq!(
        delta_request(&encode(&invalid, DELTA_BYTES).unwrap())
            .unwrap_err()
            .code(),
        tonic::Code::InvalidArgument
    );
    let mut unlinked = request;
    unlinked.thread_binding = None;
    assert_eq!(
        prepared(
            wire_prepared(&unlinked).unwrap(),
            unlinked.request_id,
            unlinked.publication_id,
            &unlinked.result,
            Some(&unexpected)
        )
        .unwrap_err()
        .code(),
        tonic::Code::DataLoss
    );
    assert_eq!(
        prepared(
            wire_prepared(&unlinked).unwrap(),
            unlinked.request_id,
            unlinked.publication_id,
            &unlinked.result,
            None
        )
        .unwrap(),
        unlinked
    );
}
