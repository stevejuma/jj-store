---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 13

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

storybook-build-42: exit 0, 62.62 seconds. 
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-cold-42: exit 1, 17.08 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-42: exit 0. All1021unique cases across211files pass;12WorkflowHostSettings cases replaced by fresh passing report after unchanged helpers extracted. No missing/unexpected/duplicate cases; other passing shard results preserved.
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
pnpm exec prettier --write catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/fixtures/workflow-host-settings-play.ts
```

**Notes**

format-workflow-42: exit 0. Formatted owned fixture extraction; story IDs/names and assertions retained.
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
node --test .local/main-qualification-2026-10-16/catalog-diagnostic-43.test.mjs
```

**Notes**

catalog-diagnostic-43: exit 0. Observability-only hooks logged page errors and failed requests without changing assertions or route behavior. Exact cold catalog passed; Chromium reported static script/style net::ERR_INSUFFICIENT_RESOURCES during concurrent embedded-runtime compilation. Original unmodified cold42 failure remains recorded. Required fresh unmodified cold pass after native build stops; no error suppression/retry added.
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
pnpm dev:disk
```

**Notes**

disk-report-45: exit 0. 71.7 GiB managed build roots, 83.1 GiB free; within 80/40/30-GiB policy. No cleanup performed.
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
node scripts/ci-storage.mjs embedded
```

**Notes**

embedded-runtime-18: exit 0, 2963.39 seconds. 
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
pnpm test:local-trusted
```

**Notes**

local-trusted-entrypoint-18: exit 0, 32.85 seconds. 
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
pnpm desktop:test
```

**Notes**

desktop-tests-18: exit 0, 33.84 seconds. 
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
env -u JJ_STORE_TOKEN -u JJ_STORE_OPERATOR_TOKEN pnpm dev:cargo -- test --locked -p jj-store --test executable_native -- --ignored --exact os_credential_facility_roundtrip os_credential_initial_admin_provisioning_is_repeatable executable_login_logout_uses_os_credentials_across_processes
```

**Notes**

keychain-native-31: exit 0, 15.47 seconds. 
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store --test desktop_supervision -- --ignored --exact desktop_owner_eof_stops_real_service_and_retains_installation
```

**Notes**

desktop-supervision-31: exit 0, 10.67 seconds. 
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store --test work_rustfs -- --ignored --exact workstore_real_rustfs_restart_checkpoint_copy_and_historical_recovery
```

**Notes**

work-rustfs-31: exit 0, 12.79 seconds. 
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store-server --test repository_conversion -- --ignored --exact cold_rustfs_conversion_round_trip_retains_nonempty_history_across_restart
```

**Notes**

conversion-rustfs-31: exit 0, 48.95 seconds. 
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store --test server_executable -- --ignored --exact configured_capture::cold_configured_codex_capture_initializes_publishes_and_preserves_pause git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
```

**Notes**

server-capture-git-workflows-31: exit 0, 74.51 seconds. 
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
JJ_STORE_WORKING_TREE_TEST_JJ=$PWD/target/debug/jj-store-jj pnpm dev:cargo -- test --locked -p jj-store-service --test working_tree_local --test working_tree_observer -- --ignored
```

**Notes**

working-tree-local-31: exit 0, 70.93 seconds. 
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
JJ_STORE_WORKING_TREE_TEST_JJ=$PWD/target/debug/jj-store-jj JJ_STORE_WORKING_TREE_CONTRACT_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm dev:cargo -- test --locked -p jj-store-http --lib local_browser::working_tree::tests::registered_observation_route_reads_real_checkout_without_publication_or_content_leaks -- --ignored --exact
```

**Notes**

working-tree-http-31: exit 0, 6.04 seconds. 
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-working-tree.test.ts
```

**Notes**

working-tree-ui-capture-31: exit 0, 3.85 seconds. 
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
JJ_STORE_AGENT_TEST_JJ=$PWD/target/debug/jj-store-jj pnpm dev:cargo -- test --locked -p jj-store-jj --test agent_workspaces -- --ignored --exact isolated_workspace_preserves_dirty_source_and_retains_cleanup_revision
```

**Notes**

agent-workspace-jj-31: exit 0, 56.93 seconds. 
<!-- /run -->
<!-- /evidence-field -->

