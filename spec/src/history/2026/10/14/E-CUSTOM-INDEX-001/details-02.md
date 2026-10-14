---
schema: 1
id: E-CUSTOM-INDEX-001
---

# E-CUSTOM-INDEX-001 details 2

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
pnpm dev:cargo -- test -p jj-store-service --test workflow_store --locked --target-dir .local/build-targets/custom-index
```

**Notes**

Before implementation: both existing workflow tests pass.
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
pnpm dev:cargo -- test -p jj-store-service --test workflow_store --locked --target-dir .local/build-targets/custom-index index::unchanged_workflow_roots_do_not_reconstruct_objects -- --exact
```

**Notes**

New unchanged-root regression failed: 2 workflow-object reads, expected 0. The test was subsequently expanded into the persistent generation/source-admission regression.
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
pnpm dev:cargo -- test -p jj-store-service --test workflow_store --locked --target-dir .local/build-targets/custom-index index::unchanged_workflow_roots_do_not_reconstruct_objects -- --exact
```

**Notes**

Warm unchanged roots perform zero workflow-object reads.
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
pnpm dev:cargo -- check -p jj-store-read-index --target-dir .local/build-targets/custom-index
```

**Notes**

Isolated derived engine compiles.
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
pnpm dev:cargo -- test -p jj-store-read-index -p jj-store-service --test costs --test queries --test persistence --test workflow_store --test work_store --test content_search --test collab_pagination --test agent_sessions --test review_threads --locked --target-dir .local/build-targets/custom-index -- --nocapture
```

**Notes**

Initial broad direct run exposed an agent-session stack overflow from the enlarged async materializer and review closure admission on cache hits. Both were corrected without weakening existing tests.
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
pnpm dev:cargo -- test -p jj-store-service --test agent_sessions --locked --target-dir .local/build-targets/custom-index session_pages_retain_one_agent_root_across_later_publication -- --exact
```

**Notes**

Boxed materialization/workflow futures remove the agent-session stack overflow.
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
pnpm dev:cargo -- test -p jj-store-service --test review_threads --locked --target-dir .local/build-targets/custom-index warm_review_activity_keeps_source_limits_access_and_complete_closure_admission -- --exact
```

**Notes**

Fresh complete ReviewStore closure admission retained after reverting the attempted cache shortcut.
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
pnpm dev:cargo -- clippy -p jj-store-read-index --all-targets --locked --target-dir .local/build-targets/custom-index -- -D warnings
```

**Notes**

Strict all-target engine lint check.
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
pnpm dev:cargo -- test -p jj-store-read-index -p jj-store-service --test costs --test queries --test persistence --test workflow_store --test work_store --test content_search --test collab_pagination --test agent_sessions --test review_threads --test agent_checkpoints --test agent_relations --test native_collab --test local_work --test local_collab --test local_agent --test local_agent_pages --locked --target-dir .local/build-targets/custom-index -- --nocapture
```

**Notes**

119 tests across 16 direct binaries pass before the subsequently authorized lint cleanup.
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
pnpm dev:cargo -- test -p jj-store-server --test configuration --test custom_data_indexes --locked --target-dir .local/build-targets/custom-index
```

**Notes**

17 configuration tests pass; the real-storage test is intentionally ignored in this ordinary run.
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
pnpm dev:cargo -- test -p jj-store-server --test custom_data_indexes --locked --target-dir .local/build-targets/custom-index custom_indexes_preserve_pinned_history_after_real_storage_restart_and_cache_loss -- --ignored --exact --nocapture
```

**Notes**

Isolated real embedded RustFS restart, historical/latest pins, persistence restore, damage and deletion reconstruction pass. Test service cleaned up.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-wal --test workflow_store -p jj-store-rpc --test work --test collab --test review --test application_mutations --locked --target-dir .local/build-targets/custom-index
```

**Notes**

Existing WAL workflow Definition fixture was missing conditions; fixed with default conditions and rerun in final transport tests.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/docs.mjs check
```

**Notes**

New crate initially lacked documentation coverage; added dedicated topic and reviewed ownership documentation.
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
node scripts/docs.mjs check
```

**Notes**

Mermaid rendering passed; mkdocs invocation failed because uv was absent from the command PATH. Existing /Users/stevejuma/.local/bin/uv is used in the rerun.
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
node scripts/contracts.mjs --check && pnpm test:contracts
```

**Notes**

Contract fixture compilation intersected lint edits and caught wrong array conversions/ID comparison types; corrected before final verification.
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
node scripts/docs.mjs check
```

**Notes**

Documentation reference verification intersected those same intermediate compile failures; corrected before final verification.
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
pnpm install --frozen-lockfile
```

**Notes**

Workspace dependencies installed from existing lockfile.
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
pnpm dev:target:register -- --target .local/build-targets/custom-index --reason "Persistent custom data index qualification"
pnpm dev:disk
```

**Notes**

Registered isolated build root; disk admission passed and live state was preserved.
<!-- /run -->
<!-- /evidence-field -->

