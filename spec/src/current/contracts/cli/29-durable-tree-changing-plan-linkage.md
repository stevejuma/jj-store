# Durable tree-changing plan linkage

The installation-private plan owner MUST use an explicitly created, separately
locked catalog outside checkouts, scoped to the command journal's installation,
repository, physical workspace, JJ workspace and profile. It retains at most 128
canonical plans, 32 MiB each and 64 MiB aggregate, without eviction. Reads never
initialize missing state. Exact replay syncs selected bytes and conflicting
request reuse fails. Unknown replacement or sync acknowledgements remain
unknown.

Journal composition MUST authenticate and find the original request before
touching plan storage. With lock order journal then plan store, validate the
original prepared key, recovery, retained commit/archive, native candidate,
physical selection, source/candidate trees and ordered affected paths. Persist
and sync the plan before selecting only its digest and encoded length in a
version-4 journal. Preserve version-1/2/3 bytes when no plan reference exists.
Tree-plan references and unchanged-tree checkout witnesses are mutually
exclusive; new references require retained Prepared state. Terminal retries may
read an existing reference but cannot attach a new one or reset an outcome.

Tree-changing local publication requires the saved reference. Loading MUST
resolve and validate the exact original plan before provider revalidation; no
replacement may be generated for missing or inconsistent state. These private
inputs do not attest publication, physical installation, selected combination
after-state or command success, and add no route or writable capability.

The private CLI composition fixtures additionally qualify complete original
physical inventory selected alongside the exact saved plan. Journal version 5
references inventory digest, encoded length and plan digest after catalog
version-2 body sync. Original binding/authentication precedes catalog access;
new changed-tree publication requires both references. Legacy inventory-free
records retain bytes and read behavior, with no terminal upgrade or missing-body
repair. These library paths add no CLI syntax, installed binary replacement or
production mutation capability.
