# Complete supported configuration settings

JST-UI-016 includes typed editors in Local service (resource limits), Local
service / Protection (installation-wide protected bookmarks/commits), Sync
(repository upstream, credential environment reference, CA path and every
bounded worker limit), and Search (polling, timeout and snapshot interval
alongside existing scopes/budget). Local service / Launch and Storage
configuration show the remaining deployment, authority and backup configuration
in divided read-only rows with explicit launch, migration or deferred-feature
reasons. No supported operational configuration is omitted.
Service/protection/sync saves activate through **Reload saved settings**. Search
retains explicit Apply and also participates in the shared reload. Each section
retains its draft and base file revision across refresh, tab changes and
repository navigation; stale observations cannot silently rebase a draft and
unknown saves cannot be blindly replayed. Operational feedback uses JST-UI-019
notifications under a stable installation scope; repository navigation cannot
replay an unchanged dismissed condition. Editors are installation-owned and
unavailable on authenticated/non-Admin connections. The presentation convention
below governs new and refreshed Settings and forms across the app. Repository
and global Settings both adopt it; unrelated consumers retain their explicit
default presentation.

## Settings and form presentation

Use shared presentation components composed from released Design Core primitives
and application theme tokens. Keep full-width cards in a single column with
compact spacing, subtle borders, moderately rounded corners, tinted headers and
restrained, relevant icons. Group read-only information, editable configuration
and maintenance into separate logical sections; avoid nested card borders.

Read-only properties use consistently aligned key-value rows with full-width
separators. Copyable identifiers stay on one line with an ellipsis while hover
and Copy retain the exact full value. Descriptive paths and capability lists
wrap. Icons, labels, values and compact copy/Open controls align vertically at
the centre. Editable rows put labels and help on the left and controls on the
right; numeric inputs use a content-appropriate width rather than stretching
across the column. Align controls inside full-width wrappers to the trailing
edge too. Dropdowns use a maximum 20rem width, shrinking with the available
column rather than stretching across it. Toggle descriptions occupy the
remaining width; runtime management rows place the description left and actions
right in one shared action-row composition, with no empty inter-item gaps.
Harness groups have full-width separators between collapsed and expanded rows.

Agents / Configuration / Capture options exposes a shared slider beside each
bounded numeric entry: readers 1–4, discovery interval 1–30 seconds, tracked
sources 1–4,096, and pending capacity 1 byte–64 MiB. Sliders edit only drafts
and retain exact numeric entry, units, disabled/invalid states and existing
Save/Cancel semantics. The byte-budget slider uses MiB choices plus its exact
minimum and draft; it never rounds a typed value on mount, unit changes or
refresh. Large ranges must use bounded meaningful slider steps rather than
enumerating every byte. Loading placeholders include the corresponding slider.

Shared numeric settings rows combine a number input with an attached unit
selector, using exact canonical integer values for configuration. Repository
Search (index budget and worker limits) and Sync configuration adopt time units
(ms, sec, min), binary size units (bytes, KiB, MiB, GiB), and fixed count units.
Changing the display unit does not modify a draft or enable Save. Fractional
canonical values, unsafe values and values outside existing bounds remain
invalid; never silently round or clamp them. Repeating display conversions are
marked approximate beside the exact canonical value. Labels, help, bounds,
validation and read-only/disabled states remain accessible. Optional sliders
display both endpoint labels in the selected unit with compact formatting, using
at most two decimal places. Tiny minima display as zero; a tiny nonzero maximum
retains meaningful significant digits. This is label-only rounding, while exact
entry and canonical bounds are retained. Long endpoint labels wrap without
overflowing. Unit changes never change canonical slider bounds, steps or values.
Sliders operate within the existing canonical range. A narrower convenience
range may expand to include larger valid typed values without modifying the
draft. Compute memory uses a 1–64 GB slider in whole-GB increments, expanding to
include a larger valid limit up to 1024 GB. Mounting or refreshing the slider
never changes an empty automatic value. Invalid memory drafts prevent
configuration saves, including from another tab. Optional per-field icon-only
Reset forms the trailing segment of the input after its unit selector, with an
accessible label and tooltip. Show Reset only while the draft differs from its
authoritative default, including empty or invalid entries; hide it after reset
and return focus to the input. The split control retains a complete neutral
outer border; focus draws one ring around the whole control, without additional
inner rings or a highlighted border. Unit selectors and Reset use subtle
theme-based hover states; disabled controls remain subdued. Reset restores only
the documented default in the draft; it never saves or applies. Defaults beside
field descriptions remain distinct from saved/running state. Omit redundant
"Using default value" and "Changed from default" labels under inputs; retain the
exact-value helper for converted units. Compact and responsive layouts use the
same component; property-local Reset is an input action, like Browse or Copy.
Local service limits, Agents capture limits and Compute memory use the same
joined numerical control. Preserve optional automatic/unset semantics for
Compute memory: empty remains null, not zero. Do not invent Reset defaults when
the service advertises no authoritative default. Immediate preferences still
save immediately; configuration commands keep their existing explicit save and
explicit activation semantics.

