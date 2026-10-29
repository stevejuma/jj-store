//! Project complete native groups using existing controller entity identities.
use crate::*;
use serde_json::json;

pub(crate) fn normalize(
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
) -> Result<()> {
    let [observation] = batch.observations.as_slice() else {
        return Err(Error::Invalid("native capture observation count"));
    };
    if observation.payload.media_type() != SanitizedMediaType::Json
        || batch.representation != SanitizedRepresentation::SanitizedNative
    {
        return Err(Error::Invalid("native capture representation"));
    }
    let mut envelope = NativeCaptureEnvelopeV1::decode(observation.payload.bytes())?;
    if envelope.ordinal != observation.source_sequence || envelope.policy != batch.sanitizer_version
    {
        return Err(Error::Invalid("native capture progress or policy"));
    }
    // A deferred prefix only releases text. Its old tool/lifecycle metadata
    // must not be observed twice. NativeCapture already accumulated this text
    // into the triggering record's complete assistant projection.
    let mut frame = envelope
        .frames
        .pop()
        .ok_or(Error::Invalid("native capture frame"))?;
    frame["controllerSessionId"] = json!(envelope.binding.worker);
    // Translate lifecycle vocabulary only in the transient projection. The
    // original envelope, native cursors and unknown fields remain evidence.
    match frame["type"].as_str() {
        Some("session_closed") => frame["type"] = json!("closed"),
        Some("error") => {
            frame["type"] = json!("record");
            frame["record"] = json!({"status":"outcome_unknown"});
            frame["sessionUsable"] = json!(false);
            frame.as_object_mut().unwrap().remove("assistantText");
        }
        _ => {}
    }
    let mut projected = batch.clone();
    projected.observations[0].payload =
        SanitizedPayloadV1::new(SanitizedMediaType::Json, &serde_json::to_vec(&frame)?)?;
    super::controller::normalize(&projected, output)
}
