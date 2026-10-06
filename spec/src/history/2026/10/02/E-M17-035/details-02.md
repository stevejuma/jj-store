---
schema: 1
id: E-M17-035
---

# E-M17-035 details 2

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
cargo test -p jj-store-capture --lib current_binding_distinguishes_absence_from_unknown_io_outcomes --locked > /tmp/jj-store-agent-capture-m172-rotation-classifier-final.log 2>&1
```

**Notes**

One metadata outcome classifier passes; absence differs from unknown OS errors.
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
cargo test -p jj-store-capture --test source_registry --test source_registry_policy --test codex_bound_source --test codex_activity --test codex_resume --test source_parking --test codex_recovery --locked > /tmp/jj-store-agent-capture-m172-rotation-runtime-final.log 2>&1
```

**Notes**

44 direct runtime cases pass; two explicitly ignored cases remain unqualified.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_activity --test capture_activation --test capture_pipeline --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline_worker --locked > /tmp/jj-store-agent-capture-m172-rotation-server-final.log 2>&1
```

**Notes**

54 direct native cases pass, including all 18 supervisor cases and the four rotation cases.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-rotation-check-final.log 2>&1
```

**Notes**

Affected packages and direct CLI consumers compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-rotation-clippy-final.log 2>&1
```

**Notes**

Affected packages and direct CLI consumers pass strict lint.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-rotation-rust-format-final.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rotation-docs-final.log 2>&1
```

**Notes**

Changed canonical documents and ledger formatted.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-rotation-validate-final.log 2>&1
```

**Notes**

Canonical specification validates.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-rotation-first-final.log 2>&1
```

**Notes**

Same-change chapter protection passes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-rotation-boundaries-final.log 2>&1
```

**Notes**

17 package ownership boundaries pass.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-rotation-progress-final.log 2>&1
```

**Notes**

129 tasks validate; E-M17-035 preserves the previous 289 entries and keeps M17/M17.2 In progress.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rotation-doc-check-final.log 2>&1
```

**Notes**

Checking formatting...
[warn] spec/src/milestones.md
[warn] Code style issues found in the above file. Run Prettier with --write to fix.

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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rotation-doc-format-sealed.log 2>&1
```

**Notes**

Seal ledger formatting after adding command evidence.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rotation-doc-check-sealed.log 2>&1
```

**Notes**

Final changed-document format check.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-rotation-validate-sealed.log 2>&1
```

**Notes**

Final canonical validation.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-rotation-first-sealed.log 2>&1
```

**Notes**

Final protected-chapter validation.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-rotation-progress-sealed.log 2>&1
```

**Notes**

Final evidence bookkeeping.
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

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI files and directory/socket admission, actual native MemoryStore publication. No RustFS, real JJ directory proof, live harness or cold whole-server qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted or wire format changes. Current-only AgentStore, source catalog/admission, source cursor, physical preparation reservation, exact native request/actor/receipt identities remain unchanged. Read-only strict current-binding inspection and memory-only compact retirement add no identity, lease, progress or publication capability.
<!-- /evidence-field -->

