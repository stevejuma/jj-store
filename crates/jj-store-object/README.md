# jj-store-object

Bounded object storage, immutable staging and conditional writes. The common
`ObjectStore` interface has a memory implementation for fixtures and an S3
implementation used with configured RustFS or compatible storage.

Implement `ObjectStore` for another transport, or use the `memory` and `s3`
modules. Supply explicit keys, byte limits and write conditions, and preserve
`WriteOutcome::Unknown` as an unresolved outcome. Repository publication is
performed by the WAL coordinator after object staging.

See [object storage and identity](../../docs/site/technical/storage.md) and
[publication and recovery](../../docs/site/technical/publication.md).
