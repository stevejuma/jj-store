---
schema: 1
id: E-M12-005
---

# E-M12-005 details 2

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
jj --no-pager st
```

**Notes**

Clean child of verified local fetch slice before work.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

New fixture oid versus ObjectId reference type mismatch; fixed fixture.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Before-state proof: push is unavailable, as expected.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo check -p jj-store-wal --locked
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 23
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo check -p jj-store-cli --locked
```

**Notes**

Missing public reexports, resolved revset API adaptation and CommandError hint formatting; corrected.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Root reexport edit had missed inline list; explicit reexports corrected.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo check -p jj-store-cli --locked
```

**Notes**

Run twice while reexports were repaired; resolved revset lifetime held immutable workspace borrow through apply. Explicitly dropped validation state before mutable transaction.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

First selected native push, dry-run, original parent and stale lease passed; repeated after adding actual post-receive process kill and concurrent local advance, passed.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Expanded status JSON, native undo, checkpoint, manifest-last native copy and cold memory-store recovery passed. Recovery repairs acknowledged-unknown push without repush or rolling back advanced local bookmark.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

New transport functions followed existing test module; moved test module last.
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
cargo fmt --all && cargo test -p jj-store-service --locked --test git_bridge git_push_executor_cannot_be_spoofed_or_changed_by_another_writer -- --exact
```

**Notes**

Initial executor spoof and fabricated acknowledgement rejected; expanded another-writer check requires rerun.
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
pnpm exec prettier --write spec/src/git-interop.md spec/src/cli.md spec/src/mutations.md spec/src/jj-integration.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Spec/progress passed; protected WAL change also requires publication chapter, now updated. Boundary command did not run.
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
cargo fmt --all && cargo check -p jj-store-wal --locked
```

**Notes**

Borrowed-observation refactor attempt first stopped at a Python assertion without changes; subsequent WAL check passed. Corrected refactor and focused observation regression follow.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-cli --all-targets --locked -- -D warnings
```
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
cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact && cargo test -p jj-store-service -p jj-store-jj --locked --test git_bridge
```

**Notes**

Exact expanded executable target and 8 JJ/2 service Git scenarios passed, including another-writer event denial and causal expectation reuse after borrowed-observation refactor.
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
cargo test -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-rpc --locked
```

**Notes**

All relevant suites passed; 2 real RustFS tests ignored.
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
cargo test -p jj-store-cli --locked --test executable_native --test native_rpc --test native_lifecycle --test administration_rpc --test server_executable --test local_trusted_cli --test attachment_recovery --test workspace_migration
```

**Notes**

All 8 direct CLI consumer binaries passed; 16 executable native tests, 182 lifecycle and 7 RPC tests among them. 3 OS credential and 2 real RustFS tests ignored.
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
pnpm contracts:check
```

**Notes**

Generated HTTP/SSE contracts unchanged; 78 actual responses and 6 schema regression tests passed, TypeScript compiled.
<!-- /run -->
<!-- /evidence-field -->

