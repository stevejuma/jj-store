# jj-store-git

Git-backed AgentStore, WorkStore, ReviewStore, ThreadStore and ReactionStore
beside stock jj-lib. Immutable metadata and original request receipts remain
selected by `refs/lapis/state/v1`. Loading and reads never initialize identity.
This crate has no native object/WAL, service, search, UI or runner dependency.

Open an existing complete SHA-1 Git database with `metadata::MetadataStore`,
then create `LocalRepository` with its JJ repository directory for device actor
provenance. A non-JJ host can supply its own private provenance directory and
`StoreHost`. `GitProvider` exposes five individual `add_*_store` registrations;
`register` installs all five under one implementation name. Select those names
independently through `StoreConfig` and open them through `StoreFactories`. The
host supplies authorization, actor identity and time through `StoreContext`.
Factory selection grants no access by itself.

```rust,ignore
let provider = jj_store_git::GitProvider::new(repository);
let mut factories = jj_store_core::StoreFactories::empty();
provider.register(&mut factories, "git")?;
// Another library may register its own WorkStore without implementing siblings.
custom_work::register(&mut factories, "custom")?;
let mut config = jj_store_core::StoreConfig::all("git");
config.enable(jj_store_core::StoreKind::Work, "custom");
let stores = factories.loader().open(&config, context)?;
let work = stores.work().unwrap().snapshot().await?;
```

Use `RepositoryStores::execute_agent` for capture with a Thread binding. It
requires the actual selected Thread implementation to share the provider's
runtime publication group. Disabled stores, unknown names, mismatched repository
bindings and incompatible atomic participants fail explicitly. Separate
providers do not gain a distributed transaction from matching scope strings.

Code-anchored records require a trusted `evidence::EvidenceValidator`. The
`jj-store-jj` companion feature supplies the qualified JJ 0.45.1 validator,
including exact operation/view identities and complete source closure. A bare
Git provider supports unanchored collaboration and rejects anchored data until
that capability is installed. No new repository marker or format is introduced.

The `metadata` module exposes low-level immutable staging for migrations. Staged
candidates are invisible through selected domain readers. Prefer the store
traits for application code. After an unknown outcome, retain the exact request
and use its status operation; never create a replacement request ID.
Provider-neutral receipts retain the exact canonical provider receipt for
existing CLI and service contracts.

See the [embedding guide](../../docs/injectable-stores.md) and its independent
consumer example for complete construction, extension and recovery details.
