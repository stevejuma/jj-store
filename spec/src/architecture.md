# Architecture

The local service consumes `jj-review` directly for portable source validation.
The dependency audit permits this owned model boundary while rejecting
ReviewStore dependencies on publication and core-model dependencies on reviews.
Its retained-record read port supplies only canonical AgentStore objects from an
explicit committed companion selection. The model checks exact identities and
payload bytes; it cannot open folders, admit callers or publish metadata. The
service preserves those provider responsibilities and keeps portable review
publication gated until retention and shared command qualification pass.

The Tantivy boundary owns credential-free search sessions and generation
continuations over service-authorized native snapshots. HTTP and server
composition consume these owned contracts; publication remains independent.

Crates are added as milestones begin, not as empty placeholders. Native codecs
initially live with JJ integration. The server uses in-process stores rather
than calling its own RPC. Local indexes are derived and rebuildable. The source
repo is colocated for development. Native product storage has no Git carrier;
the separate local Git-backed companion profile is governed by local-jj.md. Its
WorkStore facade uses existing service/domain dependencies; disposable
filesystem qualification uses a test-only temporary-directory dependency.
Portable source identity types stay in the neutral model layer; their pinned
upstream conversions and Git evidence capsule framing stay in JJ integration.
The adapter uses the existing SHA-256 primitive directly for capsule integrity.
Agent projection, catalog edits and bounded reads use owned immutable interfaces
in WAL. The service's Git record carrier preserves the existing repository-bound
envelope bytes, while companion ref publication remains in JJ integration.
Neither the shared reducer nor the object interface can select repository state.

The custom CLI consumes jj-collab's owned typed Thread, Reaction and
custom-emoji contracts directly. Its filesystem commands select the existing
local companion provider without runner, server, Git transport or storage
dependencies in the model. This dependency adds no external package version or
persistence format.

Native RPC consumes the service facade's owned collaboration-model reexport. Its
codec keeps raster uploads and downloads in bounded protobuf byte fields, while
commands and pages use typed service JSON. No transport codec owns actor
admission, root selection or publication, and no upstream JJ dependency is
added.

`jj-collab` owns persistent Thread/human-message and Reaction/custom-emoji
models with independent root types. It depends only on owned model, AgentStore,
ReviewStore and WorkStore types plus bounded canonical/Unicode/raster libraries.
It has no publication, upstream JJ, transport, search, UI or runner capability.
Its pure projectors stage graphs; persistence providers authorize and select
roots. The async target-reader contract supports both storage profiles without
an execution runtime dependency. WAL composes existing immutable read facades;
the domain checks membership and visibility while owning providers verify
repository envelopes and retain authorization and publication authority. The
local service consumes jj-collab directly for the same pure commands and uses
independent canonical Git-blob descriptors plus existing source record and
capsule carriers. Source closure validation and physical retention remain
adapter concerns; the domain cannot publish a Git ref or repository manifest.

Browser presentation owns the copied Community source closure locally and may
not resolve the Community package or sibling checkout. Design Core and Mira
remain foundational dependencies. All copied components use native application
models; Community/Nostr storage, authentication and controller runtimes are not
part of this application's presentation dependency graph. Browser and catalog
bundles deduplicate Svelte and the released Bits UI overlay runtime shared by
Mira and Design Core; independent modal managers cannot own the same document
body lock.

The shared overlay runtime is pinned to released Bits UI 2.19.4. Its deferred
focus callbacks stop after the owning layer is destroyed, including when a
successful file save replaces its editor. Runtime diagnostics remain enforced;
updates to this dependency qualify the complete browser interaction and
accessibility suites. Automatic data refresh remains a controller concern and
does not add publication or storage dependencies.

## Requirements

The local UI development profile separates Vite source serving from the existing
native backend executable. Vite owns the browser loopback port and proxies
versioned API requests to a separate literal-loopback backend port. The existing
fixture remains authoritative and no build is required for UI edits. A
full-stack rebuild/restart remains an explicit development command; production
continues to serve the built asset manifest through Rust.

The CLI has one collaboration input-admission module for both providers. Native
reads and mutations consume only the typed RPC client, while filesystem commands
retain the companion facade. An uncertain RPC command response is reported with
its original request ID rather than silently retried.

