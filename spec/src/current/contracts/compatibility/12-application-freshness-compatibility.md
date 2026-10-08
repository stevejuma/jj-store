# Application freshness compatibility

Bootstrap optionally advertises `application_events` and additive version-1
ready/changed/reset SSE at `/v1/events`. Existing repository WAL SSE cursors and
all native/companion/object publication formats retain their meanings. Clients
use the new feed only when advertised; older servers retain bounded visible
fallback and their original repository subscriptions. The server adds notify 8.2
as a host-only filesystem hint dependency. It enters no storage, model or WAL
authority, and committed metadata checks remain required after any hint.

The version-1 application event domain enum adds `agent_execution` and
`agent_settings` for LocalTrusted Admin subscriptions. Generated OpenAPI,
TypeScript and actual HTTP/SSE captures qualify this additive boundary. Agent
journal/setup formats and native/companion publication bytes are unchanged;
clients without the shared feed retain bounded outage fallback. Cold event
acceptance remains separate from JST-COMPAT-003 release/platform qualification.

The version-1 LocalTrusted read schema gains an additive typed stack-history
request/response variant. Git SHA-1 heads and optional boundaries remain
explicit; operation/binding pins and opaque cursor admission are unchanged.
Offline OpenAPI and client generation plus actual local HTTP response capture
cover this shape. Native object codecs, companion metadata, WAL and checkout
formats do not change.

LocalStacksCommand version 1 adds Update and Delete variants for active
workspace settings and local configuration removal. Generated OpenAPI/TypeScript
remain source-derived; existing configuration version 1 TOML and native JJ
0.45.1 identities are unchanged. Focused real registered-folder captures and
cold browser acceptance qualify these additive variants, independently of
storage publication or package release qualification.

LocalStacksCommand version 1 also adds selected-file Commit and an advisory
LocalStacksPreview endpoint. Source-derived OpenAPI and TypeScript capture both
shapes. Real JJ 0.45.1 fixtures verify selected/unselected content and retained
merge/WIP change identities; TOML version 1 and native storage formats stay
unchanged. Hunk and later graph capabilities require their own acceptance.

LocalStacksCommand v1 adds CreateStack, CreateBranch, RenameBranch, Reword and
Empty; LocalStacksInspection v1 is an additive bounded advisory read. Native JJ
0.45.1 and configuration TOML v1 remain unchanged.

Additive version-1 Transfer/Split/Edit local command variants preserve TOML v1
and native JJ 0.45.1 formats. Selected tree deltas retain file type/executable
metadata through native tree values; no generic patch persistence is introduced.

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
Editor. Multi-sided, structural, mode-conflicted and oversized conflicts retain
native values and report that JJ resolution is required. Conflict paths are
bounded to 50 per current read.

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
