---
schema: 1
id: E-INBOX-FRESHNESS-001
---

# E-INBOX-FRESHNESS-001 details 2

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
jj --no-pager st
```

**Notes**

Default workspace; parent 2cd2903b59a7a12c6066ea8ff558bef757adc9a4. Existing task changes retained. Enclosing change klrylvzw.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/sidebar-read-lifecycle.test.ts
```

**Notes**

Before target: missing invalidate/setVisible APIs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/sidebar-read-lifecycle.test.ts web/tests/vite/repository-thread-navigation.test.ts
```

**Notes**

Seven tests after lazy-reader implementation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-changes-cache.test.ts
```

**Notes**

Before target: missing cache module; no tests ran.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-changes-cache.test.ts
```

**Notes**

Three tests: shared cancellation, failed-read retention, exact keys and eviction.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts web/tests/vite/sidebar-read-lifecycle.test.ts web/tests/vite/agent-changes-cache.test.ts web/tests/vite/projects-freshness.test.ts
```

**Notes**

18 passed; new compute test used unsupported settings domain. Strict parser closed the fixture feed; corrected to agent_settings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts web/tests/vite/sidebar-read-lifecycle.test.ts web/tests/vite/agent-changes-cache.test.ts web/tests/vite/projects-freshness.test.ts
```

**Notes**

20 passed, including removed-repository dirtiness and event-only compute settlement.
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
pnpm catalog:test
```

**Notes**

Final shared unit lane: 149 files, 894 passed and one skipped. Earlier complete run: 147 files, 887 passed, one skipped; expanded run: 149 files, 892 passed, one skipped. Previous metadata supersession and mock stream double-close failures were repaired before passing those lanes.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New navigation story supplied Playwright exact options to Testing Library. Removed unsupported options; string role names remain exact by default.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors and warnings; 356 module boundaries.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/{AgentExecution,AgentInboxNavigation,AgentInboxTree,AgentChatChanges,ConversationMetadata,AgentSessionSettings,StoreRepositoryNavigation,SettingsNavigation}.stories.svelte catalog/components/store-threads/{StoreThreads,StoreThreadNavigation}.stories.svelte catalog/components/store-work/{ProjectsAgents,ProjectsHub,NavigationPersistence}.stories.svelte
```

**Notes**

70 passed. New thread creation failed to update sidebar after removing its redundant GET; repaired by consuming current Threads rows. Two completed-change fixture checks relied on duplicate boot reads; fixtures now await initial capture, advertise correct publication boundaries and model unread reconnect activity. The Agents absence assertion was inherited.
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
jj workspace add /tmp/jj-store-freshness-parent-qualification --name freshness-parent-qualification -r 2cd2903b59a7a12c6066ea8ff558bef757adc9a4 -m "Qualify inherited navigation regressions"
```

**Notes**

Read-only parent qualification workspace. Initially symlinked dependencies failed browser module admission; hardlinked released dependencies initially omitted the shipped Vitest client manifest. Restoring that package asset allowed source-identical parent tests. No parent source changed.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte -t "RightRailNavigation|ChannelNavigationCreationAndMembers|SelectedLiveAvatarAndCompletedFilePanel|ReconnectWarningStaysInsideTheComposerAndClearsAfterRefresh"
```

**Notes**

Pinned parent: three passed, eleven skipped; RightRailNavigation alone failed because it expected the standalone Agents button to be absent. Repaired the assertion to require the JST-UI-015 control; other section assertions retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadNavigation.stories.svelte -t "Collapsed repository children"
```

**Notes**

Incorrect filter matched no generated story names: all four skipped. This is not passing evidence; the complete family subsequently ran.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte
```

**Notes**

Eleven interaction/accessibility stories passed, including new lazy children and existing thread creation.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

Seven passed. Asynchronous staging exposed a test capturing a null header before loading; corrected to wait for the header. Two exact intermediate change-story runs also failed on this same header assumption.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte
```

**Notes**

Both completed-change and reconnect/draft interaction/accessibility stories pass after staging and header readiness repair.
<!-- /run -->
<!-- /evidence-field -->

