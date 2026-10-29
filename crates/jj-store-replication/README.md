# jj-store-replication

Bounded, transport-neutral exchange between writable jj-store replicas. Closure
transfer, peer checkpoints and worker state preserve the separation between
received objects and accepted repository publication.

Implement `ReplicationEndpoint` and `PeerCheckpointStore` for the host's
transport and durable checkpoint location. Configure transfer/worker limits and
use `ReplicationWorker` to drive admitted synchronization. The receiving native
coordinator still validates and publishes accepted state.

See [replication](../../docs/site/technical/replication.md) and
[synchronization controls](../../docs/site/guides/sync.md).
