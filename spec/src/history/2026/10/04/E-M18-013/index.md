---
schema: 1
id: E-M18-013
---

# E-M18-013

<!-- evidence-field:id markdown -->
## Id

E-M18-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18
- M18.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mpsxrmoz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Separate broad server-library diagnostic and baseline cold-publication timeout reproduction
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1/macOS ARM64 debug profile; same checked-in native sanitizer and retained capture fixtures.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Diagnostic evidence only; no assertion, timeout, worker ownership, sanitizer, persistence or wire format weakened. Previous three management production files temporarily selected from parent vsusztpy for baseline compilation, then restored byte-for-byte from task backup before final queue regression.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
- docs/capture-preview.md
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import/handoff.rs
- crates/jj-store-server/src/capture_supervisor/opencode/tests.rs
- crates/jj-store-server/src/capture_supervisor/tests.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#accepted_source_refuses_other_request_without_replacing_original_owned_work
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-server --lib
```

**Notes**

rust-lib.log:95 passed/15 failed/110 total,863.14s. Escalated beyond focused read checks to inspect all existing control/import/runtime/storage consumers sharing the coordination boundary. Thirteen scheduled-import cases fail publication/readiness deadlines (12 at shared published helper scheduled_import.rs:76; one at handoff.rs:47); OpenCode publication did not enter gate; source-degradation readiness times out. New observation regressions, existing cancellation/revocation/physical-journal, foreground, storage lifetime and cold running-loss ownership tests pass. No full-library pass is claimed.
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
cargo test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::accepted_source_refuses_other_request_without_replacing_original_owned_work -- --exact --nocapture (current)
```

**Notes**

rust-import-regression.log: same scheduled_import.rs:76 ten-second publication wait expires; process finishes166.43s because retained cold owner drains. No control/source observation is performed after scheduling in this scenario.
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
Temporarily select parent production management status/source/import read implementations via jj file show -r @-; cargo test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::accepted_source_refuses_other_request_without_replacing_original_owned_work -- --exact --nocapture (baseline); restore and verify saved source bytes
```

**Notes**

rust-import-baseline.log: identical scheduled_import.rs:76 Elapsed failure with pre-change try-lock implementations; process finishes122.71s. This representative cold publication failure predates the queue change. Other broad failures were not individually baseline reproduced and remain unqualified. Current implementation restored after baseline compilation; final three queue regressions pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (after deferred-plan clarification)
```

**Notes**

JST-GOV-002: docs/implementation-plan.md mapped to architecture.md, which this narrow slice does not change. The extra plan note duplicated existing Deferred scope and was restored from parent instead of expanding the implementation.
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
jj restore --from @- -- docs/implementation-plan.md; verify original ledger prefix and all409 prior evidence objects
```

**Notes**

Removed only this task's redundant plan edit. Existing cold sanitizer deferral stays unchanged in the accepted plan; current diagnostics remain in E-M18-013 and docs/capture-preview.md. All409 original objects and their raw JSON prefix retained.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

fail
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold debug sanitizer/import publication qualification and the 13 scheduled
  import deadline failures remain Deferred in the agreed preview roadmap; one
  representative is baseline-reproduced.
- The separate OpenCode publication-gate and source-degradation readiness
  failures remain unqualified; assess independently before claiming broad
  server/harness completion.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Separate broad diagnostic, not the narrow Sources UI/read acceptance E-M18-012. Existing user-requested usable preview and UI priority retain explicit deferred cold sanitizer/native preparation optimization. No milestone promotion, weakened test, new publication guarantee, installation or reset. Every test process and isolated UI fixture has exited.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep these failures visible. Reopen cold sanitizer/setup and affected publication/readiness qualification as a separate deferred slice; preserve the assertions and gate/ownership contracts.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


