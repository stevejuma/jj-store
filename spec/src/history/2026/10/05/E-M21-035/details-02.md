---
schema: 1
id: E-M21-035
---

# E-M21-035 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_reword_preparation real_reword_preparation_is_unpublished -- --exact
```

**Notes**

setup-before.log: Initial inherited filter selected zero cases; no acceptance count or proof is claimed.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_reword_preparation exact_reword_preparation_retains_change_tree_and_dirty_checkout_without_publication -- --exact
```

**Notes**

setup-before-exact.log: Correct narrow existing before target: one actual reword case passed.
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
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj
```

**Notes**

setup-provider-check.log: Initial provider type check passed; subsequent retained reconstruction changes are covered by final all-target lint.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_setup_preparation actual_setup_retains_original_history_refs_and_dirty_bytes_with_unpublished_merge_wip -- --exact
```

**Notes**

setup-provider-exact.log: One red regression: fixture used a read-ignore bookmark mutation that left checkout behind the new operation. Corrected fixture initialization, restoring dirty bytes afterward.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_setup_preparation
```

**Notes**

setup-provider-repaired.log: Two cases exposed an incorrect resolved-label check. JJ represents resolved trees with one empty label; provider predicate corrected without changing the resolved-tree requirement.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_setup_preparation
```

**Notes**

setup-provider-final.log: Initial exact generation and physical/target guards pass after the resolved-label correction.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-server
```

**Notes**

setup-server-check.log: Five private-field diagnostics in the first recovery helper. Moved its implementation into the owning recovery module, retaining field privacy.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation setup::registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout -- --exact
```

**Notes**

setup-registered-exact.log: Same five provider visibility diagnostics; owning-module repair follows.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation setup::registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout -- --exact
```

**Notes**

setup-registered-repaired.log: One test tried to read a crate-private journal scope. Reused the established test scope helper rather than widening production visibility.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation setup::registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout -- --exact
```

**Notes**

setup-registered-scope-repaired.log: First actual registered/scanner/native/selected-combination setup target passes.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_setup_preparation
```

**Notes**

setup-provider-policy.log: Four generation/target/other-workspace parents passed before adding two lifecycle parents.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

setup-registered-final.log: 18 affected cases passed before the additional source-loss/GC setup parent.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_combinations --test stack_local_preparation --test stack_local_execution
```

**Notes**

setup-server-regressions.log: One existing admission transition returned Unavailable. No guard was loosened; exact compiled regression and complete family are rerun.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

setup-registered-complete.log: One existing already-published retry returned Journal(Unavailable); exact compiled regression and complete family are rerun unchanged.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_command_journal --test stack_combinations --test stack_local_preparation --test stack_local_execution
```

**Notes**

setup-server-final.log: A different existing pending-discovery case returned Unavailable during the repeated parallel run. Its exact compiled target and the complete family under serial harness scheduling are rerun; source and assertions stay unchanged.
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
target/debug/deps/stack_local_preparation-6f9fd1e099e4f734 --exact admission::authorization_stale_physical_selection_and_conflicting_identity_do_not_prepare
```

**Notes**

setup-server-policy-exact-retry.log: Exact newly failing existing admission parent passes unchanged; no scope or assertion is weakened.
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
target/debug/deps/stack_local_preparation-6f9fd1e099e4f734 --exact admission::local_execution_is_immutable_legacy_cannot_be_upgraded_and_terminal_replay_stays_terminal
```

**Notes**

setup-server-admission-exact-retry.log: Exact previously failing server admission parent passed unchanged.
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
target/debug/deps/stack_registered_preparation-9418350f0da02d0a --exact policy::already_published_original_retry_completes_checkout_after_live_policy_changes
```

**Notes**

setup-registered-policy-exact-retry.log: Exact previously failing registered policy parent passed unchanged.
<!-- /run -->
<!-- /evidence-field -->

