---
schema: 1
id: E-INBOX-FRESHNESS-001
---

# E-INBOX-FRESHNESS-001 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/{AgentExecution,AgentInboxNavigation,AgentInboxTree,AgentChatChanges,AgentDefaults,AgentFleet,AgentMentionSettings,ConversationMetadata,AgentSessionSettings,StoreRepositoryNavigation,SettingsNavigation}.stories.svelte catalog/components/store-threads/{StoreThreads,StoreThreadNavigation}.stories.svelte catalog/components/store-work/{ProjectsAgents,ProjectsHub,NavigationPersistence}.stories.svelte
```

**Notes**

Sixteen files and 87 interaction/accessibility stories passed. Includes every direct consumer of the changed fleet fixture.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI boundary passes; no Community runtime, type or checkout dependency introduced.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Inherited oversizes only: WorkspaceFiles story 678/400; dev-ui.acceptance 311/300; inbox.css 309/300; StoreAgentChatBody 309/300; comments.svelte 301/300; StoreApplication 326/320. jj file show -r @- and countedLines confirmed all six byte-identical. New overages were removed by extracting owned helpers; limits unchanged.
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
pnpm test:dev:ui
```

**Notes**

Repeated isolated cold native/Vite acceptance: no Rust builds; real writes/events, source HMR without document reload, backend-only restart preserving Vite PID, origin/host guards and clean service teardown. Last fixture /private/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-hmr-native-sb60K8. Existing development services were retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentInboxNavigation.stories.svelte -t NavigationReusesSettingsFeedbackAndTranscriptWithoutAnotherStream
```

**Notes**

The strengthened actual-shell observer fixture first had a misplaced import (one runtime error), repaired before the real regression. Then expected one settings read but observed two. This root observer used forced refresh on reactive reruns/remounts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentInboxNavigation.stories.svelte -t NavigationReusesSettingsFeedbackAndTranscriptWithoutAnotherStream
```

**Notes**

Actual shell observer now uses ensure; exact request-count regression passes (one test, three skipped), including route-reactive controller access.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Evidence file list preceded generated histories and final root observer file; unrelated stale Bootstrap/M13/M19 history regeneration was restored. Current slice paths were then added to evidence.
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
```

**Notes**

Seven validators passed.
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
jj workspace forget freshness-parent-qualification
```

**Notes**

Parent checkout was clean. Agent-created qualification directory removed with checked Python cleanup; no main files/services removed.
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
```

**Notes**

Evidence test paths must include named tests and a passing overall result cannot retain gaps. Corrected to named references and pending audit status; passing scoped commands remain explicit.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts -t "dirty failed settings"
```

**Notes**

Final outage regression proved failed dirty reads still retried during feed outage. Automatic watch ownership now pauses those reads until the advertised feed is healthy.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Historical evidence referenced the three renamed polling tests. Recorded their removal and replacement by event/no-polling coverage without rewriting committed evidence.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts web/tests/vite/application-freshness.test.ts web/tests/vite/automatic-refresh.test.ts web/tests/vite/capture-freshness.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

41 tests pass, including pausing failed dirty settings reads during an advertised feed outage.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/inbox-request-lifecycle.test.ts -t 'history captured before initial status'
```

**Notes**

Cold-start regression: a history capture preceding status was keyed as unknown. The first return performed two reads instead of one. /tmp/jj-inbox-initial-boundary-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/inbox-request-lifecycle.test.ts -t 'history captured before initial status'
```

**Notes**

Exact after target: one passed, three skipped. Initial identity comes from captured event commitment; later publication changes still recapture. /tmp/jj-inbox-initial-boundary-after.log.
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
pnpm catalog:test
```

**Notes**

Latest final lane after outage and cold-boundary repairs: 149 files, 896 passed and one skipped. Previous final-gated lane had 895 passing tests. /tmp/jj-inbox-units-final-boundary.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/{AgentExecution,AgentInboxNavigation,AgentInboxTree,AgentChatChanges,AgentDefaults,AgentFleet,AgentMentionSettings,ConversationMetadata,AgentSessionSettings,StoreRepositoryNavigation,SettingsNavigation}.stories.svelte catalog/components/store-threads/{StoreThreads,StoreThreadNavigation}.stories.svelte catalog/components/store-work/{ProjectsAgents,ProjectsHub,NavigationPersistence}.stories.svelte
```

**Notes**

Final repeat after outage and cold-boundary repairs: 16 files, 87 interaction/accessibility tests. /tmp/jj-inbox-stories-final-boundary.log.
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
pnpm test:dev:ui
```

**Notes**

Final cold isolated native/Vite run after boundary repair. HMR without document reload; real writes/events; backend restart retained Vite PID; guard checks and cleanup passed. Fixture /private/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-hmr-native-C4Hx3D. /tmp/jj-inbox-cold-final-boundary.log.
<!-- /run -->
<!-- /evidence-field -->

