# Reusable agent workflows

M34 adds the independent `jj-vault` backup coordinator. It depends on the owned
`jj-store-jj` vault interface, exposes injectable repository/progress
boundaries, and owns no UI, server, credential store or upstream JJ types.
JST-ARCH-007 ownership stays unchanged; native backup is governed by
JST-LOCAL-007.

The staged M33 extraction builds on M32's independently selected collaboration
stores. Lapis AI owns execution and source acquisition, Agent Redaction owns
native privacy, and JJ libraries own publication and conversation coordination.
Design Core owns generic Chat and Diff presentation. Hosts own credentials,
admission, context, application tools and resource lifetime.

The conversation library accepts store, runtime, context, workspace and private
journal capabilities. It has no server, transport, Tauri, search or native
object store dependency. A shared typed client adapts receipts and immutable
records to presentation without becoming publication authority. Message
publication, per-recipient prompt delivery and captured answers remain separate
outcomes. Unknown prompts never replay. Persisted agent attribution and mention
tokens do not authorize a device-local executable or restore a provider session.

Native redaction moves mechanically to `lapis-agent-redaction`. Its hook and
transcript policy IDs, scanner/rule versions, canonical output and omission
rules remain unchanged. The JJ adapter constructs existing bounded domain
payloads; the redaction library imports no JJ models. Device roots, credential
values and pending replacement prefixes remain private. Hosts admit cache
locations before construction. The TypeScript complete-block policy remains
distinct.

Source readers move separately from JJ normalization and durable publication.
Discovery is read-only and grants no repository, activity or process authority.
Source leases and acknowledged progress retain their existing identities.
Automatic discovery for ordinary Git-backed folders requires new qualification;
native discovery qualification cannot substitute for it.

The first Notes milestone supports existing Git-backed JJ vaults,
operation-pinned read-only history/diffs and a recoverable one-agent
conversation. Persistence is explicitly selected per vault. New JJ conversations
use AgentStore and ThreadStore; legacy file conversations retain an explicit
backend. No silent fallback or dual writer is permitted. Companion transport,
repository initialization/conversion and code/history mutation remain separate
work.

The Tauri host reuses physical-vault/window generation admission. Native managed
sessions have one raw event owner; sanitized durable capture precedes display
delivery and acknowledgement. Renderer subscriptions cannot take over raw
session acknowledgement. Disable and closure drain persistence before releasing
native authority. Shared presentation uses public Design Core exports with
host-owned navigation, tools and notifications.

M33's eight stages cover contracts; privacy/acquisition; conversation
extraction; local history/Tauri; Notes integration; addressed agents; explicit
import and opt-in capture; and standalone qualification. Every crate has a short
README; full documentation remains in the manual. Public package release is a
separate operator action.

M33 acquisition adapters consume `lapis-ai-capture` for approved roots, bounded
metadata walks and exact Codex header inspection. The JJ Cursor scanner retains
its required `jj-store.json` hint; the source library knows no JJ sidecar.
Existing device/inode and raw-header witness hashing remains in the JJ adapter.
Source profile inventory maps explicitly without changing serialized JJ
profiles. Availability fences cannot prove retirement after an unknown IO
outcome.

The optional `application` capture feature retains the original durable spool,
WAL request and RPC/service adapters, enabled by default for compatibility.
Without defaults, hosts receive read-only sources, sanitizers, source leases,
reader progress and exact acknowledgement types with no WAL, object store, RPC,
service or upstream JJ dependency. Hosts must durably retain sanitized records
and matching progress before acknowledging them. The feature boundary neither
supplies publication authority nor changes existing spool encodings.

The first `jj-store-conversations` slice extracts shared Control V3 stream
framing and bounded per-participant assistant/tool accumulation. A host-injected
sanitizer performs prefix replacement and final frame scanning; the library
holds undecidable prefixes only in memory. JJ's adapter preserves its admitted
native privacy policy and domain payload limits. Hosts retain one raw event
owner and journal sanitized output before publication, display or ACK.

Durable prompt coordination injects admission, context preparation, private
journal and runtime delivery ports. Journal admission either claims a new
explicit turn or returns retained outcomes. Every recipient is prepared before
any dispatch; preparation failure releases exact prior reservations. A durable
prepared fence precedes dispatch. Proven not-sent delivery releases its own
reservation; an uncertain delivery remains unknown and cannot replay. Runtime
enqueue acceptance is distinct from provider delivery and captured answers.
Existing JJ home/workspace/store preparation remains host-owned and retains its
native publication and exact request identities.

