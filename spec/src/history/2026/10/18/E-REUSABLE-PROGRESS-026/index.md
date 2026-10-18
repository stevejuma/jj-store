---
schema: 1
id: E-REUSABLE-PROGRESS-026
---

# E-REUSABLE-PROGRESS-026

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-PROGRESS-026
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-DOC-001
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/reusable-agent-workflows-progress.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33/SUMMARY.nav.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/history/M33.3/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-026/index.md
- spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-026/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

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
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --manifest-path src-tauri/Cargo.toml --bin lapis-desktop --locked conversation_read_continuation_keeps_the_original_snapshot_after_capture_advances
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --manifest-path src-tauri/Cargo.toml --bin lapis-desktop --locked conversation_read
```

**Notes**

Notes E-AI-038: the original exact test changes from one unknown-opcode failure to pass; all 11 focused read/receipt/authority/cursor/lifetime cases pass after adding guard coverage. A real native 34-chunk transcript reads 32+2 and repeats byte-identically after later capture advances. Eight-read cap, foreign/unissued/invalid cursor refusal, exact intent checks, close/reload retirement and idle writer drain are covered.
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
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --manifest-path src-tauri/Cargo.toml --locked
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --manifest-path src-tauri/Cargo.toml --features e2e --locked
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --features e2e --locked -- -D warnings
```

**Notes**

Both complete native profiles pass 174 tests, zero failures and two inherited ignored each; all-target strict lint passes in both profiles. Source E-AI-038 retains full commands and logs. Shared native frames and owner lifetime justify both complete profiles; no root Rust is run.
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
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm test:desktop:api
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo build --manifest-path src-tauri/Cargo.toml --bin lapis-desktop --locked
```

**Notes**

All four cold hidden native API cases pass, including empty-conversation read-handle transport, new handles after actual restart, old-handle refusal and unchanged source state. This is distinct from successful native transcript ranges, supplied by actual Git-backed service tests. Ordinary executable rebuilds last; installed apps/services remain untouched.
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
node --test tests/governance/*.test.mjs tests/renderer/*.test.mjs
pnpm test:ai
pnpm check:types
pnpm check:architecture
pnpm check:app-ownership
pnpm check:progress
pnpm spec:check
pnpm format:check
```

**Notes**

Notes passes 212 renderer/governance and 389 AI policy cases, zero type errors/warnings, both audits, progress (33 tasks/110 records), all four spec lanes and formatting. Missing new evidence navigation initially fails one spec lane; explicit links correct it. Complete failed attempts remain in E-AI-038; no assertion or policy is weakened.
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
pnpm progress:history
pnpm check:progress
pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book
pnpm exec spec-validator first
pnpm exec prettier --check docs/maintenance/reusable-agent-workflows-progress.md spec/src/current/contracts/architecture/26-reusable-agent-workflows.md spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/progress/M33/index.md spec/src/current/progress/M33/history/M33/index.md spec/src/current/progress/M33/history/M33/SUMMARY.nav.md spec/src/current/progress/M33/history/M33.3/index.md spec/src/current/progress/M33/history/M33.3/SUMMARY.nav.md spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-026/index.md spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-026/SUMMARY.nav.md spec/src/history/2026/10/18/index.md spec/src/history/2026/10/18/SUMMARY.nav.md
```

**Notes**

Root documentation qualification passes progress (171 tasks/900 records), five scoped validators, spec-first and changed-document formatting. History generation is followed by allowlisted restoration of 551 unrelated generated paths. Root aggregate and Rust lanes, UI/Storybook, live providers and visual baselines are out of scope for this documentation-only checkpoint. conversation-ranges-root-{history-final,progress-final,doc-final,first-final,format-final}.log. Final evidence-only consumers repeat progress/first/scoped document validation.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Root enclosing change qpqpznpn based on 5b119667, Node 24.10.0/pnpm 10.32.1; Notes uorpokkk / d00a69b381b3b6be19dc86726e3d2c1e6cd805ab based on 792cbfa87230561cf0e815ba1cc108d8356a2fcc, Node 26.9.0/pnpm 10.34.5. Rust 1.99.0; stock JJ 0.45.1; macOS ARM64, minimum 13.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Root native/store/receipt/journal/publication interfaces remain unchanged. Notes adds three private native read operations within its outer v1 text frames. Pinned views/ranges remain native JSON text; continuation positions are canonical decimal text; opaque read/stream handles are native-owned and session-only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

NOTES-07J adds retained native transcript continuation to the Notes library/host. Root changes only checklist, canonical descriptions and milestone evidence/navigation. Native range proof, cold empty-conversation handle proof and eventual managed-chat recovery remain distinct.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue NOTES-07 renderer projection, executable/credential admission, ordinary runtime dispatch and per-vault persistence, then managed chat/restart acceptance and the remaining approved stages.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-ARCH-007/JST-DOC-001: Notes consumes the already-selected owned
  Agent/Thread readers with current native admission and original provider
  proof. Original publication/store/identity/receipt formats are unchanged.
  Bounded immutable read continuation grants no mutation, executable or prompt."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33/M33.1-M33.4 remain In progress, M33.5-M33.8 Planned with their
  original gates. NOTES-07 remains the sole Active task; NOTES-06C/06C4 wait for
  managed backend integration. Native continuation is independent of renderer
  projection, managed dispatch, chat/restart and visual/provider/distribution
  acceptance.

```
<!-- /evidence-field -->

