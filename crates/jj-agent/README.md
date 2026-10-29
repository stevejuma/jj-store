# jj-agent

Repository-owned agent activity models: sanitized observations, normalizer
profiles, sessions, transcripts, tools, checkpoints and provenance. Canonical
object codecs keep immutable references stable across storage providers.

Start with `SanitizedObservationBatchV1` and a supported `NormalizerProfileV1`.
Use `AgentObjectV1`, `AgentRootV1` and their codecs to exchange validated
records; use an AgentStore provider to persist and publish them. Agent execution
belongs to the host.

For already sanitized native capture batches, use
`NormalizerProfileV1::native_capture_v1()` with the exact schema-1 envelope as
the observation payload. `NativeCaptureEnvelopeV1` validates its binding,
processing ordinal and complete frame group. A store provider still owns
publication and receipts; this model does not admit execution or persistence.

See the [capture pipeline](../../docs/site/technical/capture.md) and
[collaboration concepts](../../docs/site/concepts/collaboration.md).