The headless TypeScript client lives in `@lapismd/jj-conversations` with public
built ESM/type exports and no UI/runtime dependency. The private application
consumes those exports through its source workspace; publishable manifests keep
registry dependency metadata. Consumer commands build the package before use;
there is no source alias. Mention tokens retain exact installation setup IDs,
not member keys or execution grants. Delivery observations preserve full-width
host sequences; existing UI adapters own any display-only numeric conversion.

Typed client transports receive immutable exact encoded commands under a fixed
host scope. A lost response retains an unknown operation; retries only read
status and never silently submit it again. Restored unknown intents cannot start
execution. Message publication, recipient delivery and captured answers remain
distinct; session-only client state never substitutes for native durable
receipts or host authorization.

The local-history extraction first separates owned query errors, limits,
cancellation, accounting and non-queuing worker admission from native store
execution. Existing native and local browser public paths re-export the same
types and preserve default budgets, diagnostic schemas and worker sharing. This
internal refactor alone does not expose an independent local-history feature or
qualify a Tauri reader; those retain separate delivery gates.

The focused `local-history` profile exposes operation-pinned filesystem read
adapters and admitted private query caches without native object storage, WAL,
application services or transports. Native backend factories, conversion,
checkout and mutation remain separately gated. Hosts still supply current
physical-source admission, private cache reservations and worker lifetime;
enabling a feature alone grants no authority or repository mutation.

The reusable Tauri history adapter injects current host admission and accepts
only opaque vault and handle identities from its renderer. Tauri derives the
actual window and URL. Source paths, cache policy, repository identity and
registration generation are native-owned. Each retained read reauthorizes its
original window/vault binding; changed generations reject old handles. Bounded
preparation jobs retain cancellation and worker ownership through exit. Close,
window destruction and shutdown drain their exact jobs before releasing
resources. Default permissions grant nothing, and the adapter grants no code
mutation, repository conversion, prompt execution or companion publication.

Native host integration reuses library-owned read-only JJ workspace discovery.
The CLI and downstream hosts share default-settings resolution of the existing
workspace, GitBackend and optional companion identity. Discovery neither loads a
working copy nor reconciles heads or initializes metadata. Native physical vault
and window admission remain host-owned; discovered paths grant no authority.

The optional native history transport uses a versioned JSON text envelope for
requests and results. Tauri carries the text without converting integer tokens
to JavaScript numbers; clients decode unsafe integer values without rounding.
The inner Rust request types still deny unknown authority fields. Envelope
version and byte bounds fail closed before admission, and output encoding stays
bounded. Direct Rust service interfaces and native identities are unchanged.

The headless `@lapismd/jj-history` client derives its types and runtime shape
checks from the native adapter's checked schema. Each instance binds one opaque
vault identity and an injected text transport. An optional Tauri entry sends
only those frames. Full-width integers use exact `bigint` values when needed;
unsafe JavaScript number inputs and malformed or oversized frames fail before
transport. Responses retain their operation and handle identities. The caller
owns handle closure and scope lifetime; the client adds no retry, polling,
filesystem access, generation, cache or execution authority.

Native capture accepts an exact host-owned scope/window/admission and
worker/open-request/target binding. Its serialized owner checks contiguous
processing ordinals and native event cursors, applies injected stream privacy,
accumulates safe projections and scans the complete output again. Deferred
prefix frames and their triggering frame form one bounded capture batch with a
domain-separated content identity. An injected durable sink must confirm that
exact identity before any safe output returns. Refusal, uncertain persistence,
receipt mismatch or an abandoned capture wait permanently fences that owner; it
cannot replay the write or select an unprocessed fallback. Private prefixes
remain memory-only. Hosts still own current admission, actual durable receipts,
native callback drain and ordered delivery of every returned frame. This port
alone neither implements a Notes backend nor qualifies JJ publication.

The conversation crate's explicit `native-privacy` feature adapts the existing
Agent Redaction transcript sanitizer to native receipt-fenced capture and
selects its exact policy identity. Defaults retain the independent coordination
and stream ports with no scanner dependency. Hosts still admit cache locations,
physical roots, exact credentials and current scope before construction; this
feature neither discovers those values nor supplies a backend or execution
grant. Scanner refusal remains a fixed privacy failure before any durable sink
or output, and uncertain receipts retain the stopped-owner fence.

