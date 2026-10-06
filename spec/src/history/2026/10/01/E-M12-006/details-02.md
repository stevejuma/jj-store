---
schema: 1
id: E-M12-006
---

# E-M12-006 details 2

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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_activation_requires_owned_validation_and_retains_selected_facts -- --exact
```

**Notes**

Exact gate regression passes with owned validator; default WAL validator still fails closed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origins_keep_causal_conflicts_replay_and_cold_copy_without_transport -- --exact
```

**Notes**

New fixture adapted expectation return type/binding and checkpoint signature; corrected qualified module path before running actual target.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests::git_origins_keep_causal_conflicts_replay_and_cold_copy_without_transport -- --exact
```

**Notes**

Causal conflicts, reversed batch delivery, duplicate replay, checkpoint/cold-copy and reducer negotiation pass with no transport.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Converted fixture to a code repository with active origin ledger; real Git fetch/edit/push, leases, interruption and recovery pass.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests -- --nocapture
```

**Notes**

New legacy-origin fixture used nonexistent TokenChanged variant; changed to existing HeadsNormalized kind.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests -- --nocapture
```

**Notes**

Format-4 seed hydration passed; legacy-envelope fixture used HeadsNormalized without its required transition. Corrected fixture to an explicit exact-code-state transition.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests -- --nocapture
```

**Notes**

3 origin scenarios passed: causal/duplicate/cold-copy, malicious mapping/frozen targets/staged sources, format-4 seed/legacy digest shape.
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
cargo test -p jj-store-jj --locked --test git_bridge && cargo test -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --locked
```

**Notes**

All owning/shared suites passed (12 JJ Git tests plus WAL, replication, service and RPC). Concrete escalation rationale recorded before execution.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/git-interop.md spec/src/publication.md spec/src/jj-integration.md spec/src/replication.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Spec/progress passed; protected consumer test/RPC changes also require cli.md and mutations.md, now amended. Boundary check did not run.
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
cargo fmt --all && cargo test -p jj-store-rpc --locked --test replication authenticated_git_origin_transfer_is_data_only_and_emits_committed_freshness -- --exact
```

**Notes**

Actual authenticated RPC Git control-record transfer, receipt replay, no code movement, committed SSE hint and adapter cursor passed.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native --test native_rpc --test workspace_migration && cargo test -p jj-store-rpc --locked --test replication
```

**Notes**

16 executable native, 7 native RPC, migration and all 3 replication RPC scenarios passed; 3 OS-credential tests ignored.
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
cargo fmt --all && cargo clippy -p jj-store-wal -p jj-store-jj -p jj-store-replication -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Collapsible nested Git dependency check; corrected to a let chain.
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
pnpm contracts:check
```

**Notes**

Generated contracts unchanged; 78 actual HTTP/SSE responses, 6 schema regressions and TypeScript pass.
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
cargo fmt --all && cargo test -p jj-store-service --locked --test git_bridge git_push_executor_cannot_be_spoofed_or_changed_by_another_writer -- --exact
```

**Notes**

New review regression proved bundled started/acknowledged events bypassed previously selected predecessor ordering. Added service and owned normal-origin event admission.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-service --locked --test git_bridge git_push_executor_cannot_be_spoofed_or_changed_by_another_writer -- --exact && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests -- --nocapture
```

**Notes**

Exact ordering regression and all origin scenarios pass after admission fix.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests::format_four_seed_retains_git_and_legacy_origin_digest_shape -- --exact
```

**Notes**

Expanded seed retains unknown attempt and its source after native undo, without resurrecting the old code head.
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
cargo fmt --all && cargo test -p jj-store-jj --locked --test git_bridge git_origin_tests -- --nocapture && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Normal causal prepare/start/unknown-result origins survive reversed delivery and cold copy. Code push interrupted recovery still passes with event admission.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo clippy -p jj-store-wal -p jj-store-jj -p jj-store-replication -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Default capability fixture field reassignment rejected; corrected to explicit initializer.
<!-- /run -->
<!-- /evidence-field -->

