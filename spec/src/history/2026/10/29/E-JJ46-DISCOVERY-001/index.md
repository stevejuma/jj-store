---
schema: 1
id: E-JJ46-DISCOVERY-001
---

# E-JJ46-DISCOVERY-001

<!-- evidence-field:id markdown -->
## Id

E-JJ46-DISCOVERY-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-DOC-001
- JST-DOC-002
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

- crates/jj-store-jj/src/vault_workspace/adoption/source.rs
- docs/maintenance/coverage.json
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_adoption.rs#adopts_recorded_git_ancestry_without_touching_source_metadata_or_saved_files
- crates/jj-store-jj/tests/vault_adoption.rs#rejects_a_substituted_source_pin_and_does_not_initialize_during_discovery
- crates/jj-store-jj/tests/vault_adoption.rs#startup_capture_recovers_prepared_or_published_adoption_before_recording_later_edits
- crates/jj-store-jj/tests/vault_adoption.rs#stopping_an_unfinished_import_retains_the_receipt_and_never_replays_it
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
pnpm dev:cargo -- test --locked -p jj-store-jj --features vault-backup --test vault_adoption -- --test-threads=8
```

**Notes**

History discovery opens a stock jj 0.45.1 checkout through the loader that does not write `.jj/repo/workspace_store/type`. 4 tests passed, including the discovery immutability assertion.
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

jj-lib 0.46.0 opened a stock jj 0.45.1 repository.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change. Discovery leaves the source checkout unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Keep history discovery from writing the jj-lib 0.46 workspace-store type file.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Required CI must pass on this head. M30 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-LOCAL-007 still requires vault reads to remain immutable. Discovery
  now uses the existing simple workspace store and does not write the
  workspace-store type file.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. This discovery read does not complete release
  qualification.

```
<!-- /evidence-field -->