The additive `owned-controller@jj-native-capture-1/v1` profile accepts one exact
schema-1 native capture envelope as sanitized AgentStore observation evidence.
Its source order is the processing ordinal; native publication cursors and
original frame sequences remain separate, unchanged fields in that envelope. The
bounded group contains one triggering frame and at most one deferred text
prefix. Projection uses the existing controller entity identities and terminal
assistant/tool records without rewriting evidence. Unknown lifecycle errors
project an uncertain session; closure projects an ended session. Policy,
binding, progress, representation and group mismatches reject before staging.
This profile supplies the domain prerequisite for NOTES-07's receipt-aware store
adapter; it supplies no journal, execution grant or Notes backend.

The conversation crate's optional `store-capture` adapter consumes only owned
Agent/Thread domain and core interfaces. Hosts supply already-selected atomic
participants, an explicit repository/thread/native binding and an exclusively
owned durable private journal. A sanitized candidate is retained before any
provider read or preparation; exact provider preparation is retained before the
single publication attempt. Matching original receipts are retained before safe
output or ACK. Existing or interrupted entries use request-aware receipt replay
only. Missing preparation, unavailable receipts and unsupported replay remain
unknown; neither a journal confirmation nor a current snapshot proves the
original intent. Recovery reauthorizes through the provider and never prepares,
executes or dispatches a prompt. Existing primary sessions cannot be silently
reassigned to another thread. Versioned bounded journal DTOs are private host
recovery records, not repository objects or publication authority. Default
conversation builds retain no domain, store, scanner or runtime dependency.

The admitted Tauri history handle additionally qualifies basic operation-log
pages and exact operation-parent deltas/grouped revision changes through its
existing standard-index reader. Requests retain the original pin and use bounded
metadata parent proofs; they do not select newer heads, load source
indexes/trees or enable a generic advanced-query fallback. This producer
capability supports Notes' independent Op Log view without adding transport
fields or permissions.

The typed history client preserves constraints beside JSON Schema references,
including native enum discriminants. Generated types intersect the referenced
payload with its sibling tag, and runtime decoding validates both. Missing or
unknown operation-difference tags must fail closed; valid wire formats and
lossless integer behavior remain unchanged.

The Notes history consumer composes its own embedded public AppShell body with
Column Canvas, owned graph/identifier ports and released Diff/FileListing
contracts. Its independent Op Log retains native pins and explicit parent and
revision comparisons. UI qualification is separate from the native event owner,
selected AgentStore/ThreadStore processor and managed conversation/restart gate.

Notes' primary history review makes room for adjacent log/diff columns through
its disposable sidebar controls, while dedicated narrow-pane coverage retains
compact navigation. The embedded shell retains its public surface tokens. These
presentation choices change no storage, admission or capture interface.

Both Notes log views start directly with public Column Canvas headers, matching
jj-store's body-title omission. Compact native pin/refresh controls stay within
the header; full identities, pinned comparisons and unavailable-state recovery
remain unchanged. Shared header qualification covers both pane widths/themes.

Embedding hosts retain exact conversation preparation before explicit
publication. For the fixed Git-backed Notes profile, recovery verifies the
original canonical provider receipt against its private actor, command intent,
repository and thread result. Bare ThreadStore status and current snapshots
cannot establish that intent. Preparation/receipt recovery does not qualify
native managed capture or renderer conversation adoption; those retain their
original M33 gates.

The Notes ordinary native factory may bind a bounded opaque capture target to an
exact confirmed conversation, then compose real native privacy, selected
Agent/Thread participants and the private receipt journal. Plans retain no idle
counted calls. A processor retains its exact native binding through verified
renderer suspension; new factories require reattachment and closure drains the
retained authority. Preparation grants no executable, credential or prompt.
Ordinary runtime dispatch, renderer adoption and managed chat/restart remain
separate from this real-factory prerequisite.

