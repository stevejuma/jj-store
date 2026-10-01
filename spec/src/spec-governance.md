# Specification governance

The targeted verification policy in the tracked agent guide is jj-store's
controlling interpretation of the parent workspace test escalation ladder. The
archived attachment is non-normative; requirement changes belong here before or
with implementation. Generated book output is ignored. Only milestones.md
carries implementation status; reports hold immutable evidence rather than
competing status summaries. Integration-modified production modules are
extracted below the 300-line budget; Storybook interaction and accessibility
checks remain required while visual baselines stay separately approved. Every
directly affected boundary still needs named evidence, while unrelated
repository suites remain outside the default verification scope.

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                              |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-GOV-001 | The canonical specification MUST live under spec/src, build with mdBook, and assign unique JST requirement IDs with exactly one verification row each.                                                                                                                   |
| JST-GOV-002 | Protected implementation, interface, tooling, and agent-guidance changes MUST update their mapped canonical chapter in the same Jujutsu change.                                                                                                                          |
| JST-GOV-003 | Every implementation slice MUST update milestones.md with actual commands, results, failures, affected files, versions/formats, gaps, and next action in the same change.                                                                                                |
| JST-GOV-004 | Progress checks MUST validate task/dependency references, evidence links/test names, completion gates, and same-change ledger updates; passing bookkeeping MUST NOT substitute for passing product tests.                                                                |
| JST-GOV-005 | Verified slices MUST be committed with Jujutsu while unrelated work and local workspace configuration remain uncommitted.                                                                                                                                                |
| JST-GOV-006 | Rust formatting MUST use the pinned toolchain component and tracked rustfmt.toml, and MUST be checked by both normal and specification checks.                                                                                                                           |
| JST-GOV-007 | Verification MUST remain within the changed layer and direct consumers by default; full suites require independent cross-layer or multi-consumer impact, shared infrastructure, release qualification, or an explicit user request, with the escalation reason recorded. |

## Change map

| Protected area                                                                                                                                                                                                                                 | Required chapter     |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| `^(?:catalog/\|\.storybook/\|docs/catalog\.md$)`                                                                                                                                                                                               | `ui.md`              |
| `^rustfmt\.toml$`                                                                                                                                                                                                                              | `spec-governance.md` |
| `^crates/jj-store-model/`                                                                                                                                                                                                                      | `identity.md`        |
| `^crates/jj-store-object/`                                                                                                                                                                                                                     | `object-store.md`    |
| `^crates/jj-store-wal/`                                                                                                                                                                                                                        | `publication.md`     |
| `^crates/jj-store-replication/`                                                                                                                                                                                                                | `replication.md`     |
| `^crates/jj-work/`                                                                                                                                                                                                                             | `projects.md`        |
| `^crates/jj-(?:agent\|review)/`                                                                                                                                                                                                                | `collaboration.md`   |
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

## Verification scope

A logical behavior remains one verification surface when it spans its production
module, colocated styles, direct tests or stories, canonical chapter and ledger.
The number of edited files or stories does not trigger aggregate gates. UI-only
work runs the relevant web or catalog check plus the exact interaction,
accessibility or browser acceptance for that surface. Rust-only work runs the
affected package or crate checks and exact regression without unrelated UI or
workspace lanes.

Escalation to a full relevant suite requires impact across independent product
layers, packages, deployable surfaces or consumers, or a change to shared build,
test or runtime infrastructure capable of altering unrelated lanes. Release
qualification and an explicit user request also authorize escalation. Evidence
records the concrete reason before the wider command runs. Root aggregate
`pnpm spec:check`, `pnpm check`, `pnpm test` and `pnpm build` commands are not
default completion gates for a single-surface change.

Governance commands that read tracked specification state through child
processes use an explicit bounded output allowance large enough for the current
ledger. Growth beyond that allowance fails closed instead of truncating parent
evidence used by same-change validation.

Progress validation enumerates every direct parent of a JJ merge, audits the
union of changed paths and retains every parent's evidence signatures. Merely
combining existing entries does not satisfy the same-change integration gate;
the merge must add or update evidence absent from every parent. The published
spec-first CLI accepts explicit common-ancestor and merge commit IDs through
`--base` and `--head` for merge qualification; its default `@-` selection is
single-parent only.

