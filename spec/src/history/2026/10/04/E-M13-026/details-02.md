---
schema: 1
id: E-M13-026
---

# E-M13-026 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj --locked (code translator iterations)
```

**Notes**

Initial pinned-API errors fixed; code-engine-check/test logs retain exact diagnostics.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --test conversion --locked (initial destination)
```

**Notes**

Missing destination directory then Store Arc mismatch during checkout; repaired using the destination workspace loader.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service --locked (graph iterations)
```

**Notes**

Six ambiguous Result diagnostics then two missing facade exports; fixed explicit Result and publication facade export. graph-check*.log.
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
cargo check -p jj-store-service --locked (graph final)
```

**Notes**

graph-check3.log: compile passed; eight Option-loop readability warnings subsequently removed.
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
cargo test -p jj-store-service --test conversion --locked (graphs)
```

**Notes**

graph-tests.log: 3 graph translation cases pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj -p jj-store-service --test conversion --locked (archive initial)
```

**Notes**

conversion-primitives-final.log: WorkStateV1 intentionally has no generic Serde persistence. Replaced attempted derivation with owned WorkSnapshotArchiveV1.
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
cargo test -p jj-store-jj -p jj-store-service --test conversion --locked (archive final)
```

**Notes**

conversion-primitives-final2.log and primitives-tests-final.log: 2 real GitBackend code cases and 5 service graph/history cases pass.
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
cargo test -p jj-store-jj --test local_evidence --locked
```

**Notes**

local-evidence-regression.log: 6 retained-capsule cases pass, including reopen/GC, full fields and tampering.
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
cargo test -p jj-store-jj -p jj-store-service --test conversion --locked (root mapping final)
```

**Notes**

primitives-rootmap-final.log: 7 cases; builtin source commit identity is included in mapping report.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-wal --all-targets --locked
```

**Notes**

primitives-check-final.log: affected packages/direct consumers compile without diagnostics.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-jj -p jj-store-service -p jj-store-wal --lib --locked -- -D warnings
```

**Notes**

primitives-lint-final.log: two new large enum diagnostics fixed using Box; one inherited query/language/identity.rs collapsible_if remains outside this conversion surface.
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
cargo test -p jj-store-wal --test native_copy --test work_store --test review_catalog --locked
```

**Notes**

primitives-wal-regression.log: 1 native-copy, 5 review-catalog and 6 work-store cases pass.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/projects.md spec/src/collaboration.md spec/src/publication.md spec/src/spec-governance.md; cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-wal
```

**Notes**

Owning prose and Rust formatted.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; cargo fmt --all -- --check; prettier --check changed canonical chapters and appended evidence object
```

**Notes**

primitives-governance logs; historical ledger evidence raw bytes preserved.
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
cargo test -p jj-store-jj -p jj-store-service --test conversion --locked (builtin mapping first)
```

**Notes**

Original test intentionally excluded builtin root; explicit root report added and assertion updated to account for all source capsule identities.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first (initial primitive evidence)
```

**Notes**

Six test references needed named test selectors; pending evidence cannot be marked pass; mutations canonical mapping required. Corrected evidence and chapter.
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
cargo test -p jj-store-jj -p jj-store-service --test conversion --locked (repaired final)
```

**Notes**

primitives-tests-repaired.log: 2 JJ and 5 service cases pass without diagnostics, including explicit builtin-root mapping.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-jj -p jj-store-service -p jj-store-wal --lib --locked -- -D warnings (repaired)
```

**Notes**

New conversion enum diagnostics resolved. Only inherited query/language/identity.rs:108 collapsible_if remains; no suppression or assertion changes.
<!-- /run -->
<!-- /evidence-field -->

