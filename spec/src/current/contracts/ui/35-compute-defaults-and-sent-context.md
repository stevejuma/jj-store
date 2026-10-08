# Compute, agent defaults and sent context

JST-UI-012 extends Agents settings with Conversations, Preferences, Agent
runtimes and Agent defaults in that order; Runtimes is initially selected. Each
named section uses the shared compact heading with half-rem vertical padding.
Agent defaults uses the Add/Edit agent searchable harness/model picker,
including live or remembered session capabilities and installed-harness
discovery. Discovery starts only when the runtime and selected harness are
available; loading, unavailable choices and an explicit retry preserve saved
preferences. Choosing another harness clears incompatible model/effort/mode
overrides. The model discovery status uses a divided action row with its
description left and explicit retry right. The defaults save row joins its field
table with only a top separator. The picker fills the field-table width without
a visible label; its search input remains accessibly named. Automatically
mention agents seeds conversations without an explicit preference and preserves
existing pins/preferences. Keep awake is device-owned, initially disabled and
expires after one hour without real agent activity; status polling does not
extend it. Unsupported hosts explain capability unavailability.

Global defaults and Add/Edit agent expose collapsed Advanced environment rows
with literal values, external environment references, OS credential references
and explicit unset. Saved secret values are never returned. Defaults apply to
new agents, existing configurations retain their prior behavior, and running
workers expose pending configuration without automatic interruption.
Model/effort choices remain advertised capabilities.

Harness configuration sections are collapsed on page load, retaining drafts
across disclosure changes. Settings → Compute exposes hardware/model management,
loading/download progress and explicit controls, resource limits, and trusted
machine pairing/sharing. Local and remote inference identity remains visible.
The Compute page heading and icon match its navigation entry. Transient errors
use application notifications; validation and exact recovery remain beside
controls.

Compute uses the standard named Settings sections, compact section headings, one
outer border per divided row group, label/description and control columns, and
description-left rows with vertically stacked equal-width actions. Model run and
import commands join their related field rows within the same outer table
border, using only a dividing line. Run commands appear beneath their
description in a single row of fixed-width buttons, wrapping when space runs out
on desktop or narrow screens. Models (initially selected) contains preparation,
downloads, import and start/stop; Resources contains hardware and memory limits;
Sharing contains pairing, inference-host selection and relay settings. The
shared save action and all drafts remain available across these tabs, without
changing the route. Hardware and runtime observations use the shared read-only
rows. Long machine identities and invitations remain bounded on narrow screens.
This machine’s invitation uses the shared syntax-highlighted JSON CodeBlock with
wrapping and an explicit copy control; the peer invitation remains an editable
input. Advanced in Compute, agent defaults and Add/Edit agent uses a Lucide
chevron with explicit expanded state and keyboard activation; closing it
preserves unsaved field drafts.

Outgoing messages expose a collapsed Sent context control. Rich captured
repository/project/file/revision/review/discussion content and a raw payload
inspection remain accessible by keyboard. Queued, sent, failed and unknown
delivery are distinguished. Multiple recipients retain separate context; old
messages never fabricate missing snapshots.

Import GGUF joins its path input to an explicit file-icon Browse button when the
service advertises native file selection. Browse selects a GGUF file on the
service machine and fills the draft without importing. Cancellation, failed
dialogs and late responses after closure preserve the prior draft. Repeated
Browse/import actions are disabled while picking. Manual absolute path entry
remains available on unsupported hosts. Picker errors use shared notifications.
The bundled Buzz-derived ACP harness is displayed as jj-agent with the jj-store
app logo in runtime and harness choices. Its internal `local` identity and
private executable names remain stable. Runtime Ready requires both the packaged
agent and owned coding-tool helper; missing bundles show unavailable guidance
and never offer the Codex installer.
