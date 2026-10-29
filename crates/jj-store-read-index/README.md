# jj-store-read-index

Disposable secondary indexes for exact-source rows, ordering, facets and text
selection. An optional local disk store restores qualified derived indexes;
authoritative repository records remain in their provider.

Build an `Index` from `Row` values, adding facets and searchable text as needed.
Use `Store` with a source-qualified `Key` for persistence. The embedding host
supplies current authorization and verifies the selected source identity.

See [reads, search and refresh](../../docs/site/technical/reads.md) and
[architecture](../../docs/site/technical/architecture.md).
