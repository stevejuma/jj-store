---
schema: 1
id: E-REUSABLE-PROGRESS-009
---

# E-REUSABLE-PROGRESS-009

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-PROGRESS-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-DOC-001
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/reusable-agent-workflows-progress.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
- spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-009/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-009/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
cd ../lapis-desktop-history && CARGO_INCREMENTAL=0 MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_BUILD_JOBS=2 pnpm exec tauri build --debug --bundles app --features e2e --config src-tauri/tauri.e2e.conf.json
```

**Notes**

Isolated acceptance bundle includes 3aa56eff layout. Vite 28.45s, native 28.40s, 201.65 MiB bundle. Primary Preview app and user fixture stay untouched. Build log history-visible-native-layout-build.log.
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
cd ../lapis-desktop-history && MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_BUILD_JOBS=2 node --test --test-concurrency=1 tests/desktop/repository-visible.test.mjs
```

**Notes**

Final strict native target: one full pass in 20.97s, two cold processes, two physical foreground clicks. Native measured visibility/current-Space/unoccluded/key-window assertions plus original document focus/animation guard pass. Covers exact 128-hex pins, production ribbon, unified/split content, dirty bytes and JJ/Git metadata, external refresh without pin movement, cold restoration, new operation clearing old diff, later content and ordinary-folder refusal. Both processes close normally. E-REPO-006 retains earlier focus/driver failures and the prior 17.83s pass. Log history-visible-native-strict-key-window.log; artifacts ../lapis-desktop-history/.local/native-test/repository-visible-usFC3C.
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
cd ../lapis-desktop-history && node --test tests/governance/*.test.mjs && pnpm check:progress && pnpm spec:validate && pnpm spec:first
```

**Notes**

All 47 governance cases; 33 tasks/94 records; six validators, 109 chapters/94 requirements; spec-first three protected files/three canonical chapters. All ten changed files pass formatting. E-REPO-006 records initial bookkeeping/chapter-budget failures, their repair and exact original completed gate. Desktop committed as fb238ae2, enclosing change ssvlzxtn.
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
pnpm evidence:new -- --id E-REUSABLE-PROGRESS-009 --task M33 --task M33.4 --requirement JST-ARCH-007 --requirement JST-DOC-001 --requirement JST-GOV-002 --requirement JST-GOV-011
```

**Notes**

Created root delivery evidence. Restored 525 unrelated navigation changes. Keep only this tracker, owning ledger/histories and exact evidence/day navigation.
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
pnpm check:progress && pnpm spec:validate
```

**Notes**

Root progress: 171 tasks and 878 evidence records. All seven specification validators pass with 4,229 chapters and 207 requirements. Documentation evidence only; no root runtime or UI claim.
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
pnpm exec prettier --check docs/maintenance/reusable-agent-workflows-progress.md spec/src/current/progress/M33/index.md spec/src/current/progress/M33/history/M33/index.md spec/src/current/progress/M33/history/M33.4/index.md spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-009/index.md spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-009/SUMMARY.nav.md spec/src/history/2026/10/18/SUMMARY.nav.md spec/src/history/2026/10/18/index.md
```

**Notes**

All eight selected Markdown files pass formatting.
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
pnpm spec:first
```

**Notes**

One protected file, zero canonical chapters; unchanged root contract confirmation is accepted. Milestone stages retain original scope and status.
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

Enclosing Jujutsu change llyqswpr based on 0630f0da. Desktop native qualification fb238ae2; UI 3aa56eff; stock JJ 0.45.1, macOS ARM64; Notes API a91d5e2e and Design Core a57a38e2 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Root Markdown only. Desktop test-local select events use the mounted production handler; no native protocol, authorization, operation identity, receipt, storage or UI production format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Close NOTES-04/04B within the qualified macOS native history profile and move the sole Active marker to NOTES-06. Physical foreground assistance is explicit GAP-10, not unattended automation. The root stage statuses do not change. Independent source/UI histories and all prior failure records remain intact. Root aggregate spec:check/check/test/build, Rust and product UI suites are out of scope for this delivery-documentation update.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement injected native raw-event ownership and sanitation/durable capture before renderer exposure/ACK, then AgentStore/ThreadStore backend adapters, existing AI plugin adoption, restart/unknown-outcome proof, addressing/discussions, imports/capture and standalone qualification. Resolve unattended activation during its own qualification slice. Source review identifies raw bootstrap activity, returned opened facts and actor frames as separate exposure paths. The gate must preserve correlation/identity and unresolved-request failure semantics while owning processing through shutdown; no implementation is claimed yet.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-007, JST-DOC-001 and JST-GOV-002/011 preserve the full approved
  reusable-workflow scope, host-owned privacy/durability and honest native
  evidence. Original cold native focus and history gates are preserved and
  native window assertions strengthened in the desktop slice.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Desktop M13.2 completes its original gate using E-REPO-002
  admission/transport/drain, E-REPO-003/005 production presentation and
  E-REPO-006 actual native restart proof. M13 remains In progress; M13.3
  Planned. Root M32 stays Done; M33/M33.1-M33.4 remain In progress and
  M33.5-M33.8 Planned. Raw-event and durable conversation requirements are not
  promoted by history acceptance.

```
<!-- /evidence-field -->

