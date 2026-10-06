---
schema: 1
id: E-M15-015
---

# E-M15-015 details 4

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
pnpm check:progress (after namespace reconciliation)
```

**Notes**

119 tasks; /tmp/jj-threads-rebase-progress-second.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --lib folder_picker::tests --locked
```

**Notes**

Three exact folder-picker lifecycle tests pass; /tmp/jj-threads-rebase-folder-picker-final.log
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
CARGO_INCREMENTAL=0 cargo clippy -p jj-store-cli -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

Strict CLI/HTTP all-target lint passes; /tmp/jj-threads-rebase-clippy-fixed.log
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
pnpm web:check; pnpm catalog:check (after dependency refresh)
```

**Notes**

Both checks pass with zero errors/warnings against refreshed local dependency snapshots; catalog boundary check passes for 114 modules. /tmp/jj-threads-rebase-{web,catalog}-check-refreshed.log.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/architecture.md spec/src/spec-governance.md
```

**Notes**

Formats latest parent merge resolutions and evidence. /tmp/jj-threads-rebase-docs-second-format.log.
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
pnpm check:progress (final story-reference bookkeeping initial)
```

**Notes**

Named-test parser accepts function/test definitions, not Svelte Story names. Move the verified story label to explicit stories metadata, retain file/run evidence and unchanged named unit tests; no checker or assertion is weakened. /tmp/jj-threads-rebase-progress-complete.log.
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
pnpm spec:first
```

**Notes**

/tmp/jj-threads-rebase-spec-first-complete.log
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
pnpm spec:validate
```

**Notes**

/tmp/jj-threads-rebase-spec-validate-complete.log
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
pnpm check:progress
```

**Notes**

/tmp/jj-threads-rebase-progress-fixed.log
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
pnpm check:boundaries
```

**Notes**

/tmp/jj-threads-rebase-boundaries-complete.log
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
cargo fmt -p jj-store-cli -p jj-store-http -- --check
```

**Notes**

/tmp/jj-threads-rebase-format-rust-complete.log
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
pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/milestones.md spec/src/publication.md spec/src/replication.md spec/src/spec-governance.md spec/src/verification.md
```

**Notes**

/tmp/jj-threads-rebase-format-docs-complete.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_spool.rs#bound_enqueue_obeys_shared_mutation_lock_and_preserves_queue_budget
- crates/jj-store-cli/tests/agent_capture_publication.rs#publication_wait_allows_capture_from_another_process
- crates/jj-store-cli/tests/agent_capture_publication.rs#killed_drainer_preserves_prepared_capture_and_releases_lease
- crates/jj-store-cli/tests/local_agent_recovery.rs#local_publication_releases_enqueue_lock_and_cancellation_preserves_intent
- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- crates/jj-store-cli/src/state_lease.rs#capture_lease_releases_with_an_inherited_file_description
- crates/jj-store-cli/src/credentials.rs#competing_operator_provision_cannot_replace_the_first_credential
- web/tests/navigation-storage.test.ts#navigation restores explicit collapsed
  sections and expanded projects
- crates/jj-store-cli/src/folder_picker_tests.rs#busy_timeout_and_cancellation_hold_ownership_until_reaped
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Rebase overlaps shared native/local capture spool and both drainers, shared HTTP registrations and generated browser contracts. Start with exact bound enqueue and route-inventory regressions; widen to complete spool/native/local drain consumers, bound providers, native RPC/HTTP contract packages, all captured-response contracts, web type check and direct HTTP browser controllers. Root aggregates, full Rust workspace, new Thread UI, cold browser, real RustFS and runner qualification remain separate and out of scope. Deterministic inherited-descriptor regression additionally requires the CLI unit tests and direct native/local hook capture consumers; retain the normal parallel test runner and real subprocess fixtures. The full CLI library check exposed the same inherited-file-description defect in credential provisioning; extract one CLI state lease helper, keep OS credential storage untouched and retain credential exclusion plus folder-picker subprocess library tests. The latest parent adds navigation persistence and a shared fixture/harness slice. Reuse unchanged parent sources/evidence and verify storage units plus a fresh exact navigation Storybook interaction/accessibility run, both web/catalog checks, and no visual baseline update. Final strict lint found an inherited equivalent-condition warning in folder-picker cancellation tests; rerun that exact three-test module and strict lint after its test-only formatting fix.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the independently qualified M15 slices: reducer 3, canonical timeline/search and explicit runner/JJ ACP integration; compose Thread presentation through the current released Community channel contracts when UI work begins.
<!-- /evidence-field -->

<!-- evidence-field:interrupted_runs yaml -->
## Interrupted runs

```yaml
- command: CARGO_INCREMENTAL=0 pnpm contracts:check (interrupted earlier run)
  result: interrupted
  failures: 0
  note: User interruption ended orchestration before a completion result; not
    counted as acceptance. Restarted the exact full contract lane after
    resolving latest rebase. /tmp/jj-threads-rebase-contracts-check.log.

```
<!-- /evidence-field -->

<!-- evidence-field:limitations list -->
## Limitations

- No Thread UI is implemented by this rebase. Community channel template and
  explicit refresh remain mandatory.
- Reducer 3, canonical cross-source timeline/search, explicit runner/JJ ACP
  entry points, real-RustFS and cold bound full-flow qualification remain
  pending.
<!-- /evidence-field -->

<!-- evidence-field:stories yaml -->
## Stories

```yaml
- file: catalog/components/store-work/NavigationPersistence.stories.svelte
  name: Disclosure choices survive remount
  result: pass
  scope: cold interaction/accessibility; visual-pending

```
<!-- /evidence-field -->

<!-- evidence-field:preservation_audit markdown -->
## Preservation audit

Final live audit retains all 212 latest-parent evidence entries exactly and all 14 Thread entries exactly apart from their M14-to-M15 namespace. Every rebased ancestor is conflict-free. Parent presentation sources are retained without new Thread UI implementation; manifest/companion formats and generated contracts remain unchanged by the latest UI-only parent overlay. Root aggregates, full Rust workspace, new Thread browser full-flow, real RustFS and runner qualification remain out of scope.
<!-- /evidence-field -->

