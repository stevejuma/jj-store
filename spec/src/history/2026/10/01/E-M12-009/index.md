---
schema: 1
id: E-M12-009
---

# E-M12-009

<!-- evidence-field:id markdown -->
## Id

E-M12-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.5
- M12.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-recovery-ref-merge-and-stage-faults
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change pxpqtlyw.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Normal push concurrent-fetch test passed. New cold recovery test failed at cache permissions, then (after private-cache repair) proved older result rolls back later remote target.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1;gix 0.87.1;Git 2.55.0;Rust 1.98.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing manifest4 and GitV1 unchanged. Frozen native source view proves exact lease base; no extra intent schema.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/git_bridge/configuration.rs
- crates/jj-store-cli/src/git_bridge/device/approval.rs
- crates/jj-store-cli/src/git_bridge/device/tests.rs
- crates/jj-store-cli/src/git_bridge/device.rs
- crates/jj-store-cli/src/git_bridge/fetch.rs
- crates/jj-store-cli/src/git_bridge/push/apply.rs
- crates/jj-store-cli/src/git_bridge/push/attempts.rs
- crates/jj-store-cli/src/git_bridge/push/prepare.rs
- crates/jj-store-cli/src/git_bridge/push/selection.rs
- crates/jj-store-cli/src/git_bridge/push.rs
- crates/jj-store-cli/src/git_bridge/transport/network.rs
- crates/jj-store-cli/src/git_bridge/transport.rs
- crates/jj-store-cli/src/git_bridge.rs
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-jj/tests/git_bridge/faults.rs
- crates/jj-store-jj/tests/git_bridge.rs
- spec/src/cli.md
- spec/src/git-interop.md
- spec/src/jj-integration.md
- spec/src/publication.md
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_apply_preserves_a_concurrent_fetched_remote_advance -- --exact
```

**Notes**

Before: normal stale-helper push finish retains fetched descendant through native operation merge.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_recovery_preserves_a_later_observation_from_another_binding -- --exact
```

**Notes**

First invocation matched zero cases after fixture edit script failed; not acceptance evidence.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_recovery_preserves_a_later_observation_from_another_binding -- --exact
```

**Notes**

Interrupted private attempt cache was created with public permissions. Explicit owner-only creation fixes safe crash cleanup.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_recovery_preserves_a_later_observation_from_another_binding -- --exact
```

**Notes**

Recovery replaces External advance with Second feature. Repaired using frozen native remote base and pinned ref merge.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_recovery_preserves_a_later_observation_from_another_binding -- --exact
```

**Notes**

Exact recovery regression passes; original Git endpoint unchanged, later native observation retained, no repush.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_fault_tests -- --nocapture
```

**Notes**

Before/after manifest-write unknowns at all five durable stages; cold stable replay, one increment/receipt, code unchanged and checkpoint recovery pass.
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
cargo fmt --all && cargo check -p jj-store-cli --locked
```

**Notes**

Separated lifecycle/fetch, selection/preparation, apply/attempts, admission and ref exchange into production modules below300 lines. No new interfaces or persistence format.
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
cargo test -p jj-store-cli --locked --lib --test executable_native --test native_rpc --test workspace_migration && cargo test -p jj-store-jj --locked --test git_bridge && cargo clippy -p jj-store-cli -p jj-store-jj --all-targets --locked -- -D warnings
```

**Notes**

7 library,20 executable,7 RPC,1 migration and13 JJ Git cases pass. Clippy found preparation helper eight-argument boundary; grouped private input parameters.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo clippy -p jj-store-cli -p jj-store-jj --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/git-interop.md spec/src/cli.md spec/src/jj-integration.md spec/src/publication.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Historical device-test named pointers moved during module extraction; updated pointers to actual owning file while preserving original commands.
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
pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

94 tasks,16 protected files/5 chapters,15 package boundaries pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_push_apply_preserves_a_concurrent_fetched_remote_advance
- crates/jj-store-cli/tests/executable_native.rs#native_git_recovery_preserves_a_later_observation_from_another_binding
- crates/jj-store-jj/tests/git_bridge/faults.rs#git_stage_manifest_unknown_outcomes_cold_replay_without_duplicate_effects
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

GB4 durable stage faults, owned hard cancellation, local/remote observation concurrency and data-only peer replay are qualified together with E-M12-005/006/008. GB5 dependency now closes. Continue required GB6 credential/resource and cold real-RustFS external merge/browser gates; no complete M12 claim.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


