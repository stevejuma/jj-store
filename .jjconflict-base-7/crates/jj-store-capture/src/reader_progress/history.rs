use super::{CaptureResult, ReaderProgressV1};
use jj_agent::{IntegrationId, SourceInstanceId};
pub(super) fn same_generation(left: &ReaderProgressV1, right: &ReaderProgressV1) -> bool {
    left.cursor.integration == right.cursor.integration
        && left.cursor.source_instance_id == right.cursor.source_instance_id
        && left.cursor.generation == right.cursor.generation
}
/// Repository partitions share one physical cursor history, never separate timelines.
pub(super) fn validate(records: &[ReaderProgressV1]) -> CaptureResult<()> {
    let mut sorted: Vec<_> = records.iter().collect();
    sorted.sort_by_key(|p| {
        (
            p.cursor.integration,
            p.cursor.source_instance_id,
            p.cursor.generation,
            p.last_sequence,
        )
    });
    for pair in sorted.windows(2) {
        if same_generation(pair[0], pair[1]) {
            pair[1].validate_forward(pair[0])?;
        }
    }
    Ok(())
}
pub(super) fn latest(
    records: &[ReaderProgressV1],
    integration: IntegrationId,
    source: SourceInstanceId,
    generation: Option<u64>,
) -> CaptureResult<Option<ReaderProgressV1>> {
    let selected: Vec<_> = records
        .iter()
        .filter(|p| {
            p.cursor.integration == integration
                && p.cursor.source_instance_id == source
                && generation.is_none_or(|g| p.cursor.generation == g)
        })
        .cloned()
        .collect();
    validate(&selected)?;
    Ok(selected
        .into_iter()
        .max_by_key(|p| (p.cursor.generation, p.last_sequence)))
}

pub(super) fn validate_candidate(
    records: &[ReaderProgressV1],
    candidate: &ReaderProgressV1,
) -> CaptureResult<()> {
    for old in records.iter().filter(|old| same_generation(old, candidate)) {
        if old.last_sequence > candidate.last_sequence {
            old.validate_forward(candidate)?;
        } else {
            candidate.validate_forward(old)?;
        }
    }
    Ok(())
}
