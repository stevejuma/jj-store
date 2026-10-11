---
schema: 1
id: E-GLOBAL-SETTINGS-001
---

# E-GLOBAL-SETTINGS-001 details 2

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
pnpm test:storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte
```

**Notes**

19 baseline stories passed before the global refresh. Local log: .local/global-settings/baseline.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

Interrupted initial cold run (exit 130); new icon dependency optimization invalidated modules and old layout assertions failed. Switched to existing imported icons and restarted cold; see subsequent runs. Failure count records 12 observed story failures, not a completed suite. Local log: .local/global-settings/global-stories.log.
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
pnpm test:storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte
```

**Notes**

27 passed; ambiguous memory input label and old restart-message assertion corrected. Local log: .local/global-settings/first-focused.log.
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
pnpm test:storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

60 passed; unit-menu pointer-events timing assertion corrected. Local log: .local/global-settings/global-final.log.
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
pnpm catalog:test web/tests/vite/server-settings.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/agent-settings.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/repository-settings-ownership.test.ts web/tests/vite/repository-settings-restoration.test.ts
```

**Notes**

22 controller tests passed: installation configuration, capture, agent settings, freshness, ownership and restoration. Local log: .local/global-settings/controller-tests.log.
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
pnpm test:storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

37 passed; returning to Theme retains gallery expansion, so removed the test click that closed it. Local log: .local/global-settings/spacing-appearance.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentChatGpt.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/StoreNumericSetting.stories.svelte catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/RepositorySettingsLoading.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

97 passed. Five obsolete presentation selectors/expectations corrected. Shared composer mention failure reproduced at parent and retained separately. Local log: .local/global-settings/direct-consumers.log.
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
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte
```

**Notes**

51 passed. Added clipboard stub for the new inventory copy story; remaining composer failure inherited. Local log: .local/global-settings/followups.log.
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
pnpm test:storybook catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
```

**Notes**

Interrupted with exit 130 after the inventory copy failure; no completed pass claimed. Local log: .local/global-settings/runtime-final.log.
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
pnpm test:storybook catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
```

**Notes**

50 stories passed across runtime/defaults, service configuration, capture and multi-harness settings. Local log: .local/global-settings/final-regressions.log.
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
pnpm test:storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

22 capture stories passed after shared banner clearance. Local log: .local/global-settings/banner-final.log.
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
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/ConversationMetadata.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

21 stories passed across AgentDefaults, AgentRuntimes, ConversationMetadata and AgentSessionSettings; consolidated preferences, metadata keyboard disclosure/draft retention and existing save behavior. Local log: .local/global-settings/preferences-tests.log.
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
pnpm test:storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

13 Compute stories passed, including the shared disclosure consumer. Local log: .local/global-settings/compute-disclosure-final.log.
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
pnpm test:storybook catalog/components/store-application/AgentMentionSettings.stories.svelte
```

**Notes**

Initial parent workspace attempt with symlinked dependencies could not import Vitest setup; not accepted as baseline proof. Local log: .local/global-settings/mention-parent.log.
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
pnpm install --offline --frozen-lockfile --ignore-scripts
```

**Notes**

Isolated parent dependencies installed offline/frozen/ignore-scripts. Local log: .local/global-settings/baseline-install.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentMentionSettings.stories.svelte
```

**Notes**

Parent a8256a61: one pass, same one shared-composer failure before entering Settings (Automatically mention agents switch absent). Unrelated composer owner; no product fix or weakened assertion here. Local log: .local/global-settings/mention-parent2.log.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings. Local log: .local/global-settings/web-delivery.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story used Playwright-only exact role option; removed invalid option without changing its matching behavior. Local log: .local/global-settings/catalog-delivery.log.
<!-- /run -->
<!-- /evidence-field -->

