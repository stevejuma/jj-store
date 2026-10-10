# Interactive agent execution

Typed installation-local Agents APIs expose runners, targets, conditional saved
setups, session preparation, prompts, permissions, cancel/end/resume, workspace
cleanup, bounded activity pages and request status. Every endpoint requires
LocalTrusted Admin admission; repository commands independently revalidate
source access and physical bindings. Request bodies accept server-issued targets
and exact operation/revision IDs, not executable configuration. SSE/live
reconnect is separate from committed repository freshness.

E-AGENT-STATE-001 leaves JST-API-001 and JST-API-006 wire contracts unchanged.
The separate Stop receipt lane uses existing command/outcome endpoints and
semantic execution events. Corrected reopen settlement, activity presentation
and code-attachment grouping add no DTO fields, admission bypass, polling or
automatic prompt replay. Journal and control formats remain unchanged.

E-AGENT-RESUME-001 also preserves JST-API-001/JST-API-006 wire contracts.
Automatic Send preparation uses existing Resume/configure commands and exact
receipts. An adapter session with no acknowledged turn is prepared as a new
context session rather than loading a nonexistent rollout. No new command, DTO,
credential, permission or prompt-retry authority is introduced.

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

Removing the reader section's surrounding focus outline leaves JST-API-001
unchanged: it changes only local CSS, retaining selection, discussion and
submission behavior without a request or persisted-format change.

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
this same unchanged JST-API-001 boundary. Icon hover regions, rounded file
headings and highlighting numbered captures preserve that boundary. The optional
display-only numbering marker adds no HTTP/ACP field; retained source text,
anchors and native formats stay unchanged. ScrollArea viewport ownership,
full-frame catalog layout and caret/draft styling also leave JST-API-001
unchanged; they add no request, persistence or dispatch.

Queued comment batches, linked replies and source navigation also preserve this
boundary: their typed submission snapshots and sent/replied states are UI
presentation contracts only. Catalog acceptance and reply completion are
simulated in memory, with no HTTP/ACP dispatch, durable comment authority or
persisted-format change (E-CHAT-BODY-007; JST-API-001 unchanged).

Resizable full-height captured-read sidebars and simplified catalog/read
headings retain this unchanged JST-API-001 boundary (E-CHAT-BODY-008). Pane
widths live only in the UI. Captured-number gutter projection leaves source text
and anchors intact. Neither introduces a request, dispatch, credential or
persisted-format change.

Footer runtime selectors and harness avatars preserve JST-API-001 unchanged
(E-CHAT-BODY-009): their choices and selections are host-owned display props.
The catalog retains simulated send-time runtime identity only in memory, with no
live catalog, settings API or ACP dispatch.

Seeded reference discussions preserve JST-API-001 unchanged (E-CHAT-BODY-011).
The catalog layers labelled sent comments and simulated linked replies over
captured content in memory, without submitting a prompt or storing discussions.
Quote typography and compact repeated reply context remain presentation-only.

Reversible captured-code middle folds preserve JST-API-001 unchanged
(E-CHAT-BODY-012). Show more/less changes only revision-keyed local disclosure
state; immutable captured text, comment anchors and API/dispatch stay unchanged.

Transcript reply sections and double-check navigation preserve JST-API-001
unchanged (E-CHAT-BODY-013). Stable comment IDs and grouped simulated replies
remain UI-only; this slice adds no live prompt protocol, parser, dispatch or
persisted format.

The decorative agent-reply prefix (E-CHAT-BODY-014) also leaves JST-API-001
unchanged: it is a display-only widget with no source-text or dispatch changes.

The live document-conversation extension is governed by
[Conversation submissions](../collaboration/41-conversation-comment-submissions.md).
Bounded typed submissions and exact source/answer links must preserve request
receipts, independent recipient outcomes and pinned authorized reads.
Qualification is recorded separately from the UI prototype.

Thread commands add create_comment_discussion and submit. Existing native/local
command routes and original request/status receipts carry these bounded typed
intents; no actor is accepted from the browser. Optional source selectors on
read Thread versions and CommentBatch link targets expose immutable source and
submission identities. Old command variants and absent-field bytes remain
readable. Submission bodies cap comment count at 128 and combined draft/comment
quote bytes at 32 KiB, independently of the ordinary message limit.

