# Agent capture

`jj agent status` probes Codex, Cursor, and OpenCode against the exact supported
versions and reports detection, desired enablement, and setup separately.

`jj agent enable --non-interactive --agent <codex|cursor|opencode>` records an
explicit device-local choice. Use `--dry-run` to preview the resulting registry
without changing it. Detection does not authorize every installed harness.

The registry is user-restricted, contains no credentials or transcript content,
and is bound to one native repository, physical workspace, and stable device
identity. A missing executable, failed probe, or different version is reported
without forwarding harness stderr.

Hook/plugin installation and capture ingestion are separate setup steps. Until
`jj agent setup` reports an installed integration, enabled means desired policy;
it does not claim that capture is active.
