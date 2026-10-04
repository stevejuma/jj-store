use crate::resolver_text::{
    ceil_boundary, common_prefix, common_suffix, floor_boundary, occurrences,
};
use crate::{
    AnchorId, ByteOffset, CodePlacementTargetV1, CodeTargetArtifactV1, CodeTextAnchorV1,
    PlacementCandidateV1, PlacementEvidenceV1, PlacementKindV1, PlacementResultV1,
    ResolverBudgetV1, Result, RetainedTextV1, result, unavailable_evidence, valid_repo_path,
};

struct ArtifactResolution {
    placement: PlacementKindV1,
    candidates: Vec<PlacementCandidateV1>,
    evidence: PlacementEvidenceV1,
}

pub(crate) fn resolve_code_text(
    anchor_id: AnchorId,
    original: &CodeTextAnchorV1,
    original_text: &RetainedTextV1,
    target: &CodePlacementTargetV1,
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
    original.text.validate_bytes(original_bytes)?;
    if original_bytes.len() > budget.max_bytes {
        return Ok(unresolved_budget(anchor_id));
    }
    let mut charged = original_bytes.len();
    let mut resolved = Vec::new();
    let mut saw_unavailable = None;
    let mut saw_missing_lineage = false;
    for artifact in &target.artifacts {
        if !valid_repo_path(&artifact.path) {
            continue;
        }
        let target_bytes = match &artifact.text {
            RetainedTextV1::Available(bytes) => bytes,
            unavailable => {
                saw_unavailable = unavailable_evidence(unavailable);
                continue;
            }
        };
        charged = charged.saturating_add(target_bytes.len());
        if charged > budget.max_bytes {
            return Ok(unresolved_budget(anchor_id));
        }
        let exact_identity = target.operation_id == original.operation_id
            && target.commit_id == original.commit_id
            && artifact.path == original.path
            && artifact.file_id == original.file_id;
        if exact_identity {
            original.text.validate_bytes(target_bytes)?;
            if original_bytes != target_bytes {
                return Err(crate::Error::Invalid("exact code artifact bytes"));
            }
            resolved.push(ArtifactResolution {
                placement: PlacementKindV1::Exact,
                candidates: vec![candidate(
                    target,
                    artifact,
                    original.text.start_byte,
                    original.text.end_byte,
                )],
                evidence: PlacementEvidenceV1::ExactIdentity,
            });
            continue;
        }
        let Some(lineage) = &artifact.lineage else {
            saw_missing_lineage = true;
            continue;
        };
        let lineage_matches = lineage.from_operation_id == original.operation_id
            && lineage.from_commit_id == original.commit_id
            && lineage.from_path == original.path
            && lineage.from_file_id == original.file_id
            && lineage.to_operation_id == target.operation_id
            && lineage.to_commit_id == target.commit_id
            && lineage.to_path == artifact.path
            && lineage.to_file_id == artifact.file_id;
        if !lineage_matches {
            saw_missing_lineage = true;
            continue;
        }
        resolved.push(map_artifact(
            original,
            original_bytes,
            target,
            artifact,
            target_bytes,
            budget.max_candidates,
        )?);
    }
    if resolved.is_empty() {
        let (placement, evidence) = if target.artifacts.is_empty() && target.lineage_complete {
            (PlacementKindV1::Deleted, PlacementEvidenceV1::TargetDeleted)
        } else if let Some(evidence) = saw_unavailable {
            (PlacementKindV1::Unavailable, evidence)
        } else {
            let evidence = if saw_missing_lineage {
                PlacementEvidenceV1::MissingLineage
            } else {
                PlacementEvidenceV1::UnsupportedTarget
            };
            (PlacementKindV1::Unresolved, evidence)
        };
        return Ok(result(anchor_id, placement, Vec::new(), evidence));
    }
    if resolved.len() == 1 {
        let only = resolved.pop().expect("one result");
        return Ok(result(
            anchor_id,
            only.placement,
            only.candidates,
            only.evidence,
        ));
    }
    let candidates = resolved
        .into_iter()
        .flat_map(|value| value.candidates)
        .take(budget.max_candidates)
        .collect();
    Ok(result(
        anchor_id,
        PlacementKindV1::Ambiguous,
        candidates,
        PlacementEvidenceV1::MultipleCandidates,
    ))
}

