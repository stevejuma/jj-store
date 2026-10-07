# Compute, agent defaults and sent context

JST-UI-012 extends Agents settings with Conversations, Preferences and Agent
defaults above runtime rows; Runtimes is initially selected. Automatically
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

Outgoing messages expose a collapsed Sent context control. Rich captured
repository/project/file/revision/review/discussion content and a raw payload
inspection remain accessible by keyboard. Queued, sent, failed and unknown
delivery are distinguished. Multiple recipients retain separate context; old
messages never fabricate missing snapshots.
