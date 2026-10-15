# Forge-agnostic stack submission

## Requirements

| ID            | Requirement                                                                                                                                                                                                                                                                                                                                 |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-FORGE-001 | Ordinary local Git-backed JJ stacks MUST submit one PR/MR per explicit bookmark segment through shared CLI/application orchestration, with GitHub, GitLab and Forgejo adapters and per-instance capabilities.                                                                                                                               |
| JST-FORGE-002 | Submission MUST pin the local operation, exact branch heads, endpoint identities and expected remote heads, push only required branches, and independently execute creation, retargeting and managed-description steps. Shared dependencies MUST be deduplicated; ambiguous topology, conflicts and private commits MUST block publication. |
| JST-FORGE-003 | External effects MUST retain request identity, exact intent and per-step outcomes in private device state. Unknown outcomes MUST be observed before another attempt and MUST NOT be automatically replayed. These records MUST NOT become code authority.                                                                                   |
| JST-FORGE-004 | PR titles and authored descriptions MUST be editable. Managed navigation MUST be bounded by owned markers and preserve authored text. Status/link maintenance MUST follow observed remote lifecycle changes, retaining drafts and explaining stale observations.                                                                            |
| JST-FORGE-005 | Checks, runs, jobs, logs and permitted rerun/cancel controls MUST be capability-gated and associated with the selected remote revision. Merge MUST respect provider protection, expected-head guards and dependency order; queued/accepted MUST remain distinct from merged.                                                                |
| JST-FORGE-006 | Local rewrites, pushes and app-initiated base changes after landing MUST require explicit Restack. Fork-only parent branches MUST use labelled cumulative upstream PRs with bottom-first merge order. Eligible GitHub native stacks MAY use native membership and asynchronous prefix merge.                                                |
| JST-FORGE-007 | Tokens MUST remain outside checkouts, URLs, logs, receipts and browser persistence. Configured endpoints MUST be explicit approved device-local connections. Production UI MUST use owned components and released Design Core/Mira.                                                                                                         |

## Composition and compatibility

`jj-forge` owns transport-independent contracts, provider adapters,
deterministic planning and external-effect journals. Local JJ
inspection/transport remains in `jj-store-jj`; service/HTTP and standalone CLI
share the adapter. Storage, manifest authority and WAL are independent of forge
runtime dependencies.

The first profile is ordinary local JJ 0.45.1 with SHA-1 Git objects. Native
jj-store stack submission and further forges require separate qualification.
GitHub native stacks are an optional capability; portable PR chains remain the
common model. Fork PRs may have cumulative diffs when the upstream lacks the
intermediate branches. Their managed section explains this mode.

Connections retain API origin, source/target project, remote and target branch
outside checkouts. Secrets are supplied per session or through automation
credentials. GETs and previews never snapshot/reconcile code or publish state.
Existing local Stacks undo/redo cannot undo external forge effects.

## UI and maintenance

Branch and Workspace columns share PR presentation and submission. When an
approved forge is configured and the topmost bookmark has no PR, its action row
offers Create PR immediately before Push. Lower unsubmitted bookmarks omit the
action. Creation opens within that bookmark card, above its commits, with the
branch name as the default title and the maintained Markdown chat composer.
Explicit creation retains the existing pinned submission plan, expected heads,
stack dependencies, draft/ready choice and exact external-effect journal; it
pushes only through the existing guarded submission action. An incomplete,
failed or unknown preview cannot supply a publishable form. Dependency PRs stay
visible for review. Cancel/reopen, refresh and inline/floating transitions
retain session-only authored drafts and pending identities; closing an unknown
action never replaces or replays it. Cached plans retain their original guards
and may be rejected as stale; explicit reopening never weakens server admission.

