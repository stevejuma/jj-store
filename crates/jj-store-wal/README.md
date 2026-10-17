# jj-store-wal

Native repository publication, verified dependency inventories, immutable
catalogs, durable receipts and recovery. A manifest CAS selects the exact
published transaction and its domain roots.

Construct `publication::Coordinator` with an `ObjectStore`, a qualified
`DependencySource`, a `PublicationGuard` and explicit limits. Use its typed
publication and status methods with stable request/publication identities.
Readers follow captured manifest selections; checkpoints preserve original
history and receipts.

See [publication and recovery](../../docs/site/technical/publication.md) and
[object storage](../../docs/site/technical/storage.md).
