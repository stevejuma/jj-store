# Specification governance

Use the parent workspace test escalation ladder. The archived attachment is
non-normative; requirement changes belong here before or with implementation.
Generated book output is ignored. Only milestones.md carries implementation
status; reports hold immutable evidence rather than competing status summaries.

## Requirements

| ID          | Requirement                                                                                                                                                                                               |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-GOV-001 | The canonical specification MUST live under spec/src, build with mdBook, and assign unique JST requirement IDs with exactly one verification row each.                                                    |
| JST-GOV-002 | Protected implementation, interface, tooling, and agent-guidance changes MUST update their mapped canonical chapter in the same Jujutsu change.                                                           |
| JST-GOV-003 | Every implementation slice MUST update milestones.md with actual commands, results, failures, affected files, versions/formats, gaps, and next action in the same change.                                 |
| JST-GOV-004 | Progress checks MUST validate task/dependency references, evidence links/test names, completion gates, and same-change ledger updates; passing bookkeeping MUST NOT substitute for passing product tests. |
| JST-GOV-005 | Verified slices MUST be committed with Jujutsu while unrelated work and local workspace configuration remain uncommitted.                                                                                 |
| JST-GOV-006 | Rust formatting MUST use the pinned toolchain component and tracked rustfmt.toml, and MUST be checked by both normal and specification checks.                                                            |

## Change map

| Protected area                                                                                                                                                                                                                                 | Required chapter     |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| `^(?:catalog/\|\.storybook/\|docs/catalog\.md$)`                                                                                                                                                                                               | `ui.md`              |
| `^rustfmt\.toml$`                                                                                                                                                                                                                              | `spec-governance.md` |
| `^crates/jj-store-model/`                                                                                                                                                                                                                      | `identity.md`        |
| `^crates/jj-store-object/`                                                                                                                                                                                                                     | `object-store.md`    |
| `^crates/jj-store-wal/`                                                                                                                                                                                                                        | `publication.md`     |
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

The M0 contract harness is validated in both normal tests and spec checks. Real
storage qualification remains an explicit lane and cannot be inferred from the
harness self-tests or a source audit.

## M0 extension qualification

Native compatibility probes are normal Rust test targets. Their source/fixture
updates require canonical chapter changes and same-change ledger evidence.

The external compiled-storage lane is explicitly opt-in with the isolated RustFS
harness. Its immutable report records the command and actual test output; normal
test output must report this deployment test as ignored, not passed.

The opt-in `pnpm web:test:real` lane is distinct from captured-response browser
tests. It must cold-build and start the checkout-owned pinned RustFS and native
server with its configured Tantivy worker, prove authenticated snapshot-bound
search, publish from another native CLI process, observe SSE without automatic
snapshot replacement, complete an explicit refresh and clean up its owned
services. It receives ephemeral credentials only through process memory and
records no secret in Playwright artifacts or canonical evidence.

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
overlap failures.

Normal checks also run the Community-derived browser architecture audit. It
enforces counted-line limits and exact no-growth budgets for existing hotspots,
rejects production imports from catalog fixtures, and keeps reusable
presentation modules independent of API/controller ownership. Its focused
negative tests run in the governance lane.

The tracked pnpm hook may substitute an available sibling Design Core source for
dependency, development-dependency and optional-dependency fields only. Peer
ranges, the published package range and the committed registry lockfile remain
unchanged. Governance tests cover both local-source resolution and the registry
fallback when that sibling is absent; local installation uses `--lockfile=false`
so validation state cannot rewrite the release lock.

M4.8 review acceptance covers populated and filtered branch dividers, the
absence of duplicate history-page titles, shared column header styling, and
directory navigation history. It also covers the Files page navigation
breadcrumb, body-left file-tree sidebar padding, full-height divider, file
action row, bordered directory table card, commit/action summary row, in-card
snapshot selector, distinct table-header grey and root README/LICENSE preview
tabs. Selected file acceptance covers copied metadata, file statistics, filename
source headers, theme-background source bodies, markdown preview/source toggling
and `.rs` source highlighting through the public CodeBlock tokenizer prop. Cold
acceptance locates the persistent column controls after the redundant title bars
are removed. Screenshots remain visual-pending and cannot substitute for
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
