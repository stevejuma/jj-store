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
mention agents is one saved installation-wide preference shared by Settings and
every composer agent picker. Either switch saves immediately using the existing
conditional settings command, without saving other defaults or secret drafts.
Saved values and settings events update both surfaces; legacy conversation flags
cannot override them. Recipient pins remain conversation-scoped. Disabling
suspends automatic insertion while retaining pins and explicit draft mentions.
Failed saves keep the authoritative value and exact retry identity visible;
retrying a preference never sends a message. Keep awake is device-owned,
initially disabled and expires after one hour without real agent activity;
status polling does not extend it. Unsupported hosts explain capability
unavailability.

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

The Model field uses a Popover-hosted Command View with search, keyboard
selection, a selected check and focus return. An owned catalog extends service
installed/current identities with the Buzz/mesh-llm curated and advanced GGUF
references. Each choice and the selected trigger show estimated weight size,
Fits well/Tight fit/Trade-off/Too large or Fit unknown, plus Recommended and
Installed where applicable. Model descriptions explain the intended use.
Recommendation tiers follow Buzz when they fit comfortably, falling back to a
smaller comfortable choice. Estimates use 70% of observed service-machine RAM,
capped by the unsaved memory limit, with Buzz's 60%/90%/110% fit thresholds.
This is advisory RAM headroom, not a measured GPU/free-memory guarantee; the
menu explains runtime/context overhead. Missing hardware or unknown imported
sizes remain unknown, and unknown hardware offers no recommendation. Selection
preserves exact custom/imported IDs, changes only the draft and never downloads,
configures or starts compute. Serving/pending controls remain disabled and
narrow menus stay within the viewport. Weights still require explicit download.

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

The shared agent chat row preserves this disclosure alongside separately
rendered XML context. Delivery updates enrich its captured state while retaining
the original authored message, identity and observation time.

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

Every Settings page, including Advanced and Add/Edit agent options, uses the
released Design Core shadcn Select for finite dropdown choices and the existing
Popover/Command composition for searchable choices. Native select controls are
not presented. Labels, keyboard navigation, focus return, disabled options,
null/default values and unsaved selections remain supported.
