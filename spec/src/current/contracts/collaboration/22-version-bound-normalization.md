# Version-bound normalization

Normalization validates the complete sanitized observation batch before
dispatch, then accepts only the exact integration/version pairs Codex 0.156.1,
Cursor 3.22.7 and OpenCode 1.18.30. An unknown profile returns one safe
unsupported-version diagnostic without native record dispatch. Cursor records
also carry and verify their embedded product version; OpenCode session events
verify the version when the native callback supplies it. Diagnostics contain a
stable code and optional source sequence, never rejected source content.

Stable session, turn, message and tool IDs are SHA-256 domain-separated over the
integration, capture-source identity, entity kind and scoped native ID. Native
aliases remain explicit lookup facts. Replaying the same sanitized batch
therefore returns byte-equivalent normalized records and aliases. Native tool
names remain intact while the common kind is conservative; unknown names map to
`other`, and a file observation without a native tool-call ID cannot create one.

Lifecycle records remain source ordered. Capture completeness, permission state
and execution state are separate fields. The reducer enriches one stable tool
from input and output observations, does not regress a terminal state when a
late running event arrives, and reports conflicting terminal observations
without selecting one by wall-clock time.

The Codex fixture follows the locally installed 0.156.1 hook fields. The
OpenCode fixture follows the exact `@opencode-ai/plugin` and SDK 1.18.30
callbacks; the published plugin archive observed during qualification had npm
integrity
`sha512-bXWcA2VD478PXXke1qAYANTq58fCcbgY0GAUqZIFZqPpo48FoG8RH37nNZ78eHtz+ddiKIhFTBZJRaHF3IgFqw==`.
Both remain sanitized synthetic contract fixtures rather than captured model
runs. M9.6 additionally probes the actual local executables through the bounded
device registry and previews their owned project integration in disposable
workspace/state directories. Codex 0.156.1 and OpenCode 1.18.30 match their
qualified contracts. Cursor 3.22.7 remains synthetic contract acceptance only:
the installed agent reports `2026.09.26-dd393fe`, is recorded as
`unsupported_version`, and cannot be enabled. Qualification never invokes a
model, sends captured content or modifies the active workspace hooks.
