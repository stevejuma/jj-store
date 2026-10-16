# Transport semantics

Authenticated operator discovery and restart-bound agent configuration use
`/v1/operator/access` and `/v1/operator/agents`. Configuration writes require
`manage-agents` and the exact full-file revision. Repository onboarding uses
`/v1/repos/{repo}/agent-access` and its original-intent `/status` endpoint,
requiring current repository Admin and an enrolled token matching the specified
principal. Unknown grants retain their snapshot and request identity. These
routes are admitted by the selected-server gateway independently of local
runtime installation. See [security](security.md).

Device-local remote connections forward supported public HTTP and activity
requests only to a registered origin. HTTPS, protocol and installation identity
are checked before use; redirects, arbitrary targets and LocalTrusted authority
are refused. Tokens and connection handles remain in memory. Browser admission,
CSP and repository authorization remain enforced on both sides.

Authenticated agent APIs additionally enforce operator agent/principal
allowlists. Private native Unix sockets delegate the initiating principal's
current repository grants to admitted workspaces, without process-supplied
credentials or administrator fallback. Existing native wire and publication
formats remain unchanged.

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

The LocalTrusted `GET /v1/local/agents` session DTO includes read-only
`recorded_session_ids`, `workspace_name` and `workspace_operation`. Committed
capture aliases supply the exact recorded IDs. Ready retained workspace proofs
supply the name and operation; cleanup removes both workspace projections.
Legacy journals load with defaults and status recomputes the values. These
installation-scoped presentation fields do not modify repository AgentStore
formats, authorize code access, or dispatch execution. See
[agent execution](../guides/agent-execution.md) for their browser use.

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

LocalTrusted Admin subscriptions also receive body-free `agent_execution` and
`agent_settings` notices. These invalidate status and defaults separately;
repository/workspace changes invalidate execution targets. No prompts, settings
values or credentials are carried in the feed. Repeated internal journal writes
with unchanged visible state produce no notice.

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

## Installation-local agent execution

The `/v1/local/agents` route surfaces require protected LocalTrusted Admin
admission. Bodies contain stable request IDs, server-issued targets and exact
operation/revision selections; they cannot supply cwd, executable, environment
or actor. Current code access is rechecked for status, activity and commands.
Bounded SSE sequences expose sanitized live activity; publication counters mark
committed repository evidence separately. Lost outcomes preserve exact requests
and drafts and never trigger prompt replay.

Installation Agents status includes optional session `setup_id` and read-only
`connected`, computed from the current ACP process and handshake. Setups have an
optional/default-empty description and bounded avatar string. Only setup writes
accept up to 2 MiB; other execution request limits remain 512 KiB. Conditional
save revisions and original request identities retain their existing semantics.

Runtime settings add three independent LocalTrusted Admin endpoints:
`GET /v1/local/agents/runtimes`, `POST /v1/local/agents/runtimes/install` and
`GET /v1/local/agents/runtimes/outcome?request=<original-id>`. Installs accept
only the fixed runtime ID, install/update action, version and original request
ID. Packages, paths, URLs, environment and credentials cannot come from the
browser. Reads and receipt queries never install or dispatch an agent.

## Reusable ACP conversation controls

LocalTrusted Admin may use POST /v1/local/agents/control to Start or Stop an
exact saved agent and POST /v1/local/agents/chat for an explicit conversation
Send. Both carry stable request identities. Chat carries an origin, exact agent
IDs and separately authorized registered repository attachments. Status includes
conversations and authoritative live agent states. Stop turn, End participation
and Stop agent have different lifetimes. Generated installation API reference
documents the additive request/response fields; native manifest/ref-CAS
authority is unchanged.

`GET /v1/local/agents/catalog?runner=codex` (or `cursor`) returns sanitized
advertised configuration choices under LocalTrusted Admin admission. It starts
no repository work or application chat and sends no prompt. The installation
owns its private discovery directory and bounded process lifetime; callers
cannot supply a directory, executable, environment, actor or prompt. Choices are
cached in memory and checked again during real chat negotiation. Discovery
failure retains the agent editor draft and supports explicit Retry models.

## Installation compute and agent defaults

LocalTrusted Admin reads and writes `/v1/local/agents/settings` and
`/v1/local/compute`. Writes carry independent expected revisions and stable
request identities. `/v1/local/agents/secrets` writes a device credential; reads
never return its value. Settings persist opaque references, ordinary environment
values and explicit overrides outside repository state. Catalog queries for a
saved setup use its effective configuration.

Compute operations explicitly prepare the native runtime, download or import
weights, start or stop a model, and pair or revoke a trusted machine. A lost
operation response is checked through its original agent outcome receipt; it
does not authorize repeating downloads or inference. Private loopback inference
requires a process token, and paired transport requires mutual machine
admission. No compute operation grants repository or tool access.

LocalTrusted Admin Compute status advertises native model-file selection when
available. The empty-body `POST /v1/local/compute/pick` opens a fixed GGUF
dialog on the service machine and returns only an optional versioned path. It
saves no configuration or model weights. Cancellation and picker failures
preserve the draft; Import is a separate durable command.

## Conversation comments and answer context

ThreadStore submissions carry an exact parent revision, optional authored
message, bounded comment drafts or immutable references, and a stable request
identity. Managed Send and standalone Prompt accept these batches, including
comments-only prompts. Their outcome reports comment publication separately from
per-agent delivery. The retained Sent inspector exposes the sanitized versioned
comment-reply instructions and frozen batch references actually delivered.

