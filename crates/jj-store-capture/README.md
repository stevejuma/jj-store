# jj-store-capture

Private agent capture, sanitization, source discovery and durable spool recovery
for supported harness profiles. Captured observations become repository-owned
AgentStore records only through an admitted publisher.

Start with a supported `profile` and `agent_sanitization`, retain batches in
`agent_capture::AgentCaptureSpool`, then use `agent_publication` or
`local_agent_publication` for the selected provider. Preserve prepared data and
original request identities across retries; host policy owns source admission.

See the [capture guide](../../docs/site/guides/capture.md),
[capture pipeline](../../docs/site/technical/capture.md) and
[preview runbook](../../docs/capture-preview.md).
