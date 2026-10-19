# jj-store-jj

Integration with the pinned stock `jj-lib`: native object/backend adapters,
operation and view codecs, operation-pinned reads, local Git repository access
and qualified source evidence. Upstream JJ types stay at this boundary.

Native and local readers share owned budgets, diagnostics, cancellation and
worker admission. Their existing public query paths are preserved. The optional
`local-history` feature exposes the same operation-pinned filesystem history,
content and diff readers without native object storage, WAL or application
runtimes. `companion` alone does not enable history reads.

The default `native` feature provides the full adapter. For a Git-only
embedding, select `default-features = false` and `features = ["companion"]` in
the Cargo dependency. `local_repository::metadata_store` resolves the existing
GitBackend without reconciling heads; `MetadataStore::carrier()` includes the
qualified `local_evidence::JjEvidenceValidator` for `jj-store-git`.

For read-only history, select `default-features = false` and
`features = ["local-history"]`. Implement `local_browser::Access` with current
physical-source authorization and a registration generation. Create a bounded
`local_browser::Executor`, select an operation from `heads`, and use
`query::cache::managed::Namespace` to admit a private disposable index before
`prepare_managed_code_index`. Keep the prepared operation pin for subsequent
`read_code_index` requests. Neither the cache nor the feature grants source
access. Source indexes, metadata and dirty working files stay read-only.

`local_repository::inspect_workspace` resolves an existing workspace (including
shared repository links), Git storage and optional companion identity with
library-owned default settings. `WorkspaceInfo::open_metadata` rechecks those
observations; `default_metadata_store` opens an already admitted repository
metadata directory. These entry points need no upstream JJ types in the host.
They never initialize metadata or grant physical-source access.

Use `local_evidence::inspect` to capture exact source capsules for typed store
commands. See
[local repository integration](../../docs/site/technical/local.md),
[read semantics](../../docs/site/technical/reads.md) and
[codec coverage](../../docs/codec-coverage.md).

See the [embedding guide](../../docs/injectable-stores.md) and its independent
consumer example for complete construction, extension and recovery details.

For host-admitted automatic vault checkpoints, enable `vault-backup` and use
`vault_workspace::{Binding, Session}`. Bind canonical native paths and a durable
vault identity; call `capture` on a bounded worker. External state needs no JJ
entry in the vault. Completed checkpoints retain ordinary Git ancestry, while
immutable history reads remain separate. `jj-vault` provides reusable
scheduling.

`vault_workspace::RemoteTransfer` adds explicit native-approved Git backup.
Compose it with the host's in-memory credentials, transfer on a bounded worker,
then use `Session` to import and preview/restore the selected checkpoint.
`prepare_push` and `publish` retain exact-head guards and a durable publication
intent; resume its original pending identity after an unknown outcome. Network
work holds no capture lease. Forge account selection and automatic sync policy
belong to the embedding host.
