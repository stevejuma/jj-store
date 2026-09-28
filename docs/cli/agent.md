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

The registry is user-restricted, contains no credentials or transcript content,
and is bound to one native repository, physical workspace, and stable device
identity. A missing executable, failed probe, or different version is reported
without forwarding harness stderr.

Before changing a project file, setup stores its exact original bytes and the
intended content hash in the device-local registry area. Repeated setup is
idempotent. Status reports a later edit as drifted, and setup refuses to replace
it. Symlinked project configuration paths fail closed.

Hook/plugin installation and capture ingestion are separate steps. Installed
means the owned project artifact matches its recorded hash; capture becomes
active only after the hook ingestion command is available and qualified.
