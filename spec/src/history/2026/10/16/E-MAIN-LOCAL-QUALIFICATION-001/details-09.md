---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 9

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
pnpm build-storybook
```

**Notes**

storybook-build-17: exit 0, 79.29 seconds. 
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- test --locked -p jj-store-cli --test server_executable --no-run
```

**Notes**

cli-test-selector-before-18: exit 1. Actual test selection fails: package ID specification jj-store-cli did not match any packages. Corrected both npm entry points to the real package jj-store.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-cold-17: exit 0, 18.5 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --lib -- --exact capture_management::tests::scheduled_import::accepted_source_refuses_other_request_without_replacing_original_owned_work capture_management::tests::scheduled_import::handoff::import_waits_for_existing_raw_active_reader_and_keeps_the_same_session_identity capture_management::tests::scheduled_import::handoff::compact_active_handoff_reuses_cursor_alias_and_publishes_one_native_session capture_management::tests::scheduled_import::lost_caller_and_full_mailbox_keep_one_original_command_without_body_ownership capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress capture_management::tests::scheduled_import::restart::accepted_import_resumes_tail_after_restart_without_a_second_command capture_management::tests::scheduled_import::restart::original_selector_reconciles_after_new_runtime_without_resetting_selected_history capture_management::tests::scheduled_import::restart::recovery::accepted_restart_cannot_transfer_consent_to_a_replacement_physical_file capture_management::tests::scheduled_import::restart::recovery::accepted_restart_preserves_durable_pause_and_recovers_after_resume capture_management::tests::scheduled_import::restart::recovery::accepted_restart_renews_repository_binding_before_capturing_tail capture_management::tests::scheduled_import::retained_runtime_import_publishes_inactive_history_with_existing_source_budgets capture_management::tests::scheduled_import::restart::recovery::paused_recovery_settles_while_management_holds_its_policy_gate capture_management::tests::scheduled_import::scheduled_import_pause_replay_and_tail_preserve_original_receipt_and_cursor capture_supervisor::opencode::tests::blocked_api_publication_retains_drain_ownership_while_ingress_continues capture_supervisor::tests::observations::source_degradation_is_cached_and_clears_after_successful_recovery
```

**Notes**

capture-supervision-04: exit 1, 315.29 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress -- --exact --nocapture
```

**Notes**

capture-prefix-19: exit 1, 174.74 seconds. 
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
pnpm web:build
```

**Notes**

web-build-17: exit 0, 211.48 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress -- --exact --nocapture
```

**Notes**

capture-prefix-20: exit 1, 81.19 seconds. 
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress -- --exact --nocapture
```

**Notes**

capture-prefix-21: exit 1, 106.8 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress -- --exact --nocapture
```

**Notes**

capture-prefix-22: exit 1, 95.24 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-full-16.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-full-16: exit 1, 929.16 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --lib capture_management::tests::scheduled_import::multi_step_import_prefix_survives_worker_parking_without_starving_ingress -- --exact --nocapture
```

**Notes**

capture-prefix-23: exit 0, 87.52 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

appearance-23: exit 0, 20.93 seconds. 
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
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

workspace-clippy-12: exit 0, 61.81 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
```

**Notes**

pinning-23: exit 0, 15.82 seconds. 
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-1-24.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-1-24: exit 1, 271.26 seconds. 
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
pnpm web:check
```

**Notes**

web-check-25: exit 0, 24.34 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-25: exit 0, 35.25 seconds. 
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
pnpm check:ui-ownership
```

**Notes**

ui-ownership-25: exit 0, 4.41 seconds. 
<!-- /run -->
<!-- /evidence-field -->

