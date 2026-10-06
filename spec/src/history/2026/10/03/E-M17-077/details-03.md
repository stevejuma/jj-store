---
schema: 1
id: E-M17-077
---

# E-M17-077 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress -- --exact --test-threads=1
```

**Notes**

Exact failing case passes unchanged in isolation (9.12s); this alone did not satisfy the complete management module gate. /tmp/jj-store-publication-management-regression.log.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked capture_management -- --test-threads=1
```

**Notes**

All original 50 management tests pass with unchanged assertions/deadlines, sequentially (141.38s). Parallel-load deadline qualification remains limited by the recorded failed run; no full server/workspace pass claimed. Existing linker diagnostic retained. /tmp/jj-store-publication-management-serial.log.
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
node /tmp/jj-store-capture-ledger.mjs /tmp/jj-store-e-m17-077.json
```

**Notes**

Affected Rust/canonical scope or evidence bookkeeping validated; all 331 parent evidence objects/raw prefix preserved.
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

Affected Rust/canonical scope or evidence bookkeeping validated; all 331 parent evidence objects/raw prefix preserved.
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
pnpm spec:validate
```

**Notes**

Affected Rust/canonical scope or evidence bookkeeping validated; all 331 parent evidence objects/raw prefix preserved.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check
```

**Notes**

Affected Rust/canonical scope or evidence bookkeeping validated; all 331 parent evidence objects/raw prefix preserved.
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

Changed canonical documents formatted; spec-first passed nine protected files and three owning canonical chapters.
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
pnpm spec:first
```

**Notes**

Changed canonical documents formatted; spec-first passed nine protected files and three owning canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; OpenCode 1.18.30 profile; pinned Kingfisher sanitizer.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version-1 private publication catalog adds optional typed snapshot digest/count; original JSONL range encoding is unchanged. No public wire, repository or AgentStore format change. Existing lock, bounds and exact native receipt chain retained. Current AgentStore only.
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

186 selected top-level tests pass: 32 exact publication tests, 101 direct reader/native API and management consumers, three catalog units and 50 management units run sequentially. The isolated deadline case is a duplicate verification and is not added to this count. An earlier parallel management run exceeded one existing ten-second deadline; the exact unchanged test and complete unchanged module pass serially. Parallel-load timing remains explicitly unqualified, with no assertion/timeout relaxation or full-suite claim. Strict affected-package/all-target checks and canonical governance pass. Shared catalog/inspection lock has file, API and native-management consumers, explaining selected escalation. Unknown/mismatched outcomes, empty status counts, replacement generations, pressure, missing/unqualified history and body-free read-only inspection retain exact receipt authority. Original JSONL encodings are unchanged. Controlled API/native MemoryStore acceptance is distinct from real harness/RustFS and configured startup. Unchanged UI, public contract generation, unrelated packages and root aggregates are out of scope. M17/M17.2 stay In progress: public snapshot management/import/export, common identities and Cursor, followed by M17.3-M17.7. All 331 historical evidence objects/raw prefix preserved.
<!-- /evidence-field -->

