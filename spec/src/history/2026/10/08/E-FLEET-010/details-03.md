---
schema: 1
id: E-FLEET-010
---

# E-FLEET-010 details 3

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
CARGO_BUILD_JOBS=2 pnpm dev:backend:build
pnpm web:build
```

**Notes**

Backend passed. Initial web-build.log failed stale generated API type reference; web-build-final.log failed unreviewed documentation fingerprints. Regenerated reference, updated manuals and reviewed services/configuration/HTTP/application/contracts. web-build-complete.log passed with 141 documentation assets and valid links/anchors.
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
node .local/agent-events-2026-10-08/cold-browser.mjs
```

**Notes**

cold-browser.log: fresh privacy preparation exceeded initial 60-second preflight. Second cold-browser-final.log used an invalid in-checkout cache outside its allowed fixture root; aborted owned process and corrected to a cache inside the disposable fixture. No user fixture changed.
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
pnpm check:ui-ownership
cargo fmt --all -- --check
```

**Notes**

ownership.log and rust-format.log passed.
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
pnpm check:architecture
```

**Notes**

architecture.log: existing scripts/dev-ui.acceptance.mjs (311 counted lines) plus new chat growth (305). Moved chat types to their owner; architecture-complete.log leaves only the pre-existing dev-ui.acceptance.mjs size violation. This slice does not change that script.
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
CARGO_BUILD_JOBS=2 pnpm dev:restart
```

**Notes**

dev-restart.log: preserved built-UI restart succeeded. Live bootstrap capability true, agents runtime ready, one saved setup and two sessions retained; no prompts or Start/Send actions.
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
node .local/agent-events-2026-10-08/cold-browser.mjs
```

**Notes**

cold-browser-complete.log: fresh real native/RustFS fixture, 1 cold browser test passed. Two tabs each made one initial status/settings/targets read, then zero additional reads over 12 idle seconds. External setup edit updated both tabs; settings edit caused exactly two settings reads and no targets; native CLI code edit invalidated targets without settings; offline edit replayed after reconnect and six-second idle stayed quiet. No browser errors. Owned fixture cleaned. Fresh privacy compilation needed roughly 85 seconds; the longer preflight did not change product policy.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-*.test.ts
pnpm web:check
```

**Notes**

agent-unit-complete.log: 70 pass after chat-type extraction. web-check-types.log: 0 errors/warnings.
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
pnpm docs:check
```

**Notes**

docs-check.log: generated reference and final source review fingerprints pass.
<!-- /run -->

<!-- run -->
### Run 9

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

architecture-complete.log: only pre-existing scripts/dev-ui.acceptance.mjs, 311 counted lines. The exact parent file was checked with countedLines and also has 311. Changed production chat now fits the limit.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

First ledger run: spec validation passed, progress and spec-first rejected missing file#test names and unowned generator formatting churn. Corrected exact references, added M24 files and restored unrelated historical/milestone bytes.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

spec-first-final.log required mapped CLI, compatibility and mutations chapters. Added the owning supervisor/semantic-source/additive-domain contracts and preserved the unchanged execution, mutation and publication boundaries; spec-first-complete.log then required their file coverage, added here.
<!-- /run -->
<!-- run -->
### Run 30

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Final changed-file `pnpm exec prettier --check` passed (format-check.log); final `pnpm catalog:check` passed with 0 errors/warnings and 298 modules (catalog-check-complete.log). Final progress-complete.log: 146 tasks, 664 evidence records. spec-validate-complete.log: 7 validators, 3215 chapters, 181 requirements. spec-first-verified.log: 34 protected files and 7 canonical chapters pass. Functional scope passes; overall evidence remains pending only for the explicitly retained pre-existing architecture/full-audit gaps.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Repository-wide failures remain in E-GOV-005 and E-GOV-006; this is scoped
  acceptance, not a full-suite pass. The existing dev-ui.acceptance.mjs
  file-size violation is independently retained.
- Provider installation/authentication and signed desktop/platform qualification
  are outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Rust/Cargo 1.98.1; JJ 0.45.1; Playwright 1.63.0; Vitest 4.1.11; bundled jj-acp control 3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Two additive application event domains, agent_execution and agent_settings, retain event protocol version 1. No native object, manifest/CAS, WAL, receipt, setup, journal, credential or repository format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cross-layer event producer and browser consumption; full catalog units justified by the shared scheduler and all direct fixture stories by changed fixture lifecycle. Existing user-requested repository audit is E-GOV-005/E-GOV-006; unrelated full root/Rust workspace lanes are out of scope. No visual baseline approval, live prompts, provider install, reset, remote push or release.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Triage the pre-existing dev-ui acceptance file-size violation and independent repository audit failures; retain M24 adapter, permission and desktop gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-COLLAB-034/035/036, JST-API-001/006 and JST-UI-005/015 now
  explicitly govern installation execution/settings signals and quiet
  event-driven readers with bounded outage/failed-read recovery; canonical
  API/UI/collaboration chapters updated. JST-DOC-001/002 manuals and generated
  references updated. Service mutation, CLI execution authorization, native
  compatibility and publication contracts are unchanged: notices are hints and
  every reopened read retains existing authority. Shared scheduler preserves the
  already required outage backoff."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This slice qualifies event-driven freshness and
  development restart only; additional version-3 adapters, live
  permissions/cancellation and signed desktop/platform gates remain independent.
  Existing source/audit gaps are not waived.

```
<!-- /evidence-field -->

