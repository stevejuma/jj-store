# Architecture

The Tantivy boundary owns credential-free search sessions and generation
continuations over service-authorized native snapshots. HTTP and server
composition consume these owned contracts; publication remains independent.

Crates are added as milestones begin, not as empty placeholders. Native codecs
initially live with JJ integration. The server uses in-process stores rather
than calling its own RPC. Local indexes are derived and rebuildable. The source
repo is colocated for development; product storage has no Git carrier.

## Requirements

| ID           | Requirement                                                                                                                                                                                                                             |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ARCH-001 | The repository MUST provide a Rust workspace with independently owned native storage, publication, JJ integration, services, transports, adapters, and presentation boundaries.                                                         |
| JST-ARCH-002 | Model, object storage, and WAL MUST NOT depend on UI, Tantivy, Git transport, SQL, Nostr, or agent runtimes; upstream JJ integration MUST stay behind owned adapters.                                                                   |
| JST-ARCH-003 | First release MUST serve new native repositories through LocalTrusted loopback HTTP and owner-only Unix RPC backed by persistent embedded RustFS, with no required authentication, Git, SQL, relay, or external object-store authority. |
| JST-ARCH-004 | Browser and catalog source MUST enforce counted-line budgets, keep production independent of fixtures, and separate presentation from API and controller ownership.                                                                     |

Storage harness self-tests are part of normal validation. `pnpm test:rustfs`
executes only isolated disposable Docker probes and writes a scratch report. An
approved immutable report is retained with its exact image/topology.

## M0 extension qualification

M0 introduces only CLI and JJ compatibility crates. The upstream example is a
Git-backed test executable; it does not advertise a production backend. Both JJ
crates now resolve together through the committed Cargo lockfile.

## Object foundation

The model crate owns typed IDs, object kinds and envelope framing without
importing JJ. The JJ integration crate owns native schemas and identity checks.
Version 1 metadata is bounded independently of the streaming file transport.

The object crate owns the S3 transport and deterministic fault store. It depends
on the model and transport libraries without importing JJ. Upload preparation
uses bounded temporary disk space rather than buffering unbounded file blobs.

The WAL crate starts with a storage-independent bounded dependency walker. Its
source interface returns owned keys and verified descriptors; the JJ crate
implements native edge enumeration. No native JJ type crosses into the WAL.

The WAL crate owns immutable publication records, persistent radix catalogs,
manifest CAS, durable receipts, replay and checkpoints. Its catalogs depend only
on owned model types and the object transport. Native dependency enumeration
continues to enter through the verified-source interface.

The `jj-agent` crate owns stable agent identities, canonical AgentStore object
schemas and typed strong-edge validation. It depends only on the neutral model
layer and cannot import WAL, services, native JJ or presentation code. The WAL
crate stages its validated immutable objects through the existing
repository-bound record envelope, verifies complete bounded dependency closures
and selects only typed agent roots through the shared manifest CAS.

The `jj-review` crate owns stable review identities, exact immutable artifact
anchors and canonical ReviewStore object schemas. It may depend on `jj-agent`
for immutable activity-object references and on the neutral model for native JJ
identities, but cannot import WAL, services, native JJ or presentation code. The
WAL layer depends on `jj-review` for typed object, root and delta validation,
then owns repository-bound staging, root-pinned reads and publication. No
reverse dependency or second storage or transaction service is introduced.

The service crate owns repository/operator token policies and request-bound
authorization. Its publication guard uses owned WAL snapshots/settings and
requires an explicit native semantic validator, keeping authentication shared
between future HTTP and RPC adapters.

The development toolchain uses Rust 1.98.1 and includes rustfmt with a tracked
edition-2024 policy. Rust formatting is part of the shared formatting commands,
normal checks and canonical specification checks.

The released RustFS embedded crate cannot share a Rust dependency graph with the
pinned `jj-cli` because both require different unconditional sys crates with the
same native mimalloc link identity. Its exact unpatched crate therefore builds
in a separately locked helper executable. Server composition owns the helper's
private credential environment, concrete loopback address, persistent volume,
readiness and shutdown channels, and process lifetime while accessing authority
through the same owned `S3Store`. The helper is a packaged runtime boundary and
does not become a separately administered storage deployment.

