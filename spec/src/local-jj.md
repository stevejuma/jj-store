# Local Git-backed JJ collaboration

M13 adds an owned companion metadata store to ordinary Git-backed JJ
repositories. Upstream GitBackend and JJ operation publication remain code
authority. Native repositories continue to use their existing manifest, WAL and
immutable objects. The local profile shares the domain models, service policy
and presentation.

## Requirements

| ID            | Requirement                                                                                                                                                                                                                                                                                                                                                                     |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-LOCAL-001 | Local collaboration MUST use immutable Git objects selected only by expected-old-value publication of `refs/lapis/state/v1`. First metadata write initializes identity atomically; reads do not initialize. Independent domain versions, original request receipts and selected history MUST survive restart.                                                                   |
| JST-LOCAL-002 | Selected metadata and exact code evidence MUST have real Git reachability, including nonstandard JJ dependencies and archived operation/view/extra data. Upstream GC and loss of derived indexes MUST NOT invalidate retained anchors. Unknown formats and incomplete closures MUST fail closed.                                                                                |
| JST-LOCAL-003 | Server-free CLI and local UI MUST share owned domain commands, validation, limits, sanitization and stale-version semantics with the native profile. Native codecs and identities MUST retain their existing interpretation. Git identities MUST use explicit profile-aware types.                                                                                              |
| JST-LOCAL-004 | LocalTrusted serving MUST open explicitly registered folders without requiring RustFS, pin explicit operation and metadata snapshots, and expose versioned capabilities, queries and existing applicable code mutations. Reads MUST NOT reconcile heads; displayed snapshots change only on explicit refresh.                                                                   |
| JST-LOCAL-005 | UI code commands MUST journal the exact unpublished JJ operation before upstream head publication, retain stable request identities and unknown outcomes, preserve concurrent siblings, and never resurrect undone or abandoned operations. Checkout files remain under foreground JJ handling.                                                                                 |
| JST-LOCAL-006 | Local import/export MUST create independent destinations, preserve all collaboration history and exact referenced evidence, report identity mappings, initialize a fresh destination operation log, and verify complete bounded closures before final publication. Source state and existing destinations MUST remain untouched; unsupported active projections fail preflight. |

## Publication profile

Initial qualification is JJ 0.45.1, Git SHA-1 and macOS ARM64. The owned ref is
outside branches, tags and upstream `refs/jj`; ordinary Git and JJ network
commands do not transfer it. No metadata synchronization is introduced.

Metadata commits retain preceding selected metadata commits through parent
edges. Domain bytes and immutable evidence use actual tree entries; retained
code commits use parent edges and nonstandard tree/blob dependencies use
explicit tree entries. JSON object IDs alone never establish retention. Staged
candidates are invisible. Publication errors resolve through selected receipts
where possible and otherwise remain unknown; they are not automatically treated
as CAS conflicts.

Native V1 records remain readable and unchanged. New anchor versions distinguish
native and Git evidence. Metadata and code publications are separate
transactions, not one atomic snapshot. Ordinary code undo does not rewind
collaboration state. Device paths, credentials, access grants and active
execution journals are not portable metadata. Source actor identities remain
provenance after conversion.

## Delivery gates

ReviewStore shared-engine extraction preserves native root bytes and all native
cross-object audits. A reusable catalog or reader alone does not qualify local
review publication, portable artifact validation or review CLI support. Shared
command projection MUST retain all native catalog, relation, comment ownership
and linear/merged event reconstruction checks. External dependency verification
and publication are qualified separately by each provider.

Local ReviewStore publication uses canonical repository-bound records and a
selected record-to-Git-blob inventory. Root reads audit selected retention and
the complete shared review closure; staged objects remain unavailable through
pinned reads. Command receipts precede entity preconditions, ref races reapply
typed intent to the selected root, and review updates preserve other domains.
Repository-wide threads can initialize metadata on first write. Until portable
code and agent anchor validation/retention is qualified, those targets MUST fail
closed rather than substitute repository or display-only identities.

Local WorkStore pagination reuses the native record and four-MiB page limits.
Continuations bind an immutable selected metadata commit, filters, page size and
offset. A continuation can only name selected history, never a staged candidate;
it remains usable across CLI processes while that bounded history is retained.
Actor identities are device state under the shared JJ repository, so physical
workspaces and the local UI use the same actor. They carry no access grants and
are excluded from portable transfer.

