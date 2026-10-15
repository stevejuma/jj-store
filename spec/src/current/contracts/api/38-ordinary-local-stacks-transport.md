# Ordinary local Stacks transport

JST-API-001 and JST-API-002 include `POST /v1/local/repos/{repo}/stacks`.
Version-1 commands are Refresh, Setup, Create, Switch, Apply, Unapply,
SetTarget, Update and Delete. Update carries workspace identity, name and target
bookmark, changes the active named workspace atomically, and preserves its WIP.
Delete carries a named workspace identity and removes only its local
configuration, retaining checkout, branch references and native unfinished work.
Commit creates a commit after an applied local bookmark and squashes exact
selected WIP files into it while retaining the WIP child. Its selections
identify source commit and repository-relative path. Advisory previews use
`POST /v1/local/repos/{repo}/stacks/preview` with the same pinned request and
return a bounded summary without snapshotting, creating configuration or
publishing JJ state. Unsupported hunk selections remain rejected until
qualified. Admission resolves the registered folder, validates binding
generation and LocalTrusted Admin/same-origin access; client filesystem paths
are forbidden. Commands validate the current operation and TOML revision.
Refresh is explicitly a snapshotting mutation; operation-pinned query reads
remain immutable. The response contains the selected JJ operation, local
settings revision and named merge/WIP references, plus any post-publication
checkout/settings warning. It has no prepared operation, archive, durable
receipt or automatic replay contract.

Stacks canvas spacing and fixed columns are client presentation governed by
JST-UI-020; they do not alter local Stacks transport or configuration. Workspace
Settings row ordering, borders and Save/Cancel presentation also remain
client-only; Update/Delete request contracts are unchanged. The compact Stacks
Details rail and conditional action visibility use existing navigation and
eligibility state without changing transport, local commands or TOML settings.

Focused Stacks repository navigation adds only browser presentation fields:
`context=stacks` is valid on existing Log/Op Log routes, and
`stack_tab=branches|workspace` is valid on Stacks routes. Duplicate or
incompatible fields are invalid; legacy Stacks routes default to Workspace.
Exact operation and commit/operation selections keep their existing read
contracts. Focused history reuses repository-wide reads, while the local Stacks
owner survives tab changes and is disposed on repository/binding/access changes.
No HTTP endpoint, generated DTO, local command or TOML format changes.

Local Stacks adds advisory POST `/v1/local/repos/{repo}/stacks/inspect` with
exact operation/configuration revision and up to 500 visible commit IDs. It
returns mutable flags and protection reasons without initializing or
snapshotting. CreateStack/CreateBranch/RenameBranch/Reword/Empty variants use
the same guarded preview and apply boundary.

LocalStacksCommand adds Transfer (exact target and source files), Split and
Edit. Advisory preview validates the same source/target policy as execution; all
source operations/configuration revisions remain exact. Whole-file inputs do not
grant display-only hunk authority.

Native local Stacks hunk selections are issued by the guarded read-only POST
`/v1/local/repos/{repo}/stacks/hunks`. Tokens bind native source change
identity, canonical path and complete before/after bytes; apply rederives them
at the exact operation/configuration pin. Client patches and display-only IDs
are rejected. Partial text deltas use native squash with emptied sources
retained, without reconstructing the megamerge. Binary, conflict, mode/type and
bounded-diff limits remain whole-file-only. Net branch comparisons remain
presentation-only.

Local graph commands use native `move_commits` insertion/reordering and full
`squash_commits` for squash/uncommit. WIP is retained as a native child; graph
rewrites do not reconstruct the merge. Branch squash/rebase discovers at most
500 commits in its segment and stops at the next user bookmark boundary. Visible
source identity, configured immutability and workspace-target protection are
validated at preview and apply; retention commits are not graph sources.

The ordinary local Stacks mutation profile now includes native bookmark deletion
(top bookmark only; lower bookmarks and commits retained), abandonment, selected
whole-file/native-hunk WIP discard, and complete native text-conflict
resolution. Conflict terms are read lazily through guarded POST
`stacks/conflict`; ordered three-term text conflicts use the existing Merge
Editor. Multi-sided text conflicts expose bounded ordered versions for in-app
pairwise Merge Editor comparison and explicitly staged complete-text resolution.
Structural, mode-conflicted, binary and oversized conflicts retain native values
and an explicit unsupported reason; deletion terms never become empty-file
selections. Conflict paths are bounded to 50 per current read.

