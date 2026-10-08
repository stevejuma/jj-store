# Interactive agent execution

Typed installation-local Agents APIs expose runners, targets, conditional saved
setups, session preparation, prompts, permissions, cancel/end/resume, workspace
cleanup, bounded activity pages and request status. Every endpoint requires
LocalTrusted Admin admission; repository commands independently revalidate
source access and physical bindings. Request bodies accept server-issued targets
and exact operation/revision IDs, not executable configuration. SSE/live
reconnect is separate from committed repository freshness.

Status `revision` is the conditional saved-setup revision. Session activity,
publication and process shutdown do not change it; a setup save or removal does.
Duplicate original requests retain their receipt without advancing it. A
concurrent setup edit rejects the stale precondition and retains the browser
draft. Older private execution journals seed the independent setup revision from
their retained installation revision when opened.

Agent recovery toast presentation leaves JST-API-001 unchanged: the browser
matches the pending request to its exact outcome before offering recovery.
Dismissal, navigation and workspace review send no agent command. Only explicit
submission of a confirmed not-known request reuses its original body and ID; an
unknown outcome is never treated as proof that a prompt was not dispatched. No
new endpoint, request field, receipt format or authorization path is added.

Status session views expose optional original setup_id and a read-only connected
boolean. Legacy journals without setup_id load as unassigned; editing/deleting a
setup does not alter existing session snapshots. Connected is projected from a
live supervised process and completed handshake, never accepted from a browser
mutation or reconstructed from persisted idle/running history.

JST-API-001 and JST-API-006 include read-only recorded_session_ids,
workspace_name and workspace_operation on admitted local session views. The
capture normalizer's exact committed aliases supply recorded IDs; a ready
retained workspace proof supplies its name and operation. Cleanup omits
workspace values. Status clears stored projections and recomputes them for
legacy sessions after independent repository admission. These fields support
exact UI identity/navigation joins; they add no launch/command inputs or
canonical repository fields.

Optional read-only last_activity_at_millis records the device's observation time
of the latest retained execution event. It persists across restart, defaults to
unknown in legacy journals and never advances from status reads or setup saves.
Wall-clock rollback cannot decrease the retained time. No browser command
supplies or edits this field.

Setup description defaults to empty for legacy requests/journals and is bounded
to 280 characters. Avatar remains a string with bounded image/emoji forms. Only
setup writes accept a 2 MiB body; other execution bodies retain 512 KiB.

Each execution event optionally retains observed_at_millis, the device
wall-clock observation time captured when that event is appended. It is
read-only, survives journal restart/resume and publication, and defaults to
unknown for legacy events. It is not a provider-authored timestamp and does not
change native AgentStore identities or authority. Refresh, status reads and
rendering never fabricate missing times.

GET /v1/local/agents/feedback?session=<execution-id> is a bounded read-only,
LocalTrusted Admin projection of repository-selected message targets and
ordinary ThreadStore replies. It maps request/role rows through the owned
capture normalizer, validates exact message membership and captures Agent/Thread
roots in one authorized selection. It does not expose staged objects or grant
linked repository access. The response includes row_id, typed target, discussion
thread IDs and exact visible human-message revisions. Reads never create a
thread, initialize collaboration domains, apply preferences or dispatch a
prompt. Typed repository Thread creation adds create_agent_discussion with an
exact AgentMessage target. Existing collaboration commands, actors, observed
reaction slots, original request/status receipts and admission limits remain
authoritative.

Optional write_unavailable explains the current inbox write capability without
hiding readable targets or replies. Native code repositories with an existing
origin ledger and no WorkStore home retain the reducer-3 publication refusal.
The captured profile selects native or registered-folder inbox transport. Local
collaboration snapshot/query/detail, command/status and retained raster reads
use the separate /v1/local/repos/{repo}/collaboration namespace, current
companion pins, signed query-specific continuations, binding generation and the
existing device actor. Local receipts remain explicitly local; no native
snapshot fields or identifiers are fabricated. Reads recheck current selection
and registration, and query/response budgets fail closed. Raster JSON preserves
validated bounded bytes and media type without fetching external URLs. This
projection never grants write access or weakens command revalidation.

Prompt preparation returns agent_feedback_unavailable (503) when its authorized
repository discussion snapshot cannot be read, or agent_feedback_limit (413)
when the selected new feedback exceeds 128 replies/32 KiB. Both are proven
pre-dispatch refusals: the composer retains the draft, explains recovery and
allows a new explicit Send. No execution intent is saved for these refusals.
Private revision delivery metadata is not exposed as a public session field.

GET /v1/local/agents/runtimes exposes the fixed tooling catalog and
prerequisites. POST /v1/local/agents/runtimes/install accepts only version,
original request_id, runtime_id and install/update action. GET
/v1/local/agents/runtimes/outcome queries that original receipt. All three
require LocalTrusted Admin. Commands cannot supply a package, URL, executable,
path, environment or credentials. Tooling outcomes remain separate from session
outcomes and conditional setup revisions.

M24 adds
[reusable agent conversations](../collaboration/37-reusable-agent-conversations.md)
under JST-COLLAB-034 through JST-COLLAB-036. Existing M23 qualification remains
unchanged.

GET /v1/local/agents/catalog?runner=codex (or cursor) requires LocalTrusted
Admin and exposes only sanitized advertised configuration choices. Unknown
harnesses and extra directory/prompt/actor inputs are refused. The bounded
no-prompt discovery, ephemeral adapter negotiation, process-slot admission,
cleanup and memory-only cache are governed by the M24 conversation chapter.

Markdown context and its single retained inspector leave JST-API-001 HTTP and
ACP request shapes unchanged. Existing prompt and context-delivery payloads
carry the same authored text and sent-context envelope. Additional internal
section revision metadata is additive; old snapshots remain readable and are
never rewritten from current workspace state.

The standalone commentable chat-body catalog prototype leaves JST-API-001
unchanged. Immutable fixture props and local discussion callbacks introduce no
request, endpoint, schema, receipt, authorization or AgentStore mutation.
Opening captured tool details never reads the current filesystem or executes a
tool. Disclosure alignment, tool-title typography and end-navigation refinements
preserve this same unchanged JST-API-001 boundary. Middle folding and code
backgrounds change only retained presentation; they never truncate or modify
captured events. XML context rendering and caller-owned local reaction callbacks
add no HTTP, ACP, AgentStore or ReviewStore mutation contract. The optional
document-tail draft/footer submits only through a UI caller callback; the
catalog never sends an agent prompt or writes repository state. The
screenshot-based final review preserves this unchanged JST-API-001 boundary:
tool alignment, patch-line colors and captured-line classification affect only
display props and local navigation totals, with no request or persisted-format
change. Adjacent-block focus-outline clearance is a layout-only refinement under
this same unchanged JST-API-001 boundary.
