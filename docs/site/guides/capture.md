# Agent capture

Capture retains sanitized activity from supported coding-agent sources. It does
not launch or continue an agent. Configure automatic readers separately from
explicit workspace hook integrations.

Visible capture status and source coverage react to semantic worker changes.
Unchanged scheduler activity causes no browser reads. Configuration drafts and
exact import/control request identities survive refresh; receipt inspection
remains authoritative for an unknown command outcome.

## Automatic Codex capture

Configure the supported Codex reader, then verify its source discovery in
Settings.

### Configure the Codex reader

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

### Check sources and discovery

Open Settings → Agents → Capture to inspect effective configuration and actual
startup-resolved roots. Sources shows observed sessions, Imports shows history
requests, and Diagnostics shows worker/queue health. Wait for discovery to
observe a source before submitting a supported new task start. If that start was
missed, file growth alone does not authorize historical ingestion.

## Cursor and OpenCode capture

Choose the profile for your harness: Cursor uses binding hooks, while OpenCode
reads an existing local API.

### Enable Cursor and install its binding hooks

Settings → Agents provides independent Cursor Desktop and CLI controls and an
OpenCode API control. Cursor Desktop 3.22.7 and CLI 2026.09.26-dd393fe use their
qualified transcript profiles. Enable the appropriate profile, then install its
binding hooks in the linked native workspace:

```sh
jj agent setup cursor --transcript-capture --dry-run
jj agent setup cursor --transcript-capture
```

These hooks identify the active source; the reader publishes transcript records.
They do not publish duplicate hook messages.

An existing owned setup must be removed before changing its mode. Desktop roots
default to `HOME/.cursor/projects`; CLI uses `CURSOR_DATA_DIR/projects` when
configured, otherwise the same home convention. Custom roots must agree with the
harness configuration and approved capture roots.

### Connect an existing OpenCode API

OpenCode 1.18.30 reads an already-running, explicitly configured local API. Set
its credential-free literal-loopback root URL in Agents settings; this profile
has no filesystem roots. The application does not start OpenCode. An unavailable
endpoint reports a source problem without disabling other configured harnesses.

### Apply capture settings and check limits

After saving, restart the backend to apply configuration. Sources and Agents
retain their repository and harness filters. Inactive-history import remains
unavailable for these configured Cursor/OpenCode profiles. Codex Desktop retains
its separate installed-product qualification gate.

## Import and interruption

Use Import history explicitly when earlier source content is needed. Selection
uses body-free source identities; an unbound source may be withheld because its
repository cannot be admitted. Preserve the original request ID if the
acknowledgement is unknown. Status inspection does not scan transcript folders
or read pending raw bodies.

### Inspect an interrupted import

A cold debug sanitizer can take more than two minutes to prepare. Busy
inspection does not mean the original import stopped. Captured rows stay visible
while refresh runs, and current runtime coverage can be reconstructed after
restart from durable private source identity and cursor state.

### Pause, resume and restart a reader

Pause/Resume controls runtime policy. Resume cannot override disabled startup
configuration. After changing configuration, restart the backend using the same
installation state; do not reset the fixture to make a reader start.

## Explicit hooks

### Preview and install hook integrations

`jj agent status` distinguishes detection, desired enablement and installed
setup. `enable --non-interactive --agent ...` records a device-local choice;
`setup` installs the owned integration for an exact supported version. Preview
with `--dry-run` before editing project hook files. Setup preserves unrelated
configuration and records original bytes plus intended hashes privately.

### Remove owned hooks and retry the spool

`remove` removes only the recorded owned artifact and refuses drifted or
symlinked targets. `drain` retries sanitized spool entries in source order.
Unknown publication keeps the exact original pending request. See the
[agent reference](../cli/jj-agent.md) for all commands.

Seeded sample sessions or a recognized harness name are not proof of capture
support for another version. Explicit provenance associations/checkpoints remain
distinct from automatic hook attribution.
