# Change map

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
