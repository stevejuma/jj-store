# Explicit Git interoperability

Native jj-store remains the working repository. Git fetch and push are explicit
CLI operations over a private disposable bare cache. The attached bridge
proposal is reference material; these requirements own the accepted contract.
The bridge follows GB0 through GB6 as M12.1 through M12.7 in the single ledger.

## Requirements

| ID          | Requirement                                                                                                                                                                                                           |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-GIT-001 | Parsed native Git commands MUST route through the existing CliRunner hook and workspace loader; ordinary backends MUST retain upstream dispatch, with explicit native capability errors before unsupported effects.   |
| JST-GIT-002 | Accepted exact Git representations, mappings, observations and attempts MUST be retained by native immutable storage and manifest CAS; deleting all private Git caches MUST preserve identities and recovery.         |
| JST-GIT-003 | The fixed conversion profile MUST preserve native identities and exact imported Git bytes, original parent OIDs, modes, symlinks, gitlinks and commit tags; unsupported or lossy conversions MUST fail explicitly.    |
| JST-GIT-004 | Fetch MUST verify complete bounded dependency closure and atomically publish native code and observation evidence using native ref merge/tracking semantics without replacing concurrent local work.                  |
| JST-GIT-005 | Push MUST freeze selected refs, durably prepare intent and dependencies before transport, use destination-bound exact leases, and publish only selected Git closure; multi-ref updates MUST require atomic transport. |
| JST-GIT-006 | External uncertainty and local apply status MUST remain distinct; recovery MUST inspect and repair idempotently without blind repush, and native undo MUST retain external outcome evidence.                          |
| JST-GIT-007 | Bridge facts MUST participate in native copy, checkpoints, cold recovery and qualified data-only replication; concurrent observations MUST preserve causal conflicts and replay MUST never execute Git.               |
| JST-GIT-008 | Endpoint bindings and credentials MUST remain device-local outside checkouts; transport MUST be bounded and use approved HTTPS/SSH/local paths without hooks, filters, arbitrary helpers or shell interpolation.      |

## Ownership and activation

CLI modules own parsed command adaptation, local bindings, private cache and Git
subprocesses. JJ integration owns conversion and native ref semantics. WAL owns
typed immutable records and publication; service/RPC owns authorized requests.
No transport dependency enters model, storage or WAL.

Bridge activation is explicit through native `git remote add`, never repository
opening. A receipt-bearing format-4 transition preserves every existing root,
receipt, origin and native object identity. Existing formats stay readable.
WorkStore homes remain locally supported; their origin ledger stays frozen until
a separately qualified WorkStore reducer exists. Bridge support cannot infer
permission for linked code repositories.

## Supported profile and command gates

Retain JJ 0.45.1 and gix 0.87.1. Initial transport qualification uses Git 2.55.0
and SHA-1. Preserve fetched lightweight and annotated commit tags; tag
publication remains deferred. Unsupported selected refs/options reject the whole
command before external writes. Native init/attach retains its existing
spelling; native Git init/clone does not create a new initialization contract.

GB0 establishes routing and explicit unsupported results only. GB1 qualifies
exact representations and cache loss. GB2 adds remote configuration and fetch.
GB3 adds single-ref push, dry-run and attempt status/recovery. GB4 qualifies
faults, concurrency and code-repository replication. GB5 completes tested
bookmark selectors, remote rename/remove and atomic multi-ref pushes. GB6
qualifies the cold real-RustFS external merge cycle and resource/credential
gates. No stage can advertise an unimplemented later capability.

## Identity and outcomes

Git OIDs carry their hash algorithm and remain distinct from native CommitIds,
ChangeIds, representation profiles, remote identities and request/attempt IDs.
Original Git bytes are retained before acceptance; metadata validation precedes
upstream lossy decoding. Mappings preserve aliases and reject ambiguity rather
than choosing arrival order. New native children use original mapped parent
OIDs.

A frozen push includes full ref names, selected native targets, desired Git
OIDs, known-OID/known-absent expectations and destination binding version.
Unknown expectations are not pushable. The ledger records prepared,
transport-started, result-recorded and applied states, plus
unknown/partial/apply-pending outcomes. Observed desired state is satisfaction
evidence, not proof of attempt authorship. Recovery does not roll back or
silently retransmit an external update.

## GB1 storage gate

The bridge root selects immutable Git object and representation records through
the existing radix catalog. Raw bodies use the native streamed file codec;
record validation checks OID/profile/closure and publication retains the
complete native dependency set. Missing or changed records fail cold recovery
and copy. No accepted state relies on a cache-local extra table. Code
repositories carry manifest-selected Git facts through the qualified data-only
origin reducer. WorkStore homes keep their existing origin freeze.

## Atomic acceptance and logical facts

Typed Git acceptance binds a stable request ID, immutable Git records and an
optional native head transition. The authenticated service supplies principal
and publication identity. It verifies exact Git bodies, conversions and native
closure before one guarded manifest CAS selects both the code operation and Git
catalog. CAS retries append to current immutable facts and preserve unrelated
domains and sibling operations; they never replace a stale whole view.

