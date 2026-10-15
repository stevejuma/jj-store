---
schema: 1
id: E-FORGE-AUTH-LIVE-001
---

# E-FORGE-AUTH-LIVE-001

<!-- evidence-field:id markdown -->
## Id

E-FORGE-AUTH-LIVE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-FORGE-002
- JST-FORGE-003
- JST-FORGE-005
- JST-FORGE-007
- JST-API-001
- JST-API-002
- JST-UI-020
- JST-SEC-002
- JST-SEC-006
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change qkmlnnoy
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
- spec/src/current/progress/M31/index.md
- spec/src/history/2026/10/15/E-FORGE-AUTH-LIVE-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-FORGE-AUTH-LIVE-001/details-02.md
- spec/src/history/2026/10/15/E-FORGE-AUTH-LIVE-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
- web/src/lib/controller/forge/connection.svelte.ts
- web/src/lib/controller/forge/controller.svelte.ts
- web/tests/vite/forge.test.ts
- web/tests/vite/integrations.test.ts
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
jj --no-pager st; owning forge/API/workspace chapters and M31 review
```

**Notes**

Before state: clean forge-stacks workspace over implementation change znyqwmzm (1f41fc70). OAuth permission grant was pending; no broader test work was changed.
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
User-approved GitHub device authorization through the browser
```

**Notes**

User explicitly approved Authorize jj-store. The old device code had expired; requested a fresh code from the local service, entered it on GitHub and clicked Authorize stevejuma. GitHub required sudo reauthentication; the user completed authorization. No optional organization Grant buttons were selected by the agent.
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
Local integrations inventory; account metadata inspection; security find-generic-password -s md.lapis.jj-store.forge.v1 -a OPAQUE_ACCOUNT_ID
```

**Notes**

Initial inventory returned forge_state_busy after account metadata and the native Keychain item had been saved. Metadata-only native lookup succeeded; neither -g nor -w was used, native output was discarded, and no secret was read into tool output. Live native read had not yet passed.
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
Explicit task-owned development overrides; pnpm dev:backend:restart; cold Integrations reload
```

**Notes**

Restarted the final task-owned executable, preserving Vite 7460 and the existing fixture. Inventory then returned the saved stevejuma OAuth account with available=true; cold browser reload showed Connected. An early read during startup returned no JSON; readiness was established before final verification. No plaintext fallback, credential extraction, binary replacement or installed service change. .local/forge-integrations-authenticated-restart.log; github-connected-after-restart.png.
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
Sample workspace opening and automatic GitHub binding
```

**Notes**

Opened the existing operation-pinned jj-sample-repo workspace. One saved account bound automatically to the existing origin connection. PR #1 and six revision-bound live CI observations loaded. An automatic refresh overlapped selected-PR observation and produced forge_state_busy, identifying a controller request/journal-lease collision.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/integrations.test.ts
```

**Notes**

Before regression: one failure/four passes; overlapping observations reached three simultaneous transports against a single journal lease. New regression expected one. .local/forge-live-lease-before.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Focused controller checks after repository-owned request serialization
```

**Notes**

The first rerun passed the new queue regression but exposed an older selection test that required two concurrent held requests. Updated its scheduling to select the next PR before releasing the old response, assert that stale checks remain absent, then release the current response. Original selected-revision, pending, busy and final-result assertions remain; no stale-selection guard was removed. .local/forge-live-controller-tests.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts -t "loads selected PR checks automatically"
```

**Notes**

Exact selection regression passes: one passed, 13 filtered. The late old selection cannot replace the selected PR checks. .local/forge-live-selection-regression.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/integrations.test.ts web/tests/vite/forge.test.ts web/tests/vite/forge-details.test.ts web/tests/vite/forge-branch-status.test.ts web/tests/vite/global-settings-history.test.ts
```

**Notes**

44 passed across five files, including serialized requests, rejection recovery, no dispatch after close, retained unknown effects, accounts, setup notices and exact Settings navigation. .local/forge-live-controller-acceptance.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge
```

**Notes**

43 passed across nine direct-consumer story files using a fresh Vitest/Vite process. Interaction/accessibility coverage includes account setup, checks, selection, editing, jobs/logs and floating lifecycle. Visual baselines remain pending. .local/forge-live-story-tests.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries pass for 455 modules. .local/forge-live-web-check.log; forge-live-catalog-check.log.
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
Explicit task-owned overrides; pnpm dev:backend:restart; cold sample-workspace reload and PR/check inspection
```

**Notes**

A second restart retained the saved account and bound repository. Cold opening of PR #1 displayed target trunk, exact SHA 94d788424958bec452e987cb754cbb12e301ea8d, four failing and two successful observations, and no forge error toast. The checks remained in the PR body; live Jobs inspection returned the failing mock-test and successful mock-lint in the same panel. .local/forge-live-bound-restart.log; github-pr-checks-after-cold-restart.png.
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
Read-only device baseline comparison and jj --ignore-working-copy op log
```

**Notes**

Verified that binding adds an account reference without changing connection/endpoint identities, trunk target, receipts, operations or Restack journals. Native sample operation remains 27cc809145c0. No push, submission, merge, rerun, cancel, authored PR edit or code rewrite was invoked. .local/forge-live-before.json contains only non-secret device facts.
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
Changed-file Prettier; node scripts/web-architecture.mjs; pnpm check:ui-ownership; pnpm check:progress; pnpm spec:first; pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Focused source ownership, module bounds, fresh evidence, explicit M31 review and governing document checks passed. Root aggregate and unrelated Rust lanes remain out of scope.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live Enterprise/PAT, Windows/Linux credential stores, packaged application,
  native-storage submission and additional provider qualification remain open.
- Disposable-forge mutation qualification remains separate; this live run used
  read-only PR/CI actions on the existing sample repository.
- Previously recorded unchanged Clippy style warnings, unrelated dark diff-text
  contrast and visual baseline approval remain deferred.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FORGE-AUTH-LIVE-001 details 2](details-02.md)
