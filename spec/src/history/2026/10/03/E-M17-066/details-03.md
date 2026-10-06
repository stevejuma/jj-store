---
schema: 1
id: E-M17-066
---

# E-M17-066 details 3

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
pnpm exec prettier --write spec/src/cli.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-reader-doc-format-final.log 2>&1
```

**Notes**

Final ledger and CLI scope correction. Repeated after recording actual results.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-import-reader-progress-final.log 2>&1
```

**Notes**

Final ledger and CLI scope correction. Repeated after recording actual results.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-import-reader-canonical-final.log 2>&1
```

**Notes**

Final ledger and CLI scope correction. Repeated after recording actual results.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-import-reader-doc-check-final.log 2>&1
```

**Notes**

Final ledger and CLI scope correction. Repeated after recording actual results.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; qualified synthetic Codex CLI 0.156.1 and desktop 0.159.2/native MemoryStore/device durability. No real harness/RustFS release qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive readers array in private import state.json version 1, bounded by 4096 receipts and existing 2-MiB catalog. Original receipt/intent/source binding immutable. Reader links carry ordinal, source identity, initial positive numeric generation and exact physical/header digest. Receipt-only catalogs remain readable. No HTTP/CLI wire, native AgentStore, control, registry, spool or cursor format changes. Current AgentStore only.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ktluwwtv
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

Original source/native Write/prefix and retained policy/import gates select reader association after the actual common leased reader opens and before returning imported work. Owned capture format compares header digest with its physical/header witness; no format hashing duplicated in server. Atomic synchronized association replacement refreshes the held selected stamp without reacquiring locks or changing original accepted receipts. Exact retry/reopen and later numeric continuation retain initial reader witness. Renewed imported ingestion/publication verify or reconstruct the same link before body work; changed reuse withholds without retargeting or repairing original intent. Body-free metadata lookup does not inspect source/transcript/lease/owner and survives absent files. Receipt-only legacy device catalogs remain unqualified until original-source/native admission reconstructs their link; legacy AgentStore remains removed. Duplicate, foreign, invalid, changed and oversized reader metadata fail closed. Separate process exits while holding the selected gate, then parent recovers exact association and receipt. Concrete escalation: import catalog and held settlement plus actual source driver/pipeline affect owning server library (native/scheduled import, policy/control/import) and direct admission/authority/source/resume/parking/reanchor/recovery/pipeline/worker/supervisor/runtime/activation/management/both-journal consumers. These lanes run serially with original deadlines and within-scenario concurrency, including existing cold/lifecycle ownership fixtures. Capture binding/registry-import suites and capture/server/direct CLI all-target check and strict lint are the direct other consumers. CLI runtime tests, generated contract/actual response runners, UI/catalog/Storybook, root aggregates and unrelated workspace/RustFS/live harness qualification are out of scope because no wire/command/UI/shared infrastructure changed. Public bounded import phase projection, typed HTTP/CLI/browser, automatic accepted-request restart dispatch, remaining readers/enabled startup and M17.3-M17.7 remain pending; full goal active, M17.2 In progress. Preserve all 320 prior entries and raw historical JSON prefix. Initial implementation and stamp-fixture compiler diagnostics resolved without new dependency or weakening. One multi-file patch missed formatter-wrapped function context and mutated no files; reapplied against exact text. Final selected suites pass 218 top-level tests across 18 binaries: capture binding/registry-import 21, owning server library 91, direct server consumers 106 across 15 binaries. Nine new regressions (one capture, four journal and four native) pass; original missing-association native regression fails one to zero. Existing cold factory, storage/lifecycle ownership and cancellation fixtures pass. Capture/server/direct CLI all-target compilation and strict lint pass. All seven canonical/spec-first/boundary/architecture/progress/Rust/document consumers pass; final ledger and corrected CLI source-selector scope also pass canonical/progress/document checks. Existing macOS unwind-table linker warning remains unsuppressed. No unresolved functional failures. Private association slice complete; bounded import phases and public status/HTTP/CLI/browser flow are next. Automatic accepted-request restart dispatch, remaining harness readers, enabled-startup real harness/storage qualification and full M17.3-M17.7 remain pending. Full goal stays active; M17.2 remains In progress. All 320 prior evidence entries and historical raw JSON prefix remain unchanged.
<!-- /evidence-field -->

