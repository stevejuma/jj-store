---
schema: 1
id: E-UI-096
---

# E-UI-096 details 2

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
jj workspace add ../jj-store-notifications --name notifications -r @-; source: jj workspace add ../design-core-notifications --name notifications -r @
```

**Notes**

Isolated kkoupnws from committed c8b7ea72 and source lqkpxqtl from 04b1ec1b; unrelated main navigation edits excluded. No installed executable or existing service replaced.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
source: pnpm test:storybook src/shared/workspace/plugins/notifications/ControlledNotifications.stories.svelte (before public API)
```

**Notes**

2 failed controlled stories plus 2 runtime diagnostics: existing renderer required app.notifications. Source owns controlled presentation API; no consumer CSS or dependency patch.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 30
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --offline --ignore-scripts --lockfile=false --config.pnpmfile=.local/notifications.pnpmfile.cjs; pnpm web:check (initial link validation)
```

**Notes**

Initial link: source qualification duplicated Svelte/Lezer identities and produced 30 type errors. Explicit file: installation fixed package resolution; no tracked manifest or lockfile edits.
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
source: pnpm install --offline --frozen-lockfile --ignore-scripts; pnpm packages:pack; pnpm pack --pack-destination .local; consumer: pnpm install --offline --ignore-scripts --lockfile=false --config.pnpmfile=.local/notifications.pnpmfile.cjs
```

**Notes**

packages:pack was a no-op because source version 0.1.2 is already published. Explicit pnpm pack created a new local tarball including runtime helpers and public type; ignored consumer hook installs that exact tarball. This qualifies changed source, not published 0.1.2 or the declared ^0.2.0 range.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte (initial)
```

**Notes**

20/28 passed. Five duplicate empty notification landmarks and three canvas/validation assertions failed. Source omits empty landmark semantics; only relocated toast selectors changed to the document, folder validation kept inline. Follow-up 28/28 passed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte (initial)
```

**Notes**

34/38 passed. Conflict success assertions still queried inline; Work unknown outcomes needed their original named inline message. Follow-up with StoreApplication added had 44/50 pass: two duplicate success outcomes after snapshot routing, two Work pending messages, a relocated Settings assertion and contextual stale fingerprint. Fixed actual lifetime/context bugs; kept exact recovery and stale selection feedback inline.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build; pnpm exec playwright test --config .local/notifications-playwright.config.ts notifications.spec.ts local-search-settings.spec.ts local-storage.spec.ts local-sync.spec.ts local-workspaces.spec.ts mutations.spec.ts file-editor-navigation.spec.ts conflicts.spec.ts (initial)
```

**Notes**

13/17 passed. Top-end portal obscured conflict/header controls and Sync refresh; moved foundational portal to bottom end. Exact four-case rerun passed conflicts but exposed dismissal replay during non-mutating Sync refresh; pending state now includes loading/refreshing in producers and recovery observers. Exact notification test then passed.
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
pnpm web:check; pnpm catalog:check (during StoreApplication refactor)
```

**Notes**

Missing mobile navigation aliases after removing two derived variables to meet the line budget. Restored aliases and consolidated persisted navigation layout state; subsequent types and architecture passed without raising budgets.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/notification-service.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

22/22: stable identity/dismissal/recovery, timed pause/resume, sticky scopes, actions/disposal; validation versus operational errors, retained drafts/revisions, refresh/cancel and reset/late responses. .local/notifications-unit-final.log.
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
pnpm test:storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

27/27 exact rerun; no duplicate completion toast on snapshot navigation, retained named unknown-outcome recovery and inline stale fingerprint. .local/notifications-stories-regressions.log.
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
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

852 modules and unchanged per-file budgets; owned UI boundary passes. No Rust or Community dependency introduced.
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
pnpm test:storybook catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

78/78 across15 affected files, interactions and configured accessibility. Packed source dependency. .local/notifications-stories-final.log.
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
pnpm web:build; pnpm exec playwright test --config .local/notifications-playwright.config.ts notifications.spec.ts local-search-settings.spec.ts local-storage.spec.ts local-sync.spec.ts local-workspaces.spec.ts mutations.spec.ts file-editor-navigation.spec.ts conflicts.spec.ts
```

**Notes**

17/17 cold built-UI tests under production CSP. Fresh isolated server and captured API mocks; no native/storage qualification substituted. .local/notifications-browser-final.log.
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
pnpm web:check; pnpm web:build; pnpm exec playwright test --config .local/notifications-playwright.config.ts notifications.spec.ts mutations.spec.ts
```

**Notes**

Final logical-resource scopes include workspace, bookmark, directory, evolution and operation comparison selections. Types0 errors/0 warnings; fresh production bundle;6/6 exact lifecycle/mutation tests after this refinement. .local/notifications-browser-scope.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Both Svelte checks0 errors/0 warnings,165 catalog modules,852 architecture modules, UI ownership passes. StoreApplication stays within its unchanged320-line budget.
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
pnpm exec prettier --write changed files; pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Formatting and126-task bookkeeping pass;50 protected files/four canonical chapters;five validators/22 chapters/153 requirements. No existing milestone promoted.
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

<!-- evidence-field:scope markdown -->
## Scope

Production notification service/root, Settings, repository and command feedback, thread/task/review actions and their direct consumers. Source public API/CSS required Design Core full nonvisual lanes, recorded in its verification chapter. Consumer follows the targeted UI policy: root aggregates, Rust/RustFS/native desktop storage suites and schema generation are out of scope. Cold browser uses an isolated static server on127.0.0.1:7568 serving this built web/dist with the exact STRICT_CSP from assets.rs and captured mocked APIs; it is not native runtime or storage qualification. Registry publication/consumer range refresh and visual baselines remain separate; no existing milestone promoted.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Release the separately committed Design Core extension and validate registry-resolved consumption in a separately authorized release slice. M4.31 remains in progress for that gate; OS delivery and notification history remain deferred.
<!-- /evidence-field -->

