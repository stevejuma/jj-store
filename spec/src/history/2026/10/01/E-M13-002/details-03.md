---
schema: 1
id: E-M13-002
---

# E-M13-002 details 3

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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-cli -- --check
```

**Notes**

Affected package Rust formatting passes.
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
pnpm check:architecture
```

**Notes**

312 modules pass architecture/size checks.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md
```

**Notes**

Ledger and verification table formatted.
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
pnpm check:progress
```

**Notes**

102 tasks pass evidence bookkeeping.
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
pnpm exec prettier --check docs/cli/work.md docs/cli/capabilities.md spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/mutations.md spec/src/projects.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

All changed documents pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test native_lifecycle http_work_acceptance::http_workstore_pages_commands_permissions_and_contract_capture
```

**Notes**

Native HTTP/RPC WorkStore authorization, receipt and contract regression passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- M13.3-M13.6 remain planned; unrelated aggregate/UI/real-RustFS lanes out of
  scope for this WorkStore slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib/jj-cli 0.45.1; Git 2.55.0; Rust 1.98.1; macOS ARM64. Development sources confined to local-jj; shared ignored Cargo target cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local Work root v1, V2 local profile snapshot/receipt/page; native V1 untouched. SHA-1 Git metadata identifiers distinct from native IDs. Actor-v1 private device record; canonical bounded cursors bind selected history.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

M13.2 WorkStore/CLI gate complete. Implement M13.3 portable anchor/capture evidence before direct local server/UI.
<!-- /evidence-field -->

