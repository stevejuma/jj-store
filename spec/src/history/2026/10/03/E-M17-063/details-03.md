---
schema: 1
id: E-M17-063
---

# E-M17-063 details 3

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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-scheduled-progress.log 2>&1
```

**Notes**

Initial pending-slice check passed; repeat after final evidence.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_admission --test capture_authority --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_management --test capture_activation --test capture_control_state --test capture_import_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-direct.log 2>&1
```

**Notes**

Owning library: 82 passed, 1 failed; remaining selected binaries not executed. Preserved cached completed prefix initially returned ChangedDuringRead after a rewritten prefix, violating the existing exact rejection assertion. Invalidate only a completed witness at a changed snapshot, reconstruct accepted bytes before body work, and preserve incomplete scan state/budget. Restore original refusal behavior; no test weakened.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md > /tmp/jj-store-agent-capture-m172-scheduled-prefix-doc-format.log 2>&1
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import::rewritten_selected_prefix_and_physical_replacement_cannot_inherit_consent --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-prefix-repaired.log 2>&1
```

**Notes**

Existing exact rewritten-prefix/physical-replacement refusal regression passes with completed snapshot invalidation. Original assertion and deadline unchanged.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-scheduled-check.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-scheduled-lint.log 2>&1
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_admission --test capture_authority --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_management --test capture_activation --test capture_control_state --test capture_import_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-scheduled-direct-final.log 2>&1
```

**Notes**

Final 189 top-level tests across 16 binaries pass, including all nine scheduled imports, ten native imports, stale actual raw-state handoff, executor settlement and existing cold owner/storage qualification. Serial suites; existing deadlines and within-scenario concurrency preserved.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-scheduled-rust-format.log 2>&1
```
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-scheduled-doc-check-interim.log 2>&1
```

**Notes**

Interim documents formatted; repeat after final evidence.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-scheduled-ledger-format-interim.log 2>&1
```
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-scheduled-canonical-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-scheduled-spec-first-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-scheduled-boundaries-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-scheduled-architecture-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-scheduled-progress-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-scheduled-rust-format-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-scheduled-doc-check-final.log 2>&1
```

**Notes**

Final protected source/doc/evidence checks pass; repeat ledger consumers after recording these results.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-scheduled-doc-format-final.log 2>&1
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-scheduled-ledger-format-final.log 2>&1
```

**Notes**

Repeated after final verified run metadata. Final canonical/progress/doc consumers also repeated with their same recorded commands.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI JSONL 0.156.1 and native MemoryStore; no real harness/storage release qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/CLI/native/source/spool/control/import catalog format change. Opaque body-free dispatch and single runtime mailbox; one original command per retained source slot. Current AgentStore only; accepted-prefix and import-only marker retain existing v1 formats.
<!-- /evidence-field -->

