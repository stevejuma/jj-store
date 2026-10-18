---
schema: 1
id: E-REUSABLE-PROGRESS-017
---

# E-REUSABLE-PROGRESS-017

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-PROGRESS-017
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.3
- M33.4
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
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-017/index.md
- spec/src/history/2026/10/18/E-REUSABLE-PROGRESS-017/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
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
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --manifest-path src-tauri/Cargo.toml --bin lapis-desktop --locked host::ai
```

**Notes**

Executed in lapis-desktop-history: all 16 AI/host cases pass, including all nine backend cases. Covers exact frames/caller/vault admission, idempotent open, retained receipts and exclusive writer ownership, cancelled blocking IO retaining native serialization, cancelled close retaining its fenced owner, reload reattachment with a fresh handle, stale closes/generations and physical root replacement. The terminal-window regression actually fails before the fix, then passes. Source E-AI-033 retains the exact red runs and patch-preparation failure.
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
MACOSX_DEPLOYMENT_TARGET=13.0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo build --manifest-path src-tauri/Cargo.toml --bin lapis-desktop --locked
```

**Notes**

Executed in Notes: final current source passes 136 tests per native profile, zero failures and two inherited ignored helpers each. Both strict all-target Clippy profiles pass. The ordinary executable builds last, after the acceptance app. Initial 135-case profiles remain distinct from final source in E-AI-033.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm test:desktop:api
```

**Notes**

Executed in Notes: all three actual cold hidden-WebView cases pass. New backend case proves actual IPC and exact grant, real JJ source-byte preservation, no companion initialization, original receipts across later choices, blocked database close retaining owner, actual renderer reload with a fresh handle, stale-close rejection, disable/replacement/stale-session handling, plain-folder JJ refusal, delayed old-vault close, concurrent open/release, saved JJ choice/original receipt reopening after cold process restart and normal vault closure. Existing admission and pinned-history cases pass. Visibility/document/native keyboard-focus assertions stay false throughout. Debug acceptance app, current renderer and verified ACP resources build. Source E-AI-033 retains the initial missing command/permission failure and exact-capability audit failure; only the named local main-window permission is added.
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
pnpm check:types
pnpm check:architecture
pnpm check:app-ownership
node --test tests/governance/*.test.mjs tests/renderer/*.test.mjs
pnpm progress:generate
pnpm check:progress
pnpm spec:validate
pnpm spec:first
pnpm spec:check
pnpm format:check
```

**Notes**

Executed in Notes: zero Svelte errors/warnings, source/App audits, all 198 renderer/governance cases, 33-task/102-record ledger, six validators/118 chapters/94 requirements, final spec-first (22 protected files/five canonical chapters), all four spec lanes including mdBook and full formatting pass. Capability regressions retain exact local scope and reject broad, remote, wrong-window and missing-grant mutations. Original milestones/gates remain unchanged.
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
pnpm evidence:new -- --id E-REUSABLE-PROGRESS-017 --task M33 --task M33.3 --task M33.4 --requirement JST-ARCH-007 --requirement JST-DOC-001 --requirement JST-GOV-002 --requirement JST-GOV-011
pnpm progress:history
pnpm check:progress
pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book
pnpm exec spec-validator first
```

**Notes**

Root documentation consumers pass: 171 tasks/890 records; five selected validators/4,258 chapters/207 requirements; one protected file/one canonical chapter in spec-first. Unrelated generated projections are restored, retaining only owning delivery documentation, selected M33 histories and new evidence/day navigation. All changed Markdown is formatted after the final generator. Root Rust/UI/product suites remain out of scope.
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

Enclosing Jujutsu change nmsxxqsl, based on 3d8ef9a9. Notes source 81765a34b83db9ec3b66f334eea5cfc79192863d (lsvupuqo), based on a48fce96. Shared jj-store source d824a18f; Lapis AI source 8a8f9149 (clean source revalidated). Rust 1.99.0, pnpm 10.34.5, Node 26.9.0, JJ 0.45.1; macOS ARM64 with 13.0 deployment target for final native/cold API qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Root delivery documentation/evidence only. Notes adds bounded version-1 JSON text backend frames and opaque owner handles, retaining shared RequestId/Selection/SelectionReceipt DTOs. Private profile schema, original choice receipts, repository Agent/Thread objects, native capture/journal formats, publication and legacy file conversation formats are unchanged. A named local main-window command grant is added; no raw execution target or capture processor is enabled.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

NOTES-07E ordinary native backend ownership and transport. Native profile/owner service is now consumed by the actual host, with current exact vault/caller admission, retained open identity, native transition ownership through IO, supervised release and terminal window destruction. Cold API acceptance is actual hidden WebKit, separate from visible UI, production renderer persistence and managed capture. Native worker revocation and backend drain are connected, but actual capture callback/receipt publication and managed conversation restart still need their store/processor adapters. No user profile, installed app, fixture reset, provider, publication or baseline changes. Root product/UI/Rust aggregates are out of scope for this docs-only delivery record; applicable document consumers are checked.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep NOTES-07 as the only Active task. Connect native conversation/session preparation, real AgentStore/ThreadStore and capture processor construction, then per-vault renderer persistence and shared presentation. NOTES-06C/06C4 remain Waiting for that actual managed integration; NOTES-06D/08/09 keep their original app/capture/restart gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-007 injection, owned interfaces and publication contracts are
  unchanged in jj-store. This record accepts the qualified Notes host consumer,
  updates JST-DOC-001 delivery documentation and retains JST-GOV-002/011
  evidence/milestone rules. Root runtime, APIs, repository formats and UI remain
  unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: NOTES-07E completes only native host ownership/IPC. NOTES-07 remains the
  single Active task for actual store/processor and renderer adoption;
  NOTES-06C/06C4 are Waiting. M32 stays Done; M33/M33.1-M33.4 In progress and
  M33.5-M33.8 Planned with original gates preserved. Native API choice/receipt
  restart is not managed chat/capture/restart qualification. Visual, live
  provider, additional platform and release gates remain open.

```
<!-- /evidence-field -->

