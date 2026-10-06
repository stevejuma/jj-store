---
schema: 1
id: E-EXEC-011
---

# E-EXEC-011

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/thread-transport.ts
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/local_browser/collaboration.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser-input.mjs
- scripts/agent-execution-browser-ui.mjs
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-discussion-api.mjs
- scripts/agent-execution-discussion.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/local-jj/07-m15-additive-collaboration-publication.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-010/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-010/details-02.md
- spec/src/history/2026/10/07/E-EXEC-010/index.md
- spec/src/history/2026/10/07/E-EXEC-011/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-011/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/local-inbox-threads.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/thread-commands.ts
- web/src/lib/controller/thread-history.ts
- web/src/lib/controller/thread-types.ts
- web/tests/vite/local-inbox-threads.test.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change tzmwuwnp. macOS ARM64; JJ 0.45.1, Rust 1.98.1, Node 24.10.0, pnpm 10.32.1, Python 3.14.7, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Owned Community presentation revision 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0; released Design Core ^0.2.0. Cold acceptance uses installed Codex CLI 0.156.1 through the explicit sibling jj-acp debug executable and real embedded RustFS; no other installed runner requalified.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 status remains unchanged; E-EXEC-010 adds cold registered-folder
  inbox qualification and this record confirms unchanged authority/maintenance.
  Native code reducer 3 and broader M13/M15/release/platform/visual gates remain
  open. Historical outcomes are preserved.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-011
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-GOV-008
- JST-ARCH-001
- JST-ARCH-004
- JST-DOC-002
- JST-MUT-001
- JST-CLI-004
- JST-COMPAT-001
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
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Browser acceptance grew to 310 counted lines, above the unchanged 300-line budget. Extracted the existing recorded-history verifier into its owning acceptance helper; kept the assertion and budget.
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
pnpm check:architecture
pnpm check:boundaries
pnpm check:ui-ownership
```

**Notes**

1258 browser modules and existing Rust layer/source ownership boundaries passed; no external Community import, foundational component patch or source-budget exception.
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
pnpm docs:generate
pnpm docs:review application "Reviewed registered-folder inbox transport profile adoption, retained replies and exact receipt recovery; shared Community message presentation remains unchanged"
pnpm web:build
```

**Notes**

Actual mapped source reviews were refreshed, generated public API/manual references match, and built web/document closure passed with nine offline diagrams and 141 files. Earlier source reviews are retained in coverage.json. Routine Vite bundle/CSS and macOS compact-unwind advisories remain unmodified.
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
pnpm dev:backend:build
env JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj pnpm dev:backend:restart
```

**Notes**

Explicit development backend built and replaced while Vite remained running. Initial restart omitted ACP launch environment, making local agents unavailable; corrected trusted environment restored the current UI. No reset, installed CLI replacement, unrelated service restart or real-chat mutation.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

A narrowed navigation regeneration wrote an unintended nested SUMMARY.md; removed only that newly generated file and retained the existing root summary and original historical records. evidence:new/navigation refresh also normalized unrelated historical/progress whitespace; restored those paths from the parent and kept only the new scoped evidence navigation.
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
pnpm docs:check
pnpm check:architecture
pnpm format:rust:check
```

**Notes**

Final mapped-source/manual closure, 1258-module architecture check after test repairs, and pinned Rust formatting pass. Generated source formatting produced no production changes. Broader aggregate lanes remain out of scope.
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
Changed-file Prettier check
pnpm exec prettier --write spec/src/current/progress/M23/index.md spec/src/current/progress/M23/history/M23/index.md
```

**Notes**

Initial changed-file check found two M23 ledger/history formatting issues after the new entries; formatted only those two files, preserving all historical result descriptions. Final changed-file formatting check and specification/progress/spec-first commands are recorded next. Rejoined the appended history entries into the existing Markdown table; original historical wording and outcomes remain intact.
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
pnpm spec:validate
pnpm check:progress
pnpm spec:first --base 377de62c --head @
Changed-file Prettier check using jj diff --name-only and execFileSync argument array
```

**Notes**

Specification: seven validators, 3088 chapters and 178 requirements pass. Progress: 144 tasks/634 evidence records; original E-EXEC-008/009 outcomes preserved. Explicit-parent spec-first: 33 protected files, four updated owning canonical chapters with fresh unchanged maintenance confirmation. Final changed-file formatting passes. This is the targeted cross-layer transport and direct consumer surface; root aggregate suites, unrelated UI/Rust lanes and visual baselines were not run.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged native identity, manifest/ref-CAS authority, upstream JJ 0.45.1 codec, exact original receipts, launch/environment/credential admission and owned Community presentation formats. Additive transport/profile schemas are governed by E-EXEC-010 and owning topics.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Fresh unchanged-contract confirmation for protected service/CLI/tool/schema/document paths in the registered-folder transport slice. Existing provider revalidation, local device provenance, bounded workers, exact receipts and current pin reads are reused. Native code replication refusal is retained. Root aggregate pnpm spec:check/check/test/build, full Rust workspace, unrelated stories, release/visual qualification are out of scope under the targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain scoped evidence and source ownership; native reducer-3 and wider local/release/visual gates remain separately qualified.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-MUT-001 retains owned provider authorization, exact expected-state
  revalidation and original receipt semantics; no new authority or implicit
  dispatch. JST-CLI-004 keeps explicit dev binaries, preserved fixture and
  trusted launch configuration. JST-GOV-002/008 and JST-ARCH-001/004 retain
  layering, source ownership and budgets. JST-DOC-002 retains generated contract
  references and source-reviewed manuals. JST-COMPAT-001 keeps JJ 0.45.1
  codec/identity and native format qualification unchanged. Additive behavior is
  reviewed in E-EXEC-010.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


