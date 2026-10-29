---
schema: 1
id: E-JJ46-WORKSPACE-001
---

# E-JJ46-WORKSPACE-001

<!-- evidence-field:id markdown -->
## Id

E-JJ46-WORKSPACE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-001
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

- crates/jj-store-cli/src/attachment.rs
- crates/jj-store-cli/src/migration.rs
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/vault_workspace/binding.rs
- crates/jj-store-jj/src/vault_workspace/initialize.rs
- crates/jj-store-jj/src/vault_workspace/storage/transition.rs
- docs/maintenance/coverage.json
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_publication.rs#observation_confirms_acceptance_without_publication_and_pending_recovery_follows_a_root_relink
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
node scripts/validation.mjs run validation:rust -- test --jobs 1 --locked -p jj-store-jj --features vault-backup --test vault_publication --test vault_adoption -- observation_confirms_acceptance_without_publication_and_pending_recovery_follows_a_root_relink stopping_an_unfinished_import_retains_the_receipt_and_never_replays_it
```

**Notes**

Both tests passed in the portable Docker worker at 4 CPU and 12 GiB with one Cargo job. This is not native qualification.
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

jj-lib 0.46.0. The workspace store file is `workspace_store/index`. `repo/index` is the commit index directory.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change. Callers open the existing jj-lib 0.46 workspace store directory.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Open the jj 0.46 workspace store at `workspace_store` during vault, cache, attachment, and migration initialization.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Required CI must pass on this head. M30 stays In progress. A Docker pass is not native qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-LOCAL-007 and JST-JJ-001 still require the same vault backup and
  native store behavior. Initialization now opens jj-lib 0.46's workspace store
  directory instead of the commit index directory.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. This portable Docker pass does not complete
  release qualification.

```
<!-- /evidence-field -->
