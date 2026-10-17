# jj-store-native

AgentStore, WorkStore, ReviewStore, ThreadStore and ReactionStore
implementations backed by the existing native manifest/WAL coordinator. This
crate depends on owned storage interfaces and runs without application services
or upstream JJ.

Construct `NativeProvider` with an admitted `Coordinator` and a host-owned
authority namespace. Register all five implementations with `register`, or use
individual `add_*_store` methods; select them through `StoreConfig` and open a
`StoreContext` supplying authorization, actor identity and time. Loading never
initializes domains. The host explicitly enables native Agent/Review state.

Prepare Agent commands with `RepositoryStores::prepare_agent` and durably retain
the returned request before `execute_agent`. Existing `AgentDeltaRequest`
callers use `from_agent_delta` and `to_agent_delta` to preserve their exact
actor, request/publication IDs, delta, result and optional Thread binding.
Unknown outcomes retain the original request for status lookup or retry.

All publication still uses the coordinator's manifest CAS, WAL, original
receipts and recovery. Cross-domain sources require compatible selected
participants; separate providers have no distributed transaction. Native
portable capsule support and WorkflowStore injection remain outside this
interface.

See the [publication guide](../../docs/site/technical/publication.md) and
[architecture guide](../../docs/site/technical/architecture.md) for full
context.

See the [embedding guide](../../docs/injectable-stores.md) and its independent
consumer example for complete construction, extension and recovery details.
