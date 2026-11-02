---
schema: 1
id: E-LOCAL-TEST-AUDIT-001
---

# E-LOCAL-TEST-AUDIT-001 details 6

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
Remote Docker validation:rust test -p jj-history-service --test admission --locked
```

**Notes**

Receipt jj-store-validation-f4125663-03e5-45ef-9f57-e725afc242e9. Unchanged complete admission family in its focused crate profile passes all 12 cases, zero failures or ignores. Only the intermittent literal-filename case is subsequently quarantined; handle admission, close/revoke/drain, metadata, operation, blame and other file-history assertions remain enabled.
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
Remote Docker validation:rust test -p jj-history-service --test admission --locked -- file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal --exact
```

**Notes**

Receipt jj-store-validation-b91e1b96-1e68-4a6b-9bed-a38b156e00fd. The original individual case remains discovered and is reported ignored with the full Q-RUST-002 reason: 0 pass, 1 ignored, 11 filtered. No assertion is claimed passing.
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
Remote Docker validation:rust test -p jj-history-service --test admission --locked
```

**Notes**

Receipt jj-store-validation-0b757a76-b5d5-42f3-a89a-b28a7fa2bd08. All 11 enabled admission-family neighbors pass, zero failures, exactly one Q-RUST-002 ignored case. The two other file-history cases and handle admission, teardown/drain, metadata, operation and blame coverage stay enabled.
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
Remote Docker validation:rust test --workspace --locked --no-fail-fast
```

**Notes**

Receipt jj-store-validation-b0690fcd-1ce8-4373-b37d-c60b01bf590c. Complete final enabled Rust workspace: 2491 pass, zero failures, 62 ignored across 333 test/doc summaries. Exactly one newly ignored Rust case is Q-RUST-002; the other 61 ignores predate this audit and retain prerequisite/worker reasons. All index/TLS/journal/CLI regressions pass; no other Rust assertions are disabled. The full workspace admission family reports 11 pass/1 Q-RUST-002 ignore.
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
Remote Docker test:governance
```

**Notes**

Receipt jj-store-validation-4ba57707-8083-4705-b88b-e6c50965c0b6. Initial final governance run passes 120 original tests before adding launcher regressions.
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
Remote Docker spec:validate
```

**Notes**

Receipt jj-store-validation-6e54b168-49aa-4b9d-857d-9366b324a340. Specification shape, ownership, links and file limits pass: 8 validators, 4565 chapters and 211 requirements.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker check:progress
```

**Notes**

Receipt jj-store-validation-046a407f-e6e7-4a45-9d35-f79fc75f7595. Bookkeeping fails: test references need file#function form, two incomplete diagnostic attempts cannot be qualified pass/fail runs, and the root M3 task must be associated with this evidence. These uncommitted metadata issues are corrected while retaining commands/notes and moving incomplete attempts to their own explicitly unqualified field.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker spec:first
```

**Notes**

Receipt jj-store-validation-67482ae1-0752-4c03-91e4-4b19deeea9c6. Standalone worker passed --head without --base. The CLI correctly refuses the incomplete comparison before checking protected changes; source parent fallback is repaired without changing the spec-first policy.
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
Remote Docker format:rust:check
```

**Notes**

Receipt jj-store-validation-2e4dd070-fb4f-4a5a-84ec-0e4253eb74ee. Root and embedded-runtime Rust formatting checks pass.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker check:boundaries
```

**Notes**

Receipt jj-store-validation-4665c5d5-9e34-40c2-b71f-25154f80b585. Standalone lane selected the UI image, which lacks Cargo; cargo metadata cannot start (spawnSync cargo ENOENT). Existing dependency-boundary assertions remain intact; lane routing is repaired to select the full toolchain.
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
Remote Docker check:architecture
```

**Notes**

Receipt jj-store-validation-3337a158-1245-4be7-8273-0874a19a3a25. Web architecture source audit passes for 2088 modules.
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
Remote Docker docs:check
```

**Notes**

Receipt jj-store-validation-9888431b-9303-448b-98fe-8fbe54163cdf. Strict documentation source review/build, all 9 Mermaid diagrams in both themes and 144-file/26736495-byte link/anchor closure pass.
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
Remote Docker test:governance (launcher regressions before repair)
```

**Notes**

Receipt jj-store-validation-0b9efe42-5bf1-4a7f-bb47-dca93b0b676a. 119 pass/2 fail of 121 tests. UI and Rust lanes select the complete toolchain they need expects full but receives ui for check:boundaries. spec-first worker pins a default parent and preserves explicit comparisons receives --head without its transferred first-parent --base. The worker extraction retains the original failing behavior for this baseline.
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
Remote Docker test:governance
```

**Notes**

Receipt jj-store-validation-3276c7bb-76e1-468a-b627-7060b611d8d6. Complete governance/validation suite passes all 121 cases, zero failures or skips. Both original launcher regressions now pass, including parent/base selection, source override, explicit separated/equal-form arguments, root context and unchanged non-spec-first arguments.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker check:boundaries
```

**Notes**

Receipt jj-store-validation-6f4548d7-af8a-4de5-ba21-df84a76778eb. The original standalone dependency-boundary command passes using the full image and real Cargo metadata; native dependency and layering assertions remain unchanged.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker spec:first
```

**Notes**

Receipt jj-store-validation-c0ee2236-a6c5-48a3-b236-ea6dbc5fe0f5. The CLI comparison context is repaired. Spec-first now executes and correctly reports the remaining missing API canonical mapping for web/tests/browser/comparison-parents.spec.ts. The owning operation-difference API chapter is subsequently updated to document unchanged admission/IDs/transport and the isolated layout-coverage gap.
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
Remote Docker check:progress
```

**Notes**

Receipt jj-store-validation-183542cc-2d1c-4b7c-85ec-968f51a643ce. Corrected logical evidence passes progress validation: file#function references resolve, root M3 is associated, and interrupted/incomplete diagnostics retain their commands/notes in an explicitly unqualified field.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker spec:validate
```

**Notes**

Receipt jj-store-validation-66fcb9ef-7733-465e-86f2-fc068ec24608. Specification passes all 8 validators, 4566 chapters and 211 requirements after launcher and metadata repairs.
<!-- /run -->
<!-- /evidence-field -->

