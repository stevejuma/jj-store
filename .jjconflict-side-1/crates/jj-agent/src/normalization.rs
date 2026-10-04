use crate::{
    NormalizationBatchV1, NormalizationDiagnosticKind, NormalizedActivityRecordV1,
    NormalizerProfileV1, Result, SanitizedObservationBatchV1, ToolResultObservation, normalizers,
};

/// Transient parser output. Results become activity only after call resolution.
pub struct NormalizationCandidates {
    pub activity: NormalizationBatchV1,
    pub tool_results: Vec<ToolResultObservation>,
}

pub fn normalize_batch(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
) -> Result<NormalizationBatchV1> {
    let NormalizationCandidates {
        mut activity,
        tool_results,
    } = normalize_batch_candidates(profile, batch)?;
    for result in tool_results {
        let known = result.same_batch_call(&activity).cloned();
        activity.resolve_tool_result(&result, known.as_ref())?;
    }
    activity.sort_source_records();
    Ok(activity)
}

/// Persistence providers may additionally resolve against their pinned selected root.
pub fn normalize_batch_candidates(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
) -> Result<NormalizationCandidates> {
    batch.validate()?;
    let mut candidates = NormalizationCandidates {
        activity: NormalizationBatchV1 {
            schema_version: 1,
            profile: profile.clone(),
            aliases: Vec::new(),
            records: Vec::new(),
            diagnostics: Vec::new(),
        },
        tool_results: Vec::new(),
    };
    if !profile.supported() {
        candidates.activity.diagnostic(
            NormalizationDiagnosticKind::UnsupportedVersion,
            "profile-version",
            None,
        );
    } else if profile.integration != batch.integration {
        candidates.activity.diagnostic(
            NormalizationDiagnosticKind::IntegrationMismatch,
            "batch-integration",
            None,
        );
    } else {
        normalizers::normalize(
            profile,
            batch,
            &mut candidates.activity,
            &mut candidates.tool_results,
        )?;
    }
    Ok(candidates)
}

impl NormalizationBatchV1 {
    pub fn sort_source_records(&mut self) {
        self.records.sort_by_key(|record| match record {
            NormalizedActivityRecordV1::SessionState(v) => v.source_sequence,
            NormalizedActivityRecordV1::Message(v) => v.source_sequence,
            NormalizedActivityRecordV1::Tool(v) => v.source_sequence,
            NormalizedActivityRecordV1::Permission(v) => v.source_sequence,
            NormalizedActivityRecordV1::FileObservation(v) => v.source_sequence,
        });
    }
}