Managed conversation Send and standalone Prompt admit optional frozen comment
submissions. Prompt text may be empty when a nonempty valid submission exists.
Execution outcome reads expose optional comment publication state separately
from dispatch state, including when prompt preparation failed before admission.
The same authorization applies to receipt inspection. Omitted optional Prompt
fields preserve earlier request digests. Ordinary Thread submissions retain an
optional exact reply parent; agent prompts cannot duplicate ordinary text.

Feedback exposes exact captured string parts, discussion revisions and selected
CommentReply links. These are publication-selected facts, not browser-inferred
answers. Failed or unknown answer publication remains visible as pending
execution status and retries the original link request.

E-CHAT-LIVE-007 retains JST-API-001/003 request and event contracts. A managed
admission response in dispatching state is not an agent acknowledgement. The UI
may use matching captured events to trigger the existing exact child-outcome
GET; mismatched request/session identities cannot settle a pending submission.
This read introduces no endpoint or redispatch and preserves unknown outcomes.

E-CHAT-LIVE-008 retains JST-API-001 Sent-context contracts: the compact
disclosure and rendered/JSON toggle change only presentation. Captured sanitized
values, delivery labels, retained revisions and lossless numeric serialization
are unchanged; inspection creates no request or execution.

E-CHAT-LIVE-009 retains JST-API-001 contracts: execution stdout projections and
plain avatar status dots do not alter captured payloads, feedback source parts,
status facts or publication. A terminal-only source remains a whole-section
selector; journal display text does not create a committed range identity.

Agent defaults table composition (E-CHAT-LIVE-010) retains JST-API-001
unchanged: full-width fields and joined Advanced/action rows alter presentation
only. Model discovery, configuration drafts, explicit saves, request identities
and provider capabilities use the existing contracts without a schema or
dispatch change.

Inline Document input (E-CHAT-LIVE-011) retains JST-API-001 unchanged: it uses
the same app-owned message/comment draft and existing Send/receipt paths as
Classic. Input layout, editor selection and composition deferral introduce no
request, dispatch, authorization or persisted-format changes.

Sent-context surfaces and runtime action alignment (E-CHAT-LIVE-012) retain
JST-API-001 unchanged: resting background, full-width captured Markdown and
trailing menu placement affect presentation only. Inspection preserves captured
values and delivery state; menu movement does not alter installation commands,
request identities, authorization or persisted formats.

Repository-grouped session navigation (E-CHAT-LIVE-013) leaves JST-API-001
unchanged. Repository IDs and names are local presentation fields derived from
the existing admitted session and repository catalog. Grouping, disclosure and
selection retain exact per-chat workspace/operation links, without a new read,
execution command, binding change or persisted-format change.

Repository header navigation (E-CHAT-LIVE-014) retains JST-API-001 unchanged:
one group action opens the existing normal repository file route by exact ID. It
does not select an arbitrary session workspace/operation, alter captured session
bindings or dispatch an execution command.

Compact conversation history loading and completed empty states
(E-CHAT-LIVE-015) retain JST-API-001 unchanged: explicit presentation loading
and host-owned empty snippets consume the existing retained-event reads without
changing paging, supersession, source identity, execution, publication or
receipt contracts. Expanded Sent-context hover is also presentation-only.

Session workspace inspection adds version-1 read-only POST
`/v1/local/agents/workspace/inspect` under JST-API-001/006. A request names the
execution session, independently authorized code repository, Files or Changes
selection, optional relative path and continuation. It cannot supply a physical
root, executable or arbitrary revision expression. The retained binding selects
the owned physical workspace; typed operation-pinned native/local readers supply
committed source facts. No inspection snapshots or publishes state.

Pages contain at most 50 paths, exact selection fingerprints, continuation,
comparison boundaries and content/unavailability facts. Current listings include
tracked and nonignored untracked files within explicit traversal/content bounds.
Continuations bind the complete observation; changed observations are stale.
Metadata paths, symlink traversal and unqualified text are not readable. Deleted
files have an absent after-side, distinct from unavailable content. Branch,
Since last commit and Last turn retain the precise definitions in the
conversation contract; missing or ambiguous boundaries never imply no changes.

