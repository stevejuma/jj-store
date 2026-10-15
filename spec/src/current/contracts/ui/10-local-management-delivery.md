# Local management delivery

This staged extension records global Settings section/tab in the URL with
validated startup restoration and makes repository Settings routed and automatic
snapshot refresh contract. It adds Local service, Repository, Agents, Search,
Storage and Sync sections as their operations become available. The milestone
ledger is the single source of delivery status; this section defines the
accepted scope.

Local management uses versioned owned capabilities, profile-aware workspace
links, settings revisions and bounded operation reports. Service interfaces
carry owned types; server composition owns configuration and workers; CLI
integration owns physical workspace discovery and qualified agent setup. A
local-management capability cannot be obtained through a remote bearer session.
Existing exact authority/origin/protocol checks apply. GETs must not initialize
metadata, install hooks, probe executables or rewrite configuration. Explicit
setup previews name affected files; mutation requests carry typed intent, not
shell commands or arbitrary configuration paths.

Users may explicitly link an existing local folder after canonical path,
repository, backend and installation validation. Linking does not initialize
companion metadata, migrate files or snapshot the working copy. Links remain
device-local, outside checkout content, and are revalidated before each action.
The native profile uses its verified local-service binding. The Git-backed
profile reuses M13 discovery and companion authority; its browser management
capability stays unavailable until the owning M13.4 serving gate is qualified.
The profiles must not reinterpret one another's identities or prepared captures.

Agents show detection/version support, desired enablement, installation drift
and queue health for the selected physical workspace. Enable/setup and removal
reuse the existing ownership records and sanitizer. Removal preserves queued
captures. Automatic recovery runs with the service and shares the existing
`jj agent drain` implementation, stable request identities and backend dispatch;
a second retry command is unnecessary. A single drain lease prevents concurrent
publishers, but brief spool mutation locks must allow enqueue while publication
waits. Native recovery retains the hook's admitted principal. Unknown outcomes
remain pending, and queues cannot be silently discarded by unlinking a folder.
No agent execution or historical transcript backfill is introduced.

Search exposes enablement, metadata-only or explicit bookmark/workspace file
scopes, include/exclude prefixes and index budget. Save uses revision-checked
atomic updates of the startup-selected configuration, preserving unrelated
settings. Desired/effective states are distinct. Apply drains only the affected
worker, verifies a replacement generation before activation and preserves pinned
readers; repository access and RustFS remain available. Retry follows normal
recovery, whereas Rebuild constructs a new text-search projection from selected
committed data. Native graph preparation remains a separate action.

A failed Search save retains its scoped failure notice alongside the original
draft revision. Successful manual or automatic status reads do not establish
save success and MUST NOT clear that notice. Explicit cancellation of the draft
or a successful explicit save resolves the failure. Status observation never
retries Save or Apply. E-UI-CI-003 records the notification/refresh race under
JST-UI-016/019; storage and HTTP formats are unchanged.

Storage offers current readiness, checkpoint coverage, bounded integrity checks
and non-destructive checkpoint publication through the running owner. Reports
identify the captured position and any incomplete check; uncertain publication
retains its original target and resolves through selected authority. Sync uses
existing controls for configured upstreams, local-only status, pause/resume,
finite frontier waits and the existing reconciliation preview/apply workflow.
WorkStore replication remains unavailable. Settings opened from the Projects hub
preserve that route; repository actions require explicit repository context.

Forms preserve drafts until Save/Cancel. Operation status remains queryable when
Settings reopens, operational errors use scoped notifications while validation
and exact recovery status stay beside their controls, and teardown or repository
switching cannot apply a stale response. Use Community composition and public
Design Core controls, with keyboard/mobile and cold browser acceptance. No new
backup/restore, destructive GC, global cache deletion, service installation,
upstream credential editing, Git transport management or remote administration
is included. Qualification and visual baseline approval remain separate.

Settings content fills the available width with a one-rem inline inset and no
extra top padding; the first section adds no top margin. Storage readiness,
maintenance, graph-index status and native Local folders use the same rounded,
bordered Settings rows as other sections. Settings action buttons retain their
text and decorative leading Lucide icons through the public Button icon slot.
The Appearance opacity reset uses a normal-size Lucide reset icon. Stacked
operation-head copy controls retain their compact width.

Repository, Search, Storage and Sync subsection tabs occupy a full-width sticky
strip. The section icon and title come first, followed by horizontally
scrollable tabs. Search configuration/scopes/health, native Local folders,
Storage and Sync place their current refresh action at the far right as a ghost
icon-only button with its accessible name and public shadcn tooltip. Existing
capability, loading and pending-action guards remain intact. Global Back remains
in the Settings sidebar; repository Back uses the body header.

Repository settings are scoped by their repository URL and have no repository
picker. Installation-wide Protection and storage configuration live under Local
service. Dirty drafts, pending commands and active work retain their existing
owner-change guards; same-repository settings navigation preserves that owner.
An unresolved repository action may remain retained while changing only a
repository Settings category or subtab. Every other route field stays exact,
including the repository, operation, workspace and file selection. Entering or
leaving Settings, Global Settings, hubs and any changed owner remain guarded;
category navigation never rebinds or replays the pending action.

Repository review creation and Refresh share one trailing action group at the
far right of the Reviews header.