fn map_artifact(
    original: &CodeTextAnchorV1,
    original_bytes: &[u8],
    target: &CodePlacementTargetV1,
    artifact: &CodeTargetArtifactV1,
    target_bytes: &[u8],
    max_candidates: usize,
) -> Result<ArtifactResolution> {
    let target_text = std::str::from_utf8(target_bytes)
        .map_err(|_| crate::Error::Invalid("target text utf-8"))?;
    if original_bytes == target_bytes {
        return Ok(ArtifactResolution {
            placement: PlacementKindV1::MappedUnchanged,
            candidates: vec![candidate(
                target,
                artifact,
                original.text.start_byte,
                original.text.end_byte,
            )],
            evidence: PlacementEvidenceV1::UnchangedBytes,
        });
    }
    let start = original.text.start_byte.get() as usize;
    let end = original.text.end_byte.get() as usize;
    let prefix = common_prefix(original_bytes, target_bytes);
    let suffix = common_suffix(original_bytes, target_bytes, prefix);
    if end <= prefix {
        return Ok(mapped(target, artifact, start, end));
    }
    if start >= original_bytes.len().saturating_sub(suffix) {
        let target_start = target_bytes.len() - (original_bytes.len() - start);
        let target_end = target_bytes.len() - (original_bytes.len() - end);
        return Ok(mapped(target, artifact, target_start, target_end));
    }
    let selected = &original_bytes[start..end];
    let occurrences = occurrences(target_text, selected, max_candidates);
    if occurrences.len() > 1 {
        let candidates = occurrences
            .into_iter()
            .take(max_candidates)
            .map(|offset| {
                candidate(
                    target,
                    artifact,
                    ByteOffset::new(offset as u64),
                    ByteOffset::new((offset + selected.len()) as u64),
                )
            })
            .collect();
        return Ok(ArtifactResolution {
            placement: PlacementKindV1::Ambiguous,
            candidates,
            evidence: PlacementEvidenceV1::MultipleCandidates,
        });
    }
    let (candidate_start, candidate_end) = occurrences.first().map_or_else(
        || {
            (
                floor_boundary(target_text, prefix.min(target_bytes.len())),
                ceil_boundary(target_text, target_bytes.len().saturating_sub(suffix)),
            )
        },
        |offset| (*offset, *offset + selected.len()),
    );
    Ok(ArtifactResolution {
        placement: PlacementKindV1::CandidateModified,
        candidates: vec![candidate(
            target,
            artifact,
            ByteOffset::new(candidate_start as u64),
            ByteOffset::new(candidate_end as u64),
        )],
        evidence: PlacementEvidenceV1::ModifiedBytes,
    })
}

fn mapped(
    target: &CodePlacementTargetV1,
    artifact: &CodeTargetArtifactV1,
    start: usize,
    end: usize,
) -> ArtifactResolution {
    ArtifactResolution {
        placement: PlacementKindV1::MappedUnchanged,
        candidates: vec![candidate(
            target,
            artifact,
            ByteOffset::new(start as u64),
            ByteOffset::new(end as u64),
        )],
        evidence: PlacementEvidenceV1::ValidatedLineage,
    }
}

fn candidate(
    target: &CodePlacementTargetV1,
    artifact: &CodeTargetArtifactV1,
    start_byte: ByteOffset,
    end_byte: ByteOffset,
) -> PlacementCandidateV1 {
    PlacementCandidateV1::CodeText {
        operation_id: target.operation_id,
        commit_id: target.commit_id,
        path: artifact.path.clone(),
        file_id: artifact.file_id,
        start_byte,
        end_byte,
    }
}

fn unresolved_budget(anchor_id: AnchorId) -> PlacementResultV1 {
    result(
        anchor_id,
        PlacementKindV1::Unresolved,
        Vec::new(),
        PlacementEvidenceV1::BudgetExhausted,
    )
}
