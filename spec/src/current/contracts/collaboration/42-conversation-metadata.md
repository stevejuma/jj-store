# Conversation details and generated metadata

JST-COLLAB-034 through JST-COLLAB-036 extend to immutable application metadata
selected by an optional ThreadStore metadata catalog. AgentStore owns captured
transcripts. Metadata references exact session snapshots; it never copies raw
transcripts, creates human messages or assigns a primary session Thread.

Targets distinguish live conversation identities from recorded sessions within
one repository. Known live/capture aliases resolve to one target. Every revision
retains its owner Thread, actor, request, predecessor, custom title, generated
metadata and per-participation ACP suggestions. Publication validates source
membership through the same native/local authority and durable receipts. Absent
catalogs preserve canonical legacy bytes. Tombstoned owners are not searchable;
native capability refusals remain unchanged.

Display precedence is custom title, generated title, unambiguous ACP suggestion,
first submitted-message excerpt, then New conversation. Agent names remain
captured identity. Explicit title regeneration adopts the result only at its
original title precondition; metadata refresh preserves a custom title.

A dedicated installation provider/model configuration defaults to unconfigured.
The bundled runtime performs one bounded tool-free summarization request with no
ACP session, coding workspace, MCP connection or ordinary conversation turn.
Managed compute must already be running. Credentials retain existing custody.

New conversations generate once after their first committed answer and settled
recipients. Old history, GETs and restart never initiate inference. Subsequent
committed content marks results stale. Jobs freeze their model and source
snapshots; duplicate requests reuse the same job. Unknown results never trigger
automatic inference replay. Retained results resume only receipt-backed
publication. Failure preserves prior metadata and title edits.

The version-1 prompt treats transcript data as untrusted. It requests a title,
factual summary, keywords and goals, uses delivered comments and concise tool
headers, excludes tool bodies and hidden reasoning, and records
partial/truncated coverage. Metadata is searchable through bounded authorized
projections. Native metadata preparation reads exact captured activity without
reloading display-only feedback or discovery enrichment. Source authorization,
immutable snapshots and bounded capture coverage remain enforced.

An owned `jj-store.sent-context.v1` human-message envelope contributes only its
authored text. Inspector payloads, bootstrap instructions and captured context
are excluded. Rich text parts use declared visible text; unknown JSON formats
are not inferred from arbitrary nested fields.

M24 remains In progress. Live-provider, native/local cold restart and supported
transfer evidence remain separate from fixture, type and interaction acceptance.

ACP supplies optional session_info_update.title observations; it defines no
standard title-regeneration request. The app records per-participation
suggestions and their committed snapshot provenance. Multiple differing
suggestions do not win a shared conversation title by arrival order. Generated
metadata is application-authored enrichment, independent of agent answer tags.

| Surface                        | Delta/ACP reference                                                | Application contract                                                                      |
| ------------------------------ | ------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| Details and title regeneration | Delta exposes details/regeneration; ACP optionally reports a title | Shared details, conditional rename/reset/regenerate and immutable provenance              |
| Summary, keywords and goals    | No portable ACP discovery-generation operation                     | Dedicated tool-free provider completion and repository-backed values                      |
| Comments-only exchange         | Delivered comments can be prompt context                           | Include only delivered immutable comment/reply bodies, excluding drafts/unstarted context |
| Refresh/restart                | ACP title suggestions may arrive with normal activity              | No GET, settings-change, history-open, restart or old-conversation backfill inference     |
| Agent identity                 | Harness identity is separate from a thread title                   | Captured agent name/avatar remain unchanged                                               |

Publication freezes the metadata revision, preserves concurrent title changes
and retains source snapshots even when newer capture makes the result stale.
Local publication validates those exact sources against the selected committed
AgentStore inventory, including retained snapshots outside the active session
closure. Unselected staged objects remain inadmissible; public snapshot reads
retain their pinned-closure checks. Completed receipts remain bounded; the
journal retains at most 4096 requests. At most eight inference jobs may be
active/queued, serialized through one worker. Cancellation has its own identity
and cannot retract publishing/committed state. An unavailable or malformed
generation preserves prior repository metadata. The version-1 helper performs
exactly one HTTP attempt; unknown inference is never retried by the provider
transport or host. Explicit retained-result publication after a known stale
refusal uses current metadata preconditions and keeps the newer custom title.
Unknown publication uses its original receipt.

See [Metadata API](../api/43-conversation-metadata.md),
[Details UI](../ui/38-conversation-metadata.md) and
[Metadata CLI](../cli/37-conversation-metadata.md).

Historical local capture directories enrich from their requested committed
companion snapshot; metadata lookup never forces a current-head precondition.
Stopping an unknown generation with no publication request only ends local
waiting, retains its original identity and does not claim remote cancellation.
Unknown publication cannot be discarded through that action.
