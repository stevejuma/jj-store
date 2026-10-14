# Custom data index ownership

JST-ADAPT-008 adds `jj-store-read-index` as an isolated derived-data engine. It
depends on no native JJ, WAL, object-store, service or runtime crate. The
service supplies freshly admitted typed projection facts and owns request
policy, historical pins and native hydration. This dependency cannot flow back
into storage, model or WAL. The engine's redb and derived version-1 codecs never
replace authoritative persistence.

One native RepositoryHost owns the credential-free cache and persistent engine.
HTTP and RPC endpoints attach request-scoped sessions to that owner. Conversion
to another storage source resets the owner. Direct development/test sessions
retain bounded memory projections without creating files.

Only source-qualified rows may be restored. Incremental layers mask exact
replacements and removals, keep original identities and compact after four
layers. Persisted source facts, checksums and posting invariants are verified
before reuse. All source closure, authorization, native publication, receipts
and unknown-outcome contracts remain unchanged.
