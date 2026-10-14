# Root-pinned review store

Repository-card review activity reads MUST enumerate only the first requested
radix-ordered window (1..256 threads) of the admitted selected root and return
its thread count, explicit incomplete-count flag and maximum accepted event
timestamp. A partial window MUST NOT claim a complete repository count or latest
timestamp. The timestamp includes every selected event in each returned thread,
including merged event graphs; event sequence/order alone is not a time maximum.
Activity reads MUST NOT resolve anchors, comment revisions, conflicts or bodies
for presentation. Provider admission still audits complete selected
dependencies; this optimization does not weaken closure validation or domain
capacities. Native requests retain ordinary concurrency, timeout and fresh epoch
admission; local requests require the explicit committed companion and current
folder identity. Empty reads create no state. No durable activity aggregate is
added.

The WAL coordinator stages each validated review object in the existing
repository-bound envelope and traverses every typed ReviewStore, native code and
AgentStore strong edge before staging a root. A review root independently audits
the semantic value kind of its thread, comment, thread-event, relation and
status catalogs. Work-request, alias and source catalogs still fail closed until
their typed object schemas are implemented.

Original-anchor and exact-artifact relations are bounded immutable linked pages
of stable thread IDs and current thread-snapshot references. Status pages use
the same bounded entry shape for open and resolved threads. Catalog keys are
derived from the typed anchor, artifact or status subject. Root audit verifies
the key, page-chain subject, unique threads, current thread-catalog selection,
original anchor or artifact identity and selected workflow state. A stale
relation snapshot or an open thread in the resolved catalog fails closed.

`ReviewDeltaV1` carries expected and replacement selections for thread
snapshots, comment revisions, thread-event pages, original-anchor relations,
artifact relations and statuses. It rejects no-op edits, duplicate catalog keys
and semantic kind substitution. Applying a delta checks every expected selection
against one pinned root, rewrites only affected persistent radix paths and
stages a fully re-audited replacement root. Thread commands rebuild the affected
direct-index pages in the same review-domain publication, including moving a
thread between open and resolved status pages. Staged objects and roots remain
invisible until the shared domain publication selects them.

A `ReviewStoreSnapshot` binds exact lookup, bounded radix-ordered enumeration
and object reads to the selected root closure. Reading an unselected staged
object returns stale. Catalog enumeration accepts at most 256 selections per
call and uses the last returned stable catalog key for continuation. Old
snapshots retain their original roots and selections after later COW updates.

Local repository-wide ReviewStore threads use the same command projector and
root-pinned reader. Their companion descriptor and receipt select the canonical
root and complete retained record inventory in one review-domain publication.
Local code/agent anchors remain unsupported until their external evidence and
ownership have a qualified provider; repository threads do not qualify them.

E-CUSTOM-INDEX-001 retains fresh complete native provider audits for every first
ReviewStore request. Summary reuse follows that admission and cannot mask a
corrupt comment revision, reduced caller limit or revoked grant. No durable
review activity aggregate is added.
