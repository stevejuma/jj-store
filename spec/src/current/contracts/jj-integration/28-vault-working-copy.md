# External vault working copies

JST-JJ-003/005 and JST-LOCAL-007 retain upstream identity and operation codecs.
The optional `vault-backup` feature adds host-owned external repository and
working-copy paths through upstream JJ workspace APIs. Native initialization,
working-copy locking, immutable tree writes and retained checkpoint transactions
stay in `jj-store-jj`; `jj-vault` owns scheduling through owned interfaces.
Read-only source admission and ordinary immutable history queries remain
separate from this explicitly enabled writer. Existing Stacks/manifest behavior
does not change. M34 records focused proof and any user-deferred qualification.

File and whole-vault restore resolve retained checkpoint bytes, save an original
safety commit and persist receipts around guarded replacement. Native
identities, codecs and immutable read behavior remain unchanged
(JST-JJ-003/005).

Native remote transfer is separate from capture and holds no workspace lease
during network IO. Imported Git commits keep upstream native identities and are
explicitly pinned in the managed view. Fresh-device restoration uses the
existing guarded safety-checkpoint/receipt path and retains both local and
imported ancestry. Push intent and observed outcomes are device-local; immutable
readers never transfer, merge, snapshot or publish.

Guarded restoration opens parent directories with no-follow capability APIs,
component by component. It rechecks this on apply rather than trusting preview
paths. Symlink aliases cannot reach private metadata or bypass backup
exclusions.

Synchronization uses upstream native recursive commit-tree merge and an owned
`lapis-sync` bookmark. Conflict resolution writes immutable candidate trees;
materialization retains the existing safety-checkpoint/receipt path and checks
both the selected local checkpoint and saved-file tree before writing. Native
identity and immutable-reader behavior stay unchanged (JST-JJ-003/005).

Publication recovery exposes a path-free original-request summary. Observation
never sends a push, including for a prepared request or read-only account. An
explicit user stop retains a terminal stopped receipt, clears only its own
active pointer and never cancels or undoes remote state. Replays of a stopped
request cannot publish or clear a newer intent. Root relink preserves the
original request through the stable vault/private-repository binding. Native
identities, immutable reads and network/capture separation stay unchanged.

Historical revision and changed-file paging walks retained native ancestry at an
explicit checkpoint anchor. Concurrent capture cannot shift that anchor; query
pages retain full native IDs. Read queries neither snapshot saved disk files nor
create operations (JST-JJ-003/005).

Native Git credential construction accepts an explicitly selected provider login
for Forgejo. GitHub retains its existing username; header injection fails before
transfer. Native JJ identities, immutable reads, operation codecs and
publication/receipt semantics are unchanged (JST-JJ-003/005).

Managed storage relocation copies the complete native repository/working-copy
files and validates upstream operation/commit identities without re-encoding
native fields. Current native heads are never reconciled during a move. The
transport fence is independent of the capture lease, preserving network/capture
separation; storage movement takes both exclusively. Path lineage belongs only
to private binding/receipt metadata (JST-JJ-003/005, JST-LOCAL-007).
