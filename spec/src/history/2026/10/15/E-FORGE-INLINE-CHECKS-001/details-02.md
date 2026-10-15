---
schema: 1
id: E-FORGE-INLINE-CHECKS-001
---

# E-FORGE-INLINE-CHECKS-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte
```

**Notes**

Initial before-state: 4 failed. Checks were outside the PR body and header hover differed from its background by only 2 RGB units. Corrected the new story theme setup from an ineffective parameter to the existing colorMode global before repeating the before-run. Log .local/forge-inline-checks-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte
```

**Notes**

Exact before-state with the correct light/dark globals: 4 failed. Both inline checks stories could not find CI checks in the originating PR, and both theme hover assertions failed. No production changes had been applied. Log .local/forge-inline-checks-before-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte
```

**Notes**

Exact after-run: 4 passed in a fresh browser process. Covers visible themed header hover, keyboard inspection of a collapsed PR, in-body jobs/logs, no extra column, retained diff/editor/draft and float/redock. Log .local/forge-inline-checks-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte -t InspectChecksExpands
```

**Notes**

Extended the collapsed-PR regression to reopen checks that had already been visible before collapse: 1 failed, 3 intentionally skipped. The first implementation toggled retained checks off while expanding the PR. Log .local/forge-inline-checks-reopen-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte -t InspectChecksExpands
```

**Notes**

Exact reopen after-run: 1 passed, 3 intentionally skipped. Inspection of a collapsed PR always reveals checks; an expanded PR retains normal explicit toggling. Log .local/forge-inline-checks-reopen-after.log.
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
pnpm catalog:check
```

**Notes**

Initial and final checks have zero Svelte errors or warnings; catalog boundaries pass for 452 modules. Logs .local/forge-inline-checks-types.log and .local/forge-inline-checks-types-final.log.
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
node scripts/web-architecture.mjs
```

**Notes**

Maintained module-size/import boundaries pass for 1960 modules. Log .local/forge-inline-checks-architecture.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI/released dependency boundary passes. Community PR detail/disclosure composition was inspected first, then released Design Core Button/ColumnCanvas APIs. Existing owned components and stylesheet are reused with no source copying, new dependency, alias or foundation patch. Log .local/forge-inline-checks-ownership.log.
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
pnpm evidence:new -- --id E-FORGE-INLINE-CHECKS-001 --task M31 --requirement JST-UI-020 --requirement JST-FORGE-005 --requirement JST-API-001
```

**Notes**

Created fresh evidence/day links and restored 446 unrelated generator-only navigation changes from the parent. Existing evidence remains unchanged; M31 was reformatted.
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
python3 Storybook HTTP/index health check (5-second timeout)
```

**Notes**

Warm review Storybook became unresponsive during source HMR. Verified the task-owned listener PID 95531, exact command, cwd and port before restarting only this workspace preview. No shared infrastructure changes or broader test repairs.
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
python3 Storybook cold HTTP/index health check (5-second timeout)
```

**Notes**

After restart the index returned HTTP 200 and listed all four Inline checks stories, but the initial contextual iframe request exceeded the five-second cold compilation budget. Rechecked the same process with a bounded twenty-second request after compilation; no test or infrastructure changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte catalog/components/store-forge/StoreForgeBranchStatus.stories.svelte
```

**Notes**

Initial and final direct-consumer runs: 31 PR stories passed in fresh browser processes. The final run includes the collapsed/reopen regression and action-only hover assertions after the user requested a transparent title trigger. Covers rendered details, descriptions/editing, selection guards, stale/unavailable checks, failure toasts, guarded merge/capabilities, branch badges, neutral light/dark hover, inline submission, shared PR/file columns and floating lifecycle. Logs .local/forge-inline-checks-consumers.log and .local/forge-inline-checks-consumers-final.log.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6046 --exact-port --no-open --disable-telemetry; python3 Storybook HTTP/index health check
```

**Notes**

Restarted only the verified task-owned review preview. The existing contextual iframe and index return HTTP 200. The index includes all four new Inline checks stories.
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
pnpm check:progress
```

**Notes**

Fresh evidence and explicit unchanged M31 milestone review pass.
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
pnpm spec:first
```

**Notes**

Owning UI/forge requirements and explicit unchanged API confirmation cover the protected presentation changes.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Document size limits, governance and navigation pass.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForgeInlineChecks.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte spec/src/current/contracts/api/38-ordinary-local-stacks-transport.md spec/src/current/contracts/architecture/24-forge-stacks.md spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M31/index.md web/src/lib/ui/store-forge/StoreForgeDetails.svelte web/src/lib/ui/store-forge/StoreForgePanel.svelte web/src/lib/ui/store-forge/forge-inline.css web/src/lib/ui/store-forge/store-forge.css
```

**Notes**

All changed authored files pass. Generated evidence uses its canonical renderer.
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

Node v26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11/Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only disclosure and contextual header styling. No API, forge action, controller, pending request/receipt, credentials, authored draft, operation-pinned diff, persistence, storage, manifest or WAL format changes. Existing capability, revision and explicit remote-effect guards remain authoritative.
<!-- /evidence-field -->

