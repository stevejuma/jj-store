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
