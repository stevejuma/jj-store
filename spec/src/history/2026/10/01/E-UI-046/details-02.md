---
schema: 1
id: E-UI-046
---

# E-UI-046 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-collaboration.test.ts -t 'review edits|unknown review edits'
```

**Notes**

Before: edit is not a function. .scratch/native-actions-unit-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte -t 'Mobile details control|Agent response activity'
```

**Notes**

All 20 skipped; no acceptance credit.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Before: header end alignment and inline agent root; 18 pass. .scratch/native-actions-stories-before-full.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-collaboration.test.ts
```

**Notes**

Final 16/16 including edit-start revision, pending identity and disconnected receipt. .scratch/native-actions-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Intermediate suite import errors: invalid Svelte state initializer assignment; 17 passing tests. Replaced with a declaration initializer.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

First: wait for Community delayed hover before pointer action; scope new reply controls and remove duplicate outcome alert. Next rerun: two invalid test-only ControlOrMeta key expressions; use supported editor clear. .scratch/native-actions-message-stories{,2}.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte
```

**Notes**

Final three direct native-comment interactions pass in fixed-story rerun; intermediate assertions confused CodeMirror placeholder text with document text. Assert visible placeholder and absence of saved draft. Original/read-only/unknown cases retained.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

40 pass; two fixture expectations: composer placeholder and superseded tab order. Direct unchanged consumers pass. .scratch/native-actions-direct-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

11 pass; old ProjectsHub array order still on one line. Update to requested Activity-first order.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

18/18 after tab order correction and exact identifier menu assertions. .scratch/native-actions-final-focused-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeAgentMenus.stories.svelte
```

**Notes**

Extracted native-menu story passes; exact session/root/tool/message values, no row selection and portaled toolbar retention. .scratch/native-actions-agent-menu-story.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Final 13/13 after extracting comment removal and standalone menu story; .scratch/native-actions-extracted-stories.log.
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
pnpm web:check
```

**Notes**

Final zero errors/warnings. Intermediate two full-width version type errors and one missing pending-request initializer; preserve number|bigint and initialize the field.
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
pnpm catalog:check
```

**Notes**

Final zero errors/warnings and 96 modules; earlier six type errors from fixture content narrowing, literal permission widening and unsupported ByRole exact options, then one missing draft field. All repaired.
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
pnpm check:architecture
```

**Notes**

Extracted composer, task removal, fixture edits and standalone agent menu story; intermediate four counted-line failures repaired without raising budgets.
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
pnpm web:build
```

**Notes**

Cold generated UI build and manifest; existing large-chunk advisory remains. .scratch/native-actions-web-build.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Intermediate ledger lacked named test fragments and recorded component removal; repair ledger references without altering tests. Story cases are recorded in stories/runs because the named-test checker recognizes unit definitions only.
<!-- /run -->

<!-- run -->
### Run 18

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

34 protected files, 4 canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

