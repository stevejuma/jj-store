---
schema: 1
id: E-M21-040
---

# E-M21-040 details 4

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
pnpm docs:check
```

**Notes**

docs-check-final.log: strict source reviews, parser references, nine locally rendered diagrams and valid 139-file documentation closure.
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
pnpm check:progress
```

**Notes**

progress-initial.log: all 142 task rows validate; enclosing change evidence covers all modified files and removed historical test name.
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
pnpm spec:first
```

**Notes**

spec-first-final.log: final mapped gate passes 44 protected files and 11 canonical chapters.
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
pnpm spec:validate
```

**Notes**

spec-validate-final.log: 5 validators, 23 chapters and 173 requirements pass after core evidence/verification updates.
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
pnpm contracts:check
```

**Notes**

contracts-check.log: all 267 actual registered HTTP/SSE responses conform, including six real ordinary Stacks states; 7 contract-tool tests and direct generated TypeScript contract check pass. Complete existing response inventory ran because the endpoint/contract inventory changed.
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
cargo test -p jj-store-http --lib --locked local_browser::stacks::tests::route_admits_only_registered_local_generations_and_typed_commands
```

**Notes**

http-admission-final.log: 1/1 exact registered local-generation, typed path-free body and foreign-origin admission regression.
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
pnpm check:progress
```

**Notes**

progress-final.log: all 142 tasks and current same-change file/test evidence pass after full contract closure.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Commit/file/hunk staging and transfer,
  create/reword/reorder/split/squash/delete graph/menu commands and scoped undo
  remain capability-disabled, pending review of this checkpoint.
- Linked-native/manifest publication is outside ordinary local Stacks. Linked
  physical stock-JJ checkouts are qualified separately by native
  sparse/local-config tests.
- Historical retained-executor modules and evidence remain in source for their
  separate profile and shared scan dependencies; the new production local path
  does not select or depend on them. Wholesale historical-code deletion is not
  part of this core checkpoint.
- Atomic TOML replacement is not a cross-file JJ transaction. Applied operations
  with checkout/settings failures return warnings; unknown transport outcomes
  require state/op-log inspection and never automatic command retry.
- Production source and a cold built UI are verified; existing Storybook 6057
  remains a mock demo. No installed service or package was replaced. Visual
  baselines and package publication remain excluded.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Pause for implementation review after this verified core is committed. Continue whole-file/commit, then server-issued hunks and remaining graph/menu actions only after explicit approval.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

The usable ordinary-local core passes 7 real jj-lib tests, 49 direct UI units, 28 Storybook interaction/accessibility stories, 1 cold built production browser flow, 6 prior bounded-reader tests and the expanded ordinary/prepared reference regression. All 267 actual HTTP/SSE samples, 7 contract-tool tests, type/build/lint/ownership/architecture/documentation/spec checks pass. 521 previous raw evidence bodies are byte-identical. M21/M21.2 remain In progress; pause before later command engines. Historical retained execution remains source only, outside the new production path.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#megamerge_membership_preserves_dirty_work_and_named_workspace_switching
- crates/jj-store-jj/tests/local_stacks.rs#stale_commands_are_rejected_and_refresh_tracks_external_edits
- crates/jj-store-jj/tests/local_stacks.rs#malformed_toml_is_an_error_and_does_not_replace_saved_settings
- crates/jj-store-jj/tests/local_stacks.rs#cli_branch_advancement_refreshes_merge_and_retains_wip
- crates/jj-store-jj/tests/local_stacks.rs#megamerge_conflicts_remain_native_jj_conflicts
- crates/jj-store-jj/tests/local_stacks.rs#settings_save_failure_reports_the_applied_jj_operation_without_replay
- crates/jj-store-jj/tests/local_stacks.rs#linked_physical_workspaces_keep_separate_toml_and_honor_native_sparse_checkout
- crates/jj-store-http/src/local_browser/stacks/tests.rs#route_admits_only_registered_local_generations_and_typed_commands
- crates/jj-store-cli/tests/local_browser/prepared_references.rs#prepared_reference_windows_seek_visible_changes_before_hydrating_targets
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/real/stacks-browser.spec.ts#cold_local_megamerges_apply_unapply_and_named_wip_survive_reload
- web/tests/vite/production-stack-adapter.test.ts#confirmed local membership
  settles before the new operation graph index is prepared
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- web/tests/real/stacks-browser.spec.ts#cold_registered_stacks_browse_real_ancestors_files_and_diffs_without_writes
<!-- /evidence-field -->

