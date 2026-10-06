# Publication profile

Initial qualification is JJ 0.45.1, Git SHA-1 and macOS ARM64. The owned ref is
outside branches, tags and upstream `refs/jj`; ordinary Git and JJ network
commands do not transfer it.

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
