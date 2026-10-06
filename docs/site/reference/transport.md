# Transport semantics

The application exposes HTTP under `/v1` and native RPC through the configured
connection. [HTTP reference](http-api.md) is generated from the registered
OpenAPI contract. Native RPC serves the custom JJ integration; both transports
consume owned service interfaces and share authorization and semantic policy.

## Read sessions and pagination

Open a read against an explicit authorized selection. Files, graphs and
collaboration catalogs must retain their appropriate code operation and domain
root. GETs never reconcile native heads. Local filesystem selections also bind
the device binding generation and independent companion metadata commit.

Pagination is bounded. Native continuations are opaque and signed for their
query/snapshot/access context. Local CLI continuations can require explicit
metadata selection. Do not interpret cursors as row offsets or carry them to
another repository, credential, query or backend.

Cold prepared-history requests return `409 index_rebuild_required` with the
exact native `snapshot` or local `local_snapshot`. Start/status/cancel
index-task routes remain authorized for that repository and selected operation.
Local tasks also bind the current registration generation. Cancel has no request
body and an interrupted observation is not confirmed cancellation. Keep
observing the captured task until its terminal status; never retarget it to a
newer selection. These tasks build derived indexes and do not publish code.

Native and local `GET .../commits/{commit}/blame` require `at_op` and a
canonical `path`; optional `continuation` resumes cached JJ annotation.
Responses carry exact file identity, line count, compact one-based end-exclusive
origin ranges, shared commit summaries and completion/work information. Source
remains in its existing file response. An expired or evicted continuation
returns `409 blame_reset`; restart only the same pinned selection. Authorization
is rechecked on every request, including cache hits. Failed preparation stops
progress polling. No changed-path index is built for Blame.

Local `stack_history` queries retain an exact operation, head and optional base.
They share JJ Log's managed code index, including bounded automatic preparation
and current binding authorization. The explicit prepared-index reader remains
supported. Their opaque continuation belongs to that range and page size; raw
internal anchors are refused. The client checks the nested range as well as the
envelope and refuses oversized or non-advancing pages. This additive read
variant grants no stack mutation or physical checkout capability.

Local `working-tree/observe` is a read-only POST with a version, expected
registered binding generation and exact operation. The server resolves the
folder and compares bounded current checkout/sparse/ignore witnesses around a
two-pass physical scan. Its complete response contains changed-path summaries,
operation/view/WC identities and read fingerprints, with no physical contents or
global ignore-policy paths/bytes. Stale bindings or checkout state fail rather
than returning a partial scan. These fingerprints do not issue staging tokens,
assign changes to a combination or enable checkout commands.

### Local Stacks

`POST /v1/local/repos/{repo}/stacks` operates on the registered physical JJ
workspace. `refresh` snapshots current working files using JJ policy and returns
an operation pin. Setup, create, switch, apply, unapply and set-target commands
also name the expected operation, binding generation and local TOML revision.
Their response identifies the resulting operation and named merge/WIP commits.
Settings live in `.jj/jj-store/stacks.toml`; code stays in JJ storage. This
local path uses normal JJ transactions and checkout. Previews are advisory and
lost responses are resolved by inspecting current state/op history before
another attempt. No command receipt or automatic replay is promised. Historical
query pins continue to read their original operations.

The same endpoint accepts local bookmark, commit, graph, discard, conflict,
Undo/Redo and explicit Reapply commands. Guarded `stacks/inspect` inspection
supplies native mutability; lazy `stacks/hunks` issues selections bound to
source identity, canonical path and complete file bytes. Apply rederives those
tokens at the exact operation/configuration pin; client display hunks are not
accepted. `stacks/conflict` reads bounded native terms for the qualified text
editor. Transfers use native squash and descendant propagation, preserving
retained merge/WIP identities. Net branch comparisons grant no mutation
authority. Undo/Redo restores native operation history and leaves local TOML
unchanged.

## Mutations

Owned workspace-discovery input binds a repository, current registration
generation and exact operation. Its response distinguishes absent, ready, stale
and pending state and retains a scoped complete selected-combination catalog.
Catalogs are limited to 64 combinations, 4096 stack members per combination and
eight MiB. These records and existing-state projections have no HTTP route yet;
runtime must join the observed checkout and original pending requests before
reporting usable state. Missing configuration is distinct from an unreadable
store, and reading either never initializes setup.

The service defines versioned stack command status, pending and cancellation
inputs for future local transport. Status requires the original stable scope,
request ID and intent digest; cancellation also names the original preparation.
Results contain the original validated preview, historical receipt and replay
flag. They never carry private execution data or a replacement current snapshot.
These contracts and journal projections do not add HTTP routes or enable stack
mutations. Cancellation identity validation is not an execution protocol.

Typed commands carry a stable request identity, exact intent and applicable
expected versions. The authenticated actor is derived by the service; caller
labels cannot assign authorship. Preparation/preview and publication are
separate where the command contract requires them.

Responses distinguish committed/replayed results, conflicts or rejection, and
unknown outcomes. Preserve the original request after a timeout. Status is
currently authorized receipt inspection. `not_known` does not establish that
publication failed. Do not build an automatic retry loop that changes the
request ID after each timeout.

## Events and derived data

Committed SSE events are freshness hints. The client reopens authorized reads
rather than treating an event payload or a search hit as repository authority.
The advertised `/v1/events` feed multiplexes authorized repository and
installation domains into one session subscription. It replays a bounded
process-local cursor or sends reset when continuity is lost. Heartbeats arrive
every 15 seconds and streams renew after five minutes; unchanged renewal causes
no data reads. The client marks affected scopes dirty before accepting the
cursor and refreshes only visible consumers. Hidden scopes refresh when
accessed. Serial bounded fallback runs only when the feed is unavailable and
pauses while hidden. Historical code pins, unsaved drafts and unknown command
identities survive ordinary refresh.

Use lossless JSON handling for integers beyond JavaScript's safe number range.
Identity widths and tagged variants are part of the schema. Uploaded bodies,
query work and page output have explicit caps; exceeding one is not a reason to
truncate a typed identity or strip a conflict term.

## Admission and assets

Local requests retain exact loopback origin/protocol rules; shared requests use
current scoped credentials. Static application and Help assets have their own
allowlists. Unknown `/v1` paths never fall back to HTML, and filesystem paths
from a request never select arbitrary static files.

The ordinary local Stacks endpoint also accepts version-1 Update and Delete
commands. Update edits the active named workspace's name and target together;
Delete removes only its saved local configuration. Both validate the shown
operation and TOML revision. Native work is retained; unknown outcomes require
inspection instead of automatic replay. See
[Local repositories](../technical/local.md#local-stack-workspaces).

Local Stacks state additionally returns bounded topology at its selected
operation: ordered column heads and aliases, explicitly applied members,
integrated branches and completeness. This is a derived projection; saved TOML
membership and branch identities remain unchanged. `unapply_branches` validates
the complete nonempty, unique member list under the same guards and removes it
atomically. Reconciliation previews exact legacy merge heads eligible for
conservative retirement. Frontend focused history records live or pinned intent
in `stack_history`; every server history request still names an exact operation.
