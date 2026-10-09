---
schema: 1
id: E-CHAT-WORKSPACE-001
---

# E-CHAT-WORKSPACE-001 details 2

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
pnpm contracts:check
```

**Notes**

Inspection DTO generation matches; 300 actual HTTP/SSE responses conform, seven contract validation tests and the TypeScript contract check pass. Later review-target schema changes are a separate slice.
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
pnpm spec:first
```

**Notes**

Inspection slice passes: 26 protected files and seven owning chapters.
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

Seven document validators pass.
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
pnpm check:progress
```

**Notes**

Final named-test references and changed-file coverage pass.
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
pnpm spec:first
```

**Notes**

Owning local read chapter also required by protected-change mapping; added explicit workspace inspection semantics.
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
cargo check --locked -p jj-store-server -p jj-store-http
```

**Notes**

Final typed-reader cache and incomplete-response changes compile.
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
cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

Typed-reader flow passes: one test in 155.97 seconds including Unicode, frozen Last turn, stale cursors and denied foreign bindings.
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
cargo check --locked -p jj-store-server -p jj-store-http
```

**Notes**

Initial implementation: temporary borrow E0716, then module-name shadowing; corrected in owning source.
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
cargo test --locked -p jj-store-service --test agent_workspace_files
```

**Notes**

Two tests pass, repeated after current-list changes: metadata/path exclusion, ignored and unchanged files, symlinks and request rejection.
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
JJ_STORE_AGENT_TEST_JJ=$PWD/target/debug/jj cargo test --locked -p jj-store-jj --test agent_workspaces inspection_reads_exact_revisions_without_snapshotting_dirty_files -- --ignored --exact
```

**Notes**

Preliminary fixed-executable reader passed. Replaced by typed readers during API contract review; this superseded implementation is not final acceptance.
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
cargo check --locked -p jj-store-server -p jj-store-http
```

**Notes**

Passed after queue integration and again after typed native/local readers.
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
cargo test --locked -p jj-store-server --lib queued_turns_wait_for_the_exact_active_completion_and_cancel_without_dispatch
```

**Notes**

One queue unit test passes. macOS linker reports large __eh_frame warning.
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
cargo test --locked -p jj-store-server --lib latest_completed_turn_requires_its_own_exact_repository_boundaries
```

**Notes**

One exact-boundary test passes; later incomplete turns cannot borrow earlier snapshots.
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
cargo test --locked -p jj-store-http --lib agent_execution::tests::workspace_inspection_is_local_admin_and_accepts_no_filesystem_root
```

**Notes**

One Admin/path-admission test passes.
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
cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

First run mistook pre-dispatch snapshot sequence growth for completed execution. Strengthened fixture to wait for exact per-agent completion receipts.
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
cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

Receipt-fenced flow passes on preliminary reader: current Unicode edits/untracked files, frozen Last turn, Since last commit, deleted files and unchanged checkout. Typed-reader rerun follows separately.
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
cargo test --locked -p jj-store-cli --test agent_execution
```

**Notes**

Escalated to both controller consumers because shared dispatch timing changed: 10 pass, one pre-existing tooling label assertion fails (JJ Agent versus jj-agent), two installed-provider cases intentionally ignored. Corrected only the stale runner display-label assertion; installer label and receipt assertions remain intact.
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
pnpm contracts:generate
```

**Notes**

Generated OpenAPI and TypeScript initially and again with the explicit incomplete-comparison reason.
<!-- /run -->
<!-- /evidence-field -->

