# jj-store-core

Injectable AgentStore, WorkStore, ReviewStore, ThreadStore and ReactionStore
interfaces, canonical record codecs and shared immutable projection engines.
This crate is independent of upstream JJ, native storage and application hosts.

Register implementations by name in `StoreFactories`, select each domain with
`StoreConfig` and open them with a host-owned `StoreContext`. `StoreHost`
supplies admission, actor identity and time. `RepositoryStores` coordinates
commands that need compatible participants in one publication provider.

Use `jj-store-git` for Git companion persistence or `jj-store-native` for the
coordinator's manifest/WAL publication. Detailed ownership and persistence
context lives in the
[architecture guide](../../docs/site/technical/architecture.md) and
[publication guide](../../docs/site/technical/publication.md).