Logical remotes use stable IDs with immutable name/lifecycle events.
Observations bind that ID, a device-approved endpoint fingerprint, exact refs,
complete fetch scope and causal predecessors. Incomparable observations remain
available and ambiguous expectations cannot authorize a push. Complete absence
applies only to the recorded scope. Neither record contains an executable URL or
credential. Read/status calls pin the Git root and never reconcile operation
heads.

## Device and transport boundary

One invocation-owned native factory registry may bind exactly one pending Git
acceptance to its OpHeadsStore. Ordinary snapshot/reconciliation runs before
that binding, and normal JJ transaction finish supplies the resulting operation
and working-copy update. There is no process-global pending request. A no-op
fetch publishes no code operation. Private cache attempts never become native
working repositories or source checkouts.

The local GB2 slice supports one named remote, default selection and literal or
single-star bookmark filters. Transport captures a bounded complete heads/tags
namespace in one fetch and filters its completed refs before native conversion.
This proves literal absence without an earlier advertisement race; omitted
bookmark refs remain untouched. Commit tags retain original target and peeled
commit identities. Unchanged refs do not introduce visible heads or run a merge,
and a repeated unchanged observation publishes neither code nor bridge state.
This local slice does not qualify code-origin activation or push.

## Durable push attempts

Push attempts are immutable native Git records with an authenticated executor,
frozen source operation, explicit bookmark refs, destination fingerprint and
causal observation bases. Prepared representations and the intent MUST be
manifest-selected before the transport-started event is committed. Events retain
causal predecessors and per-ref acknowledged, observed-satisfied, rejected or
unknown outcomes. An applied event names the accepted observation and is
selected with the native ref transaction. All stages use stable request
identities.

`jj store git status` reads durable attempts without reconciling code. Explicit
`jj store git recover <attempt>` checks the original device binding, inspects
refs and repairs satisfied native bookkeeping. It never sends another push. An
interrupted started attempt remains externally unknown until evidence is
recorded; cache deletion and native undo cannot remove its intent or results.
Unsupported selectors, signing/conflict profiles and push options reject before
transport. Dry-run performs bounded planning/conversion with no intent, ref
publication or external write.

The initial existing-bookmark push profile additionally proves that a new
intent's source operation belongs to accepted code-operation ancestry and its
frozen targets equal that operation's native bookmark fields. A staged operation
or arbitrary caller-selected target is insufficient. Retained intent replay and
cold verification use immutable view evidence; later native undo does not erase
external evidence. Generated selectors need their own qualified source proof.

## Data-only Git origin reduction

Git origins select an immutable cumulative Git catalog and complete strong
native closure. Receivers form a canonical union of records, reject identity
reuse with different bytes, verify original Git bytes and native mappings
through the pinned JJ owner, and retain concurrent observations as causal
alternatives. New push intents must also prove their source against accepted
code-origin history and the frozen bookmark view. Inventory presence alone is
not source admission.

The origin optional Git root is omitted from older envelopes, preserving their
original digests. Git origins use semantic format 2 and require reducer 2 in
peer negotiation. Git roots and objects are immutable transferable data;
endpoint bindings, credentials, hooks and transport commands never cross this
boundary. Import, checkpoint, copy and cold recovery execute no Git subprocess.

A combined code/Git origin remains one manifest CAS visibility unit. Replay
returns the original destination receipt without changing WAL or domain
versions. Format-4 seed checkpoints retain the complete Git catalog alongside
the existing code, AgentStore and ReviewStore baseline. WorkStore seeds and
writable replication remain unsupported.

A first-origin seed is the explicit accepted-state baseline, with source-side
admission checked against retained committed WAL history before authoring. Its
receiver verifies immutable view/conversion evidence without reconstructing
pre-seed code causality. Later origins prove new intents against accepted code
origins. This keeps an undone but externally effective attempt recoverable
through seed hydration without resurrecting its old code head.

Normal acceptance and origin reduction require a push event's intent and causal
event predecessors to have been selected by an earlier acceptance. Bundling
transport-started and acknowledged-result facts cannot bypass durable ordering.
A seed retains previously accepted history under its explicit baseline profile.

## Bookmark publication profile

GB5 qualifies bookmark patterns, revisions, all/tracked/deleted bookmark
selection, change-generated and explicitly named bookmarks, plus atomic
multi-ref transport. This profile publishes bookmarks only; tag publication and
Git push options stay unsupported. A selected unsafe ref rejects the native plan
before external writes. Every update retains its own exact destination lease.
More than one update requires remote atomic capability, with no fallback to
partial publication.

Generated selectors resolve names and targets once, validate the complete plan
and conversion, and then explicitly prepare the local bookmarks in an accepted
native operation before the durable intent. That frozen operation's bookmark
fields provide the same source proof as existing bookmarks. Dry-run publishes
neither generated bookmarks nor an intent. Interrupted publication retains the
prepared local bookmark and durable attempt for explicit recovery.

Remote rename preserves its logical identity, destination fingerprint and causal
facts while atomically moving native tracking metadata. Removal tombstones the
alias and removes only its native tracking view; local bookmarks, immutable
objects, origins and attempts survive. Reusing the name creates a new logical
identity and cannot inherit the removed remote's leases.
