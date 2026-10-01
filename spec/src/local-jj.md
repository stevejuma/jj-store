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

Local WorkStore pagination reuses the native record and four-MiB page limits.
Continuations bind an immutable selected metadata commit, filters, page size and
offset. A continuation can only name selected history, never a staged candidate;
it remains usable across CLI processes while that bounded history is retained.
Actor identities are device state under the shared JJ repository, so physical
workspaces and the local UI use the same actor. They carry no access grants and
are excluded from portable transfer.

Portable evidence uses an explicit source profile and SHA-256 capsule identity.
The Git profile records the complete selected operation and view, including
predecessor maps, and the complete backend commit metadata needed by their
bounded ancestry/evolution closure. Operations and views retain their exact
upstream content identities. Git commit/tree/blob identities remain SHA-1; the
all-zero upstream root sentinel is never treated as a physical Git object. This
archives source context without installing or replaying an old operation graph.
Native V1 envelopes and codecs remain unchanged.

M13.1 proves versioned publication, receipt replay, initialization races and Git
reachability before dependent implementation. M13.2 adds WorkStore and direct
CLI access. M13.3 adds portable AgentStore/ReviewStore evidence. M13.4 adds
direct local serving, code commands and browser presentation. M13.5 adds local
transfer. M13.6 qualifies cold end-to-end behavior and native regressions.
Milestone status and actual command evidence live only in milestones.md.