The local AgentStore uses the existing canonical repository-bound immutable
record envelope and one shared normalization, transcript and catalog engine. The
selected domain descriptor binds the complete record-to-Git-blob inventory;
reads audit retained membership and expose only the pinned root closure. A
sanitized capture request binds its original batch and profile before reduction,
so receipt replay precedes overlap checks and generated object identities do not
alter intent. A ref race retries against the selected root; source overlap still
fails stale. Publishing AgentStore preserves WorkStore and other domains. Native
V1 code associations require native evidence and are rejected by this provider
unless a native evidence provider is available. Portable association/checkpoint
commands bind original records, exact capsule bytes and expected immutable
selections. The provider validates source identity, operation/profile and commit
membership, retains the complete physical Git closure and selects the evidence
inventory alongside the AgentStore root. Missing, mismatched or unselected
capsules fail closed. Corrections retain prior versions and require exact stale
preconditions; receipt replay precedes those checks.

CLI provenance publication synchronizes its complete typed request in private
device state before companion ref CAS. The journal binds repository and actor;
retry cannot recapture evidence or substitute another record under the same
request identity. Explicit-operation evidence inspection stages only immutable
objects and does not select metadata or alter JJ operation heads.

CLI hook ingestion retains the existing sanitizer-before-durability boundary.
Its shared private spool persists the complete local typed request before a
companion ref publication, checks the receipt's repository, actor, request,
intent and batch, and removes an entry only after a matching committed result.
Unknown outcomes retain that exact request across process restart and block
later source batches. Native prepared deltas and local prepared requests cannot
be delivered through each other's provider. Local session continuations bind
selected metadata history and stay usable across CLI processes.

Local recovery uses the same per-spool drain lease as native recovery and takes
the capture mutation lock only for bounded local journal I/O. Companion
publication never holds the enqueue lock. Cancellation and unknown outcomes
release the drain lease without discarding the exact prepared local request.

Portable evidence uses an explicit source profile and SHA-256 capsule identity.
The Git profile records the complete selected operation and view, including
predecessor maps, and the complete backend commit metadata needed by their
bounded ancestry/evolution closure. Operations and views retain their exact
upstream content identities. Git commit/tree/blob identities remain SHA-1; the
all-zero upstream root sentinel is never treated as a physical Git object. This
archives source context without installing or replaying an old operation graph.
Native V1 envelopes and codecs remain unchanged.

Portable agent provenance models retain typed operation capsules and commit
evidence pairs. Code changed/no-change/not-observed is validated independently
of capture completeness and execution outcome. Shared non-code policy does not
grant a provider permission to accept missing source capsules: local persistence
must qualify selected evidence and its retained closure separately.

M13.1 proves versioned publication, receipt replay, initialization races and Git
reachability before dependent implementation. M13.2 adds WorkStore and direct
CLI access. M13.3 adds portable AgentStore/ReviewStore evidence. M13.4 adds
direct local serving, code commands and browser presentation. M13.5 adds local
transfer. M13.6 qualifies cold end-to-end behavior and native regressions.
Milestone status and actual command evidence live only in milestones.md.

## Pending capabilities and acceptance

The landed companion foundation, WorkStore CLI, agent capture/provenance and
repository-wide review provider do not complete the local collaboration profile.
The remaining capabilities below are unimplemented or unqualified. Their status
and command evidence are recorded only in [the milestone ledger](milestones.md).
Local review capability advertisement remains disabled (`reviews: false`) until
the portable anchor and CLI gates pass. Only repository anchors are qualified
for local review persistence; native code/agent anchors fail closed. Metadata
network synchronization is outside this profile.

