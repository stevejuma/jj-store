---
schema: 1
id: E-M12-007
---

# E-M12-007

<!-- evidence-field:id markdown -->
## Id

E-M12-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-bookmark-selection-and-remote-lifecycle
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change szlvumzz.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact new selector fixture failed at the single-ref capability gate after two fixture compile errors (nonexistent endpoint/permit API). Implementation checkpoint compiled only.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1;gix 0.87.1;Git 2.55.0;Rust 1.98.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest 4;Git records V1;conversion profile jj-git-sha1-0.45.1-v1;no schema change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/git_bridge.rs
- crates/jj-store-cli/src/git_bridge/device.rs
- crates/jj-store-cli/src/git_bridge/push.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/tests/executable_native.rs
- spec/src/git-interop.md
- spec/src/cli.md
- docs/cli/commands.md
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
cargo fmt --all && cargo check -p jj-store-cli --locked
```

**Notes**

Inherited checkpoint compile only.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_bookmark_selectors_remote_lifecycle_and_atomic_push_preserve_scope -- --exact
```

**Notes**

First executable run passes multi-ref lease rollback, selectors, generated dry-run/native source proof, tag retention, rename identity and remove/recreate. Expanded negative cases rerun pending.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_bookmark_selectors_remote_lifecycle_and_atomic_push_preserve_scope -- --exact
```

**Notes**

Expanded remote atomic-capability refusal, unsupported tag/options/colliding generated name and removed-remote no-contact recovery pass.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --lib --test executable_native --test native_rpc --test workspace_migration && cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

6 library, 17 executable, 7 RPC and 1 migration cases pass; 3 OS credential tests ignored. CLI-only relevant suites; unrelated Rust/UI lanes out of scope.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_bookmark_publication_matches_pinned_upstream_scope -- --exact
```

**Notes**

Native versus pinned ordinary JJ path agrees on selected/named/deleted refs, dry-run and rename/remove preservation.
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
pnpm exec prettier --write spec/src/git-interop.md spec/src/cli.md spec/src/milestones.md docs/cli/commands.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

123 requirements,94 tasks,6 protected files/3 chapters and15 package boundaries pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_bookmark_selectors_remote_lifecycle_and_atomic_push_preserve_scope
- crates/jj-store-cli/tests/executable_native.rs#native_git_bookmark_publication_matches_pinned_upstream_scope
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

GB5 behavior qualified; M12.6 stays In progress until dependency M12.5 fault/cancellation gate completes. Continue owned transport lifetime and GB6 credentials/resource/cold real-RustFS/browser cycle.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


