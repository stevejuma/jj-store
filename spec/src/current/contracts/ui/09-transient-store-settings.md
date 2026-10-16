# Settings navigation and lifecycle

A rejected native browser Back or Forward returns to the guarded entry without
rewriting the destination's stored history entry. Duplicate browser
notifications do not repeat the reversal or clear its notice. After the draft or
pending action is resolved, the original destination remains reachable through
history.

<span id="transient-store-settings"></span>

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

Global Settings opens from the profile menu and contains Profile, Custom emoji,
Appearance, Integrations, Local service, Agents and Compute. Integrations owns
GitHub account setup and repository access, using native OS credential custody.
Its accounts destination retains the same history/Back behavior, with no secret
or repository context in navigation preferences. Local service owns
installation-wide Protection, authority storage and backup configuration, and
folder registration. Its destination is recorded as
`#/settings?section=agents&tab=runtimes`. Missing fields select Local service /
Overview; unknown, duplicate and incompatible fields fail closed. Category and
tab changes participate in history. Reload and shared links restore the selected
destination; a root app startup resumes the last open Settings section/tab from
a validated device preference. Explicit URLs take precedence, leaving Settings
clears the startup preference, and no credentials, drafts or repository
selections are stored there. Back to jj-store restores the exact retained
session application route and resumes updates; a fresh settings link falls back
to Projects. Global Settings navigation does not perform ordinary graph/file
reads or replace controller owners.

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

General offers a red **Remove repository** footer for LocalTrusted Admin
Git-backed repositories. Its confirmation identifies the selected repository and
explains that the folder, files and history remain on disk. Cancel performs no
request. Removal uses the existing device registration API and its exact
registry revision. An unknown acknowledgement retains that revision for status
inspection/retry; a new registration cannot be removed by silently rebasing the
original intent. Pending removal blocks leaving its Settings owner, while
category changes retain it. Successful removal releases the active repository,
refreshes discovery and opens Projects / Repositories. Failures use JST-UI-019
toasts and keep confirmation available. Existing Settings drafts and actions
must be resolved before removal. General offers **Delete repository** for
supported LocalTrusted Admin native repositories on managed embedded storage.
Its confirmation names the repository and states that code, history and
collaboration objects will be permanently deleted from server storage. Local
folders remain untouched. A stable request/epoch is retained before dispatch;
partial/unknown deletion keeps the exact intent for status and retry, including
an installation-owned continuation after reload. Success returns to Projects /
Repositories. Unsupported profiles expose no destructive command.

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

Settings suspends teleported conversation, workspace and inspection sidebars
through their public visibility state; hiding only their source wrapper cannot
suspend projected content. Focused recorded responses close while Settings is
visible, then restore the retained selection and drafts on return. Retained
conversation hosts capture the current projection for callbacks that may outlive
a mounted reader, without reading destroyed reactive owners.

Global Settings retains its public controls and sidebar. Its fixed page heading
and contextual Refresh sit above actual icon-and-underline subtabs; Appearance
has no title-only navigation strip. Repository Settings omits repeated category
headings and title-only strips; a second row contains only real subtabs.
Contextual refresh belongs to the category row. Observation subscriptions depend
on the stable selected category, so completing status reads cannot restart them
or flicker loaded content. Global collapsed navigation remains accessible
through labelled icon controls.

Initial repository Settings reads use category/subtab-specific skeletons that
match card headers, aligned property rows, editable controls and bottom action
footers. Known profile differences are retained, with no invented interactive
controls or values. Each pending region has one accessible loading status;
decorative placeholders are hidden from assistive technology and respect reduced
motion. Navigation stays above the bounded Scroll Area and narrow layouts stack
without overflow. Skeletons replace only data that has not loaded: background
refresh retains existing values, empty results, drafts and pending commands.
Global Settings follows the same loading conventions for lazy module loading and
initial data reads. Each Appearance, Local service, Agents and Compute
section/tab uses its own card, row, disclosure and footer geometry. The global
page heading and selected tabs remain visible while content loads; retained
values are never replaced during background refresh.

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

Local service lists Git-backed JJ registrations using shared cards with compact
name, wrapping path and availability rows. Reconnect/Remove occupy a footer for
their registration; app-wide Add/Refresh occupy a separate management footer.
Global and repository Settings follow the shared
[settings/form convention](28-complete-supported-configuration-settings.md):
labels/help left, controls right, full-width dividers, tinted card headers and
relevant icons. Save/Cancel and maintenance commands occupy separate bottom
footers with optional explanation left and wrapping buttons right, primary last.
The folder-link form keeps Cancel/Link in the footer below its path and
guidance. Rows stack on narrow screens. Appearance option groups retain their
existing choices, previews and immediate saves.