Browser architecture uses Community's counted-line metric: blank and
comment-only lines are excluded; complete Svelte markup and styles count. New
production modules and scripts are limited to 300 lines, stories to 400 and
tests to 500. Existing larger files have exact no-growth baselines and must be
extracted when integration changes their responsibilities. Production web
modules cannot import catalog fixtures, and presentation modules cannot import
the API or controller layers.

The production mutation controller is an asynchronous controller concern. It
owns captured native request identity, preview/apply/status transitions, session
cancellation and committed-snapshot refresh without importing visual components.
Repository action menus and dialogs consume only the shared controller contract;
the catalog supplies the same contract from an in-memory source and production
cannot depend on those fixtures. A committed receipt may invalidate protected
query state, while a preview, rejection or unknown outcome cannot replace the
displayed snapshot. Conflict descriptor loading, bounded upload staging and
atomic command construction remain controller/model concerns; the conflict
editor receives only shared state/actions and never imports HTTP transport.

The native backend adapts JJ's futures file streams to the owned Tokio
transport. Its query engine derives bounded copy records from native history;
those derived results never become publication authority or require a listing
API.

The service implements the owned native-store authority interfaces through the
shared guarded coordinator. JJ factories and semantic object inspection remain
in the JJ integration crate. CLI-package integration fixtures exercise the full
native library lifecycle without moving upstream types into service code.

The RPC crate depends on owned model, object-I/O and service interfaces. Its
streaming object transport implements ObjectStore only for repository-bound
native keys and rejects mutable writes. Remote JJ authority adapters live in the
CLI integration crate; upstream JJ types do not enter the RPC implementation.

The CLI crate now builds the development executable named jj and owns local
connection discovery, credential-facility access and command presentation. The
installed system jj remains untouched. Connection records and derived JJ caches
are physical-client state and do not become server authority.

Typed provisioning and token administration extend the existing native RPC V1.
Service-owned snapshots and secret-free results cross the transport boundary;
RPC does not import native JJ types or gain raw storage-publication access. An
installation-owned registry separately retains repository aliases and token-ID
discovery candidates. Repository creation reserves an alias before publication,
then exposes the alias and initial admin-token candidate in one atomic registry
replacement after authoritative publication succeeds or is recovered. The index
never supplies authorization; HTTP reopens each candidate through its current
manifest-selected policy.

The server composition crate owns bounded TOML configuration, operator-policy
files, RustFS construction, process exclusivity and the foreground listener.
Both executable entry points live in the CLI crate to share presentation and OS
credentials; server composition never depends on command presentation.

Configuration format 2 is the only accepted server configuration. It keeps
access policy, authoritative storage and backup projection as independent tagged
settings. The current application target uses LocalTrusted access and embedded
storage; its runtime owns installation state, the RustFS child, readiness,
sentinel verification, shutdown and the lifetime filesystem lock while returning
the existing owned `S3Store` contract. Older formats are rejected. Explicit
LocalTrusted search entries start rebuildable background indexing through an
installation-derived read-only principal. Authenticated shared serving, external
authority, backup/restore and replication are deferred and are not first-release
acceptance gates.

The access policy is independent of transport. Authenticated HTTP and RPC keep
repository bearer validation, while LocalTrusted HTTP and Unix RPC derive stable
principals from the installation identity after transport admission. Both paths
enter the same guarded service sessions, semantic policy, limits and manifest
publication boundary. Bootstrap discovery reads only the private installation
catalog; opening a listed repository still verifies native authority through the
ordinary service APIs.

The CLI owns resumable physical attachment journals and atomic metadata staging.
They retain no credentials and are not authority; publication acknowledgements
continue to come from the native service's manifest-selected receipts.

