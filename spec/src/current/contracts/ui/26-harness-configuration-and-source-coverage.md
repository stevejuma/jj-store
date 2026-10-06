# Harness configuration and source coverage

JST-UI-016 capture configuration MUST separate installation-wide enablement and
resource limits from harness-specific settings. Supported harnesses share one
rounded Settings group with divided, keyboard-accessible expandable rows. Each
row has its bundled avatar, hover and visible keyboard-focus feedback, and
nested settings in the existing divided Settings columns. The qualified Codex
row contains CLI and Desktop capture toggles, folder overrides and resolved
server defaults. Collapsing a section MUST retain the installation-owned draft
and leave global settings and save/cancel controls available. Additional
editable harness sections require qualified server configuration support.

Sources MUST use a semantic data table within the rounded Settings border, with
source, state and history-action columns. Source identity includes the bundled
harness avatar beside its stable label and version. A recorded repository
appears beneath the title as an icon-bearing link to its existing authorized
repository page. Missing binding remains explicit; navigation cannot discard
settings drafts or unresolved commands. Labelled harness, repository and status
filters sit outside the border, aligned to the trailing edge above the table,
and combine over loaded rows only. They MUST NOT alter observed coverage, source
selectors, paging cursors or import intent. Source labels stay stable across
filtering. The status and repository filters MUST use a searchable CommandView
in a popover with multiple toggleable selections and visible, accessible
selection feedback. Selecting options keeps the popover open; Escape closes it
and returns focus to its trigger. A row matches any selection within each
filter, intersecting repository, status and harness restrictions. Empty
selection or the corresponding All option clears that filter's restriction.
Repository options derive from all loaded sources before filtering, deduplicate
full recorded repository IDs and use the existing repository names with compact
ID context. No recorded repository binding selects unknown observations without
claiming that the source is unbound. Repository selection MUST match the full
cached identity and grant no repository access. Search affects options only,
resets on reopening, and preserves selections; the trigger summarizes the
selected label or count. History imports use icon-only buttons with accessible
names and descriptive tooltips, retaining capability, runtime and exact-selector
guards; deferred harnesses cannot import history.

A full-width footer cell spans the columns and reports coverage status with a
spinner during source refresh, retaining the prior rows and counts, and styled
shown/loaded/observed count badges, the last successful observation through the
shared relative-time component, and More sources when a cursor exists. This
browser observation time MUST NOT imply a transcript event or source-file
modification time. Only validated accepted coverage or paging advances the time;
loading, failed, busy or cancelled reads and filters retain it. Empty combined
filters are explicit; More sources remains available independently of the
filters. Tables scroll within their boundary on narrow screens without widening
the page.
