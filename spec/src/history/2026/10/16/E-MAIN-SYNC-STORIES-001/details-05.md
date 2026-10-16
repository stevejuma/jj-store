---
schema: 1
id: E-MAIN-SYNC-STORIES-001
---

# E-MAIN-SYNC-STORIES-001 details 5

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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-4-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=4/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-4-67: exit 0, 323.93 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 37.01 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

lazy-before-87: exit 0, 34.96 seconds. 
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
pnpm exec prettier --write catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

lazy-format-87: exit 0, 0.71 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

lazy-after-87: exit 0, 24.89 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 1, 233.07 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-87: exit 1, 0.08 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 18.73 seconds. 
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
pnpm exec prettier --write catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

lazy-format-87: exit 0, 0.42 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

lazy-after-87: exit 0, 22.91 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 0, 223.67 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-87: exit 0, 0.06 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 21.02 seconds. 
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

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Rust 1.99.0; JJ 0.45.1; Design Core 0.5.0; Mira 0.0.3; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product, native identity, transport, account, credential, snapshot or receipt format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed mechanical-main synchronization and private catalog fixture acceptance only. This record supplies fresh combined-parent evidence required by the existing merge policy; it does not claim comprehensive local cold/profile or final governance qualification. The immutable E-MAIN-LOCAL-REPAIRS-001 and E-MAIN-PICKER-REPAIRS-001 records retain the qualified integrated source and their actual limits. The main synchronization merged aad2ad7f into PR #10 as 20a34951, with zero source-file differences from qualified head 94259ebe and GitHub behind_by=0. After that identical-source merge, full UI units pass (180 web; 1193 catalog) and fresh complete Storybook acceptance passes (1021 cases / 211 files), including a complete replacement of the failed shard. Full local qualification remains separately pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the complete served/cold local-release/profile and final documentation/governance lanes, append full qualification evidence, and update draft PR #10. No merge or release is authorized.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "All integrated product contracts retain their qualified first-parent
  bytes across main synchronization. The existing direct-parent governance
  regression deliberately rejects reuse of parent evidence during a merge; this
  fresh record covers the complete combined PR range rather than weakening that
  check. Full Storybook rerun recorded 1019 passes and two private lazy-content
  failures: repository General tab not yet mounted and Agent response
  placeholder not yet containing its avatar. The ten cases passed in isolation
  before preparation, which does not erase those full-shard failures. The
  affected story files now await their exact owned lazy modules in beforeEach,
  as existing catalog fixtures do. Settings awaits repositorySettings; recorded
  identity awaits agentsRecorded and agentSidebar. Their first complete
  replacement passed both original cases but exposed directory navigation before
  its Files module mounted (280 passes/one failure). That private story now
  prepares views.files while retaining its manually delayed folder response,
  Finish folder load control and exact skeleton assertions. Production loading,
  startup skeletons, lazy module implementation, assertions, deadlines,
  accessible identities, lifecycle and full Axe rules are unchanged. Exact
  eleven cases, complete 281-case replacement shard, exact 1021-case union and
  catalog types qualify the private setup. Earlier native five-case, HMR and
  Git-cycle repair acceptance remains source-applicable because production bytes
  have not changed."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Explicit M4.4, M16, M24, M30 and M31 review retains each recorded
  status. This source-preserving synchronization and catalog setup proof does
  not finish complete local qualification, hosted checks, native submission,
  live providers, other platforms, signed desktop, remote Forgejo protocols or
  visual baselines.

```
<!-- /evidence-field -->

