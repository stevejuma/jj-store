# Change map

E-CREATE-FORMS-001 adopts the existing JST-UI-016 Settings presentation in
server-repository and project creation/editing. Icon rows and creation-specific
layout remain application-owned under JST-GOV-002/008, using maintained local
compositions and released Design Core primitives. Creation requests, authority,
draft owners and persistence formats are unchanged.

E-REMOTE-ADMIN-001 adds independent operator agent administration and explicit
identity/repository access onboarding. Agent management uses freshly loaded
`manage-agents` policy; saved configuration activation is now governed by
E-APP-RELOAD-001. Native token grants use existing manifest-selected
administration and original receipts. Storage, WAL, AgentStore and installation
directory formats remain unchanged. New application-owned forms/controllers
compose maintained Settings components and released dependencies, with ownership
declared in provenance.

M27 adds an application-owned server selector, fixed-origin gateway and
operator-defined authenticated agent broker. New UI compositions remain under
`web/src/lib` with ownership declarations; dependencies stay released. The cold
production-App acceptance harness uses deterministic APIs and an isolated Vite
process, preserving existing services/data. This fixture evidence does not
qualify installed agents, desktop packaging or real-storage deployment gates.

E-REMOTE-AGENTS-002 adds isolated macOS two-instance real RustFS and cold
built-App acceptance with synthetic credentials and a deterministic ACP process.
The application-owned harnesses exercise actual API/Origin admission,
publication, revocation and restart recovery and own all temporary state/process
cleanup. The registered-Origin correction retains JST-SEC-009/JST-API-011
authority; JST-GOV-002/008 ownership and released dependencies remain unchanged.
Installed providers, Linux real storage and desktop delivery remain separate
gates.

E-PROJECT-AGENTS-001 retains JST-UI-015 ownership: repository session selection
and row composition are application-authored. They reuse maintained avatar/row
presentation and exact admitted associations without a Community runtime or
catalog dependency.

E-UI-UNAVAILABLE-001 retains JST-UI-015/019 ownership: the application-authored
unavailable-content composition uses released Design Core Empty primitives.
Maintained Community derivations keep their recorded source attribution; no
Community source or catalog runtime dependency is introduced.

E-AGENT-STATE-001 retains JST-GOV-002/008 ownership: the shared agent footer and
history-state helpers are application-authored production components declared in
provenance. Community and Design Core remain inspected presentation references;
no Community, catalog or external source dependency is introduced.

E-UI-VIEW-READINESS-001 retains JST-UI-015 source ownership. The shared file
tree now owns its existing row and toolbar stylesheet at the component import
boundary, with the original source revision and license retained in provenance.
Loading that presentation no longer depends on visiting another lazy page. No
external source dependency or public component contract is introduced.

E-UI-VIEW-LOADING-001 records the local package hook against a packed artifact
and registry fallback, preserving peer ranges and portable dependency metadata.
Development optimization and production lazy chunks require separate cold
evidence; neither substitutes for registry release or visual baseline approval.
Focused import, delivery and conflict-context checks pass; broader interaction
failures remain explicit in the evidence and milestone review.

E-INBOX-CONTROLS-001 retains JST-UI-015 ownership: inbox filters, ID-only device
preferences and responsive navigation styles are maintained under `web/src/lib`.
Their provenance identifies the owned derivation; production imports no
Community or catalog source. Public Design Core primitives remain dependencies.

E-SETTINGS-REPOSITORY-001 retains JST-GOV-008 ownership: Identity and snapshot
rows are an application-authored component using the maintained CommunityChip
and public Design Core controls. Empty-state selection reuses the existing owned
picker and authorization; no Community source is copied or imported.

`docs/ui-provenance.json` requires both `spec-governance.md` and `ui.md`. The
isolated `docs/capture-preview.md` runbook maps to `cli.md` and
`collaboration.md`; its launcher and admission examples remain governed
contracts. Provenance changes are protected ownership and presentation changes.

The `docs/agent-execution.md` runbook maps to `documentation.md` and
`collaboration.md`; its explicit launch, configuration, continuation and
credential-boundary examples remain governed contracts.

The `docs/local-compute.md` runbook maps to documentation, collaboration and
compatibility chapters. Its helper packaging, model lifecycle, private pairing
and credential examples remain governed contracts.

Local Git-backed companion implementation additionally requires local-jj.md;
native identity and publication chapters continue to govern their existing
formats.

