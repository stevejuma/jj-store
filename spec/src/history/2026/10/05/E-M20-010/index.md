---
schema: 1
id: E-M20-010
---

# E-M20-010

<!-- evidence-field:id markdown -->
## Id

E-M20-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
- M20.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zwssuxyr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

owned-stack-command-contract-foundation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; pnpm 10.32.1; integration parent 1bc03153
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned service protocol version 1. No HTTP endpoint, generated transport, native object codec, companion ref, WAL or private device storage format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/stack_workspace/command.rs
- crates/jj-store-service/src/stack_workspace/content.rs
- crates/jj-store-service/src/stack_workspace/mod.rs
- crates/jj-store-service/src/stack_workspace/prepared.rs
- crates/jj-store-service/src/stack_workspace/receipt.rs
- crates/jj-store-service/src/stack_workspace/request.rs
- crates/jj-store-service/src/stack_workspace/selection.rs
- crates/jj-store-service/tests/stack_workspace_contracts.rs
- spec/src/api.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Approved integrated UI has only numeric mock revisions; no service stacks protocol module. Exact regression initially fails to import the absent module.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Define exact physical/code/combination selection, explicit native/Git identities, ordered merged-tree terms, server-retained file/hunk claims, typed intent digest, exact preparation/output effects and apply binding, and receipt distinction for unknown publication/incomplete checkout/retirement. This establishes validated records; it does not implement provider execution or advertise capabilities.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_intent_binds_all_execution_preconditions
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_wire_rejects_foreign_fields_and_identity_profiles
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_exact_selections_bind_source_parent_and_file_state
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_apply_names_exact_preparation_not_a_new_command
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_receipt_separates_publication_checkout_and_retirement
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_preparation_identity_binds_exact_published_output
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_setup_is_explicit_and_commands_are_bounded
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_wire_schema_keeps_separate_code_profiles
- crates/jj-store-service/tests/stack_workspace_contracts.rs#local_stack_native_tree_selection_retains_ordered_conflict_terms
<!-- /evidence-field -->

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
cargo test -p jj-store-service --test stack_workspace_contracts --locked
```

**Notes**

Before: module was absent; expected unresolved import. contracts-before.log.
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
cargo test -p jj-store-service --test stack_workspace_contracts --locked
```

**Notes**

Initial five regressions, expanded to eight (contracts-after.log) then nine including ordered native conflict trees (contracts-final-tests.log). Final 9/9 pass.
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
cargo check -p jj-store-service --all-targets --locked
```

**Notes**

Affected package and all targets compile; contracts-service-check.log. Final repeat after ordered tree extension: contracts-service-check-final.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

Unchanged dependency diagnostics: jj-store-jj git_read/extras.rs manual_range_patterns, references/prepared.rs map_entry, query/cache/resident.rs and query/index.rs too_many_arguments. contracts-clippy.log. None of these files differs from the integration parent.
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
cargo clippy -p jj-store-service --all-targets --locked --no-deps -- -D warnings
```

**Notes**

Unchanged service reads.rs OpenRead large_enum_variant; file matches parent. contracts-service-clippy.log. No lint suppression or baseline change; package check and exact contract regressions pass, Clippy is not claimed passing.
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
cargo fmt -p jj-store-service
```

**Notes**

Changed Rust source and focused tests formatted; no unrelated Rust changes.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries pass; contracts-boundaries.log.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/mutations.md spec/src/api.md spec/src/spec-governance.md
```

**Notes**

Changed specification prose only; ledger never formatted.
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
pnpm spec:first
```

**Notes**

9 protected files and 4 canonical chapters; contracts-spec-first-final.log.
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
pnpm spec:validate
```

**Notes**

5 validators, 23 chapters, 173 requirements; contracts-spec-validate-final.log.
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
pnpm check:progress
```

**Notes**

142 tasks and new contract evidence pass; contracts-progress.log.
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
python3 .local/stacks-backend-evidence/verify-contract-ledger.py
```

**Notes**

481 historical evidence objects remain byte-identical.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Production adapter/bounded reads and generated executable HTTP contracts
  remain to implement.
- Durable preparation/publication/checkout journal and restart qualification
  remain required before mutation controls.
- Named combinations, exact staging/commits, graph actions, linked native
  checkout and real-app lifecycle qualification remain active scope.
- Five inherited Clippy diagnostics remain unresolved in unchanged read/index
  files; no complete Clippy pass is claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Deliver bounded operation-pinned stack reads and lazy production adapter/controller injection, then qualify durable command execution. Continue M21.2; this contract foundation is not feature completion.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Root aggregates, full Rust workspace suites, unchanged UI reruns, package publication, installed-service changes, screenshot comparisons and visual baseline updates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