The M0 contract harness is validated in both normal tests and spec checks. Real
storage qualification remains an explicit lane and cannot be inferred from the
harness self-tests or a source audit.

M9 owns the LocalTrusted AgentStore and ReviewStore delivery. Its publication
foundation is governed by `publication.md`; new domain crates and capture,
anchor, review or collaboration behavior are governed by `collaboration.md`.
Browser diff-line and agent-tool review creation, exact anchor resolution, route
retention and inline presentation are jointly governed by `collaboration.md` and
`api.md`. This includes byte-offset to line-coordinate presentation and the
exact CSP allowance required by the pinned Community emoji dependency. Each
separately registered captured-response fixture in `scripts/contracts.mjs` must
be recorded here and in the M9 evidence ledger so generation cannot widen
runtime acceptance without same-change governance review. Upstream Community and
Design Core changes remain separate source-owned changes with consumer
acceptance in this repository.

M10 owns local-first writable replication. Its origin ledger and imported
publication path are jointly governed by `replication.md` and the existing
publication requirements. Native head integration is also governed by
`jj-integration.md`; agent and review fact reducers remain governed by
`collaboration.md`. Backup and passive promotion retain their independent
adapter requirements.

Tracked agent guidance and CLI activation documentation describe the implemented
single-upstream profile as active while keeping its expanded qualification and
stock-JJ filesystem mirroring explicitly deferred. Scope alignment updates must
preserve that boundary in this chapter and in the milestone ledger.

The checkout-owned development harness stores its interactive fixture only in
the ignored `.local/dev-demo` subtree. Restart and stop preserve that fixture;
reset is the explicit destructive reseed boundary. Reset publishes
representative Codex, Cursor and OpenCode activity through the production hook
path, then creates repository, operation, revision, code-text, code-artifact,
agent-text and agent-artifact review anchors through the production ReviewStore
command route. Cold browser acceptance must provide a separate temporary root so
its publications and cleanup cannot alter interactive state. Harness state and
process markers remain separate so a stop cannot leave a stale PID that blocks
the next restart. When inline anchored review patches coexist with a primary
changed-file disclosure, browser acceptance MUST select the primary disclosure
explicitly. The loopback-only harness may recognize an exact validated Gravatar
request only to abort it locally; it MUST NOT permit an external response or
widen general network access.

## M0 extension qualification

Native compatibility probes are normal Rust test targets. Their source/fixture
updates require canonical chapter changes and same-change ledger evidence.

The external compiled-storage lane is explicitly opt-in with the isolated RustFS
harness. Its immutable report records the command and actual test output; normal
test output must report this deployment test as ignored, not passed.

The opt-in `pnpm test:local-release` lane is distinct from captured-response
browser tests. It must cold-build, initialize a persistent format-2 LocalTrusted
installation, start checkout-owned embedded RustFS and the native server,
auto-open the sole catalog repository without credentials, publish from another
native CLI process over the owner-only Unix socket, observe token-free SSE
without automatic snapshot replacement, complete an explicit refresh, and record
cold/restart startup time plus server/RustFS RSS against the macOS ARM64
ceilings. `JJ_STORE_REAL_KEEP=1` may retain the owned stack for review;
otherwise the lane cleans it up. Runtime metadata and Playwright artifacts
record no secret. The development stack additionally configures token-free
LocalTrusted search and waits for its first current Tantivy generation before it
reports readiness. Authenticated/shared deployment, registry-only M4
finalization and shared deployment acceptance are deferred and do not gate this
lane.

Publication faults are exercised in the normal WAL tests. The opt-in deployment
lane records both the initial compiled native publication run and a separate
post-kill recovery process; neither is inferred from normal ignored tests.

## Rust formatting

The pinned Rust toolchain includes rustfmt. The root rustfmt.toml fixes Rust and
formatting editions to 2024 and uses Unix line endings. `pnpm format:rust`
formats every workspace crate and the separately locked embedded RustFS runtime,
including examples and tests; `pnpm format:rust:check` checks those files
without changing them. `pnpm format` and `pnpm format:check` include Rust
alongside Prettier. Both `pnpm check` and `pnpm spec:check` reject Rust
formatting drift.

The RPC layer may consume the owned object streaming interface. Its native JJ
trait adapters remain in the CLI crate, and direct upstream JJ dependencies in
RPC are rejected by the existing boundary checker.

