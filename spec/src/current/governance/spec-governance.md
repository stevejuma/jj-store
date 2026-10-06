# Change map

`docs/ui-provenance.json` requires both `spec-governance.md` and `ui.md`. The
isolated `docs/capture-preview.md` runbook maps to `cli.md` and
`collaboration.md`; its launcher and admission examples remain governed
contracts. Provenance changes are protected ownership and presentation changes.

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
