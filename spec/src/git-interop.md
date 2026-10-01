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
and copy. No accepted state relies on a cache-local extra table.
Replication-enabled code repository activation remains unavailable until GB4
qualifies its reducer.

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