Commands belong in a dedicated footer at the bottom of their relevant section,
never in ordinary key-value rows or centred beneath content. Footer text is
optional and sits on the left; related buttons align right with the primary
action last and secondary actions immediately before it. Maintenance commands
remain separate from Save/Cancel. A form spanning several sections uses one
shared save footer, separated from the previous card by the same 1rem gap even
inside a wrapper. Every inline banner has 1rem clearance above and below,
including standalone notices between cards; banners within cards also have 1rem
horizontal insets. On narrow screens, label/control columns stack and footer
buttons wrap or stack without overflow. Existing inline copy/Open controls and
input Browse addons remain attached to the value they act on.

Search Configuration contains Index status (saved, currently active, operation),
Search options (enablement and budget with Save/Cancel), and Index maintenance
(Apply, Retry, Rebuild). File scopes and Advanced reuse these groupings for
their existing fields. Sync Configuration separates current status, upstream
fields and bounded limits and retains one form-level Save/Cancel footer. No
mockup-only sync settings or commands are introduced. Storage and Advanced
follow the same status/maintenance separation and footer convention.

Primary navigation retains pills; real secondary tabs use compact icons,
left-aligned labels and a 3px active underline in the theme accent colour,
overlapping the strip bottom divider. Active labels are bold. The secondary
strip uses the body horizontal padding; no title-only strip is shown. Primary
and secondary navigation stay outside the bounded Design Core Scroll Area used
by repository and global settings. Card headers meet the first row without extra
spacing. Keyboard selection and overflow remain unchanged. Global Settings
retains its sidebar categories and a fixed page heading with contextual Refresh.
Global section/subtab URLs, history and root-startup resume follow JST-UI-012.
Appearance uses Theme and Preferences subtabs.

Presentation changes MUST retain validation, capability and permission filters,
disabled states, loading feedback, confirmation flows and all original request
identities. Section grouping cannot create additional saves, alter action
availability or replace controller owners. Drafts, history, polling, exact Back
restoration and authorization contracts remain unchanged.

Shared read-only form rows place navigation to the left of Copy. Copy stays at
the far right, and rows without navigation reserve no empty action slot. Exact
copy values and independent navigation hit targets remain unchanged.

Use the same joined focus treatment for Folder path/Browse controls. Browse
remains an independently accessible input action; selecting a folder only edits
the draft, and Link/Save remains separate. All new and refreshed app forms use
these guidelines and the matching `web/src/lib/ui/AGENTS.md` implementation
guide.

Initial loading placeholders follow the selected settings page's card, row,
control and footer geometry, with a single named busy status and decorative
placeholders hidden from assistive technology. Respect reduced motion and mobile
stacking. Background reads retain loaded content, empty results, drafts and
pending identities instead of replacing them with skeletons.

Global Appearance uses the shared Theme and Preferences cards while keeping
previews and device-local immediate saves. Local service separates status,
folder registrations, resource/protection editors and deployment inventory.
Launch groups server/network, installation files, and identity/access. Storage
groups connection, runtime/data, identity/credential references, and backup
inventory. Common setup/migration reasons belong in headers; rows retain
saved/running differences. Configured paths and endpoints have exact Copy
controls, while absent values and ordinary scalar properties remain plain text.
Agents uses consistent preference, runtime, account, capture, source and
diagnostic groups. Runtimes consolidates conversation and device preferences in
one Preferences card. Conversation metadata expands there, initially closed;
collapsing keeps its editor mounted and retains drafts, pending identities and
existing Save/Disable semantics. Compute separates model operations,
hardware/resource configuration and peer management. Per-item maintenance
remains scoped to its item; commands occupy footers while Copy/Open, Browse and
Reset stay with their values. No new settings, API contracts, configuration
formats, authorizations or controller owners are introduced by this
presentation.

Server repository creation and project creation/editing compose these same
Settings cards, tinted headings and divided icon-bearing rows. Name and home
controls sit to the right of their labels/help and stack on narrow screens;
Markdown descriptions occupy a full-width row. Creation commands share one
bottom dialog footer with the close action before the primary command. The
presentation retains writable-home filtering, validation, drafts, exact pending
requests and status/retry controls; it does not change creation authority.

The right-aligned top-header Application menu uses an icon-only vertical kebab
and contains **Reload saved settings**; omit this action from body headers. The
Settings sidebar version appears below Help at the bottom. The shared reload
action applies saved values for the selected server within the caller's
discovered authority. It preserves unsaved drafts, controller owners, active
sessions and original pending identities. Accessible progress distinguishes
applied, deferred and failed domains; unknown outcomes retain the original
intent and offer Check original outcome. Refresh remains a read-only action.
Activation guidance uses reload for operational settings and separate
migration/lifecycle reasons for read-only infrastructure.
