# jj-store-jj

Integration with the pinned stock `jj-lib`: native object/backend adapters,
operation and view codecs, operation-pinned reads, local Git repository access
and qualified source evidence. Upstream JJ types stay at this boundary.

Native and local readers share owned budgets, diagnostics, cancellation and
worker admission. Their existing public query paths are preserved. A focused
standalone local-history feature remains under development; `companion` alone
does not enable the browser's history reader.

The default `native` feature provides the full adapter. For a Git-only
embedding, select `default-features = false` and `features = ["companion"]` in
the Cargo dependency. `local_repository::metadata_store` resolves the existing
GitBackend without reconciling heads; `MetadataStore::carrier()` includes the
qualified `local_evidence::JjEvidenceValidator` for `jj-store-git`.

Use `local_evidence::inspect` to capture exact source capsules for typed store
commands. See
[local repository integration](../../docs/site/technical/local.md),
[read semantics](../../docs/site/technical/reads.md) and
[codec coverage](../../docs/codec-coverage.md).

See the [embedding guide](../../docs/injectable-stores.md) and its independent
consumer example for complete construction, extension and recovery details.
