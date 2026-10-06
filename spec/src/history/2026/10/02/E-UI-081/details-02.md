---
schema: 1
id: E-UI-081
---

# E-UI-081 details 2

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
pnpm catalog:test web/tests/vite/thread-navigation.test.ts (before)
```

**Notes**

Standalone section and missing owner forwarding reproduced. /tmp/jj-thread-navigation-before.log.
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
pnpm catalog:test web/tests/vite/thread-navigation.test.ts web/tests/vite/production-threads.test.ts
```

**Notes**

25passed. /tmp/jj-thread-navigation-unit-first.log.
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
pnpm catalog:test web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/thread-navigation.test.ts web/tests/vite/production-threads.test.ts
```

**Notes**

28passed: independent native owners, paging/window retention, late credential teardown and foreign/denied reads. /tmp/jj-thread-navigation-unit-qualified.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-threads/StoreThreadNavigation.stories.svelte (initial)
```

**Notes**

Test setup expected a custom label on Breadcrumb, while the public primitive uses breadcrumb; mobile sidebar starts closed. Corrected selectors and opened the accessible mobile sidebar. /tmp/jj-thread-navigation-story-first.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial)
```

**Notes**

Web0errors0warnings; catalog11errors0warnings from unsupported Testing Library exact role option. Removed invalid options; exact string matching retained. Logs web-first/catalog-first.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial/second/third)
```

**Notes**

Initial controller313/300, navigation368/364, shell323/320; formatting then left controller311/300 and navigation378/364; final intermediate controller302/300. Extracted reader factory, effect binding and picker/action, reused lifecycle callbacks/source accessor. No budget changes. Logs architecture-first/second/third.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-threads/StoreThreadNavigation.stories.svelte (second)
```

**Notes**

Assertions omitted normal breadcrumb whitespace. Corrected to anchored whitespace-tolerant directory text; repository, thread, links, icon-only action and indentation assertions retained. /tmp/jj-thread-navigation-story-second.log.
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
pnpm test:storybook catalog/components/store-threads/StoreThreadNavigation.stories.svelte (qualified)
```

**Notes**

3passed including desktop/mobile/compact owner switching, Browse action, directory breadcrumbs, no standalone section and child icon indentation greater than16px. /tmp/jj-thread-navigation-story-third.log.
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
pnpm web:check; pnpm catalog:check (after picker extraction)
```

**Notes**

0errors0warnings;146catalog modules. Logs web-final/catalog-final.
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
pnpm catalog:test web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/thread-navigation.test.ts web/tests/vite/thread-navigation-integration.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-automatic-refresh.test.ts (first extended)
```

**Notes**

41passed/1failed: optional cleared replyId was undefined, test expected literal null. Normalize optional absence with ?? null while retaining exact owner, thread and cleared reply assertions. /tmp/jj-thread-navigation-unit-final.log.
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
pnpm catalog:test web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/thread-navigation.test.ts web/tests/vite/thread-navigation-integration.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-automatic-refresh.test.ts (qualified)
```

**Notes**

42passed across7files; exact owner connection, authorized native fallback, unsupported/unauthorized rejection, stale navigation guard and independent sidebar freshness included. /tmp/jj-thread-navigation-unit-qualified-final.log.
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
pnpm test:storybook catalog/components/store-threads catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

47passed across8files, interaction/accessibility. Fresh Vitest browser process. /tmp/jj-thread-navigation-consumers.log.
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
pnpm check:architecture (qualified)
```

**Notes**

788modules, no module budget or ownership violations. /tmp/jj-thread-navigation-architecture-final.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Mapped api.md and spec-governance.md missing from current change. Added bounded authorization/teardown and UI qualification contract. /tmp/jj-thread-navigation-spec-first.log.
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
Targeted pnpm exec prettier --write (changed files; subsequent extraction and integration-test paths)
```

**Notes**

Formatted all changed production/test/fixture/provenance/UI chapter files; final canonical ledger/chapter formatting and checks follow.
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
Cold live browser sidebar selection, thread context and Browse threads action (CUA temporary background tab)
```

**Notes**

Projects hub lists thread children under their owner. Selecting test connects jj-store demo and restores its exact thread. Breadcrumb shows owner/Threads/test; group Browse threads returns directory with owner/Threads and retained operation. Child icon left42.59375px vs repository23px. Screenshot inspected and saved; tab closed, user tab unchanged. No live data writes or service resets.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check (30 changed files)
```

**Notes**

Final web/catalog0errors0warnings;146catalog modules; ownership1validator; spec-first26protected files/4chapters; spec validation5validators/21chapters/147requirements; progress120tasks; all changed files formatted. Logs web-qualified/catalog-qualified/ownership/spec-first-qualified/spec-validate/progress/format. Final evidence insertion is formatted and rechecked before committing.
<!-- /run -->
<!-- /evidence-field -->

