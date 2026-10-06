---
schema: 1
id: E-M17-073
---

# E-M17-073 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test opencode_recovery_index --locked
```

**Notes**

New fixture initially used unsupported json! array-repeat syntax; corrected. /tmp/jj-store-opencode-index-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test opencode_recovery_index --locked
```

**Notes**

Before interface implementation: six missing admitted_opencode_sessions method diagnostics. /tmp/jj-store-opencode-index-interface-before.log.
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
cargo test -p jj-store-capture --test opencode_recovery_index --locked
```

**Notes**

Three selected tests and an invoked fresh-process recovery child. /tmp/jj-store-opencode-index-after.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test opencode_recovery_index --test opencode_ingress --test source_registry --locked
```

**Notes**

Final twelve selected top-level tests plus recovery child; existing source identity/exclusivity/restart and sanitized ingress consumers pass. /tmp/jj-store-opencode-index-final.log.
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
cargo test -p jj-store-capture --lib --locked source_registry::
```

**Notes**

All four owning registry units, including exact index refusal without eviction. /tmp/jj-store-opencode-index-unit.log.
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
cargo check -p jj-store-capture -p jj-store-server --all-targets --locked
```

**Notes**

Changed capture package and direct server consumers compile. /tmp/jj-store-opencode-index-check.log.
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
cargo clippy -p jj-store-capture -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Strict two-package lint. /tmp/jj-store-opencode-index-clippy.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all
```

**Notes**

Run after implementation and fixture updates.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/verification.md
```

**Notes**

Changed prose formatting; evidence formatted independently to preserve historical raw JSON.
<!-- /run -->

<!-- run -->
### Run 10

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

Five validators, 21 chapters, 160 requirements.
<!-- /run -->

<!-- run -->
### Run 11

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

17 package boundaries.
<!-- /run -->

<!-- run -->
### Run 12

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

732 web modules; new Rust modules below 300 lines.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

First check preceded ledger insertion: same-change ledger/evidence absent. Append helper syntax/replacement issues were corrected and historical raw prefix restored from parent before validation.
<!-- /run -->

<!-- run -->
### Run 14

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

129 tasks; bookkeeping only.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

New evidence/table widths needed formatting. Formatted only after asserting historical prefix/object preservation.
<!-- /run -->

<!-- run -->
### Run 16

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

Rust formatting checked.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

Final changed-document formatting passes; all 327 historical raw JSON prefix/objects preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; qualified OpenCode 1.18.30; reused pinned sanitizer cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private source catalog version 1 adds optional opencode_sessions metadata: validated native ID, endpoint/witness digests, exact product version and source ID. Absent index is empty; existing 2 MiB/4096 budgets remain. No repository/wire/spool or legacy AgentStore change.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

