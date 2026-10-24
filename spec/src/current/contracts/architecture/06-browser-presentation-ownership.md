# Browser presentation ownership

Visual Delta is a development-only registry dependency of the Storybook host.
Its public preset, Node CLI and Playwright exports own capture execution and
profile resolution; jj-store owns its shared/local configuration, catalog
registration and artifact paths. This tooling adds no dependency to production
browser components, native storage or the server, and does not replace existing
interaction/accessibility qualification.

Dependency acceptance now uses published Design Core `0.3.1` and Mira `0.0.3`
through portable npm ranges. Source ownership and public contracts remain
unchanged; local overrides for these released packages are retired.

Production-owned Community adaptations, presentation models and styles live
under `web/src/lib`. The isolated catalog imports those modules and supplies
only fixtures, in-memory transport-free state and story harnesses. Production
source cannot import the catalog. See [browser requirements](../ui/index.md).

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

The local pnpm hook consumes Design Core's explicitly packed development archive
under the sibling's ignored package directory. Sibling source alone MUST NOT
substitute a dependency. Missing archives retain registry ranges; peer ranges
are never rewritten. The publishable manifest and tracked lockfile retain
registry resolution. Local validation records the archive digest and checks its
public contents against the qualified input. The prepared archive contains
JavaScript rune modules, declarations, Svelte components and CSS under the same
public subpaths. Components remain consumer-compiled Svelte; this does not
qualify a registry release with missing public APIs.

The browser enables Svelte dependency optimization for package components and
rune modules. Main page views and optional inspectors use typed dynamic imports
selected by current route or explicit opening. Controller and context owners
stay mounted. Successful imports are shared; failures allow explicit retry
without automatic reload. Closing a loaded overlay retains its component and
existing draft/cleanup behavior. A late import cannot render in an unmounted
region. Development prebundling and emitted production chunks have independent
cold acceptance; neither changes API or publication authority. Production chunk
grouping and one shared stylesheet keep the complete lazy asset inventory within
existing native count, manifest, per-file and total byte limits. Page entries
remain outside the shell's static import closure.

Agent capture uses MongoDB Kingfisher 2.7.0 in-process at source revision
`63dccba231b6b01990c84319d25f92f49428a431`. The scanner is compiled without
default features, so validation and network-backed checks are unavailable. The
workspace pins Kingfisher's matching Vectorscan fork because Cargo does not
inherit a Git dependency's workspace patch. No JavaScript redaction package or
temporary raw-input file participates in the capture boundary.

The TagList adoption consumes published `@lapismd/design-core` `^0.5.0` through
the registry lockfile. TagList owns chip layout, overflow measurement, native
CSS and popup lifecycle; domain label/tone mapping remains under `web/src/lib`.
Community-derived compositions retain their original attribution. No source
alias, package patch or local archive participates in this acceptance.

jj-store explicitly imports its own Tailwind utilities entry in the web and
catalog stylesheets. Its existing utility-based application compositions no
longer depend on a shared package exporting a compiler entry. Design Core
components continue to import their own native CSS; this adds no source alias,
upstream override or copied component style.

M33 retains Community-derived UI adapters and legacy presentation field mapping
under `web/src/lib`. The independently authored setup-ID codec and delivery
projections move to the headless conversation SDK. Original Community design
credit remains recorded; no Community UI or Nostr implementation becomes a
shared dependency. Presentation ownership and attribution requirements remain
unchanged.
