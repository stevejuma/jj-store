# Agent capture

Capture retains sanitized activity from supported coding-agent sources. It does
not launch or continue an agent. Configure automatic readers separately from
explicit workspace hook integrations.

## Automatic Codex capture

The current automatic preview supports the exact Codex CLI JSONL profile and
product version below in the native LocalTrusted application:

```toml
[agent_capture]
enabled = true
max_readers = 4
rescan_seconds = 5
max_sources = 1024
max_pending_bytes = 67108864

[[agent_capture.harnesses]]
profile = "codex-cli-jsonl-v1"
product_version = "0.156.1"
```

Omitted roots resolve at startup from `CODEX_HOME/sessions`, or
`HOME/.codex/sessions`. Explicit configured roots replace that selection. The
server verifies the repository and its exact service connection before admitting
activity. A different physical workspace of that same registered repository can
qualify; unrelated folders do not.

Open Settings → Agents → Capture to inspect effective configuration and actual
startup-resolved roots. Sources shows observed sessions, Imports shows history
requests, and Diagnostics shows worker/queue health. Wait for discovery to
observe a source before submitting a supported new task start. If that start was
missed, file growth alone does not authorize historical ingestion.

## Import and interruption

Use Import history explicitly when earlier source content is needed. Selection
uses body-free source identities; an unbound source may be withheld because its
repository cannot be admitted. Preserve the original request ID if the
acknowledgement is unknown. Status inspection does not scan transcript folders
or read pending raw bodies.

A cold debug sanitizer can take more than two minutes to prepare. Busy
inspection does not mean the original import stopped. Captured rows stay visible
while refresh runs, and current runtime coverage can be reconstructed after
restart from durable private source identity and cursor state.

Pause/Resume controls runtime policy. Resume cannot override disabled startup
configuration. After changing configuration, restart the backend using the same
installation state; do not reset the fixture to make a reader start.

## Explicit hooks

`jj agent status` distinguishes detection, desired enablement and installed
setup. `enable --non-interactive --agent ...` records a device-local choice;
`setup` installs the owned integration for an exact supported version. Preview
with `--dry-run` before editing project hook files. Setup preserves unrelated
configuration and records original bytes plus intended hashes privately.

`remove` removes only the recorded owned artifact and refuses drifted or
symlinked targets. `drain` retries sanitized spool entries in source order.
Unknown publication keeps the exact original pending request. See the
[agent reference](../cli/jj-agent.md) for all commands.

Codex Desktop and automatic OpenCode/Cursor readers have independent, incomplete
qualification. Seeded sample sessions or a recognized harness name are not proof
of automatic capture support. Explicit provenance associations/checkpoints also
remain distinct from automatic hook attribution.