| Protected area                                                                                                                                                                                                                                 | Required chapter     |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| `^(?:catalog/\|\.storybook/\|docs/catalog\.md$)`                                                                                                                                                                                               | `ui.md`              |
| `^(?:docs/(?:site/\|maintenance/)\|mkdocs\.yml$\|pyproject\.toml$\|uv\.lock$)`                                                                                                                                                                 | `documentation.md`   |
| `^desktop/`                                                                                                                                                                                                                                    | `desktop.md`         |
| `^rustfmt\.toml$`                                                                                                                                                                                                                              | `spec-governance.md` |
| `^crates/jj-store-model/`                                                                                                                                                                                                                      | `identity.md`        |
| `^crates/jj-store-object/`                                                                                                                                                                                                                     | `object-store.md`    |
| `^crates/jj-store-wal/`                                                                                                                                                                                                                        | `publication.md`     |
| `^crates/jj-store-replication/`                                                                                                                                                                                                                | `replication.md`     |
| `^crates/jj-workflow/`                                                                                                                                                                                                                         | `architecture.md`    |
| `^crates/jj-work/`                                                                                                                                                                                                                             | `projects.md`        |
| `^crates/jj-(?:agent\|review\|collab)/`                                                                                                                                                                                                        | `collaboration.md`   |
| `^crates/jj-store-capture/`                                                                                                                                                                                                                    | `collaboration.md`   |
| `^crates/jj-store-jj/`                                                                                                                                                                                                                         | `jj-integration.md`  |
| `^crates/jj-store-service/`                                                                                                                                                                                                                    | `mutations.md`       |
| `^crates/jj-store-rpc/`                                                                                                                                                                                                                        | `jj-integration.md`  |
| `^crates/jj-store-server/`                                                                                                                                                                                                                     | `cli.md`             |
| `^(?:crates/jj-store-http/\|web/\|docs/browser\.md$)`                                                                                                                                                                                          | `api.md`             |
| `^crates/jj-store-(?:adapters\|tantivy)/`                                                                                                                                                                                                      | `adapters.md`        |
| `^(?:crates/jj-store-cli/\|bins/\|docs/cli/)`                                                                                                                                                                                                  | `cli.md`             |
| `^(?:crates/jj-store-testkit/\|runtime/\|fixtures/\|schemas/\|proto/\|docs/upstream-lock\|docs/embedded-rustfs-qualification\|docs/codec\|docs/compatibility\|docs/licenses/\|docs/evidence/\|scripts/.*(?:rustfs\|spike\|upstream)\|deploy/)` | `compatibility.md`   |
| `^(?:scripts/\|AGENTS\.md$\|spec-validator\.config\.mjs$\|spec/book\.toml$\|\.gitignore$\|\.pnpmfile\.cjs$\|\.prettier)`                                                                                                                       | `spec-governance.md` |
| `^(?:Cargo\.(?:toml\|lock)$\|rust-toolchain\.toml$\|package\.json$\|pnpm-lock\.yaml$\|README\.md$\|LICENSE\.md$\|docs/implementation-plan\.md$\|docs/reference/)`                                                                              | `architecture.md`    |
| `^crates/jj-store-(?:cli\|jj\|wal\|service\|rpc)/(?:src\|tests)/.*git[_-](?:bridge\|interop)`                                                                                                                                                  | `git-interop.md`     |

Git interoperability additionally protects
`^crates/jj-store-(?:cli|jj|wal|service|rpc)/(?:src|tests)/.*git[_-](?:bridge|interop)`
and requires `git-interop.md`.

M24 extends the owned Community-derived composer audience copies, with exact
saved ACP identities and installation-local status. Attribution remains in
`docs/ui-provenance.json`; Community imports remain prohibited.

The M24 picker alignment copies Community's addressed-agent mention button into
the owned UI and reuses the owned picker switch/pin presentation. Provenance
records its source revision and Apache-2.0 license; dispatch and audience
storage continue to use exact local saved-agent identities.

M25 governs bundled agent/compute helper integration through compatibility,
architecture, collaboration and desktop chapters. Source/fixture evidence, real
local model tools, trusted transport and platform distribution evidence stay
separate. Cross-layer verification selects each affected direct boundary rather
than unrelated storage qualification. Model weights are explicit local data and
never tracked source or packaged credentials.

The local-compute runbook maps to documentation, collaboration and desktop
chapters. Its configuration, custody and staging examples remain executable
contracts, with platform distribution gates recorded in M25.

M25 Settings refinements extract the copied agent editor's unchanged saved
preference summary into an owned presentation component. Its original path,
source revision and Apache-2.0 attribution remain in `docs/ui-provenance.json`.
The jj-agent artwork reuses the app's maintained desktop logo with local asset
attribution; it introduces no Community or external asset dependency. Native
GGUF selection and runtime naming are qualified separately from unchanged
storage, credential and platform distribution authority.