The boundary checker treats `jj-agent` and `jj-review` as owned layers even
though their names do not use the `jj-store-` prefix. `jj-agent` may depend on
the neutral model layer. `jj-review` may additionally depend on `jj-agent` for
immutable activity-object references. Neither can depend on WAL, service, native
JJ, transport or presentation layers. WAL may depend on the domain crates to
validate typed roots and immutable objects before shared publication. The
boundary checker admits the exact `jj-store-wal` to `jj-review` dependency and
continues to reject review dependencies on WAL or higher layers.

Capture tooling owns the pinned MongoDB Kingfisher Rust integration directly in
the CLI crate. The local pnpm hook has no agent-redaction package exception, and
its focused tests continue to prove Design Core sibling resolution and the
no-sibling fallback. The in-process sanitizer module stays below the production
line budget and returns only validated sanitized payloads with the fixed policy
identity or a fixed omission state.

Evidence entries can explicitly identify removed files and retired named tests.
The progress checker accepts historical references only when a later entry
records the exact removal, verifies removed files are absent and keeps every
other historical evidence path and named regression live. Its Jujutsu reads use
a fixed 16 MiB output ceiling so the single growing ledger can be validated
without an unbounded child-process buffer.

Server composition has an explicit layer in the boundary checker. Regression
coverage rejects dependencies on CLI presentation and direct upstream JJ types,
while permitting the owned native/service/RPC composition interfaces.

The standalone server storage lane qualifies TLS using a generated certificate
and disposable credentials. Retained reports state transport and topology scope;
previous plaintext-loopback reports remain unchanged. Protocol-rejection
fixtures assert a local transport failure rather than timing-dependent gRPC
status codes. No generated private keys are retained in reports or checkouts.

Query resource measurements run through `scripts/qualify-query-resources.py`
against an explicitly built release fixture binary. Reports retain the actual
process result and scope, including resource/admission failures. Ordinary green
unit checks do not imply a successful release resource run.

The resource harness supports separate graph and content fixture processes.
Content evidence includes positive boundary inputs and explicit oversized-input
rejections, preserving checksums, exact commands, CPU, elapsed time and RSS.

The predicate resource scenario runs the explicitly compiled release native
lifecycle test binary under an independent process timeout. It requires each
named admission/parity test and records actual process CPU/RSS separately from
compilation. A hung debug fixture must be diagnosed and recorded, not counted as
a passing deadline test.

Large graph resource qualification records cold admission and explicit index
preparation separately from prepared/warm graph queries. The harness requires a
successful preparation report for both linear and wide-head fixtures; an
explicit rebuild-required response alone is not a passing query or completed
task.

The server may depend on the HTTP transport for listener composition. HTTP keeps
its existing service/model dependency boundary and consumes only owned
contracts; it cannot depend directly on storage, CLI presentation or upstream JJ
libraries.

Generated OpenAPI and TypeScript contracts have a drift check in both normal and
specification checks. The generator uses the compiled HTTP route inventory;
request and response schema modes preserve Serde's optionality rules. Contract
tests validate the OpenAPI document and representative serialized native DTOs.
The M4.10 capture also executes the registered reference, recursive-path,
operation-change and diff-summary handlers. It proves repository authorization,
exact snapshot pinning, signed query-bound forward/backward continuations and
zero publication rather than validating generated shapes alone.

The generated-contract lane also executes the registered LocalTrusted AgentStore
session, transcript, relation and checkpoint handlers plus ReviewStore
collection, detail, command and durable-status handlers. Separate exact fixtures
capture collection/detail responses, preserve one root snapshot across resumed
pages, validate current plus historical immutable checkpoint detail and replay a
review receipt after transport reconstruction against generated OpenAPI. Every
fixture must report one executed test. A zero-test filter, schema-only assertion
or uncaptured response cannot count as collaboration transport evidence.

Browser checks include Svelte/TypeScript analysis, focused client tests and cold
Playwright interactions/accessibility against built assets. Functional and
accessibility failures remain blocking; visual review is separate and remains
visual-pending. Browser sources and tooling require same-change ledger evidence.

User-requested scope deferrals must retain original requirements and evidence.
The 27 September 2026 browsing resumption covers M4.9–M4.14; M4.15–M4.17
subsequently connect search, core repository mutations and native conflict
editing. Backend tasks may depend on completed API foundations independently of
browser progress.

