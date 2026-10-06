---
schema: 1
id: E-M12-004
---

# E-M12-004 details 2

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
cargo check -p jj-store-cli --locked
```

**Notes**

Scoped native factory registry; then device/cache/transport/fetch modules compiled.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Fixture used jj store project instead of existing top-level jj project; correct fixture spelling, no production contract change.
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
pnpm exec prettier --write spec/src/cli.md spec/src/git-interop.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Spec validated; ledger pending entry lacked actual runs while new test was running. Added actual commands/results; later commands did not run.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Corrected fixture exposed no-op transaction defect: unchanged tracked refs re-added redundant heads. Reproduced twice, corrected by processing only changed refs as pinned JJ does.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Temporary diagnostic borrowed workspace while transaction owns mutable borrow; diagnostic removed.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Original cache-loss/no-op target passed after changed-ref processing fix.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Expanded tag deletion uses RefName without Display; corrected to as_str.
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
cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Previous resumed slice pass; final expanded tests require rerun.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Expanded force rewrite/native conflict, filtered literal deletion, omitted ref retention, full deletion and annotated tag raw/peeled identities passed.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --lib git_bridge
```

**Notes**

3 focused endpoint, private path and cache resource tests passed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --locked --test executable_native --test native_rpc --test work_cli
```

**Notes**

No work_cli test target exists. No tests executed; corrected to full CLI package suite for shared factory plumbing.
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
pnpm exec prettier --write spec/src/cli.md spec/src/git-interop.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

20 chapters/123 requirements;94 tasks;11 protected files/5 chapters;15 package boundaries.
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
cargo test -p jj-store-service -p jj-store-rpc --locked --test git_bridge
```

**Notes**

Both focused service/RPC consumers passed with activation availability state.
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
cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

New test compared an allocated tag OID; fixed by deriving canonical OID strings once.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/git-interop.md spec/src/milestones.md && cargo fmt --all --check
```
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
cargo test -p jj-store-cli --locked
```

**Notes**

All CLI suites passed, including 15 executable tests, 182 native lifecycle tests, 7 native RPC tests, ordinary Git/help/completions, WorkStore, attachment/migration and agent/review consumers. 3 OS credential and 3 real RustFS tests ignored; this slice is memory/loopback/Unix evidence.
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
cargo test -p jj-store-cli --locked --test executable_native native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss -- --exact
```

**Notes**

Final source after Clippy cleanup; 1/1 expanded acceptance passed.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries && cargo fmt --all --check
```
<!-- /run -->
<!-- /evidence-field -->

