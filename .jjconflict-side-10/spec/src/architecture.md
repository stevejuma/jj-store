# Architecture

The approved M21.2 delivery order in
[the implementation plan](../../docs/implementation-plan.md#local-stacks-backend-proposal)
preserves the approved UI-first boundary. It separates operation-pinned stack
reads, recoverable local JJ execution, device-local named combinations and
physical checkout coordination. Local Git-backed operation publication and
native manifest publication remain distinct provider responsibilities. The user
approved backend implementation on 5 October 2026. Runtime capabilities remain
gated by qualified provider behavior under JST-UI-020 and JST-LOCAL-005;
approval alone does not enable commands.

Production Stacks read ownership is explicitly injected by the served App after
LocalTrusted registry, profile, read-capability and binding-generation checks.
Bounded operation-pinned references, histories and exact tree comparisons reuse
the repository transport; route restoration and cancellation retain independent
read ownership. Failed candidate refreshes cannot replace the current complete
scope. Physical scan, private combinations and recoverable execution remain
subsequent implementation boundaries.

The service owns the private physical scan and typed fingerprint independently
of upstream JJ imports. It reads descriptor-relative paths without following
symlinks and observes ignored/untracked obstacles as well as candidate files.
The integration provider must separately qualify exact code baseline and ignore
semantics before projecting Unstaged; scan data does not establish stack
membership or publication authority.

The private JJ reader now supplies an operation-pinned named-workspace baseline
with bounded raw file/link digests. The service compares this to the physical
observation, retaining tracked ignored changes and untracked file/link
obstacles. Ignore parsing stays in the JJ integration and consumes explicitly
captured bytes. The private current provider now verifies existing bounded
checkout metadata, exact tree terms/labels and sparse scope, and captures global
and repository-exclude policy. The service observation owner compares this
witness before and after the two-pass physical scan, retaining bounded admission
through cancellation and checking current authorization. The registered
observation transport exposes bounded change summaries and read fingerprints
without private policy or content. The owned physical client validates complete
records and lossless integers without historical caching. The Stacks controller
owns this observation separately, brackets it with fresh current-head checks and
cancels its own work on supersession or disposal. Historical selections remain
unchanged. Active-combination merge/WIP assignment remains required before these
reads populate complete Unstaged. No read establishes a checkout lock or
publication authority.

The server-owned stack journal selects only installation-private preparations
and historical receipt state. It composes the existing private file primitive
with owned service contracts, independent of upstream JJ imports and portable
companion state. Synced replacement, exact replay, bounded admission and
monotonic transitions qualify this metadata boundary; provider object retention,
operation-head publication and foreground checkout remain separate Stage 3
gates. The primitive grants no runtime mutation capability.

JJ integration owns the separate reserved-ref preparation retainer. It consumes
existing owned Git evidence capsules and links their complete bounded physical
code closure, independent of companion state and server journal selection.
Archival operation/view data survives source operation/index loss without
granting a head-publication or checkout capability. Runtime composition must
still join exact preparation, journal, code publication and checkout boundaries.

The JJ local execution coordinator owns a nonblocking foreground working-copy
lease with the same kernel lock identity as the pinned CLI. It reuses bounded
checkout witnesses through owned authorization interfaces, retains admission
until release and rejects stale/replaced metadata or lock identities. The lease
does not snapshot, publish or finish checkout state. Composition must separately
revalidate physical bytes and join the retained exact operation with journal,
publication and checkout recovery before enabling writes.

Local reword preparation consumes and returns the foreground lease through a
blocking worker. It pins the exact source, checks visible ancestry and bounded
unsigned/resolved input, stages a real unpublished transaction, then verifies
descendant and bookmark effects with unchanged author/tree/checkout invariants.
Owned source/prepared capsules can feed retention; no upstream repository handle
or publication ability escapes. Server command policy and exact historical
journal lookup still precede any runtime use. Other commands, source ancestry
recovery, head publication and guarded checkout remain separate execution gates.

Recoverable preparation additionally captures the complete bounded source
operation DAG and historical views in an owned archive. Descriptor-relative
operation/view reads and the existing bounded read-only Git decoder avoid source
index repair, implicit extra import and no-GC ref writes. The reserved
retainer's version-2 record binds this history to its unchanged selected capsule
and links every historical code/tree dependency. Legacy version-1 slots remain
immutable. Private upstream materialization verifies exact operation/view IDs
and fields after source loss and GC; it does not restore source metadata or
publish heads. Server request/journal composition, restoration, publication and
checkout are still required Stage 3 boundaries. Real reword preparation performs
strict source-history capture before candidate writes and returns complete
candidate ancestry, checking that every original record remains unchanged. No
implicit original-extra import or synthetic change identity reaches the mutation
transaction.

JJ integration also owns explicit immutable source-history restoration, using an
installation-private versioned witness captured under the foreground lease. The
witness and original retention key cross the data-only composition boundary; no
upstream repository or mutable handle does. Saved checkout/policy/heads and
physical Git binding can be revalidated after operation/index loss. Source
restoration consumes exact reserved history and retains the foreground lock and
worker admission, with complete conflict preflight and synced create-only
records. It neither publishes heads nor repairs extras/index/checkout state. The
server must still join original authenticated journal lookup, publication and
checkout recovery before exposing commands.

Explicit retained-extra recovery additionally consumes the saved history lease.
It captures bounded no-follow table segments and head markers, compares complete
existing metadata, and uses pinned upstream serialization in a private mirror to
verify only the missing records. Synced create-only writes preserve existing
segment/head bytes under the original nonblocking extra-table lock.
Cancellation, process loss, replacement and authorization loss retain exact
retry input. No implicit import, head publication, derived index or checkout
state is involved; those Stage 3 gates remain separate.

Explicit retained-index recovery reuses the existing bounded disk-backed builder
through an archive-only Source. Original and candidate indexes are built
separately and verified through bounded framing and upstream native decoding.
Denied fallback operation/view traversal prevents backend import. Complete
commit/change identities and parent edges are compared to the acknowledged
archive; valid existing CLI index bytes remain intact and original selections
cannot gain future candidate visibility. Synced create-only segments precede
associations, with exact retry after process loss. Head publication and guarded
checkout remain separate Stage 3 boundaries.

Exact retained operation-head publication uses a separate original-request
recovery path with the saved checkout/policy witness unchanged. Bounded private
native decoding proves current operation ancestry; it never loads a latest
repository or reconciles heads. The original foreground lease and actual
upstream head-store flock enclose complete source/extra/index preflight and
synced candidate-before-original marker selection. Concurrent sibling heads
remain intact, and a retry after later CLI work acknowledges the candidate
without reannouncing or rewinding it. The owned proof attests code publication
only; guarded checkout, runtime journal composition and writable capabilities
remain separate gates.

The unchanged-tree checkout provider pins canonical native Checkout protobuf
bytes to the original recovery witness and retained preparation. It changes only
the operation pointer under the existing foreground lease and native head lock;
physical files and tree-state bytes remain intact. Installation composition must
save the bounded checkout witness before completion. Exact retry normalizes only
the checkout fingerprint after the intended pointer selection and rejects later
CLI checkout, policy changes or replaced metadata. Deterministic private staging
accepts only an exact prefix, bounding interrupted retries without overwriting
unsafe entries. The service's independent complete two-pass physical guard
shares observation admission, checks current authorization and keeps worker
admission until cancellation exits. Ignored/untracked content, modes, links and
directory identity remain preconditions. These provider proofs do not qualify
general tree-changing checkout, durable runtime receipt composition or writable
capabilities.

Tree-changing checkout now has a separate exact private preparation path. It
reads the retained archive's named source/candidate workspace trees through
identity-checked bounded Git objects and captures native checkout/tree-state
bytes, affected file stamps and no-follow ancestor-directory identities. The
original foreground lease and worker admission remain owned through cancellation
and guard validation. Full sparse coverage and resolved regular/executable files
are the initial profile; unsupported modes, aliases, dirty affected files and
ignored/untracked obstacles fail without writes. Canonical plan data can be
reopened and compared to the original retained effects, but is not a provider
proof. A separate explicit installation-private catalog now saves bounded
canonical plans before the original journal selects a digest/length reference.
The journal-to-plan lock order, complete retention/effect linkage and immutable
replay keep interruptions discoverable without growing the command journal's
budget. Saved input does not prove code or checkout selection; interrupted
installation, native tree-state selection and verified after-state remain
required before actual tree-changing checkout or command wiring.

Complete original physical input is an owned data interface in `jj-store-jj`;
the service's independent scanner supplies its sorted file fingerprints and
directory stamps. Canonical bounded inventory must reproduce the unchanged
physical fingerprint. Optional capture retains the existing registered guard's
authorization, deadline and admission through cancellation, with explicit
refusal by validation-only guards. It carries neither upstream types nor file
contents and grants no write or completion authority. Durable original-command
linkage and provider-owned installation progress remain separate gates before
planned directory transitions can be distinguished from unrelated editor work.

The server saves complete original inventory alongside its immutable exact plan
within the existing private catalog budget. It authenticates the original
journal request before store access and selects/syncs the inventory body before
its digest/length/plan reference under journal-then-plan lock order. Catalog
version 2 and journal version 5 preserve inventory-free legacy encodings. Saved
inventory grants no provider execution authority; per-path progress and native
checkout/tree-state completion remain required before production writes.

The server now joins actual unpublished reword effects with exact private
preview/journal selection, saved recovery witness and reserved retention. The
full candidate archive is durable before retention; acknowledgement replaces
only that pending body with the exact retained commit and archive digest. Retry
looks up original principal/request/intent before live context and resumes saved
evidence without a new transaction. Catalog version 2 preserves legacy bytes and
rejects reinterpretation. JJ integration retains saved archives under its actual
foreground lock without loading or restoring lost source operation/index state.
Owned data crosses this composition; no upstream JJ imports do. Real
installation authority, combinations, publication and guarded checkout remain
required before transport or writable capabilities.

Registered local preparation context now derives the configured LocalTrusted
Admin principal, canonical folder and generation through the existing device
registry and CLI folder provider. Every access rechecks that registration. The
existing bounded physical observer supplies current metadata and exact physical
fingerprints. Trusted combination ownership supplies active identity/generation
and immutable heads for the actually observed workspace; absence never creates
setup. The JJ provider checks protected ancestry before candidate writes. Saved
request replay still precedes live context; recovery resolves registration
without scanning lost operation/index state. Durable combination ownership and
runtime transport/publication/checkout remain separately required.

The installation-owned combination catalog now stores bounded named entries,
exact target/merge/WIP and ordered stack references, original retained-code
anchors, selected state and pending provider-derived updates. Its scope
separates physical workspaces and principals. The existing private device-file
primitive provides exclusive coordination and synced atomic replacement.
Original request replay cannot restore an old active pointer. A pending update
blocks preparation; selection requires the matching journal's completed checkout
and retained execution linkage. The registered authority additionally compares
the stored operation/view/WIP with the actual current workspace. This metadata
component does not publish code, create merge/WIP commits or execute checkout;
those provider/runtime gates remain required.

Independent conversion remains inside the JJ integration, owned service and
installation orchestration boundaries. Destination providers consume explicit
versioned archives and typed translators. Test-only collaboration dependencies
exercise nonempty supported histories; service code does not import upstream JJ
types. Source authority, credentials and active journals remain separate.

The service's bounded agent timing projection uses Chrono's standard RFC 3339
decoder for preserved sanitized Codex JSONL timestamps. It remains a data-only
read of exact selected evidence, shared by native and local companion
presentation. No capture provider, persisted format, source identity or
publication boundary depends on this display projection.

The service consumes the WAL's data-only alias lookup for exact native
tool-alias matching in the OpenCode timing read projection. Native integer
creation/completion and tool spans remain derived from selected sanitized
evidence; the lookup has no harness process, API acquisition, source allocation
or publication capability and keeps service imports behind the owned WAL facade.
Cursor's nullable-path lifecycle handoff remains private bounded metadata in
capture, separate from server source catalogs and repository authority. The
reopened multi-harness plan retains independent fixture, installed-product and
cold restart qualification.

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

`jj-store-capture` owns the reusable private sanitized spool, in-process
Kingfisher policy, state leases, installation-private source registration and
receipt-aware native/local drainers. Source IDs reuse the supplied installation
identity and a durable ordinal; registration has no repository or activity
authority. It consumes owned domain, service, WAL request and RPC client
interfaces, with no upstream JJ, CLI, server, search or UI dependency. CLI
capture modules reexport these types and functions to preserve existing callers.
Server composition can use the same spool and publisher interface with
in-process authorized services. Shared control-path policy preserves the
existing checkout rejection and explicit ignored development-root exception; it
never initializes a working copy.

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

## Desktop host boundary

The independently locked desktop host invokes explicit paired binaries and
serves the existing production web bundle through its managed LocalTrusted
listener. It has no dependencies on upstream JJ, model, WAL, storage or service
crates. The root Cargo workspace excludes the host. See [desktop](desktop.md)
for lifetime and qualification contracts.

The desktop workspace owns `jj-store-terminal-engine`, adapted from the pinned
Buzz PTY/Alacritty implementation. Its bounded queues, parser, process cleanup
and in-memory grid depend only on terminal/platform libraries. The host resolves
eligible folders through existing protected loopback reads using a bounded HTTP
client, then exposes named exact-origin Tauri commands. Neither engine nor
renderer can select repository roots or publish metadata. The web bundle uses
the released Tauri JavaScript API, aligned to the host's minor version, while
the Svelte terminal controller owns tab selection and disposable subscriptions.
No shell HTTP route or core storage dependency is introduced.

Native glass and traffic-light layout depend only on the host's independently
locked Tauri, window-vibrancy and minimal AppKit bindings. The renderer receives
one boolean appearance command restricted to the managed main window and exact
origin. Native vibrancy preserves an opaque NSWindow while clearing only the
WKWebView canvas. Main-thread native geometry and tracking corrections borrow
the retained window hierarchy only within Tauri's callback or the window's
AppKit update notification. A single owned observer token is removed on host
exit; corrections are idempotent and do not poll. They store no platform pointer
and add no repository or filesystem authority.

Local identity hints remain inside the JJ adapter's read-only query metadata
boundary. Host configuration discovery and registered-folder JJ/Git fallback
MUST expose only bounded email; evaluator settings, aliases, storage, model and
WAL remain independent. HTTP composition admits the hint only for LocalTrusted
serving. Configuration reading MUST NOT invoke migration or publication.

## Usable capture preview scope

The current capture delivery sequence integrates with verified committed main,
then prioritizes the existing Capture settings and Agents UI and a runnable
LocalTrusted Codex preview. It reuses current AgentStore, native directory and
Write admission, bounded workers and receipt-backed manifest publication. The
full multi-harness/conversation/provenance/review/archive roadmap remains
separately deferred. This changes execution scope and acceptance sequencing,
without introducing another authority, runtime or legacy storage profile.

Agent capture management has a dedicated installation-scoped Agents settings
section. It composes the owned shared Settings rows and foundational controls;
repository navigation and the capture observation lifecycle remain independent.
Capture retains startup-resolved harness directories beside the cached Copy
observation, so admitted settings reads expose configuration without directory
inspection. The four Agents settings tabs share one mounted observation owner
and retain request drafts and exact pending commands across tab changes.

## Bundled application documentation

The application manual is a separate MkDocs Material build from Markdown. Its
static HTML, search and build-time Mermaid SVGs are bundled for browser and
desktop Help. Documentation generators consume command/schema definitions and do
not become repository authority.

## Bounded application read ownership

The main workspace includes the qualified M20 application freshness stack and
its bounded-read prerequisites. E-FRESH-006 records integration and retirement
of the temporary `event-freshness` workspace, preserving the qualified
production tree and existing main ancestry. This local integration changes no
authority or acceptance requirement; M20 retains its open M15.5 dependency.

Application sessions retain checked immutable catalog roots. The WAL layer owns
verified catalog point membership; the service owns request authorization,
limits and credential-free source pins; the JJ boundary requests individual
objects separately from explicit complete enumeration. Derived read/index
preparation remains subordinate to these authorities. The staged
large-repository read work is described in `docs/implementation-plan.md`; each
delivered slice requires independent evidence in the milestone ledger.

The JJ adapter pins redb 4.3.0 for disposable preparation scratch only. Its
ordered tables hold compact graph fields and traversal state with a fixed page
cache, bounded write batches and a file backend that admits growth before I/O.
Streamed derived index output shares the scratch allowance. No redb dependency
enters model, object storage, WAL or publication; scratch is never repository
authority and is discarded after preparation or failure. The output remains the
pinned upstream JJ index format rather than a new persisted repository format.

The remaining large-repository implementation is delivered as performance slices
before separate complete qualification, as requested on 4 October 2026. Each
slice retains exact correctness regressions and affected package/boundary
checks. Combined cold-browser/contracts, Linux/native large fixtures, isolated
memory and release latency, fault/real-RustFS and generated Help/source-review
qualification remain pending until a later slice. This ordering changes no
authority, persisted format, capacity or acceptance requirement and does not
mark milestones Done.

Repository directory metadata is owned by viewport leases in the application
controller. Discovery supplies names independently; the controller limits actual
directory reads to two across refresh and canceled generations. The shared local
review snapshot validator preserves repository, profile and companion pin checks
for every directory and review consumer. Viewport observation and cancellation
do not alter transport admission or authorize linked repositories.

The WorkStore pager reuses validated immutable root data through the service's
bounded collaboration materialization cache. Ordered keys, normalized searchable
fields and one-pass aggregate counts remain derived service projections. Fresh
captured manifest envelopes and request limits stay outside the cache; native
continuations retain query-matching keys and share the underlying root charge.
Local browse hydrates only its selected window with the same projector; shared
local reconstruction and application search indexes remain separate delivery
work. No domain format, storage authority or upstream JJ import moves into the
service.

Native application content search retains service-owned immutable field indexes
under repository/epoch/domain-root keys. It admits normalized body growth before
cloning fields, derives recent/search orders once and retains compact query keys
for continuation. Snippets are generated only for returned hits. The cache holds
data only, with separate bounded root and cursor retention; each request
captures its manifest envelope and rechecks viewer, inventory limits and
authorization. No search cache or field index becomes domain authority or a
persisted format. The server uses a maintained platform filesystem-notification
adapter for hint-only local repository monitoring. Native storage, manifest
publication, captured JJ reads and authorization retain their owned boundaries.
A notification never selects authority; missed notifications fall back to shared
bounded witness checks rather than per-browser history scans.

Private server execution now joins original retained reword preparation to
actual head publication, saved unchanged-tree checkout witness and typed
provider acknowledgements. Authentication and original identity lookup precede
execution; first apply verifies current registered combination and physical
inputs, while pending retry resumes original recovery. Journal
PublicationUnknown is durable before head selection; saved checkout input
precedes pointer completion. Legacy completed receipts stay historical, current
proof-free execution transitions are rejected, and exact completed/retired
replay never scans or opens a provider. Witness-bearing catalogs use private
version 3 with bounded reserved growth; ordinary reads never upgrade older
catalogs. This code/checkout join still needs combination selection, general
checkout, production transport and UI capability qualification.

Retained Prepared first apply resolves registration without source reads,
restores exact source dependencies under the original foreground lease, then
reads and checks current combination/code/physical policy before publication.
This ordering supports operation/index loss without weakening stale-selection or
editor guards. Terminal replay bypasses recovery; revoked bindings cannot
restore any source state.

Unpublished retained reword execution, including original Unknown retry, checks
current trusted immutable ancestry over the retained source before selecting a
head. Already-selected original publication uses an acknowledgement-only path
when current mutation policy is skipped. Informative status cannot reauthorize
publication; if that selected candidate disappears, acknowledgement fails
without reannouncement. Policy/status work retains foreground and worker
ownership, supports cancellation and never mutates code or source metadata.

Known Published receipts also require acknowledgement-only recovery before
checkout witness persistence. Losing current head selection cannot authorize the
already-published candidate again; keep the original receipt pending.

## Retained physical installation boundary

The JJ provider owns exact per-path progress and no-replace physical file moves.
The service guard supplies complete fresh observations under existing registered
access, shared admission and deadlines. It does not normalize files or grant a
write capability. Original checkout/tree-state bytes remain unchanged; the
sealed physical result cannot acknowledge a command or select a named
combination.

## Retained native completion boundary

The JJ provider separately owns exact native tree-state/checkout completion from
sealed physical installation. A bounded provider frame and prepared inode
identities permit original-request recovery without a new command or upstream
working-copy initialization. Native and physical proofs remain separate from the
authenticated journal and combination after-state; production Stacks stays
read-only until those joins and transport qualification pass.

## Durable checkout and combination join

The installation journal owns original command acknowledgement. Coordinated
execution saves the exact staged combination binding before publication, retains
sealed native completion as pending, selects and syncs that combination, and
acknowledges the command last. Journal-then-combination lock order avoids
inverse nesting. Neither a saved digest nor a metadata fixture grants execution;
provider verification and the owning command engine remain required.

## Initial setup ownership

Explicit local stack setup uses the JJ provider to generate and retain an
unpublished target/merge/WIP archive. The registered server authority supplies
folder access and complete physical selection without inventing an active
combination. The server derives initial installation state from acknowledged
provider retention, binds it to the original journal before publication, and
selects it after native checkout but before final acknowledgement. Browsing
never creates this state. HTTP command transport, writable controller wiring,
membership/switching and dirty-work capture remain separate qualification gates.