Contract checks validate both read/SSE and typed mutation responses captured
from real registered handlers. Mutation request schemas and generated TypeScript
narrowing reject untyped commands and caller-selected principals. Regenerating
API declarations does not close any deferred browser feature or deployment gate.

The contract lane captures typed abandon preview/apply/no-change responses as
well as foundational mutations and reads. New nonempty optional report fields
must conform to generated schemas while older V1 report bytes remain unchanged.

Contract capture includes atomic conflict preview/apply and rejection responses.
The generated command union exposes only implemented resolution actions.
Conflict-editor acceptance separately covers descriptor loading, complete
multi-path selection, stale fingerprints and the real native workflow.

The generated-contract lane also captures actual binary-upload results and
uploaded-file mutation responses. Upload IDs have canonical-width schema checks
and the staging record has an independently calculated digest fixture. Browser
tests additionally require the upload query binding and atomic command body.

Dependency checks permit service-to-adapter authorization wiring while retaining
the adapter-to-service/native/UI prohibition. Adapter runner and fault fixtures
join normal Rust and specification validation; their tests cannot substitute for
real storage deployment or Tantivy-specific durability qualification.

Generated-contract checks capture registered search page, status and freshness
error responses through a real native fixture. The capture must execute its
exact named test and produce samples; a zero-test invocation cannot count as
contract evidence. Generated API declarations remain permitted during UI
deferral.

Server composition may depend on owned adapter and Tantivy interfaces. The layer
checker continues to reject upstream JJ imports in server code and all search or
transport dependencies in authoritative storage/publication layers.

## Catalog validation

Production presentation source lives under `web/src/lib`; catalog source owns
only fixtures, stories and harnesses. Both areas require the UI chapter and
same-change ledger evidence. Normal and specification lanes include catalog
types, unit tests, Storybook interactions/accessibility and cold static builds.
The catalog may import production presentation modules but its source audit
rejects API clients and network transports. Production cannot import catalog
fixtures. The progress checker recognizes catalog and Storybook paths.

Cold catalog acceptance builds static assets and serves them on an owned
ephemeral loopback port. Browser tests reject API, external HTTP and WebSocket
requests, verify responsive navigation and fresh-instance state, and retain
unapproved screenshots only as local review artifacts. Virtual-row geometry is
compared within each component's directly owned row container and in rendered
vertical order, so nested listings and independent columns cannot produce false
overlap failures. The production web manifest includes only flat, regular
`agent-icons/*.svg` files from the public icon directory; source attribution
documents and all other public files remain outside the served allowlist.

Normal checks also run the Community-derived browser architecture audit. It
enforces counted-line limits and exact no-growth budgets for existing hotspots,
rejects production imports from catalog fixtures, and keeps reusable
presentation modules independent of API/controller ownership. Its focused
negative tests run in the governance lane.

LocalTrusted catalog stories exercise bootstrapping, multiple registered
repositories and the empty catalog using the same production component. The
Storybook lane keeps interaction and accessibility failures blocking. Cold
browser fixtures must provide the bootstrap phase explicitly and assert the
protocol header plus the presence or absence of bearer credentials for the
selected access mode. Visual baselines remain a separate approval lane.

The LocalTrusted demo seed is verified at its native boundary: the unresolved
file retains JJ conflict terms, while the divergent history presents distinct
commit IDs under one change ID. Focused real-browser acceptance asserts those
full identities through production controls. Catalog graph units separately
prove stable lane geometry, so native divergence is not inferred from color or
screen position. The final seeded working copy intentionally has an empty native
description so the production fallback is exercised without fixture-only
presentation data.

The tracked pnpm hook may substitute an available sibling Design Core source for
dependency, development-dependency and optional-dependency fields only. Peer
ranges, the published package range and the committed registry lockfile remain
unchanged. Governance tests cover both local-source resolution and the registry
fallback when that sibling is absent; local installation uses `--lockfile=false`
so validation state cannot rewrite the release lock.

M9.5 extends that guarded development resolution to the declared
`@lapismd/lapis-community` range. The manifest remains a normal registry range;
the hook substitutes only an available clean sibling checkout for local
acceptance, and its focused tests prove both substitution and registry fallback.
The catalog provenance record pins the inspected Community and Design Core
source revisions and states whether code was consumed through public exports or
adapted locally. Collaboration stories remain `visual-pending` while their
Chromium interactions and automatic accessibility checks are blocking.

