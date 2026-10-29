---
schema: 1
id: E-JJ46-COMPILE-001
---

# E-JJ46-COMPILE-001

<!-- evidence-field:id markdown -->
## Id

E-JJ46-COMPILE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-001
- JST-GIT-001
- JST-ARCH-006
- JST-LOCAL-001
- JST-ADAPT-001
- JST-OBJ-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-capture/src/cursor_binding/pending.rs
- crates/jj-store-cli/examples/upstream_custom_backend.rs
- crates/jj-store-cli/src/attachment.rs
- crates/jj-store-cli/src/git_bridge/push/prepare.rs
- crates/jj-store-cli/src/git_bridge/push/selection.rs
- crates/jj-store-cli/src/migration.rs
- crates/jj-store-cli/src/stack_commands/context.rs
- crates/jj-store-cli/tests/attachment_recovery.rs
- crates/jj-store-cli/tests/local_agent_provenance/fixture.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_details.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-cli/tests/local_trusted_cli.rs
- crates/jj-store-cli/tests/native_collab_cli.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/commands.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/content_admission.rs
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs
- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_filters.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs
- crates/jj-store-cli/tests/native_rpc.rs
- crates/jj-store-cli/tests/stack_checkout_plans/candidate.rs
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs
- crates/jj-store-cli/tests/stack_checkout_plans/coordination_unchanged.rs
- crates/jj-store-git/Cargo.toml
- crates/jj-store-jj/src/agent_workspaces/ignores.rs
- crates/jj-store-jj/src/agent_workspaces/inspection.rs
- crates/jj-store-jj/src/backend.rs
- crates/jj-store-jj/src/codec.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/conversion.rs
- crates/jj-store-jj/src/dependencies.rs
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/git_bridge/mod.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/code_index/blame_probe.rs
- crates/jj-store-jj/src/local_browser/git_read/extras.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation/visibility.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-jj/src/local_browser/working_tree/current/capture.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/src/local_evidence/operation_archive/capture.rs
- crates/jj-store-jj/src/local_evidence/views.rs
- crates/jj-store-jj/src/local_execution/recovery/extras/materialize.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes/materialize.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes/verify.rs
- crates/jj-store-jj/src/local_execution/recovery/publication.rs
- crates/jj-store-jj/src/local_execution/recovery/records.rs
- crates/jj-store-jj/src/local_execution/reword.rs
- crates/jj-store-jj/src/local_repository.rs
- crates/jj-store-jj/src/local_stacks/hunks.rs
- crates/jj-store-jj/src/local_stacks/policy.rs
- crates/jj-store-jj/src/local_stacks/settings.rs
- crates/jj-store-jj/src/policy.rs
- crates/jj-store-jj/src/query/blame.rs
- crates/jj-store-jj/src/query/blame/tests.rs
- crates/jj-store-jj/src/query/blame_probe.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/cache/constructed.rs
- crates/jj-store-jj/src/query/code_index.rs
- crates/jj-store-jj/src/query/code_index/native.rs
- crates/jj-store-jj/src/query/content/native.rs
- crates/jj-store-jj/src/query/content/references/filter.rs
- crates/jj-store-jj/src/query/content/references/prepared.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/segment.rs
- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/src/query/projection/decorations.rs
- crates/jj-store-jj/src/query_workspace.rs
- crates/jj-store-jj/src/vault_workspace/ai_undo.rs
- crates/jj-store-jj/src/vault_workspace/binding.rs
- crates/jj-store-jj/src/vault_workspace/initialize.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/restore_io.rs
- crates/jj-store-jj/src/vault_workspace/storage/transition.rs
- crates/jj-store-jj/tests/compatibility.rs
- crates/jj-store-jj/tests/conversion.rs
- crates/jj-store-jj/tests/git_bridge.rs
- crates/jj-store-jj/tests/local_evidence.rs
- crates/jj-store-jj/tests/local_stacks/topology.rs
- crates/jj-store-jj/tests/native_codec.rs
- crates/jj-store-jj/tests/native_inventory.rs
- crates/jj-store-jj/tests/stack_operation_archive.rs
- crates/jj-store-jj/tests/stack_operation_archive/adversarial.rs
- crates/jj-store-jj/tests/stack_operation_archive/history.rs
- crates/jj-store-jj/tests/stack_preparation_retention.rs
- crates/jj-store-jj/tests/stack_source_recovery/extras.rs
- crates/jj-store-jj/tests/stack_source_recovery/indexes.rs
- crates/jj-store-jj/tests/stack_tree_checkout.rs
- crates/jj-store-jj/tests/vault_backup.rs
- crates/jj-store-jj/tests/working_tree_baseline.rs
- crates/jj-store-jj/tests/working_tree_current.rs
- crates/jj-store-tantivy/src/disk.rs
- crates/jj-vault/src/search/cache.rs
- crates/jj-vault/src/search/indexed/native.rs
- crates/jj-vault/src/search/indexed/query.rs
- crates/jj-vault/src/search/mod.rs
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
node scripts/managed-cargo.mjs clippy --locked --all-targets -p jj-store-jj -p jj-store-git -p jj-store-tantivy -p jj-vault -p jj-store-object -p jj-store-http -p jj-store-service -p jj-store -p jj-store-capture -- -D warnings
```

**Notes**

Clippy denied warnings and accepted the jj-lib 0.46 call updates, the gix 0.88 Git crate, Tantivy segment paths, and digest hex encoding. No stored format or signature change.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-lib 0.46.0, jj-cli 0.46.0, gix 0.88.0, sha2 0.11, tantivy 0.27. Lapis AI stays the v0.2.0 git tag and lapis-agent-redaction stays lapis-agent-redaction@0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change. Commit, bookmark, SigV4, and cursor bytes stay the same.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Compile jj-store-jj and the CLI against the jj-lib 0.46 interfaces introduced by this Cargo rollup, and keep the published git-tag pins from main.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Required CI must pass on this head. M30 stays In progress; this compile fix does not qualify release.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-JJ-001, JST-GIT-001, JST-ARCH-006, JST-LOCAL-001, JST-ADAPT-001, and
  JST-OBJ-001 keep the same behavior. The edits follow renamed jj-lib 0.46
  interfaces and digest encoding only.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. The compile fix does not complete Cargo, native,
  Pages, or GHCR release qualification.

```
<!-- /evidence-field -->

