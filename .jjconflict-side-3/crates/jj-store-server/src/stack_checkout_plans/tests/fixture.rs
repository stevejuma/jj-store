//! Synthetic canonical framing only; real engine linkage is tested in CLI.
use super::*;
pub fn scope() -> journal::Scope {
    journal::Scope {
        installation: [1; 16],
        repository: m::RepositoryId::new([1; 16]),
        physical_workspace: jj_store_service::stack_workspace::PhysicalWorkspaceId(
            m::RecordDigest::new([2; 32]),
        ),
        jj_workspace: "default".into(),
        profile: journal::Profile::GitSha1Jj045,
    }
}
pub fn plan() -> TreeCheckoutPlanV1 {
    let mut checkout = vec![18, 64];
    checkout.extend([3; 64]);
    checkout.extend([26, 7]);
    checkout.extend(b"default");
    let mut tree = vec![42, 20];
    tree.extend([32; 20]);
    let plan: TreeCheckoutPlanV1 = serde_json::from_value(serde_json::json!({
        "version":1, "key": {"repository":scope().repository,"physical_workspace":scope().physical_workspace.0,
            "principal":m::PrincipalId::new([9;16]),"request":m::RequestId::new([11;16]),
            "intent":m::IntentDigest::new([12;32]),"prepared":m::RecordDigest::new([13;32])},
        "recovery":m::RecordDigest::new([14;32]), "retained":m::GitSha1Id::new([15;20]),
        "archive":m::RecordDigest::new([16;32]), "operation":m::OperationId::new([30;64]),"view":m::ViewId::new([31;64]),
        "physical":m::RecordDigest::new([8;32]), "before_commit":m::GitSha1Id::new([5;20]),"after_commit":m::GitSha1Id::new([6;20]),
        "before_tree":m::GitSha1Id::new([32;20]),"after_tree":m::GitSha1Id::new([33;20]),
        "checkout":checkout, "checkout_fingerprint":m::RecordDigest::new([17;32]),
        "tree_state":tree,"tree_fingerprint":m::RecordDigest::new([18;32]),
        "changes":[{"path":"file", "before":null,"observed":null,"after":{"object":m::GitSha1Id::new([19;20]),
            "content":m::RecordDigest::new([20;32]),"size":2,"executable":false}}],
        "directories":[{"path":"","identity":m::RecordDigest::new([21;32])}]
    })).unwrap();
    plan.encode().unwrap();
    plan
}
