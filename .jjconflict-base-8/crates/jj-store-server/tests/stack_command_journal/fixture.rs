use jj_store_model as m;
use jj_store_server::stack_command_journal::{Profile, Scope};
use jj_store_service::stack_workspace as s;

pub fn selection() -> s::Selection {
    s::Selection {
        version: 1,
        repository: m::RepositoryId::new([1; 16]),
        binding_generation: 3,
        physical_workspace: s::PhysicalWorkspaceId(m::RecordDigest::new([2; 32])),
        jj_workspace: "default".into(),
        code: s::CodeSelection::GitSha1Jj045 {
            operation: m::OperationId::new([3; 64]),
            view: m::ViewId::new([4; 64]),
            working_copy: m::GitSha1Id::new([5; 20]),
        },
        combination: Some(s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([6; 16])),
            generation: 7,
        }),
        working_tree: m::RecordDigest::new([8; 32]),
    }
}
pub fn scope() -> Scope {
    let value = selection();
    Scope {
        installation: [0xab; 16],
        repository: value.repository,
        physical_workspace: value.physical_workspace,
        jj_workspace: value.jj_workspace,
        profile: Profile::GitSha1Jj045,
    }
}
pub fn preview(id: u8) -> s::Preview {
    let plan = s::PreparedPlan {
        version: 1,
        request: s::PrepareRequest::new(
            m::RequestId::new([id; 16]),
            selection(),
            s::Command::Reword {
                commit: s::CommitIdentity::GitSha1(m::GitSha1Id::new([10; 20])),
                message: "Exact retained command".into(),
            },
        )
        .unwrap(),
        operation: m::OperationId::new([30; 64]),
        view: m::ViewId::new([31; 64]),
        checkout: s::CheckoutEffect {
            before: s::TreeIdentity::GitSha1(m::GitSha1Id::new([32; 20])).into(),
            after: s::TreeIdentity::GitSha1(m::GitSha1Id::new([33; 20])).into(),
            paths: vec!["src/one".into()],
        },
        combination: s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([6; 16])),
            generation: 8,
        },
        rewrites: vec![],
        bookmarks: vec![],
        conflicts: vec![],
    };
    s::Preview {
        prepared: plan.identity().unwrap(),
        plan,
    }
}
pub fn actor() -> m::PrincipalId {
    m::PrincipalId::new([9; 16])
}
pub fn apply(preview: &s::Preview) -> s::ApplyRequest {
    let request = &preview.plan.request;
    s::ApplyRequest {
        version: 1,
        request_id: request.request_id,
        intent_digest: request.intent_digest,
        prepared: preview.prepared,
        selection: request.selection.clone(),
    }
}
pub fn published(preview: &s::Preview, checkout: s::CheckoutState) -> s::ReceiptState {
    s::ReceiptState::Published {
        operation: preview.plan.operation,
        view: preview.plan.view,
        checkout,
    }
}