M4.8 review acceptance covers populated and filtered branch dividers, the
absence of duplicate history-page titles, shared column header styling, and
directory navigation history. It also covers the Files page navigation
breadcrumb, body-left file-tree sidebar padding, full-height divider, bordered
directory table card, commit/action summary row, in-card snapshot selector,
distinct table-header grey and root README, CODE_OF_CONDUCT, CONTRIBUTING,
LICENSE/LICENCE and SECURITY preview tabs. Landing-document acceptance covers
friendly labels, document icons, compact type, the width-measured `More`
dropdown, its hidden-document selected treatment and the selected-document edit
action. It also covers body repository tabs, the moved breadcrumb and
authenticated UTF-8 file edit/rename preview and publication. Selected file
acceptance covers copied metadata, file statistics, filename source headers, the
persistent Preview/Source/Edit header, body-only editing, Save/Cancel and the
description dialog, the borderless line-numbered Mira MDE body with
filename-matched language support, theme-background source bodies,
draft-preserving Markdown preview/source toggling and `.rs` source highlighting
through the public CodeBlock tokenizer prop. Cold acceptance locates the
persistent column controls after the redundant title bars are removed.
Screenshots remain visual-pending and cannot substitute for
interaction/accessibility checks.

The cold catalog lane checks native pointer and touch split-copy behavior,
constant-width geometry and reduced motion against freshly built assets.
Synthetic story hover events alone do not establish CSS hover acceptance.

Split-copy cold acceptance checks four-character resting labels and expanded
five-character prefixes with two dots and a two-character suffix. It prevents
identifier clipping while either section expands. Single-identifier stories
verify that the prefix, dots and suffix are visible immediately at rest.

The default split-copy control stays within 122px width and 27px height at the
standard root scale, including active and independent feedback states, with
balanced inline padding.

M4.16 mutation catalog stories exercise real menu, form, preview, confirmation,
apply, status and close controls against an isolated in-memory controller. The
production browser lane separately proves captured HTTP requests and a cold
native RustFS-backed mutation. Because the application canvas already owns
scrolling, repository menus and dialogs disable redundant body scroll locks and
tests require body pointer/overflow state to be restored before navigation.
Visual tests remain skipped at the user's request and stories remain
visual-pending.

Collaboration presentation acceptance keeps focused agent/review Storybook
interactions, the complete 84-story interaction and accessibility suite, a cold
static catalog browser, production controller tests and an owned LocalTrusted
reset distinct in the evidence ledger. The live reset must record that seeded
hook capture currently lacks an exact code-association publication path; catalog
association coverage cannot be presented as live Agent Context evidence.

## Native WorkStore

M11 owns native projects and tasks under projects.md. The boundary checker
registers jj-work as a neutral model-only layer, with WAL allowed to depend on
it. WorkStore persistence changes also require publication.md; native copy and
fail-closed replication require explicit regression evidence.

## WorkStore service and transports

The WorkStore transport capture executes registered work page, command and
durable-status handlers. Its generated contracts preserve typed
command/precondition fields and full entity IDs. Native transport and CLI
acceptance use disposable local sessions and do not replace installed binaries.

M11 browser evidence records the exact Projects story/unit/Playwright targets.
The cold real-browser runner accepts an explicit test-file filter and an
explicit existing pinned RustFS runtime to avoid rebuilding unchanged storage.
It retains fresh isolated initialization, native runtime verification and owned
cleanup.

M11 updates the tracked execution scope with the native local-service profile
and the WorkStore/M10 boundary. Development reset finishes the full sample and
its search projection on a temporary instance, then measures a fresh server and
storage restart through server/search readiness. Complete initial preparation is
measured separately; resource limits remain unchanged. This proves opening
persisted sample state, while search cache-loss rebuild qualification remains a
separate gate.

## Git bridge governance

Git bridge production modules and their direct tests additionally map to
`git-interop.md`, alongside their existing owning layer chapters. M12 tracks GB0
through GB6; every slice retains exact targeted evidence and explicit
unimplemented gates. Attached sketches never satisfy product acceptance.
