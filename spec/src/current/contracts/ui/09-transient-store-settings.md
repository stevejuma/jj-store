# Transient store settings

JST-UI-016 installation capture management lives in dedicated Agents settings,
independently of a selected repository. LocalTrusted bootstrap must advertise
`local_capture_management` before any capture request. The installation-owned
controller survives settings closure and repository navigation, closes its
transport at application teardown, and preserves historical code selections,
drafts and pending commands during automatic updates. Capture settings loads
policy, harness coverage, bounded sources and publication progress on opening;
visible five-second checks, acknowledged actions and focus/reconnect keep these
observations current. Reads coalesce and late responses are discarded. Closing
settings stops visible polling without losing installation-owned pending intent.
Manual refresh remains optional; no transcript paths or native session keys are
shown. Background reads never replay commands or replace accepted import intent.
Agents reuses the main application's automatic committed-data refresh and common
session response/sidebar, preserving selection and drafts. Capture actions MUST
reflect the qualified runtime profile; deferred OpenCode/Cursor imports are not
offered as working commands. Disabled capture can be configured through the
Agents Configuration tab; saving and worker availability remain independent.
Disabled, unsupported, degraded and unavailable states remain distinct. Pause
acceptance and rescan acceptance do not claim worker drain or discovery
completion.

Controls retain the exact original generation, revision, selector and nonzero
request ID while a control/import outcome is unknown. New durable commands are
withheld until exact replay or matching receipt inspection resolves that intent;
a refusal on an uncertain retry cannot erase its earlier unknown outcome. Source
continuation requires the original runtime generation, rejects repeated
cursors/selectors and never combines a stale or malformed page. Explicit
inactive history import shows accepted boundary publication independently from
live-tail coverage. Users may inspect a previous import request ID without
selecting a different source, including when its receipt is unavailable.

Global Settings opens from the profile menu and contains only Appearance, Local
service, Agents and Compute. Local service owns installation-wide Protection,
authority storage and backup configuration, and folder registration. Its
destination and selected section remain transient, outside URL/history/storage.
Back to jj-store restores the retained application route and resumes updates.

Repository Settings opens through an icon-only gear at the far right of the
repository body tab/header row, including focused Stacks views. It replaces
repository tabs with Back and capability/permission-filtered General, Local
folders, Search, Storage, Sync and Advanced categories. The left navigation
retains repository selection. Browsing controls and the Details rail are hidden;
return restores the rail's prior state. No repository picker or selection empty
page is offered. General retains the repository type chip and eligible
conversion. Local folders shows only this repository's binding/link controls;
Add folder is installation-wide. Search, Storage and Sync retain supported
subtabs. Advanced contains capability/limit facts and eligible graph
preparation. Workspace settings remains a separately labelled Stacks action.

Repository settings routes use `#/repos/{id}/settings?section=search&tab=scopes`
with an optional retained `at_op`. Missing section defaults to General; missing
subtab defaults to the supported first tab. Category/subtab navigation pushes
history and restores through reload/Back/Forward. Duplicate, malformed and
incompatible fields fail closed. After capability discovery, unsupported valid
destinations replace history with a supported default and one informational
notice. The Back control retains the exact session entry route including code,
workspace and Stacks selections; a direct/reloaded link falls back to Overview
at the retained operation. Settings transitions perform no graph/file loading or
automatic repinning. Controller owners, drafts and exact pending identities
survive same-repository transitions; hidden polling stops and owner changes
retain editing, pending-command and authorization guards. Settings response
ownership follows the connected repository, independently of code-read selection
generations. Same-repository history/return cannot discard a command reply or
abort its exact index task; owner replacement still clears tasks and rejects
late replies.

Global Settings retains its icon/title, spacing and public controls. Repository
Settings omits repeated category headings and title-only strips; a second row
contains only real subtabs. Contextual refresh belongs to the category row.
Observation subscriptions depend on the stable selected category, so completing
status reads cannot restart them or flicker loaded content. Global collapsed
navigation remains accessible through labelled icon controls.

Local service reports access mode, authoritative-storage readiness, HTTP and SSE
state and automatic update status. Repository reports the retained discovery
permission, repository and operation identities, conflicts, captured authority,
native profile, authentication, advertised APIs/actions and bounded request,
upload, read and event limits. Search loads status only when the repository
advertises search, confines load failures to its section and changes displayed
positions automatically while Settings is open. Repository settings expose
native graph index preparation separately from text-search projection health.
Graph preparation uses an operation from an existing graph rebuild-required
response before the explicitly selected snapshot operation and requires
index-task support and an exact operation, independently of text-search support.
Completing it does not refresh or repair the text-search projection. Long
identities, capability lists and numeric limits wrap without horizontal
overflow; status meaning is always present as text as well as color.