| Milestone | Pending capability                                                                                                   | Acceptance gate                                                                                                                                                                                                                                                                                                                     |
| --------- | -------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M13.3     | Portable code and agent review anchors, backend-aware review CLI, bounded review detail/pagination and durable retry | Validate exact selected capsules and retained Git closures; preserve stale semantics and original receipts across restart, unknown outcomes and ref races; reject missing or unsupported evidence.                                                                                                                                  |
| M13.3     | Automatic hook code-causality associations and native portable capsule carrier                                       | Associate original sanitized captures with observed explicit code operations without inventing causality; qualify native retained capsule publication and replay. Native providers continue rejecting portable records until that carrier passes.                                                                                   |
| M13.4     | Filesystem-only LocalTrusted server/UI, explicit device folder bindings, versioned local APIs and code reads/search  | Serve registered ordinary JJ folders without RustFS, pin explicit operation and metadata selections, preserve separate code authorization and require explicit refresh. Qualify cold startup and mixed native/local use.                                                                                                            |
| M13.4     | Recoverable local code mutations                                                                                     | Durably journal the exact unpublished JJ operation before publishing heads; prove stable unknown/retry outcomes, concurrent sibling preservation, undo/abandon safety and protection of dirty unsnapshotted checkout files.                                                                                                         |
| M13.5     | Bounded durable local/native conversion and identity mapping reports                                                 | Create an independent destination with a fresh operation log, selected code state/ancestry, complete collaboration history and exact evidence archives. Reject unsupported active representations in preflight, verify bounded closures, publish native manifests last, export only to a new folder and leave the source untouched. |
| M13.6     | Cold full-flow qualification and retained native regressions                                                         | Exercise the completed local CLI, filesystem server/UI, recovery and conversion in fresh processes; retain native identity, publication, collaboration and browser regression evidence. Focused provider tests alone do not satisfy this gate.                                                                                      |

## Local browser delivery slice

M13.4 is delivered in independently verified stages. Folder registration,
filesystem-only serving, code browsing, WorkStore editing and qualified
AgentStore reads reuse the M13.1/M13.2 foundation and recorded M13.3 provider
proofs. They do not advertise unqualified ReviewStore anchors or code mutations.
Indexed search, recoverable code commands and the remaining review gates remain
explicitly pending; this slice cannot complete M13.4 or M13.

Folder inspection and all GETs are non-publishing. Explicit Add may initialize
companion identity using the existing agent initialization command; it never
installs capture hooks, creates a project or snapshots checkout files. A durable
installation-owned request precedes initialization and binds canonical folder,
shared JJ metadata path and original intent. Replay recovers the selected
receipt before making the folder discoverable. Folder removal affects only
device discovery. Each read revalidates identity and binding generation.

Local code APIs are separate from native DTOs and preserve Git SHA-1 IDs and
explicit JJ operations. Collaboration pins a separate committed metadata
selection; staged commits and unrelated cursor histories are unavailable. Local
providers expose only bounded owned queries and existing collaboration commands.
Installation-level registration is LocalTrusted administration, and local
folders are never exposed through authenticated shared deployments.

Mixed and filesystem-only startup use the same device registry and browser. The
filesystem mode requires no object-store runtime or native RPC. Local freshness
notifications announce changed operation heads or companion refs; only explicit
refresh advances displayed snapshots. An unavailable folder, changed identity or
unsupported capability is distinguished from denied access.

Format-2 server configuration selects filesystem storage explicitly; native
storage remains the default. Filesystem startup requires literal-loopback
LocalTrusted administration and rejects native backup, search and replication
configuration. Bootstrap and discovery identify the available authority.
Installation management and operation-pinned code APIs use the separate
`/v1/local` namespace, with process-bound cursors that bind repository, binding
generation, operation and query. Native URLs and native identity widths retain
their existing contracts.

The browsing stage provides owned Git-profile graph, content, references,
operation-difference and conflict descriptors. A local binding generation and
explicit operation replace native manifest positions in its code snapshot; there
is no fabricated native epoch, WAL position or publication sequence.
Content-filter revsets remain disabled until their bounded resource proofs are
qualified. File text is bounded; binary/oversized content reports its status.

### Pinned local WorkStore transport

Local browser WorkStore pages require an explicit committed companion metadata
selection, independently from the code operation. Initial pages and resumed
queries validate that selection against bounded current committed history and
reject staged or unrelated commits. Filtered summaries use the same complete
root projection, ordering, counts and bounds as native WorkStore. Browser
continuations bind repository, binding generation, metadata selection and exact
query; changed filters or another repository fail closed.

Local browser edits and status use the existing CLI device actor and original
typed WorkStore request. Receipt replay precedes stale entity checks. Transport
timeouts after dispatch retain the exact request as an unknown outcome; removal
or replacement of a folder binding cannot grant access to a retained page.

Admitted browser collaboration facades bind the original repository identity.
Every write checks that identity before device actor initialization and on each
publication retry; a replaced companion root is stale rather than another home
to which the original command may be applied.
