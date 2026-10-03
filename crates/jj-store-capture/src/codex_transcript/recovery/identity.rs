use super::*;
use crate::{agent_capture::PendingCaptureStateV1, transcript_sanitization::TRANSCRIPT_POLICY};
use jj_agent::{
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedRepresentation, SourceKind,
    SourceTrust,
};
pub(super) struct Target {
    pub offset: u64,
    pub prefix: [u8; 32],
    pub records: usize,
    pub last_sequence: u64,
    pub omission_only: bool,
}
pub(super) fn target(
    entry: &PendingCaptureV1,
    profile: CaptureProfile,
    source: SourceInstanceId,
    header: &Header,
) -> CaptureResult<Target> {
    entry.validate()?;
    let expected_profile = match profile {
        CaptureProfile::CodexCliJsonlV1 => NormalizerProfileV1::codex_0_156_1(),
        CaptureProfile::CodexDesktopJsonlV1 => NormalizerProfileV1::codex_desktop_0_159_2(),
        _ => return Err("unsupported queued Codex profile".into()),
    };
    let metadata = entry
        .reader_metadata
        .as_ref()
        .ok_or("queued Codex reader metadata missing")?;
    if entry.profile != expected_profile
        || entry.cursor.source_instance_id != source
        || metadata.generation_digest != header.witness
        || entry.cursor.parser_version != format!("codex-jsonl@{}/v1", profile.product_version())
        || entry.cursor.native_session_key.as_deref() != Some(&header.session)
        || entry.batch.sanitizer_version != TRANSCRIPT_POLICY
        || entry.batch.representation != SanitizedRepresentation::SanitizedNative
        || matches!(entry.state, PendingCaptureStateV1::LocalPrepared { .. })
        || entry.batch.observations.len() > MAX_BATCH_RECORDS
    {
        return Err("queued Codex identity or policy mismatch".into());
    }
    let SourcePositionV1::JsonLines {
        complete_byte_offset: offset,
        prefix_sha256: prefix,
    } = entry.cursor.position
    else {
        return Err("queued Codex byte position required".into());
    };
    if offset == 0 || prefix == [0; 32] {
        return Err("queued Codex complete prefix required".into());
    }
    let mut batch_id = Sha256::new_with_prefix(b"jj-store/codex-transcript-batch/v1\0");
    batch_id.update(source.as_bytes());
    batch_id.update(entry.cursor.generation.to_be_bytes());
    batch_id.update(header.witness);
    batch_id.update(offset.to_be_bytes());
    batch_id.update(prefix);
    if batch_id.finalize()[..16] != entry.batch.batch_id.as_bytes()[..] {
        return Err("queued Codex batch identity mismatch".into());
    }
    let first = entry.batch.observations[0].source_sequence;
    let mut omission_only = true;
    for (index, observation) in entry.batch.observations.iter().enumerate() {
        let mut id = Sha256::new_with_prefix(b"jj-store/codex-transcript-observation/v1\0");
        id.update(entry.batch.batch_id.as_bytes());
        id.update(observation.source_sequence.to_be_bytes());
        if first.checked_add(index as u64) != Some(observation.source_sequence)
            || observation.observation_id
                != ObservationId::new(id.finalize()[..16].try_into().expect("digest length"))
            || observation.source_kind != SourceKind::Transcript
            || observation.source_trust != SourceTrust::LocalTranscript
            || observation.occurred_at_millis.is_some()
        {
            return Err("queued Codex observation range mismatch".into());
        }
        omission_only &= observation.payload.media_type() == SanitizedMediaType::Json
            && observation.payload.bytes()
                == br#"{"capture_omitted":true,"reason":"unverified_directory_context"}"#;
    }
    Ok(Target {
        offset,
        prefix,
        records: entry.batch.observations.len(),
        last_sequence: entry
            .batch
            .observations
            .last()
            .expect("validated observations")
            .source_sequence,
        omission_only,
    })
}