Local service lists Git-backed JJ registrations in the same bordered Settings
rows as service status, with the name, wrapping path and availability on the
left and Reconnect/Remove actions on the right. Registration guidance sits left
of Add/Refresh actions in a matching settings row. Search status and editable
enablement/budget or file-scope controls share one bordered group, with labels
and descriptions left and controls right. Save/Cancel, index maintenance and
sync controls use that same description-left/action-right composition. Each
Settings action group stacks vertically, with its buttons stretching to equal
width; Appearance option groups retain their horizontal layout. The native
folder-link form instead keeps Cancel and Link folder in a bottom button row
aligned to the right, below its guidance and a public Separator, within the same
bordered group as Folder path. Its controls wrap at narrow widths. Rows stack
their label and control columns on narrow screens. Standalone guidance, loading
and status messages use public Design Core Alert composition with an information
icon and polite status semantics; transient operational errors use the
application notification service. Descriptions and status values already
contained in rows remain within those rows. Capture policy, harness diagnostics,
source coverage and import inspection use the same bordered, divided two-column
Settings rows. Source descriptions and repository bindings sit left; coverage,
inputs and vertically stacked equal-width actions sit right. Capture refresh
occupies the far-right page header. Closing or leaving Agents stops visible
capture polling while preserving pending commands and request-ID drafts. Agents
is installation-scoped and has no repository picker.

Unsupported capture, Search, Sync and local-folder management use the public
shadcn Empty composition with a suitable icon, explicit availability text and no
unavailable controls. Graph preparation, including unavailable status, uses
bordered Settings rows.

Local JJ registration and reconnect dialogs use the same Settings-style divided
rows, with descriptions left and path/name/inspection controls right. Folder
Browse remains explicit and Inspection precedes Add. Path and name inputs retain
their standard visible borders. The path/Browse group fills the available
control column, with the input flexing and Browse retaining its natural width.
The Projects Repositories header consolidates add actions into one menu with
task and Git icons and opens Link repository through a labelled link-icon
control. Link selection and manual native-ID entry live only in a matching
dialog. Menu closure preserves the registration manager and pending identity;
form errors and unknown requests retain drafts, as does Cancel in the link
dialog. Keyboard navigation opens and closes dialogs without leaving page
interaction blocked.

The catalog uses Storybook 10.5.7, Svelte CSF 5.1.2, Vitest browser 4.1.11 and
Design Core ^0.3.1 with the existing Svelte 5/Vite 8 toolchain. Direct
adaptations retain provenance and Apache-2.0 attribution. Shared primitives
remain public Design Core imports. Stories are visual-pending until human
review.

Projects catalog fixtures use per-instance memory storage by default, so saved
sidebar and appearance state cannot leak between scenarios. Explicitly supplied
storage remains available for persistence acceptance.

Repository settings includes a Local folders section when the service advertises
workspace-link management. Its explicit path form preserves the draft through
section changes and failed requests until successful linking or Cancel. Refresh
folders rereads saved device state without changing repository snapshots. Only
currently verified links display Ready; unavailable folders retain their path
and a binding/location prompt. Unsupported connections explain availability and
expose no link action. Repository switching/disconnection clears drafts and
ignores late requests. Folder paths wrap on mobile, and form actions remain
keyboard accessible.

Repository categories and subtabs use keyboard-accessible shared body navigation
with responsive overflow. Category changes retain folder drafts. Identity and
snapshot includes an icon-bearing type chip: Local JJ for registered Git-backed
folders and Server Hosted for native server-managed storage, including
LocalTrusted. Chip tones distinguish types without relying on colour alone.
Read-only Settings values use an owned reusable row composed with public Design
Core controls. Copying is explicitly enabled for identifiers and hashes, or
implicitly available when the value has an internal Open action. Ordinary names,
statuses, counts and descriptive values remain plain text without copy controls
or row-click behavior. Clicking any part of a copyable row or activating its
copy button with Enter or Space copies the full value, with circular tick
feedback and an announced failure/retry state. The entire copy target remains
stable throughout a pointer press, including on labels and values. Each
successful copy animates the tick unless reduced motion is enabled. Long values
truncate in the available space; missing values retain their availability label
without a copy action. A separate trailing Open action is offered only for valid
app destinations and does not copy. Rows without Open reserve the same trailing
action space so their copy buttons align with rows that have a link. This empty
space adds no focusable control. Settings navigation preserves drafts and
pending-action guards. Feedback clears when the value changes or the row is
removed.

The optional native folder picker joins Folder path to a Git-icon Browse button.
The path/Browse group fills its available control column, with the input taking
the space left by Browse. Public Design Core Input Group controls give the
entire input-and-Browse group one focus ring when the input is focused. Browse
retains independent keyboard activation. Only an explicit click opens the dialog
on the local-service machine. Selection fills the draft; Link folder remains the
separate validation/save action. Cancel preserves the draft; failures retain
manual entry and explain the fallback. While picking, repeated browse and link
actions are disabled. Repository changes discard late selections. Unsupported
platforms keep the manual form.
