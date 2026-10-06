# Usable capture settings lifecycle

Opening the Agents section loads capture status and sources automatically.
Capture controls MUST remain visible and disabled while status is unknown,
loading, busy, disabled or unsupported. Initial status, configuration and source
reads use accessible skeleton rows with the existing Settings column structure;
background refresh retains loaded content and drafts. A busy read is a deferred
observation, not an unavailable error: controls remain guarded, ordinary reads
retry within a bounded cancellable window and continue the visible refresh
cycle. Real failures retain scoped notifications and exact pending commands. A
semantic event subscription retains the loaded source count and validates all
replacement continuations, refreshing after relevant changes and control
acknowledgements. A serial bounded fallback runs only while the application feed
is unavailable. Closure cancels ordinary observations and removes fallback;
installation-owned exact commands, original import selectors, receipt identity
and request-ID drafts survive navigation. Late reads cannot replace observations
from a newer view. A missing import receipt retains uncertainty and cannot prove
rejection. Unsupported OpenCode/Cursor sources have no usable import action.
Worker state and configuration setup use plain labels; receipt details remain
available. Agents reuse main's automatic pinned snapshot updates.

Runtime unavailable observations are a cumulative unsigned counter, not a count
of tracked sources. Repeated discovery failures can exceed the 4096-source
bound; valid counter values MUST NOT disable Capture status or controls. Queue
counts likewise use their unsigned wire contract rather than the source bound.
Tracked-source counts remain bounded, and malformed counters remain refused.

Agents settings MUST display each configured CLI root directory as a wrapping
path in its harness Settings rows. Paths come from the admitted status response
and reflect startup resolution, including default home selection and overrides.
Unavailable configuration metadata is explicit; counts never substitute for the
actual directories. The display does not infer transcript file paths.

Agents settings uses Capture, Sources, Imports and Diagnostics tabs in the
shared sticky section strip. Capture exposes policy controls and configured root
paths; Sources shows observed coverage and explicit history enrollment; Imports
owns request inspection/progress; Diagnostics contains worker, queue and harness
health. One mounted capture owner keeps observation active across tabs.
Selecting an import explicitly opens Imports; background updates never change
the tab. Unknown pending commands remain visible and retain exact retry/receipt
actions on every tab. Tab selection is transient, and import drafts survive tab
changes.

Capture operational errors use the shared notification service under a stable
installation scope, independent of the selected repository or Agents tab.
Loading, retries and navigation preserve dismissal; complete successful
observations clear a condition and allow a new episode. Ordinary accepted
outcomes use timed information notices. Disabled/unavailable capture content,
request-ID validation, exact pending requests and receipt/publication recovery
remain contextual without duplicate toasts.

Agent message previews opt into a generic remark extension for XML-style prompt
elements. Balanced arbitrary element names, nested sections and literal
attributes render as initially collapsed Design Core disclosures; Markdown
before, between, inside and after those elements remains on the shared safe Mira
pipeline. Fenced/indented code, inline code and escaped examples remain literal.
Self-closing elements are inert highlighted labels. A paired leaf whose trimmed
body occupies one source line and parses as one Markdown paragraph renders an
inline row with opening tag, safe inline Markdown and exact closing tag, without
a disclosure icon. This includes bodies surrounded by newlines and empty paired
leaves. Nested, multiline and block Markdown bodies retain disclosures. All
displayed tags use syntax colors for node names, attributes, quoted values and
angle-bracket punctuation, derived from generic XML syntax and the current
appearance palette. Node names mix their tag token 20% toward foreground for
contrast. Attribute identifiers use the palette's constant token mixed 15%
toward foreground, remaining distinct from punctuation and quoted values. Tags
are borderless text on the surrounding surface, without an inline-code
background. Each child level indents by 1rem, with compact and disclosed sibling
tag labels aligned in the same icon column. Attribute text remains inert;
highlighting preserves every source character. Sibling node gaps are at most
0.25rem without compressing Markdown paragraph spacing inside longer bodies.
Incomplete or mismatched tags remain readable literal text. No harness tag
allowlist, HTML attribute execution, source rewriting or persisted disclosure
state is introduced. Activity messages and recorded transcript chunks share this
behavior; ordinary comments and repository Markdown retain their existing
rendering. Exact source remains the Copy message value. Nested controls are
keyboard accessible and expansion must remeasure virtual rows without clipping
following messages.
