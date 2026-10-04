use crate::{
    AgentPlacementTargetV1, AgentTextAnchorV1, AnchorId, PlacementCandidateV1, PlacementEvidenceV1,
    PlacementKindV1, PlacementResultV1, ResolverBudgetV1, Result, RetainedTextV1, result,
    unavailable_evidence,
};

pub(crate) fn resolve_agent_text(
    anchor_id: AnchorId,
    original: &AgentTextAnchorV1,
    original_text: &RetainedTextV1,
    target: &AgentPlacementTargetV1,
    budget: ResolverBudgetV1,
) -> Result<PlacementResultV1> {
    let original_bytes = match original_text {
        RetainedTextV1::Available(bytes) => bytes,
        unavailable => {
            return Ok(result(
                anchor_id,
                PlacementKindV1::Unavailable,
                Vec::new(),
                unavailable_evidence(unavailable).expect("unavailable variant"),
            ));
        }
    };
    let target_bytes = match &target.text {
        RetainedTextV1::Available(bytes) => bytes,
        unavailable => {
            return Ok(result(
                anchor_id,
                PlacementKindV1::Unavailable,
                Vec::new(),
                unavailable_evidence(unavailable).expect("unavailable variant"),
            ));
        }
    };
    if original_bytes.len().saturating_add(target_bytes.len()) > budget.max_bytes {
        return Ok(result(
            anchor_id,
            PlacementKindV1::Unresolved,
            Vec::new(),
            PlacementEvidenceV1::BudgetExhausted,
        ));
    }
    original.text.validate_bytes(original_bytes)?;
    let candidate = || PlacementCandidateV1::AgentText {
        session_id: target.session_id,
        block_id: target.block_id,
        block_version: target.block_version,
        content: target.content,
        start_byte: original.text.start_byte,
        end_byte: original.text.end_byte,
    };
    let same_representation = original.session_id == target.session_id
        && original.block_id == target.block_id
        && original.representation_version == target.representation_version
        && original.sanitizer_version == target.sanitizer_version
        && original.normalizer_version == target.normalizer_version;
    if same_representation
        && original.block_version == target.block_version
        && original.content == target.content
    {
        original.text.validate_bytes(target_bytes)?;
        return Ok(result(
            anchor_id,
            PlacementKindV1::Exact,
            vec![candidate()],
            PlacementEvidenceV1::ExactIdentity,
        ));
    }
    let Some(lineage) = target.lineage else {
        return Ok(result(
            anchor_id,
            PlacementKindV1::Unresolved,
            Vec::new(),
            PlacementEvidenceV1::MissingLineage,
        ));
    };
    let lineage_matches = lineage.from_block_version == original.block_version
        && lineage.from_content == original.content
        && lineage.to_block_version == target.block_version
        && lineage.to_content == target.content;
    if !same_representation || !lineage_matches || !target_bytes.starts_with(original_bytes) {
        return Ok(result(
            anchor_id,
            PlacementKindV1::Unresolved,
            Vec::new(),
            PlacementEvidenceV1::MissingLineage,
        ));
    }
    original.text.validate_bytes(target_bytes)?;
    Ok(result(
        anchor_id,
        PlacementKindV1::MappedUnchanged,
        vec![candidate()],
        PlacementEvidenceV1::RetainedAgentPrefix,
    ))
}