All submitted bookmarks, including lower dependent segments, place a compact
neutral forge/PR button below the title, beside observed push status and a
Passed/Failed/Running checks badge. Checks badges share the PR control height
and radius; Running shares its neutral bordered surface and retains a spinner.
Passed/Failed retain distinct status tones. Missing, stale or incomparable
heads/checks remain explicitly unknown or unavailable; skipped checks do not
imply success. Local versus observed open-PR head comparison supplies
display-only push status, with no ancestor/ahead-count claim or additional
mutation authority. PR/create/submit controls retain opaque bordered surfaces
and distinct light/dark hover/focus. Submit stack remains in the column menu and
beside Push for an already submitted top bookmark. Submit/Create PR, Push and
branch action-bar icons share the compact PR-button height and keep their
existing command guards. PR details and file diffs share one adjacent details
column per origin, mirroring the Op Log's separate cards: an expandable PR
section first, then the selected file diff. A badge opens this column without
requiring a file; selecting or switching files keeps the relevant branch's PR
above its diff. Collapsing the PR preserves the file view; closing only the file
preserves the PR. Stack cards contain no duplicate details section. The PR
header owns its title, linked number, draft/state chip and pencil, float, checks
and close icons. The body renders branch relationships and safe Markdown,
omitting repeated title/number/state and repository-name metadata. The compact
header menu owns Draft, Restack, provider-supported merge methods and explicitly
confirmed guarded merge. An accessible pencil reveals title/composer editing,
with no redundant Description label. Inspect checks expands the PR as needed and
toggles the existing revision-bound checks/jobs/logs below the description in
its own body, including while floating. Checks start hidden; inspection opens no
extra column and preserves the file diff, draft and selected job/log view.
Missing, incomplete or stale checks never imply success. Header actions have
distinct themed hover, open-menu and keyboard-focus surfaces, retaining disabled
guards. The title disclosure stays transparent in both hosts and is interactive
only while docked. Capability and pending guards, merge protection and explicit
Restack remain unchanged.

Creation and PR details can float and redock through one application-owned,
forge-independent detachable-panel adapter over released Design Core floating
windows and its portal. The same content/editor instance moves between hosts.
Drag, resize, minimize, maximize and dock/close reuse the released window
controls. Escape redocks only the focused panel; nested menus handle Escape
first. Focus returns to the originating control, bounds stay within the viewport
and teardown removes the floating host. Floating layout and drafts stay
session-only. Panel layout never owns credentials, publication intent or
receipts. Connection/session controls and inventory remain in the existing PR
column menu. The adjacent inspection/submission column retains its shared header
height, background, accessible icon-only pencil/close controls, composer
previews and shadcn merge-method menu. Operational failures use shared toasts.
Bounded polling, selected-revision checks and log isolation remain
authoritative.

## Delivery and evidence

M31 owns implementation and qualification. Tests for this work are restricted to
changed components and their contracts, per the user's 2026-10-15 instruction.
Wider test-suite repairs and baseline approval are deferred. Live provider and
platform qualifications require actual evidence and cannot be inferred from
mocked adapter tests.

## Qualified bounds and recovery

The current implementation supports at most 64 segments/navigation entries, 128
submission and 128 Restack receipts, 1,024 PR-action receipts and 8 MiB of
private device state. Known PR observation is bounded to 256 IDs. Capacity
exhaustion fails explicitly; receipts are never silently evicted. A connection
with history cannot be rebound to another project. Journal archival/migration is
future work.

The active repository Stacks controller polls every 15 seconds, backs off to 60
seconds on failure and watches at most 16 visible branches with concurrency
four. Closing its owning repository stops polling. Description maintenance
operates while that controller is active or the CLI explicitly requests status;
there is no always-on backend maintenance daemon. Historical CI selection and
unsaved descriptions remain retained.

Shared-root or cross-combination Restack is unavailable pending qualification.
Conflicted Restack retains the applied native operation and stops publication.
Unknown rerun/cancel outcomes remain unresolved when the provider supplies no
proof; checking never automatically repeats the action. Eligible same-repository
GitHub stacks require exact ordered membership. Merge queue acceptance and its
receipt are durable; only provider observation establishes merged state.

## Comparison and next qualification

The
[comparison and rollout record](../../../history/2026/10/15/E-FORGE-STACKS-001/comparison-and-rollout.md)
records the source revisions, borrowed patterns, capability differences and
remaining acceptance work. Future adapters extend the owned provider boundary;
GitHub-specific stack membership never becomes the common stack model.
