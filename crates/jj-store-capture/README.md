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

Native privacy is delegated to the independently owned `lapis-agent-redaction`
crate. This crate retains JJ payload bounds, source normalization and
publication recovery. The two native policy IDs and existing sanitized record
formats remain unchanged.

Read-only session discovery and bounded Codex header inspection delegate to
`lapis-ai-capture`. Use `resolve_codex_roots` and `CodexDirectoryScanner` for
metadata candidates; admission and publication remain the host's responsibility.
The Cursor adapter still requires the JJ binding sidecar. JJ normalization,
source witnesses and durable acknowledged progress remain owned here.