The CLI also owns device-local AgentStore capture durability because it already
owns filesystem discovery and local process state. The neutral `jj-agent` crate
defines the versioned cursor and sanitized activity models without filesystem or
WAL dependencies. The CLI journal depends on those models and the WAL's typed
agent request, persists outside checkouts under an exclusive user-restricted
lock and never becomes repository authority. Only the WAL coordinator can
publish its prepared delta through the manifest CAS and durable receipt path.

The same CLI boundary owns a device-local integration registry outside the
checkout. It binds one stable random device identity and explicit desired
enablement to the native repository plus canonical physical workspace. Registry
writes are locked, atomic, synchronized and user-restricted. Discovery runs
bounded version probes for the exact qualified Codex, Cursor and OpenCode
versions; failed probe stderr is never retained or presented. Detection and
desired enablement remain separate from owned hook/plugin setup, so discovery
alone cannot claim capture is active.

Harness setup is also CLI-owned. It merges namespaced Codex and Cursor project
hooks or installs one namespaced OpenCode project plugin. Before any project
mutation it atomically stores exact original bytes plus original and intended
SHA-256 ownership hashes under the locked device registry. Project writes are
bounded, atomic and reject symlinked configuration paths. Restart can complete
an interrupted original-to-owned transition; later user edits are reported as
drift and never overwritten. Hook ingestion remains a separate boundary and is
not implied by installed setup state.

The CLI builds both development executables. Repository creation is a durable
client workflow that provisions its OS-held admin credential before remote
creation and then delegates physical checkout setup to resumable attachment.

The JJ integration crate owns graph DTOs and a bounded executor for native
queries. Its explicit time-context dependency and temporary cache staging remain
inside that boundary. Service read capabilities implement the executor's owned
authorization interface; upstream JJ types do not enter transport or services.

The service owns graph continuation signing and a bounded process-local snapshot
registry. It retains immutable read pins without retaining session credentials;
native query execution remains in the JJ integration crate.

The HTTP crate owns Axum request parsing, stable JSON errors, media headers and
transport admission. It consumes service-exported owned read contracts and bound
native sessions; it has no direct object-store or upstream JJ dependency. The
service exports those contracts without duplicating native codec definitions.
The server composes HTTP and native RPC on the same TLS listener and public URL.

The service owns committed-event cursors, bounded replay, current authorization,
and a fixed-capacity process-local notification channel shared by hosted
sessions. The HTTP transport owns SSE framing and independent stream admission.
Publication never awaits consumers; polling selected manifests supplies
cross-process delivery.

The HTTP crate generates the OpenAPI document from its registered route
inventory and project-owned schema traits. Released openapi-typescript tooling
generates browser types from that document. Schema traits add no storage, search
or presentation dependency to lower layers; transport generation stays in the
HTTP crate and the Node build scripts.

The browser is a Svelte 5/TypeScript/Vite application under web. It consumes
generated read contracts with an explicit lossless-integer mapping. A server
web_root selects a prebuilt static bundle; its manifest and assets are loaded at
startup with fixed bounds, so requests do not read arbitrary disk paths. The
Rust workspace remains buildable without a frontend toolchain; pnpm build builds
the browser bundle before the Rust executables.

The development demo orchestrator owns one pinned embedded RustFS child, one
recorded native server process, the built browser bundle, a private LocalTrusted
installation and one disposable native checkout under the operating system
temporary directory. The real browser acceptance runner auto-opens the verified
catalog entry without credentials, publishes through a separate Unix-RPC CLI
process and removes the owned stack. It refuses unowned listeners and process
identities and records bounded startup/RSS measurements.

The user deferred shared-deployment browser release qualification on 28
September 2026. Manual-token authenticated discovery and repository selection
are now implemented without OIDC or accounts; provider qualification and M4.14
remain future work. Functional cold local browser acceptance belongs to M8a.1;
package publication and visual baselines do not block the local application
release. The local demo configures and waits for search before reporting
readiness.

The WAL snapshot DTO shares schema metadata with the typed application API using
schemars. The locked dependency is schema infrastructure, with no HTTP/UI
runtime dependency in storage or publication. HTTP native command execution
shares the serving instance's native worker admission with read execution.

