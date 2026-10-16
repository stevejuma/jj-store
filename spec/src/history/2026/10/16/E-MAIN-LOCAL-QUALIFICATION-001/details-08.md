---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 8

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
pnpm check:progress
```

**Notes**

progress-14: exit 1, 8.16 seconds. 
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

format-14: exit 1, 42.5 seconds. 
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
pnpm spec:validate
```

**Notes**

spec-validate-14: exit 0, 3.37 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store --test agent_execution comment_answers:: -- --test-threads=1
```

**Notes**

comment-answers-04: exit 0, 111.68 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store --test capture_import_cli
```

**Notes**

capture-import-04: exit 0, 28.68 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./ui-regressions-13.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/GlobalSettingsLoading.stories.svelte catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-application/RepositorySettingsLoading.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryConversion.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreSyncReconciliation.stories.svelte catalog/components/store-application/WorkspaceFilesPerformance.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte catalog/components/store-forge/StoreForgeHeader.stories.svelte catalog/components/store-forge/StoreForgeLinks.stories.svelte catalog/components/store-forge/StoreForgeMerge.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-search/StoreRecentSearch.stories.svelte catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-workflows/WorkflowCards.stories.svelte catalog/components/store-workflows/WorkflowChoices.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte catalog/components/store-workflows/WorkflowFilePicker.stories.svelte catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte catalog/components/store-workflows/WorkflowRun.stories.svelte catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

ui-regressions-13: exit 1, 184.94 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./ui-regressions-12.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeLinks.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

ui-regressions-12: exit 0. 5 passed across provider configuration and contextual light/dark PR links.
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
pnpm dev:cargo -- test --locked -p jj-store --test native_lifecycle -- --exact content_acceptance::http_acceptance::http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication graph_acceptance::prepared_log_pages::prepared_all_log_pages_seek_deep_anchors_under_fixed_work
```

**Notes**

native-lifecycle-04: exit 0, 32.6 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store --test native_rpc inventory::browser_kind_lookup_retains_historical_roots_and_fresh_bounds -- --exact
```

**Notes**

native-rpc-04: exit 0, 14.51 seconds. 
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

web-check-12: exit 0, 19.01 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store --test stack_checkout_plans coordination_process::process_loss_after_native_evidence_and_final_ack_resumes_original_join -- --exact
```

**Notes**

stack-coordination-04: exit 0, 31.8 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-12: exit 0, 31.65 seconds. 
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
pnpm check:ui-ownership
```

**Notes**

ui-ownership-12: exit 0, 2.83 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./ui-regressions-15.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeHeader.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/WorkspaceFilePresentation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-forge/StoreForgeMerge.stories.svelte catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-search/StoreRecentSearch.stories.svelte catalog/components/store-application/StoreRepositoryConversion.stories.svelte --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

ui-regressions-15: exit 0, 88.21 seconds. 
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
pnpm web:test
```

**Notes**

web-tests-16: exit 0, 2.21 seconds. 
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

catalog-unit-16: exit 0, 22.94 seconds. 
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
node scripts/package-check.mjs
```

**Notes**

package-check-17: exit 0. 21 packages; registry dependencies, contained compilation inputs and coordinated 0.1.0 metadata passed.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- pkgid --locked -p jj-store-cli
```

**Notes**

pkgid-wrapper-diagnostic-18: exit 1. Managed Cargo does not admit pkgid. This diagnostic establishes no package-selection proof; repeat with the actual test command.
<!-- /run -->
<!-- /evidence-field -->