| ID           | Requirement                                                                                                                                                                                                                             |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-ARCH-001 | The repository MUST provide a Rust workspace with independently owned native storage, publication, JJ integration, services, transports, adapters, and presentation boundaries.                                                         |
| JST-ARCH-002 | Model, object storage, and WAL MUST NOT depend on UI, Tantivy, Git transport, SQL, Nostr, or agent runtimes; upstream JJ integration MUST stay behind owned adapters.                                                                   |
| JST-ARCH-003 | First release MUST serve new native repositories through LocalTrusted loopback HTTP and owner-only Unix RPC backed by persistent embedded RustFS, with no required authentication, Git, SQL, relay, or external object-store authority. |
| JST-ARCH-004 | Browser and catalog source MUST enforce counted-line budgets, keep production independent of fixtures, and separate presentation from API and controller ownership.                                                                     |

Storage harness self-tests are part of normal validation. `pnpm test:rustfs`
executes only isolated disposable Docker probes and writes a scratch report. An
approved immutable report is retained with its exact image/topology.

Development-only WorkStore samples live in the checkout-owned harness. Fresh
fixture setup and `pnpm dev:seed:work` use the production HTTP command boundary;
neither direct object persistence nor catalog fixtures become repository state.

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

The custom CLI depends on `jj-review` only to construct validated typed review
commands from explicit user input. Native RPC production code stays behind the
service and WAL facades: it transports bounded canonical JSON for the same
service-owned ReviewStore page/detail and command/status models used by HTTP.
This adds no reverse model dependency, raw coordinator access or second review
implementation.

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

Native navigation tree styles and catalog layout assertions have separate owners
so adding hierarchy and visual regressions preserves those no-growth limits.

The browser may bundle reviewed static integration artwork under `web/public`
and Storybook serves that same directory so catalog and production use one
asset. Git author presentation depends directly on the locked browser-safe hash
library to reproduce Community's normalized Gravatar URL; that presentation
dependency cannot enter storage, publication, service or transport crates.

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
authority and backup/restore remain deferred. A repository may explicitly
configure one upstream for writable replication; server composition owns its
bounded bidirectional worker and lifetime. The worker resolves its named
credential into process memory, persists only issuer-bound peer checkpoints and
never enters a foreground local read or mutation. Expanded pressure, browser,
cold two-instance RustFS and deployment-topology qualification remain deferred
under M10.8.

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
The native transport exposes that authority as separate prepare and publish
calls: the server owns current-root selection, peer principal and activation
time, while the CLI owns crash-safe persistence between them. Canonical JSON
payloads are explicitly byte bounded inside the protobuf envelope, and an
unknown manifest outcome remains a typed response requiring an exact retry. The
CLI recovery worker preflights one repository binding for the complete spool,
orders integrations, source instances, generations and native sequences, and
derives separate stable request and publication IDs from the repository and
capture batch. A prepared response is synchronized before its publish call; no
later source entry passes an unknown, stale, rejected or failed publication.

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
drift and never overwritten. Removal restores the exact recorded original or
deletes only a target created by setup, atomically retires the ownership record
and then disables desired policy. Its tombstone recovery is idempotent, while a
changed target or unsafe path blocks every mutation. Hook ingestion remains a
separate boundary and is not implied by installed setup state.

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

The collaboration read service consumes owned AgentStore and ReviewStore models
through the WAL facade and its audited root snapshots, then returns
service-owned DTOs. The facade exports only the validated review identities,
anchor values and workflow enums needed by this boundary; raw ReviewStore
envelopes remain internal. Separate signed process-local cursor domains retain
exact immutable roots and closures under shared request, concurrency, expiry and
object budgets. Visual components and HTTP types do not enter the service, and
neither the service nor transport can obtain a collaboration publication
capability from a read cursor.

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
installation and one persistent native checkout under the ignored
`.local/dev-demo` subtree. Durable fixture metadata is separate from ephemeral
process metadata: restart and stop preserve repository/RustFS state, while reset
is the explicit destructive reseed boundary. The real browser acceptance runner
uses a separate operating-system temporary root, auto-opens the verified catalog
entry without credentials, publishes through a separate Unix-RPC CLI process and
removes only that isolated stack. Both paths refuse unowned listeners and
process identities and record bounded startup/RSS measurements.