The Compute model menu maintains Buzz's recommendation/fit reference and a
source-pinned mesh-llm weight catalog as owned UI data. Their source paths,
revisions and Apache-2.0 attribution are recorded in `docs/ui-provenance.json`.
No runtime, build or type import from either checkout is introduced. Advisory
RAM fit remains separate from measured inference/platform qualification.

M24 rich tool presentation copies retained-value adapters and Community detail
helpers into owned production files, with pinned attribution. Shared live chat
rows retain the existing Community message composition and XML prompt renderer.

E-FLEET-012 qualifies Markdown context with focused controller/service/CLI
tests, all directly affected transcript story families, and an isolated native
real-RustFS cold acceptance harness using a deterministic ACP fixture. The new
harness never consumes provider credentials or qualifies a real adapter.

M25 provider configuration maintains Buzz's provider/catalog reference under
owned service/helper code and shared Settings rows. Attribution retains the
pinned Buzz paths/license; no Buzz checkout dependency or public shared pool is
introduced. Provider fixtures, browser forms and live authentication gates are
recorded separately.

E-CHAT-BODY-001 governs the isolated commentable chat-body prototype. Its Delta
fixture projection script retains captured display text and tool data,
normalizes device paths, recalculates UTF-8 anchors and removes runtime/provider
metadata. The raw export stays ignored. Cold browser acceptance owns an
ephemeral loopback Storybook process and browser; it does not restart the app's
development services.

E-CHAT-BODY-007 extends the catalog projection with reviewed source mappings for
Delta comments whose export omits resolved offsets. The allowlisted Hello World
fixture retains delivery/reply boundaries and validates source quotes without
interpreting CRDT fragments or introducing a Delta runtime dependency. Simulated
sends and replies remain catalog-owned; UI state does not publish agent events.

E-CHAT-LIVE-011 extracts the owned composer surface for Document and Classic
presentation while sharing the existing audience, editor and send behavior. The
Community source attribution is retained for the extracted surface; its Document
layout composes owned chat-body presentation and public Mira/Design Core
components. No runtime or source dependency on Community is introduced.

E-CHAT-LIVE-012 records local ownership for the app-authored composer selection
and Document footer-action lifecycle helpers extracted in E-CHAT-LIVE-011.
JST-GOV-002/008 remain unchanged: these declarations identify locally authored
code, retain all copied source/license attribution and introduce no external
Community dependency or ownership exception. Sent-context and runtime layout
changes remain governed by the UI chapter and focused evidence.

E-CHAT-LIVE-013 adapts the maintained Community navigation disclosure pattern
for repository groups in the owned session inbox. The new presentation component
retains pinned Svelte/CSS source attribution in docs/ui-provenance.json; native
session/repository identities and grouping remain application-owned. JST-GOV-008
ownership boundaries remain unchanged and no Community checkout dependency is
introduced.

The application-event browser harness serves only synthetic version-1 hints and
built client assets on an isolated loopback port. Its connection-count, fanout
and final-release evidence qualifies client transport; it cannot substitute for
live repository action timings or native authorization qualification.

E-THREADS-002 governs explicit demo Thread seeding and its launcher hook. The
seed covers zero, one and two attached repositories through the same receipt
authority as the live form. An ignored fixture marker prevents replay during
ordinary startup. Reset remains an explicitly authorized destructive development
operation; UI iteration and backend restart never reset collaboration state.

E-AGENT-RESUME-001 leaves JST-GOV-002/008 ownership unchanged. Development
launchers verify the selected v3 helper's prepared-context parser without
warming, opening or prompting an agent. The no-session probe rejects stale
resources before service replacement. Runtime observation and Send preparation
are application-owned helpers; no source aliases, copied dependencies or
credential authority are added. The jj-acp rejection fix has separate source
evidence and a separately rebuilt, ignored development resource.

E-CHAT-QUEUE-001 retains JST-GOV-002/008 ownership. The prompt queue and its
session-only projection are application-authored under web/src/lib, with native
ownership declarations in docs/ui-provenance.json. The shared footer composes
the public Design Core primary Button and Lucide arrow-up; existing copied
Community attribution remains intact. No external source dependency is added.

E-SETTINGS-SPLIT-001 retains JST-GOV-002/008 source ownership. Repository
Settings routes, guards and category models are application-authored; maintained
Community presentation remains owned under web/src/lib and released Design Core
supplies shared controls. Extracted header/layout helpers are declared in
provenance. Settings navigation changes client URL/session state only; server
API, configuration formats, authorization and repository authority are
unchanged.

