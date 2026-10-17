# Embedding collaboration stores

The collaboration libraries add AgentStore, WorkStore, ReviewStore, ThreadStore
and ReactionStore alongside stock `jj-lib`. An application owns its JJ
repository loader and its collaboration store loader. It can select each
collaboration implementation independently without maintaining a JJ fork.

| Crate             | Responsibility                                                                                                                |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `jj-store-core`   | Five store traits, named factories, typed commands, pinned immutable readers, projection engines and canonical record codecs. |
| `jj-store-git`    | Git companion publication through `refs/lapis/state/v1`, immutable retention and original receipts.                           |
| `jj-store-native` | The same interfaces over the existing native manifest/WAL coordinator and its publication guard.                              |
| `jj-store-jj`     | Stock JJ integration and qualified source evidence. Its `companion` feature can be used with `default-features = false`.      |

Core has no JJ, Git, native object/WAL, service, search, transport or runner
dependency. Git omits native storage and application packages. Native retains
owned core/model/WAL boundaries. The application services compose these same
providers for their existing local and native command profiles.

## Source example

The independent Cargo workspace at
[`store-consumer`](../crates/jj-store-jj/tests/fixtures/store-consumer/README.md)
contains the executable example and its own lockfile. It uses versioned source
path dependencies and builds outside the main Cargo workspace. The new source
libraries are subject to the repository's coordinated release process;
implementation and qualification do not publish them.

Its profiles cover plain Git, stock JJ with the focused companion integration,
and the native provider. It also registers a custom WorkStore without
implementing any of the other four stores. The README gives the exact commands.

## Construct and select stores

Factories are registered by implementation name for each store kind. `empty()`
starts with no registrations, and `StoreConfig::default()` enables no stores.
`StoreConfig::all(name)` selects that name for all five domains. A host can
replace one selection or disable a domain before loading.

```rust,ignore
use jj_store_core::{RepositoryStores, StoreConfig, StoreContext, StoreFactories};
use jj_store_git::{GitProvider, LocalRepository};

fn open(repository: LocalRepository, context: StoreContext)
    -> jj_store_core::Result<RepositoryStores>
{
    let provider = GitProvider::new(repository);
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "git")?;
    // Individual registrations are also available: add_agent_store, add_work_store,
    // add_review_store, add_thread_store and add_reaction_store.
    factories.loader().open(&StoreConfig::all("git"), context)
}
```

Use the provider's `scope()` when constructing `StoreContext::new(scope, host)`.
The scope binds the repository and publication authority. When a repository
identity is already admitted, bind the Git repository with `bound_to(id)`.
Unknown implementation names, duplicate registrations and binding mismatches
return explicit errors. Selection lives in the embedding host's configuration;
loading adds no type marker or portable metadata format to the repository.

The host implements `StoreHost` to supply current read/write admission, actor
identity and a bounded millisecond clock. Actor identity is independent of the
command body. Loading and snapshots never call the actor's write preparation
path or initialize a repository/domain. The default LocalTrusted application
host retains filesystem provenance and folder admission. The native application
host retains current principal, epoch and coordinator policy.

## Implement one store

Implement `StoreBinding` and the selected trait, then register a closure with
`StoreFactories::add_work_store` or the corresponding domain method. The closure
receives the host context and returns `Arc<dyn WorkStore>`. The example's
read-only custom implementation shows the complete minimal boundary.

A store implements `snapshot`, `execute` and `status`; AgentStore additionally
supports explicit preparation and atomic Thread participation. Implementations
use `context.admit(kind, access)` at their I/O boundaries. They retain canonical
domain models and provider-owned immutable selections. Factory construction
should bind capabilities without repository I/O or identity initialization.

Store handles are exposed through immutable accessors. Substitution occurs
through factory registration and configuration before loading. A missing handle
means the domain is disabled.

## Read an immutable selection

`snapshot()` returns `Pinned<T>` with repository/provider scope, domain version,
opaque selection bytes and the immutable reader or state. Retain that handle
while paging or comparing later publications. Agent and Review snapshots expose
`AgentSnapshotRead` and `ReviewSnapshotRead`; use the shared `AgentReader` and
`ReviewReader` engines for bounded catalog and content reads. Work snapshots
provide immutable `WorkStateV1`; Thread and Reaction snapshots retain their
selected roots and graphs.

Selection bytes and provider receipts retain the provider's exact identities.
Use the provider's explicit historical capabilities when reopening a saved
selection. Git publication remains independent of JJ code operations; metadata
writes and reads preserve the source operation heads.

## Execute and recover a request

Create one `StoreRequest` with a stable request ID and typed command. Preserve
that request across uncertain outcomes. `Committed` returns a typed result and
the exact original provider receipt. `Unknown` retains the request ID; resolve
it with `status` or retry the original intent. `NotKnown` describes receipt
knowledge and does not prove rejection of a possibly delivered request.

When adapting an existing native request, retain its publication ID with
`with_publication_id`. Git rejects native publication IDs. A new native request
without an explicit publication ID uses its request identity for publication;
the host retains that identity with the request.

Native Agent capture requires preparation before execution:

```rust,ignore
let prepared = stores.prepare_agent(&capture).await?;
// Durably retain this exact preparation before attempting publication.
let native = jj_store_native::to_agent_delta(actor, &prepared)?;
let bytes = serde_json::to_vec(&native)?;
let outcome = stores.execute_agent(&prepared).await?;
```

`from_agent_delta` and `to_agent_delta` preserve the existing serialized native
request's actor, IDs, delta, opaque result and accepted Thread binding. Replay
checks its original receipt before current source selections. Repreparing an
uncertain capture can change its intent; reuse the retained preparation. Native
Agent/Review enablement remains an explicit host operation through the existing
coordinator. Empty Work/Thread/Reaction snapshots stage no roots.

## Coordinate domains

Bound Agent capture uses `RepositoryStores::prepare_agent` and `execute_agent`
so the selected Thread implementation participates. Thread, Reaction and Review
commands admit their selected live sources before staging. Disabled or
incompatible sources fail explicitly. Ordinary independent commands open only
the source domains they need; historical retained dependency closure is still
audited fully.

Compatible participants share a provider-owned `PublicationGroup` capability.
Its contract covers domain representations, captured selections and one atomic
publication. A wrapper can delegate its provider's capability when it retains
those semantics. Matching implementation names or scope strings do not create
transactional compatibility. Separate providers require separate transactions.
An independent custom store can leave `publication_group()` at its default
`None`; its own ordinary operations remain usable.

## Preserve provider qualification

The Git profile is the existing complete SHA-1 companion store. Anchored code
records require an admitted `EvidenceValidator`; the focused JJ integration
supplies the qualified JJ 0.45.1 validator. Bare Git supports unanchored records
and rejects code evidence until that capability is supplied. Separately admitted
repository-review sources retain their explicit `QualifiedReviewRead`
capability.

Native publication remains manifest CAS with WAL, immutable inventories,
original receipts and recovery. Historical receipt adaptation follows the
original selected transaction's root/version. Current roots never reinterpret an
old result. Unknown outcomes and unavailable selected receipt records retain the
original identity. Existing native portable-evidence limits and WorkflowStore
provider remain in their owning documentation.

See [publication](site/technical/publication.md),
[local repository integration](site/technical/local.md),
[capture](site/technical/capture.md) and
[architecture](site/technical/architecture.md) for the full application and
persistence contracts. Milestone M32 records the exact source, provider and
consumer evidence; these APIs do not qualify new storage platforms or releases.