The deterministic reseed writes one representative root document for every
landing-document category owned by the browser: README, CODE_OF_CONDUCT,
CONTRIBUTING, LICENSE and SECURITY. A governance test invokes the exported seed
boundary in a temporary directory and verifies that complete set without
starting or changing the interactive demo stack.

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

Repository file editing consumes the public Design Core CodeEditor, which owns
the Mira MDE lifecycle. A focused production component maps the current filename
to host-provided CodeMirror language support and remains independent of
controllers and catalog fixtures. The catalog prebundles Design Core's nested
Mira package so its source-distributed Svelte editor and CommonJS dependencies
cross the Vite boundary together. The extracted editor surface keeps the owning
file panel within the counted-line budget.

M4.9 separates snapshot operation, inspected operation, commit, change and
comparison identities in the shared controller contract. Loading, refreshing,
paging, empty, failed and unavailable states are explicit. M4.10–M4.14 add the
bounded transport implementation, navigation, refresh and operational evidence.

M4.10 keeps bounded browser reads inside the established native-query,
committed-read, service-pagination and HTTP transport boundaries. Reference,
recursive-path, operation-change and diff-summary contracts add no presentation,
fixture or mutation dependency, and their cursors remain service-owned rather
than becoming native repository state.

M9.5 keeps collaboration transport, asynchronous state and presentation as
separate production modules. The HTTP collaboration source owns generated DTOs,
session-scoped query keys and ReviewCommand transport. The collaboration
controller owns paging, stale-response rejection, durable unknown-outcome state
and hash selection actions. Agent and review components depend only on the
shared controller contract, owned Community adaptations and Design Core exports.
Catalog collaboration fixtures and state wrappers remain transport-free and
cannot be imported by production. These extractions keep every production module
and complete Svelte file within the governed counted-line budget.

The local pnpm hook follows the workspace source-first boundary for Design Core.
When the sibling source exists it resolves the declared dependency from that
checkout for development validation; otherwise it leaves the semver range
intact. It never rewrites peer ranges. The publishable manifest retains registry
ranges. Temporary source validation retains registry resolutions in the tracked
lockfile and records the exact source revision separately. A source-validation
pass does not qualify a registry release with missing public APIs.

Agent capture uses MongoDB Kingfisher 2.7.0 in-process at source revision
`63dccba231b6b01990c84319d25f92f49428a431`. The scanner is compiled without
default features, so validation and network-backed checks are unavailable. The
workspace pins Kingfisher's matching Vectorscan fork because Cargo does not
inherit a Git dependency's workspace patch. No JavaScript redaction package or
temporary raw-input file participates in the capture boundary.

## Native WorkStore

The neutral jj-work crate owns project, task and comment models and command
validation. WAL owns WorkStore radix catalogs and publication; service and
transports consume owned contracts. See [native projects](projects.md).

WorkStore enablement preserves and freezes an existing origin ledger. Native
repository creation initializes that ledger independently of an upstream, so its
presence cannot reject local task creation. Format-3 publication omits new M10
origins; protocol negotiation, feed, offer, closure, import and seed routes
reject the unsupported profile explicitly. The local WAL and native copy retain
complete WorkStore authority. Existing configured replication becomes
unavailable for a WorkStore home; WorkStore does not claim remote convergence.

## Git bridge ownership

[Explicit Git interoperability](git-interop.md) adds CLI-side transport and a
native durable evidence domain. Core storage and publication consume owned
records only. No persistent writable mirror or additional transport daemon is
introduced. WorkStore origin replication retains its separately deferred gate.

The exact converter adds gix 0.87.1 to the JJ integration boundary while keeping
JJ 0.45.1 and the existing resolved Git dependency pin. Its temporary GitBackend
is a conversion tool; no Git dependency enters WAL, model or object storage.

Git transport supervision uses the already resolved rustix 1.1.5 process API
inside the CLI only. Non-reaping wait retains the owned group leader's PID until
cleanup completes, preventing a cleanup signal from targeting a recycled PID. No
transport or process dependency enters native authority layers.

The Projects hub preserves native model/service ownership and adds only an
installation UI orchestration layer and read-only HTTP summary contract.
Production build qualification uses the generated Vite dependency manifest to
serve application-reachable chunks, styles and fonts plus declared agent icons.
Unused package-emitted workers are excluded, while server asset admission limits
remain unchanged. The manifest traversal regression is wired into the existing
governance test lane.

