//! Stock jj-lib validates full operation/view identities and complete source closure.
use super::*;
#[derive(Debug)]
pub struct JjEvidenceValidator;
impl jj_store_git::evidence::EvidenceValidator for JjEvidenceValidator {
    fn validate(
        &self,
        reference: &m::portable::CodeEvidenceRefV1,
        bytes: &[u8],
    ) -> Result<jj_store_git::evidence::EvidenceCapsule> {
        use jj_store_git::evidence::{EvidenceCapsule, EvidenceCommit};
        let value = GitEvidenceCapsuleV1::decode_reference(reference, bytes)?;
        Ok(EvidenceCapsule {
            profile: value.profile,
            repository: value.repository,
            operation_id: value.operation_id,
            view_id: value.view_id,
            commits: value
                .commits
                .into_iter()
                .map(|(id, c)| {
                    (
                        id,
                        EvidenceCommit {
                            change_id: c.change_id,
                            root_tree: c.root_tree,
                        },
                    )
                })
                .collect(),
            bytes: bytes.to_vec(),
        })
    }
}
