---
schema: 1
id: E-M17-062
---

# E-M17-062 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-native-import-doc-check.log 2>&1
```

**Notes**

Initial pending-slice governance; repeat after publisher review and final ledger updates.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import::ownership::revocation_during_publisher_native_await_preserves_exact_prepared_intent_without_commit --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-publisher-exact.log 2>&1
```

**Notes**

Exact regression proves revocation inside publisher native binding preserves prepared intent and leaves native root uncommitted.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import::ownership::revocation_during_publisher_native_await_preserves_exact_prepared_intent_without_commit --locked -- --exact --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-publisher-before.log 2>&1
```

**Notes**

Temporarily omitted the inner with_guard(check) at common recovery publication. Counterfactual proof fails: outer guard returned an error after commit, but the prepared queue entry was already removed (retained length 0 instead of 1). Final source restored in Python finally block before final exact and direct suites. Temporary unused-method warning disappears with restoration.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_management::tests::native_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-exact-complete.log 2>&1
```

**Notes**

All ten final native source-import tests pass, including exact retry after unknown enrollment plus append and revocation inside publisher native await.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_activation --test capture_admission --test capture_authority --test capture_source --test capture_resume --test capture_parking --test capture_reanchor --test capture_recovery --test capture_pipeline --test capture_pipeline_worker --test capture_supervisor --test capture_runtime --test capture_management --test capture_control_state --test capture_import_state --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-direct-final.log 2>&1
```

**Notes**

Final directly affected suites pass 178 top-level tests across 16 binaries, serial with original deadlines and scenario concurrency. Includes owning library, both journals and admission/authority/source/parking/resume/reanchor/recovery/pipeline/worker/supervisor/runtime/management/activation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test source_registry_import --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-native-import-capture-exact.log 2>&1
```

**Notes**

Exact import registry suite passes twelve top-level tests with no ignored tests; fresh-process parent proofs preserved.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-native-import-check-final.log 2>&1
```
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-native-import-clippy-final.log 2>&1
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-native-import-canonical-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-native-import-spec-first-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-native-import-boundaries-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-native-import-architecture-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-native-import-progress-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-native-import-rust-format-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-native-import-doc-check-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-native-import-doc-format-final.log 2>&1
```

**Notes**

Passed after publisher review; repeated after final ledger sealing.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-native-import-doc-format-sealed.log 2>&1
```

**Notes**

Repeated after final metadata sealing; raw parent entry prefix preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic qualified Codex CLI JSONL 0.156.1 and native MemoryStore; no real harness or storage qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/CLI/native/source/spool/control/import catalog format change. Runtime-private bounded qualified target view only; public source observations contain no paths or proofs. Current AgentStore only. Import-only registry marker and accepted complete-prefix receipt retain existing v1 formats.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lwmqylok
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

Current AgentStore only. Actual retained runtime target selection follows installation admission; replay matches original physical/header/session/profile metadata before stale selector rules, without inspecting unrelated transcript bodies. Bounded source driver retains manager/control/import settlement through native Write awaits, selects durable intent before source allocation, acquires shared import-only lease and opens common spool progress. Imported pipelines retain opaque original-receipt renewal through ingestion, exact queued recovery and parking; policy and prefix checks precede body parsing, conversion/enqueue and publication. No conversion to generic active pipeline. Raw-channel reservations survive native refusal and independently owned cancellation. Preselection failure leaves identity/intent/progress untouched; postselection failure retains receipt and marker. Review hardened renewal against missing/stopped original target and ensures an accepted boundary is reconciled on retry after unknown enrollment rather than reselected after append. Concrete escalation before wider runs: shared ingress/recovery callback extraction and runtime-control publication affect direct source, pipeline, parking, supervisor and management consumers. Run owning server library and direct admission/source/resume/parking/reanchor/recovery/pipeline/pipeline-worker/supervisor/runtime/management/both-journal suites, exact capture import registry tests, plus capture/server/direct CLI all-target checks and strict lint. Suites serial with existing deadlines and within-scenario concurrency. HTTP/CLI command contracts/UI, unrelated Rust workspace, root aggregates, real harness/storage/model and cold enabled startup remain out of scope. Scheduling, frame/source budgets, storage ownership, progress/API/CLI/browser import and M17.3-M17.7 remain pending; M17.2 stays In progress and full goal active. Preserve all 316 preceding entries and raw parent bytes. No installation/publication/migration/restarts/subagents. Broad run identified a fixture coordination error, repaired without weakening lock or assertion semantics. Review also propagated the retained consent callback into the common native publisher before/after native binding and after preparation, so revocation during a publisher await cannot rely on only an outer postcommit check. Added exact prepared-intent/no-commit regression; repeat exact, owning library/direct suites and governance. Include direct capture_authority and capture_activation consumers in final run because the shared guarded publisher changed; no native service/public contract change. Exact counterfactual publisher regression fails without the inner consent callback and passes with it; the outer guard alone cannot prevent a commit after revocation during native binding. One mixed patch initially missed formatter-wrapped fixture text and applied no mutations; reapplied using exact text. Ten final native regressions now own actual enrollment, ingestion, recovery and publication guards. Final wider commands are serial and preserve every existing deadline. Final acceptance: ten exact native import regressions pass; selected server direct suites pass 178 top-level tests across 16 binaries; exact capture import registry passes 12 tests. Capture/server/direct CLI all-target compilation and strict lint pass without suppression. Seven governance/format consumers pass. No unresolved functional failures. Existing macOS unwind-table linker warning remains unsuppressed. All 316 prior evidence entries and their raw JSON prefix remain unchanged, total 317. Source-level admission/ingestion/recovery slice is complete. Executable supervised import scheduling and source/frame/storage ownership, typed progress and HTTP/CLI/browser controls, remaining readers/enabled-startup and the full M17.3-M17.7 roadmap remain pending. M17.2 remains In progress, full goal active.
<!-- /evidence-field -->