The development harness includes native local, native server-only and ordinary
Git-backed JJ fixtures. All three original projects keep the native metadata
home and link distinct repository identities. The Git-backed fixture is CLI-only
until local serving is qualified; discovery never substitutes a native copy for
its storage profile. `cli:install` builds the owned CLI package and copies the
server and custom JJ tools under distinct names without replacing system JJ.

Native Thread/Reaction roots select bounded canonical-object-to-envelope
catalogs. This retains shared domain content identities while reusing
repository-bound immutable WAL records. Complete source closures include
external AgentStore, ReviewStore and native code edges. Local companion batches
retain independently versioned domains through one owned ref and a versioned
receipt envelope.

Filesystem-only composition omits native object storage, RPC, search and
replication workers. The HTTP host is optional; native requests fail explicitly
when absent. Installation-owned bindings and the Git-profile reader use the same
HTTP admission and query-worker pool as native serving.

Device-bound collaboration facades carry the admitted local repository identity
into the existing WorkStore provider. Identity is rechecked on publication
retries, while reads validate independently selected companion history. Native
manifest authority and local Git ref authority remain separate.

Folder initialization composition persists the proposed identity before
publication and reuses the same initialization receipt. Reconnect changes only a
validated device binding; companion metadata and linked project identities
remain authoritative and unchanged.

## Bundled appearance assets

The browser bundles the pinned Shiki 4.0.2 theme data as generated semantic and
syntax palettes and Inter 5.2.8 fonts for the Buzz-derived Appearance page.
Generation uses npm theme data and the attributed owned palette adapter; no Buzz
or Community checkout is a runtime/build dependency.

The picker also includes the pre-Appearance Design Core semantic palette as the
static Original light/dark pair. Its historical snapshot remains independent of
Shiki generation; the generated import index includes both sources. Public
foundation dialog tokens and the app-owned prose composer's Mira bridge bind
surface and text roles to the resolved palette. The owned palette adapter binds
the semantic popover background to the app background across all 62 generated
styles, including search Command View, menu and Popover consumers. Compact
navigation retains its app-surface binding for the historical Original pair.
Buzz chrome reveals one viewport-height shell gradient, while glass keeps its
selected tint and blur. Standalone code/file editors retain their own editor
surfaces. These bindings stay in owned presentation and generated assets with no
foundation dependency change. Palette checks regenerate and compare the exact
source-derived bytes.

Local Appearance acceptance in main uses the existing ignored foundational
package hook against landed Design Core source. Its chat typography and native
scrolling extensions are absent from registry 0.2.0; portable manifest/lockfile
ranges remain registry-based and registry-only qualification stays separate.

The service-owned physical working-file facade uses safe descriptor-relative
filesystem operations through rustix. It owns bounded text admission, device
selection fingerprints and atomic Save; it has no upstream JJ imports and does
not compose a repository transaction. Local HTTP admission supplies the verified
folder binding. Native mutation/storage boundaries remain independent.

Tooltip sizing restoration is qualified from the independent Design Core
repository through the documented local source hook, preserving the standard
arrow and public sizing tokens without an application override.

The Projects hub delegates Agents scope and collection reads to owned
UI-controller modules. Scope discovery joins unfiltered project links; sessions
retain separate repository admission and native/local snapshots. The shared
agent-session row is owned under web/src/lib and reused by repository and
project directories. Scoped response controllers reuse the owned repository
collaboration readers, reactions and tool review actions while keeping their
data and local response route separate from the global connection and Projects
scope. Response selection cannot reload its collection, late selections cannot
replace newer results, and revoked target access clears protected responses.
Related review/repository collection dispatch remains a separate hub helper;
production never imports catalog fixtures.

## Pinned history query ownership

History query metadata, pinned standard aliases and bounded operation-filter
parsing belong to `jj-store-jj`. Native and filesystem graph readers consume
these owned interfaces through the existing service admission and read-session
boundaries. Filtering precedes pagination and charges scanned operations without
rewriting their parent identities. Query execution never publishes repository
state. The browser controller owns cancellation, operation/time pinning and
accepted-query routing; Design Core owns the generic custom-language editor.
