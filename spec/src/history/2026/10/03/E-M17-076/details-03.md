---
schema: 1
id: E-M17-076
---

# E-M17-076 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/cli.md spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node /tmp/jj-store-capture-ledger.mjs /tmp/jj-store-e-m17-076.json
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Changed Rust layer, canonical chapters or ledger only; targeted scope.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; OpenCode 1.18.30 source 3104c1428ec91f809e5ab86631300de41eb6952e; existing pinned Kingfisher cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public wire, repository, spool, reader checkpoint or source catalog encoding change. Project/session metadata and paths remain bounded owned-worker memory only. Current AgentStore only.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

27 selected top-level server tests pass: four new discovery, seven lifecycle, five read API, six source, three supervisor and two owned API-lane units. Default-instance status alone no longer misses native repository/workspace directories. Per-cycle project/directory/offered-session budgets are 4096 with one MiB response ceilings; malformed/overflow steps and unsafe paths fail closed. Ordinary project-directory metadata and global/non-Git session metadata supply candidates only; exact current native socket/catalog/Write checks bracket directory-scoped status. Duplicates are bounded, inactive metadata cannot enroll, and admitted offline-tail recovery survives project-list outage. Exact-limit global metadata reports incomplete coverage; full large-history inventory remains unqualified rather than silently complete. Direct API fixture consumers and strict all-target checking cover impact; unchanged Codex/type consumers, full server/workspace libraries, UI, public contracts and root aggregates remain out of scope. Controlled loopback API, private real files and native MemoryStore are separate from real harness/RustFS and configured process startup. M17/M17.2 remain In progress: management/import/export, common identity joins, Cursor and later M17.3-M17.7 remain. All 330 historical evidence objects and raw JSON prefix preserved.
<!-- /evidence-field -->

