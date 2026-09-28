# Agent capture

`jj agent status` probes Codex, Cursor, and OpenCode against the exact supported
versions and reports detection, desired enablement, and setup separately.

`jj agent enable --non-interactive --agent <codex|cursor|opencode>` records an
explicit device-local choice. Use `--dry-run` to preview the resulting registry
without changing it. Detection does not authorize every installed harness.

`jj agent setup <codex|cursor|opencode>` installs one enabled, exact-version
integration in the current project. Codex and Cursor receive owned entries in
their existing hook documents; OpenCode receives the namespaced
`.opencode/plugins/jj-store-agent.ts` plugin. Unrelated configuration is
preserved. Use `--dry-run` to preview the target without creating directories,
backups, ownership state, or hooks.

A real setup compiles and caches MongoDB Kingfisher's pinned in-process rules
before installing the hook. The first setup can take longer; later hook
processes reuse the private device-local cache.

The registry is user-restricted, contains no credentials or transcript content,
and is bound to one native repository, physical workspace, and stable device
identity. A missing executable, failed probe, or different version is reported
without forwarding harness stderr.

Before changing a project file, setup stores its exact original bytes and the
intended content hash in the device-local registry area. Repeated setup is
idempotent. Status reports a later edit as drifted, and setup refuses to replace
it. Symlinked project configuration paths fail closed.

Installed hooks call the internal `jj agent hook <codex|cursor|opencode>`
entrypoint. It verifies the enabled registry state, exact owned project artifact
and warmed Kingfisher cache, then sanitizes one bounded standard-input event
before allocating durable source order and enqueuing the sanitized observation.
Invalid or oversized input is represented only by a fixed omission record. Hook
commands produce no normal output and never persist raw input. Pending capture
publication is attempted for three seconds after durable enqueue. An unavailable
server, unknown outcome, stale request, or timeout leaves the capture in the
private spool without failing the hook; the next hook invocation retries the
oldest exact source-ordered request before later observations.
