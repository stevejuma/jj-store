# jj-store-service

Shared authorization, operation-pinned reads and typed application services for
native repositories and registered local JJ folders. HTTP, RPC and in-process
callers consume the same owned contracts and semantic policy.

Compose a `native::NativeSession` with the installation's access policy, storage
and limits, or use `local_repository::LocalRepository` with an admitted local
metadata capability. Use the domain services for commands and receipt lookup;
retain explicit historical read selections across later publications. Both
facades expose `collaboration_stores()` through the shared factory interfaces;
independent hosts can use `jj-store-core`, `jj-store-git` and `jj-store-native`
directly.

See [architecture](../../docs/site/technical/architecture.md),
[read semantics](../../docs/site/technical/reads.md) and
[transport semantics](../../docs/site/reference/transport.md).
