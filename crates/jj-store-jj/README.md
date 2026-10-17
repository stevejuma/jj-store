# jj-store-jj

A library component of jj-store. Native repository authority remains in
manifest-selected objects and durable publication receipts.

[Repository](https://github.com/stevejuma/jj-store) ·
[Specification](https://stevejuma.github.io/jj-store/)

Experimental 0.1 releases; consult the capability and milestone ledger. Licensed
under Apache-2.0.

## Focused companion integration

Use `default-features = false, features = ["companion"]` to embed the standalone
Git collaboration provider beside stock jj-lib without native object storage,
WAL, service or application modules. `local_repository::metadata_store` resolves
an ordinary JJ GitBackend without loading or reconciling operation heads. Its
`MetadataStore::carrier()` supplies the generic Git carrier with the qualified
`local_evidence::JjEvidenceValidator` already installed. Capture exact source
capsules with `local_evidence::inspect` and pass their bytes through the typed
store commands. The default `native` feature retains the existing full adapter.
