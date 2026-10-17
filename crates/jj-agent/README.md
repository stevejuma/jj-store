# jj-agent

Repository-owned agent activity models: sanitized observations, normalizer
profiles, sessions, transcripts, tools, checkpoints and provenance. Canonical
object codecs keep immutable references stable across storage providers.

Start with `SanitizedObservationBatchV1` and a supported `NormalizerProfileV1`.
Use `AgentObjectV1`, `AgentRootV1` and their codecs to exchange validated
records; use an AgentStore provider to persist and publish them. Agent execution
belongs to the host.

See the [capture pipeline](../../docs/site/technical/capture.md) and
[collaboration concepts](../../docs/site/concepts/collaboration.md).