Undo/Redo restores the native repository operation stack, including CLI work,
with native descriptions and remote tracking semantics. Current Git bookkeeping
is retained, as in JJ. TOML is not an Undo journal. Missing or restored
references remain visible as unavailable choices/reasons; Refresh and Switch
never rebuild membership automatically. Explicit Reapply saved branches
reconciles membership and preserves WIP. Operation/config/source guards and
unknown-outcome inspection remain required. Production file dragging uses exact
loaded source/path pairs; partial dragging uses issued native hunk tokens. Net
branch comparisons remain read-only. Undo/Redo remains available without a
selected named workspace.

Dependent Stacks presentation continues to issue the existing bounded history
query for its advertised top bookmark and target at one operation. Loaded native
parent/bookmark facts establish segment boundaries; net file comparisons use the
established lower boundary and head. This introduces no extra eager history
query or backend segment authority. Unknown paginated boundaries cannot issue a
misleading branch comparison.

Stacks context presentation retains matching text already returned by its
bounded selected-path diff. It adds no transport read, endpoint, schema or
command shape; issued hunk tokens and the existing per-path conflict command
remain authoritative.

The Stacks conflict comparison picker is frontend-only presentation over the
existing bounded ordered text terms. Native order, absent terms and exact
operation/configuration guards remain authoritative. Line/byte counts and a
bounded content preview are derived locally; no commit origin is inferred and no
additional transport read or command shape is introduced. Nested picker
dismissal and the full-body conflict surface remain session-only UI state.
Centered and trailing Merge Editor header snippets are presentation-only public
dependency seams. Wrap, horizontal-sync, whitespace and case comparison
preferences persist locally under `jj-store/merge-editor/v1`, containing only
version and wrap/syncHorizontalScroll/ignoreWhitespace/ignoreCase booleans, with
earlier two-field records retained and no command, content or credential
persistence.

The shared conflict file-loading skeleton consumes the existing lazy
selected-path read and changes no endpoint, request, response or mutation
contract.

Stacks presentation distinguishes `stackWorkspaceLoading` binding discovery from
`StackWorkspaceController.snapshotLoaded` workspace contents. A verified binding
seed passes `initialLoaded: false`; successful reads mark contents loaded
without resetting that state during refresh. These frontend states change no
HTTP/RPC contracts, repository identities, commands or persisted TOML.

## Forge transport

POST `/v1/local/repos/{repo}/forge` adds the version-1 JST-FORGE-001/007
contract to the same LocalTrusted bound-folder, same-origin and protocol-header
admission. Service consumers use owned forge types; only jj-store-jj imports
upstream JJ. Tokens are request/session input, never retained in the binding or
response. Status may maintain owned remote navigation; it never mutates local
code. PR details, plans, CI, merge queue receipts and original-effect recovery
remain separate typed actions. Operational failures use application toasts.

E-FORGE-PANEL-001 confirms JST-API-001 unchanged: shared column headers,
Markdown description rendering/composition and the shadcn merge-method picker
alter presentation only. Forge action shapes, expected-body/head guards,
capabilities, session credentials and exact pending request identities remain
unchanged.

E-FORGE-DETAILS-001 retains JST-API-001 action schemas and authority: opening PR
details performs the existing read-only checks action automatically, guarded by
the selected PR identity/epoch. It creates no pending mutation and cannot
replace a newer selection or an explicitly selected job view. Forge tokens
remain session-only, entered through the column's connection dialog.

E-FORGE-BUTTONS-001 confirms JST-API-001 unchanged: title-row forge controls and
their contextual hover surfaces, top-card submission beside Push and column
kebab-menu entry alter presentation only. Both submission entries use the same
preview and exact pending/conflict guards. PR selection, submission preview, CI
observation and exact pending actions are unchanged; clicking the control cannot
also select the containing branch row.
