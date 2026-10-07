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
