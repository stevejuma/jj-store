---
schema: 1
id: E-M13-021
---

# E-M13-021

<!-- evidence-field:id markdown -->
## Id

E-M13-021
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-UI-003
- JST-UI-005
- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ui-regression
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/projects/repository-log-message.ts
- catalog/projects/native-presentation.test.ts
- catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
- spec/src/ui.md
- spec/src/api.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/projects/native-presentation.test.ts#git_sha1_ids_remain_complete_in_operation_messages
- catalog/projects/native-presentation.test.ts#native_ids_remain_complete_in_messages
- catalog/projects/native-presentation.test.ts#partial_or_embedded_ids_remain_plain_text
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/projects/native-presentation.test.ts
```

**Notes**

Exact before: Git SHA-1 operation message remained one text segment; 4 existing/native/boundary tests passed. .scratch/oplog-git-unit-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/projects/native-presentation.test.ts
```

**Notes**

Exact after: all 5 tests pass; Git/native/change IDs retain full values, and partial/embedded tokens remain plain text. .scratch/oplog-git-unit-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
```

**Notes**

All 4 graph stories pass interaction and blocking accessibility checks, including compact Git message IDs, exact clipboard values, keyboard copying, copy/row separation and normal operation selection. .scratch/oplog-git-stories.log.
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
pnpm web:check
```

**Notes**

0 errors and warnings. .scratch/oplog-git-web-check.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story used an unsupported getByRole exact option and matcher declaration. Removed the redundant option and asserted one call plus its complete argument using supported matchers; assertions retained. .scratch/oplog-git-catalog-check.log.
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

0 errors and warnings; 119 catalog boundary modules pass. .scratch/oplog-git-catalog-check-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
```

**Notes**

Final fresh-process run: all 4 interaction/accessibility cases pass after story type repairs. .scratch/oplog-git-stories-final.log.
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
pnpm web:build
```

**Notes**

UI bundle and asset manifest rebuilt; inherited dependency CSS and chunk warnings retained. .scratch/oplog-git-build.log.
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
pnpm exec prettier --write web/src/lib/projects/repository-log-message.ts catalog/projects/native-presentation.test.ts catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte spec/src/ui.md spec/src/api.md
```

**Notes**

Changed formatter, regression, graph story and canonical chapters formatted.
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
JJ_STORE_DEV_PORT=7460 JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/local-browser-merge-prebuilt.mjs restart
```

**Notes**

Owned dev server restarted with rebuilt UI and previously qualified unchanged Rust binaries; existing three repository identities and demo contents retained. No Rust builds for this UI slice. .scratch/oplog-git-ui-restart.log.
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
CUA createBrowserTab(iab, reported local Op Log URL, visible=false); preview.getAXState()
```

**Notes**

Fresh served app on 7460 displays 87e1cd5d…8fb9 and 20d6c008…ef7c as separately labelled Copy full commit ID controls with the complete 40-character values; reported snapshot and all 5 operation rows retained.
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
pnpm check:progress
```

**Notes**

119 tasks; integration evidence and canonical bookkeeping pass.
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
pnpm spec:first
```

**Notes**

3 protected files and 3 canonical chapters pass the same-change gate.
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters, 141 requirements pass.
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
pnpm exec prettier --check web/src/lib/projects/repository-log-message.ts catalog/projects/native-presentation.test.ts catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte spec/src/ui.md spec/src/api.md spec/src/milestones.md
```

**Notes**

All changed source, test, story and canonical files pass formatting.
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
<!-- generated-navigation -->
## Pages

- [E-M13-021 details 2](details-02.md)