Local review commands additionally accept an exact WithEvidence payload for
portable file targets. Its capsule repository and immutable file identity must
match the source admitted by the provider; body, selection and evidence bounds
apply before publication. Existing transport admission and independent
repository authorization remain unchanged. A submitted capsule is not authority
to select an arbitrary filesystem root or to read staged content through an
older pin.

Conversation submissions and reply links additionally admit repository-qualified
review-comment references under JST-API-001. The trusted execution service
supplies an independent code-repository reader for these references; generic
collaboration endpoints cannot accept foreign reviews by accepting copied
bodies. Frozen context includes the exact original file anchor and bounded
source text. Receipt replay preserves publication identity, while preparing or
reading source context still requires current code-repository permission.

Workspace-bearing submissions add bounded `workspace_comments` with stable draft
IDs, session/repository bindings, exact observed revisions and fingerprints.
Empty lists are omitted for encoding compatibility. Deferred outcomes report
pending publication independently of queued delivery. Generic collaboration
commands refuse unresolved file drafts; only the authorized controller can
capture and replace them with receipt-backed repository-qualified references.

E-CHAT-WORKSPACE-005 confirms JST-API-001 unchanged: inbox-only panel ownership,
separate shell layout identities and loading placeholders do not alter workspace
inspection requests, source identities, authorization, publication or generated
transport contracts. The existing session-bound read API remains the authority.

E-CHAT-WORKSPACE-006 confirms JST-API-001 unchanged: workspace panel border,
inset and corner corrections retain the same authorized inspection requests,
immutable sources and publication contracts. No transport generation is needed.

E-CHAT-WORKSPACE-007 confirms JST-API-001 unchanged: numbered, highlighted file
readers and compact comparison controls retain exact source bytes, the existing
base selector and authorized read requests. No publication or transport changes.

E-CHAT-WORKSPACE-008 confirms JST-API-001 unchanged: the compact footer menu
copies the existing verified inspection location and workspace name. Menu
alignment, hover and copy grouping add no API, authorization, storage,
publication, checkout or agent-binding behavior.

Session status may include `workspace_name` on each code binding. It is derived
from the retained owned workspace proof, omitted when unavailable or cleaned,
and never inferred from a directory basename. Older records remain readable.
Workspace inspection reuses bounded local folder admission inside traversal;
fresh entry/completion identity and independent repository authorization remain
mandatory. Checkout revalidation can compare the exact operation, physical
metadata identity and workspace name without launching a second JJ process.
Read-only workspace watchers publish scoped `workspaces` hints on the existing
application feed, addressed to the local actor. They grant no read capability,
retain no contents and never snapshot or publish repository state.

E-CHAT-WORKSPACE-009 confirms JST-API-006 unchanged for worker delivery: open,
configure, cancellation and close frames still reach the owned harness.
Prompt-only queuing and workspace preparation must not swallow other controls.

E-UI-MENU-001 confirms JST-API-001 unchanged: normal-weight menu titles and
connection-screen Help alignment change presentation only. Menu selections,
workspace inspection, connection admission and Help reads retain their existing
request and authorization contracts; no transport generation is required.

E-CHAT-FOOTER-001 confirms JST-API-001 unchanged: preserving formatting toolbar
space and wrapping footer controls changes only layout. Composer state, Send,
runtime selections and workspace inspection use their existing requests.

E-CHAT-FOOTER-002 confirms JST-API-001 unchanged: repository button surface and
wrapped-row alignment preserve inspection, copy, selection and Send requests.

E-UI-MENU-002 confirms JST-API-001 unchanged: smaller muted menu titles alter
presentation only; menu actions, selection and execution requests are unchanged.

## ChatGPT plan accounts

JST-API-001 adds LocalTrusted Admin GET /v1/local/agents/chatgpt and POST
/v1/local/agents/chatgpt for sanitized account/authorization state and explicit
connect, cancel and sign-out commands. The application opens its fixed system
browser; responses contain no authorization URL or tokens. Account references
are additive runtime options; legacy options default to no account. The separate
temporary OAuth callback is state/PKCE-bound and is not a relaxation of API
admission. See [ChatGPT plans](../collaboration/41-chatgpt-plan-provider.md).