Standalone guidance and contextual status use public Design Core Alert
composition with an information icon and polite semantics; transient operational
errors use the application notification service. Capture policy, harness
diagnostics, source coverage and import inspection use the same cards and
divided rows. Source filters and their table retain selection and command
guards; per-source operations stay attached to the source. Capture refresh
occupies the far-right fixed page header. Closing or leaving Agents stops
visible capture polling while preserving pending commands and request-ID drafts.
Agents is installation-scoped and has no repository picker.

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
successful copy animates the tick unless reduced motion is enabled. Legacy and
repository copyable values stay on one line with an ellipsis and expose the full
value on hover and to Copy; descriptive values and capability lists wrap;
missing values retain their availability label without a copy action. A separate
trailing Open action is offered only for valid app destinations and does not
copy. Rows without Open reserve no empty navigation action slot. Settings
navigation preserves drafts and pending-action guards. Feedback clears when the
value changes or the row is removed.

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

Repository presentation uses full-width, single-column small Design Core cards,
compact divided rows and bottom-right action footers, with optional explanatory
text to their left. Labels and controls stack at narrow widths; buttons wrap
without clipping. General, Local folders, Search, Storage, Sync and Advanced
retain their existing controls and filters. Advanced has a contextual Repository
capabilities heading and separate graph, capability and limit cards. It adds no
duplicate category navigation or title-only strip. Secondary tabs use public
line tabs with compact icons, left-aligned labels, bold active text and a
thicker theme-accent underline; primary tabs retain pills. Both navigation rows
remain outside the bounded Design Core Scroll Area that owns repository settings
content scrolling. Subtabs follow the primary row without extra top padding.
Card headers adjoin their first row without an intervening gap. Overflow probes
include icons and match visible tab presentation. The shared presentation is
selected by both Settings hosts; unrelated row/tab consumers retain their
defaults. Drafts, pending commands, exact copy/Open targets, automatic updates
and route/history contracts are unchanged.

M26 Profile and Custom emoji use the installation directory capability. The
controller retains both drafts and exact pending requests through navigation,
revalidates session epochs before admitting asynchronous reads, and refreshes
after committed saves, installation freshness events and bounded fallback.
Descriptions use the maintained Markdown form editor; image uploads return
owner-only draft media until explicitly selected by a save. Field validation and
unknown-outcome recovery stay beside their controls. Email is owner-only; shared
profile projections supply display name, description and resolved avatar.

Profile and Custom emoji use the shared full-width Settings card, icon-heading,
divided form-row and footer compositions. Profile info retains its photo picker,
display name, Markdown description, private email and a copyable identity row.
Opening the photo picker keeps the other form fields mounted. Edit, Save, Cancel
and unknown-outcome recovery occupy bottom footers. Large avatar images use the
server-owned binary normalization path, also used by Custom emoji; browser
drafts and the exact original upload request survive an unknown outcome.

The circular profile photo has a separate round action inset at its lower-right
edge, separated from the image by the card background. It clears an explicitly
selected avatar when present and opens the picker with an Edit icon when absent.
The photo itself also opens the picker. Both actions retain accessible names,
visible keyboard focus and the busy/pending guards. Clearing changes only the
avatar draft; profile Save and Cancel remain explicit.

The custom emoji shortcode input has fixed decorative colon addons around the
editable shortcode; persisted values contain only the shortcode. Selected images
appear beside the bordered Choose image button in the Image row. Choosing an
emoji image creates only a session-local File and blob preview. Save emoji
uploads/normalizes the source and then publishes the definition. Selection,
replacement, navigation, refresh and Cancel do not upload images. Local previews
are released on replacement, Cancel, successful save and owner/session teardown.
Recovery retains the original definition fields and stable request identities
across an unknown upload or definition save, resuming the admitted save only
after its original media receipt is known. Global Settings groups Profile,
Appearance and Custom emoji under Personal, and Local service, Agents and
Compute under App. Its package version stays at the bottom while long navigation
content scrolls. Compact navigation retains labelled actions and the version;
routes, drafts and action guards are unchanged.