E-SETTINGS-REFRESH-001 retains JST-GOV-002/008 ownership. Settings cards,
headings, icons, action footers and extracted field/registration presentation
are application-authored under `web/src/lib` and recorded in provenance. They
compose released Design Core primitives and retain the installed registry
versions. No Community source, catalog runtime dependency, package patch or
local substitution is introduced.

For E-SETTINGS-REFRESH-001, the user's explicit 2026-10-11 direction authorizes
application-scoped CSS for repository line-tab selection. It targets public
`data-state` attributes and theme tokens while retaining released Design Core
components, keyboard behavior and package versions. This narrow exception does
not authorize dependency patches, source aliases or local package overrides.

E-PROFILES-001 governs installation profile/library authority, explicit token
binding, immutable repository emoji snapshots and owned Community Settings
copies. The isolated profile harness builds the production bundle and serves an
empty disposable real-RustFS installation in both LocalTrusted and authenticated
modes. It uses synthetic credentials, checks restart/privacy and owns cleanup;
it changes no existing deployment, installed binary or visual baseline.

E-PROFILE-AVATAR-002 keeps image normalization in the server and presentation in
owned Settings components. Vite image CSP admits the same in-memory blob media
as the built UI, retaining other admission sources and API proxy boundaries.
Community attribution remains intact; the user-supplied Buzz screenshots guide
locally authored avatar actions, shortcode adornments and sidebar grouping.

Remote-agent integration rebases the verified implementation onto the current
main workspace, preserving profile normalization and grouped Settings. Inherited
cleanup extracts unchanged owned UI, controller and launcher compositions,
retains Storybook IDs and repairs obsolete navigation/readiness scenarios. New
copy boundaries retain original provenance; dependencies remain registry-only.

The authorized cleanup adds strict appended metadata correction validation and
lossless evidence repagination under JST-GOV-004/010/011. Original historical
payloads, results, task selections and archives remain intact. Qualify all
direct UI/story families, launcher/governance consumers, merged HTTP contracts
and cold remote/built-App flows. Integration is not installed-provider, Linux
real-storage, desktop or visual-baseline qualification. Advance main only after
verification, then forget and remove the temporary workspace while preserving
development data.

E-BUILD-STORAGE-001 adds application-owned local build reporting, managed Cargo
pre/postflight and explicitly registered temporary-target lifecycle. JST-CLI-004
now governs cache budgets and guarded cleanup; JST-DESK-001 custody is
unchanged. Tests use disposable Cargo/JJ workspaces and synthetic output, native
locks and open-file consumers. Actual cleanup measures filesystem free space and
never removes application state, credentials or a live target. Shared tooling
changes require the full affected launcher, documentation and contract-helper
tests plus isolated cold UI startup; they do not expand storage/platform/release
gates.

E-APP-RELOAD-001 adds explicit runtime activation, current shared policy and
publication fencing, retaining storage/native formats and operation pins. Its
owned reload UI uses maintained Settings components and released dependencies.
Isolated acceptance covers real storage and a cold built browser; inherited
diagnostics, worker deferrals and independent platform gates remain explicit.

E-WORKFLOW-CHOICES-001 retains JST-GOV-002/008 and JST-UI-015 local ownership.
The workflow revset field and bounded source-choice reader compose existing
History, directory and repository interfaces with released Design Core. The
existing text row and generic picker stylesheet are extracted to meet component
size limits, with owned declarations in provenance. No external checkout,
backend/schema format, identity remapping or execution authority is introduced.
M28 and visual/provider qualification gates remain unchanged.

E-WORKFLOW-RUN-001 retains JST-GOV-002/008 and JST-UI-015 local ownership. The
owned workflow source-choice helper composes existing repository reads and the
maintained CommandView picker with Settings presentation. Released Design Core
is unchanged; no external checkout dependency or storage format is added.
Operation-pinned pagination, stale-reply fencing and original unknown commands
remain independent of presentation. M28 and visual/provider gates are unchanged.

E-WORKFLOW-HISTORY-001 retains JST-GOV-002/008 and JST-UI-015 local ownership.
Workflow navigation and details compose maintained Projects/task tabs, rows,
ownership groups, profile hover cards, disclosure preferences and source
identity controls, with released Design Core tooltips and Empty. Catalog
ownership is extracted without changing pending-request custody; shared
ownership groups gain only a typed runs label. Native workflow authority,
formats and execution gates remain unchanged. Focused workflow and route checks
are the UI impact surface.
