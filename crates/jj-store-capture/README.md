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

For a host-owned capture pipeline, disable the application feature:

```toml
jj-store-capture = { version = "0.1.0", default-features = false }
```

This retains source discovery, leases, bounded readers, sanitization and progress
without service, RPC, WAL or upstream JJ dependencies. `CodexTranscriptReader`
returns the same pending batch until an exact `DurableCaptureAck` arrives. The
host must sanitize, durably retain the accepted records and progress, and only
then acknowledge. Raw source records must never reach logs, UI or persistence.
A source lease does not authorize repository writes.

The default `application` feature additionally provides the existing
`AgentCaptureSpool`, prepared request recovery and native/local publishers.
Applications needing that private journal keep defaults enabled. Headless hosts
supply their own durable journal and publication adapter; disabling defaults
does not translate or migrate an existing application spool.
