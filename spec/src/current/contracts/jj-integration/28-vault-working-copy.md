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
paths. Symlink aliases cannot reach private metadata or bypass backup exclusions.
