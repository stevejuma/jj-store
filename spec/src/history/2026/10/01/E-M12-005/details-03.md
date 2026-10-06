---
schema: 1
id: E-M12-005
---

# E-M12-005 details 3

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
pnpm exec prettier --write spec/src/git-interop.md spec/src/cli.md spec/src/mutations.md spec/src/jj-integration.md spec/src/publication.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

20 chapters/123 requirements;94 tasks;13 protected files/6 owning chapters;15 boundaries.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write docs/cli/commands.md spec/src/milestones.md && cargo fmt --all --check && cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Test default selector edit required rustfmt; formatter corrected. Clippy did not run in this command.
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
cargo fmt --all && cargo test -p jj-store-service --locked --test git_bridge git_push_executor_cannot_be_spoofed_or_changed_by_another_writer -- --exact
```

**Notes**

New unselected-operation admission regression failed before fix; first repair then failed once on virtual root loading. Added explicit pinned virtual roots without weakening admission.
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
cargo fmt --all && cargo test -p jj-store-service --locked --test git_bridge git_push_executor_cannot_be_spoofed_or_changed_by_another_writer -- --exact
```

**Notes**

Accepted-ancestry admission, executor authority and immutable frozen bookmark proof pass.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact && cargo test -p jj-store-jj -p jj-store-service -p jj-store-rpc --locked --test git_bridge
```

**Notes**

Exact interrupted push/cache-loss recovery and 8 JJ, 2 service, 1 RPC Git scenarios passed after source proof.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked -- -D warnings
```
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
cargo test -p jj-store-jj -p jj-store-service --locked && cargo test -p jj-store-cli --locked --test executable_native --test native_rpc
```

**Notes**

Escalated because source admission changes the shared JJ/service security boundary and direct CLI consumers: affected JJ/service suites, 16 executable and 7 RPC scenarios pass; 2 real RustFS and 3 OS-credential scenarios ignored.
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
pnpm exec prettier --write spec/src/git-interop.md spec/src/mutations.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

15 protected files/6 chapters; 94 tasks; all governance gates passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush
- crates/jj-store-service/tests/git_bridge.rs#git_push_executor_cannot_be_spoofed_or_changed_by_another_writer
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

GB4 data-only Git origins and code activation; then GB5 selector/remote/multi-ref parity and GB6 real RustFS, credentials, process lifetime and browser freshness. All remain required before M12 completion.
<!-- /evidence-field -->