Conflict-upload orchestration belongs to service, with HTTP adapting bounded
binary streams using the existing released tokio-util dependency. Upload record
identity belongs to model; native file verification and tree rewriting remain in
JJ integration. This introduces a staging-record V1 format without changing
native, WAL, RPC or existing mutation-result codecs. Dependency locks retain
registry artifacts.

The adapter crate owns read-only committed delivery, projection checkpoint
bookkeeping and local writer ownership. It depends on model, object storage and
WAL, with no native JJ, service, transport, search engine or UI dependency.
Authorization enters through an owned callback; projection implementations own
their durable artifacts. Repository publication never invokes an adapter.

The service binds adapter source authorization to its existing repository token
session. The service may consume the adapter crate's owned callback/interface;
the adapter crate cannot depend on the service or publish native history.

The Tantivy crate owns local projection generations and indexed documents. It
consumes the committed adapter contract, with no upstream JJ imports or
publication capability. Native extraction and transport composition remain
separate boundaries. Tantivy is locked to 0.26.2 for its initial index schema.

Tantivy native extraction consumes only service-owned projection readers and
DTOs. Service adapts verified snapshot objects to native read-only storage;
upstream JJ imports remain confined to the JJ integration and CLI crates.

HTTP search consumes the Tantivy pager and adapter health contracts through an
explicit repository registry. It constructs search pagers using the existing
native content worker pool. Lower storage/publication/service layers gain no
search transport dependency; the server starts configured writers separately.

The server composes owned adapter/Tantivy interfaces for configured background
search. Indexers and their cancellation guards are owned by the foreground
server lifecycle. Publication and native storage do not depend on these workers.

## Browser presentation ownership

Production-owned Community adaptations, presentation models and styles live
under `web/src/lib`. The isolated catalog imports those modules and supplies
only fixtures, in-memory transport-free state and story harnesses. Production
source cannot import the catalog. See [browser requirements](ui.md).

Repository layouts keep versioned shell, file-tree and per-page Column Canvas
state behind an injected storage boundary. Production resolves browser storage
defensively, while catalog stories receive isolated memory storage. Bookmark,
directory, revision, operation and changed-file collections use shared bounded
virtual-row contracts; storage never contains repository identities, cursors,
credentials, selections or content.

The catalog retains independent cold static-browser acceptance. Its framework,
renderer and builder versions are aligned in the registry lockfile. The served
browser composes the same production modules through an authenticated
asynchronous controller; catalog checks reject API clients and network calls.

Repository search keeps transport and cache ownership in the controller layer.
Presentation consumes only the shared search state/actions contract, and the
catalog supplies a transport-free in-memory source. Search query keys bind the
authentication session, repository and selected operation snapshot; query text
and opaque continuations remain memory-only. The development orchestrator owns
the configured Tantivy worker with the server lifecycle and waits for its first
current generation before reporting the stack ready.

M4.9 separates snapshot operation, inspected operation, commit, change and
comparison identities in the shared controller contract. Loading, refreshing,
paging, empty, failed and unavailable states are explicit. M4.10–M4.14 add the
bounded transport implementation, navigation, refresh and operational evidence.

M4.10 keeps bounded browser reads inside the established native-query,
committed-read, service-pagination and HTTP transport boundaries. Reference,
recursive-path, operation-change and diff-summary contracts add no presentation,
fixture or mutation dependency, and their cursors remain service-owned rather
than becoming native repository state.

The local pnpm hook follows the workspace source-first boundary for Design Core.
When the sibling source exists it resolves the declared dependency from that
checkout for development validation; otherwise it leaves the semver range
intact. It never rewrites peer ranges. The publishable manifest retains registry
ranges while a locally installed lock records the explicit sibling source.

Agent capture uses MongoDB Kingfisher 2.7.0 in-process at source revision
`63dccba231b6b01990c84319d25f92f49428a431`. The scanner is compiled without
default features, so validation and network-backed checks are unavailable. The
workspace pins Kingfisher's matching Vectorscan fork because Cargo does not
inherit a Git dependency's workspace patch. No JavaScript redaction package or
temporary raw-input file participates in the capture boundary.
