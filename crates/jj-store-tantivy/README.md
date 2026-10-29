# jj-store-tantivy

Local Tantivy search projection for native repository content. Search readers
pin immutable index generations qualified by the selected source; indexes can be
rebuilt from repository history.

Implement `Extractor`, open a `TantivyProjection` with explicit limits and drive
it through the `jj-store-adapters` runner. Use the returned `Search` handle for
bounded, pinned queries, while the service rechecks current access.

See the [search guide](../../docs/site/guides/search.md) and
[read/index architecture](../../docs/site/technical/reads.md).