Notes' confirmed conversation reads verify original provider proof without
publishing or repairing private records. Agent/Thread data retain matching
source selections and bounded native JSON text preserves full-width integers.
Native totals and continuation sequences identify incomplete transcript ranges;
unfinished activity remains capture evidence. Retained opaque read/stream
handles pin that original immutable selection while later capture advances. Each
bounded page rechecks current native admission and original provider proof; only
issued exact decimal sequence positions continue. Hosts cap open reads, retire
handles on closure/reattachment and retain no idle counted writer call. Renderer
projection and authoring/execution retain separate integration gates.

The Notes repository-view port remains incomplete until its source/browser
parity matrix is satisfied. Matching the canvas/header and passing existing
interactions cannot prove missing native search, highlighted query completion,
metadata-rich comparison pickers, stacked operation/diff cards or exact file
statistics. The host reuses existing pinned query/detail APIs where available;
unknown statistics stay explicit until their bounded data producer is qualified.

E-REUSABLE-PROGRESS-028 records the Notes operation-card port and two-tab
review. Operation metadata and the selected-file diff share one Column Canvas
column. Application-owned source cards/rail/avatar fallbacks are attributed;
public Design Core primitives retain their package ownership. Unloaded parent
metadata uses the existing qualified pinned operation-graph profile with bounded
paging, exact IDs, stale-response fences and explicit refusal. No direct
unsupported operation request, native authority or storage/receipt format is
added. The review story retains both independently pinned logs in production
workspace tabs. Remaining comparison pickers, native query adoption, statistics
and whole-view acceptance retain their ordered open work.

The JJ Log parent picker port uses public Command View/Popover with bounded
exact-commit cards, native tags and keyboard search. Missing metadata stays on
the existing pinned reader; stale or substituted identities cannot replace the
comparison. The preview reconstructs owned App/controller instances after
runtime/fixture edits. Ordinary component/style HMR remains available. Other Op
Log comparison endpoints retain their open slices.

The Notes host owns its optional preview lifecycle/cache tools. Ignored records
retain no repository authority: process identity, checkout, selected port and
story-index readiness require current OS/HTTP proof. Unknown exit observations
retain ownership, and unready processes cannot authorize another start. Cache
maintenance admits only named dependency caches after live-consumer checks; it
cannot remove native data, source fixtures or static review artifacts.

Operation parent cards retain native user/host/time and exact operation IDs,
distinct from Git commit and JJ change identities. Header operation selectors
remain snapshot pins even when their public Command View presentation borrows
the workspace picker. Search/opening cannot switch repository or workspace
authority. Split-diff sizing uses owned hosts and public shared layout seams.

The user authorized focused production adoption of the reviewed UI. Shared
production components own presentation; catalog-only synthetic evolution and
inverse-operation previews grant no native authority. Workspace links and
operation revision cards use retained pinned readers. Conflict draft/staging and
preview cannot publish; native search/evolution and write capabilities keep
separate acceptance gates. Broad qualification and visual approval remain
separate from functional plugin adoption.

The Notes File history sidebar consumes the existing pinned log request with a
literal `files(root-file:"path")` predicate, matching stock JJ rather than
approximating history in the renderer. Prepared readers admit it only after
complete upstream changed-path coverage and bounded path-scan preflight. Tauri
may progressively extend the private index at the original handle under owned
preparation/cache admission. File history retains native graph order, merge and
deletion semantics, current authorization, cancellation and drained closure.
Status progress spans index preparation and querying. Unqualified file-body and
cross-operation predicates remain unavailable; schemas and grants are unchanged
(JST-ARCH-007, JST-LOCAL-004).

The admitted history bridge MAY expose the existing native blame reader through
a bounded version-one text command. Its exact commit/path and opaque
continuation remain bound to the retained handle's native operation, repository,
window/vault and generation. Renderer fields cannot override that authority.
Blame runs on existing query workers; closure/revocation drains the same read
lifetime. Default permissions remain empty. Notes copies the maintained blame
presentation and owns main-panel navigation, file scope and status/toast
lifetime.

File and whole-vault restore extend the injected `jj-vault::Repository`
interface with preview and apply methods. Native JJ imports remain in
`jj-store-jj`; host editor guards, credentials and admission remain host-owned
(JST-ARCH-007 unchanged).

Remote transfer remains host-owned and separate from the capture command lease.
The injected vault provider accepts an opaque fetched-head proof for serialized
native import and a guarded remote restore command on the same worker. Imported
restore publishes the resulting local checkpoint through existing status;
network outages cannot block that worker's final saved-file capture.
