# Typed mutations (continued 2)

M13 adds an in-process LocalRepository facade for admitted local Git-backed JJ
folders. WorkStore commands use the same neutral domain reducer and exact entity
preconditions. Local actor identity is established atomically in
repository-local device state on first write; reads never create it. Receipts
identify the Git metadata profile explicitly rather than fabricating a native
WAL snapshot.

Local AgentStore commands retain the typed sanitized capture request as their
intent, resolve selected receipts before running the shared reducer and use the
same source-order and overlap policy as native capture. Their independent state
version and canonical record inventory are selected only by companion ref CAS.
Only the canonical local AgentStore descriptor version 2 is supported. Version 1
descriptors fail closed without an in-memory upgrade or metadata mutation.
Unknown outcomes preserve the original request for retry; no code operation,
checkout or native manifest is changed by this domain command.

Explicit result-only local captures resolve exact selected native aliases
through the shared projector and retain the original request receipt through
reopen. Unmatched outputs cannot initialize fictional sessions, and matched
output does not change session lifecycle or prove tool success. Automatic server
admission remains the separately governed native LocalTrusted profile.

Local provenance commands retain the exact portable record, expected selection
and bounded canonical source capsule bytes as their intent. Shared projection
requires existing session identity and root-pinned dependency reads; corrections
retain the selected previous version. A ref race reapplies disjoint edits and
reports same-key changes as stale. Record and capsule inventories are selected
together; staging alone exposes neither.

Local checkpoint and association pages bind the catalog kind, page size, last
key and selected metadata commit. Cross-catalog cursor reuse and staged metadata
commits fail stale or invalid. Pages share the session page count and byte
ceilings and leave metadata and device actor state absent on ordinary reads.

The local session pager binds continuations to selected metadata history, page
size and the last catalog key. Staged candidates cannot be selected through a
cursor; later publications and process restart do not replace the pinned root.
Responses enforce the shared page count and a four-MiB byte ceiling. Explicit
capture delivery may establish device actor provenance after sanitization;
ordinary reads never do so.

An authorized read can produce a credential-free, immutable selected-view commit
set for search filtering. Only the service constructs this value after native
traversal and a final authorization check. It is informational, cannot publish,
and cannot replace the fresh authorization required by every search request.

Previews do not publish or grant authorization. Stale snapshots/conflicts fail
explicitly. Binary, structural, symlink, executable, and multi-sided conflicts
are not flattened into a two-sided text model. Supported term selection and
deletion are explicit; unsupported editors return a typed explanation. Server
rebase/squash/undo are deferred; client capability evidence is separate.