Assistant output may contain `<comment-reply discussion="D1" comments="C1 C2">`
blocks among ordinary prose and tools. Only exact references from delivered
context and verified captured answer ranges can produce durable ThreadStore
links. Literal code, malformed tags and foreign references remain ordinary
content; tags never execute commands or resolve reviews.

File review references additionally identify their code repository, review
thread, comment and immutable revision. Each repository remains independently
authorized. The versioned workspace inspection POST names an execution session,
verified binding and relative path; it does not accept arbitrary filesystem
roots. Bounded listings, source fingerprints, comparison boundaries and opaque
continuations describe the exact read. Unsupported baselines remain explicit.
Comparison entries may include optional additions/deletions from bounded text
diffs. Omitted counts remain unknown, including older responses and unsupported
text. The same observation and selection fence carry these counts; the client
does not request every file again to obtain them. See the generated
[installation API](api/installation.md) and
[collaboration API](api/collaboration.md) for request and response types.

Conversation metadata is exposed through LocalTrusted Admin
`GET/POST /v1/local/agents/metadata` and `GET /v1/local/agents/metadata/job`.
Each request independently checks repository access. Commands include rename,
reset title, refresh, regenerate title, cancellation and retained-result
publication. Reads never initiate inference. Generated API/CLI references
describe exact version-1 fields and bounds.

## Device ChatGPT registrations

LocalTrusted Admin admission protects GET/POST `/v1/local/agents/chatgpt`.
Responses expose account/workspace registrations and bounded authorization
attempts; commands accept opaque references and Connect request IDs. Tokens,
authorization URLs and executable selections are not accepted browser inputs or
returned data. This application-owned interface adds no native repository
mutation or RPC credential transfer. An interrupted authorization requires a new
explicit sign-in.

Installation-local Admin `POST /v1/local/threads` explicitly creates a Thread or
conditionally edits its Codebase. Stable request IDs and retained receipts
preserve unknown outcomes. Each attached repository needs independent access.
Ordinary repository endpoints admit bounded annotation containers without agent
execution; supervisor-owned conversation creation and Codebase edits remain
installation-local.

Installation directory routes require an admitted LocalTrusted administrator or
an enrolled bearer identity, independent of repository grants.
Revision-conditional commands return actor-bound durable receipts; unknown
outcomes retain the exact request for status or replay. Identity administration
uses the separate manage-identities operator capability. Public profile lookups
omit email and verifiers. Raster assets travel through authenticated binary
responses and in-memory image URLs. Portable emoji snapshots are part of the
original typed repository command and publish their closure with messages or
reactions.

## Binary directory image uploads

`POST /v1/directory/image-uploads/{request_id}` accepts
`application/octet-stream` for PNG, JPEG, GIF and WebP avatar and emoji sources
up to 20 MiB. An enrolled user or LocalTrusted administrator is checked before
body consumption. The server limits decoded pixels, resizes large images to fit
512 × 512 and preserves JPEG orientation. Already bounded images retain their
bytes; resized animations become static. Use the existing directory command
status endpoint and original source bytes for uncertain outcomes. The JSON
upload command and retained-media bounds keep their existing behavior.

## Runtime configuration activation

`GET /v1/configuration/reload` discovers supported and pending domains for the
current caller. `POST` accepts an exact saved revision, stable request ID and
explicit domain list. `POST /v1/configuration/reload/outcome` observes that
original command without resubmitting activation. LocalTrusted Admin or fresh,
enrolled operator configuration authority is required. Results distinguish
Running, Applied, Current, Deferred, Failed and Unknown. Unknown acknowledgement
retains the original identity; reload is independent of repository publication
receipts and never grants repository or execution access.

Native WorkflowService owns definition/run reads, source-event polling and
receipt-backed commands shared by HTTP, native RPC and CLI. Execution
transitions require separate configured host admission. See the
[workflow guide](../cli/jj-workflow.md).

The version-1 local settings response includes workflow admission, optional
managed-host configuration and runtime status. Conditional section writes use
`workflow`; explicit activation uses reload domain `workflows`. These controls
require LocalTrusted Admin. Existing workflow commands and native RPC retain
their formats and source authorization.

Managed local runners derive their native identity and source attachments from
the selected service and its verified workspace directory. Job context carries
repository and revision IDs; it cannot choose arbitrary local paths. Native
workflow RPC/CLI command and receipt formats remain unchanged.

## Local forge actions

`POST /v1/local/repos/{repo}/forge` uses the existing LocalTrusted bound-folder,
same-origin and protocol-header admission. It exposes versioned configuration,
preview/submit, PR editing, CI, guarded merge and original-request recovery.
`GET`/`POST /v1/local/integrations` inventories verified GitHub accounts and
handles token registration, device authorization and forgetting. Native
credential storage resolves saved secrets in the backend; API responses expose
metadata and opaque account/session references, never saved tokens or device
secrets. Explicit CLI automation credentials remain supported. Repository
detection and account binding retain endpoint identities and exact pending
intents.

Private device receipts describe external effects and never become native
repository authority. Status can maintain an owned remote description block;
local rebases and publication require explicit Restack. See
[ordinary-local review submission](../guides/git.md).
