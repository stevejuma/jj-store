---
schema: 1
id: E-M12-008
---

# E-M12-008

<!-- evidence-field:id markdown -->
## Id

E-M12-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.5
- M12.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-owned-process-lifetime
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lnwolwuz.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Hard-kill regression failed once: owned post-receive Git group survives SIGKILL of invoking CLI. Fixture cleans its disposable group on failure.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1;gix 0.87.1;Git 2.55.0;Rust 1.98.1;rustix 1.1.5 existing dependency pin
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native schema/profile change. Invocation-local supervisor argv and non-secret lifetime pipe.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/main.rs
- crates/jj-store-cli/src/git_bridge.rs
- crates/jj-store-cli/src/git_bridge/watchdog.rs
- crates/jj-store-cli/src/git_bridge/transport.rs
- crates/jj-store-cli/Cargo.toml
- Cargo.lock
- crates/jj-store-cli/tests/executable_native.rs
- spec/src/git-interop.md
- spec/src/cli.md
- spec/src/architecture.md
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Before: owned transport survives hard CLI cancellation.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --test executable_native native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush -- --exact
```

**Notes**

Supervisor kills group after CLI hard kill; attempt recovers after cache deletion while preserving concurrent local advance.
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
cargo fmt --all && cargo check -p jj-store-cli
```

**Notes**

Safe non-reaping wait API requires WaitId::Pid. Initial pin accidentally downgraded resolved rustix; restored existing 1.1.5 before acceptance.
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
cargo check -p jj-store-cli
```

**Notes**

Final non-reaping cleanup compiles using existing rustix 1.1.5; group leader PID is retained until cleanup finishes.
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
cargo fmt --all && cargo test -p jj-store-cli --locked --lib --test executable_native --test native_rpc --test workspace_migration && cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Final safe process cleanup:7 library,18 executable,7 RPC,1 migration cases pass;3 unrelated OS-credential cases ignored. Clippy passes.
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
pnpm exec prettier --write spec/src/git-interop.md spec/src/cli.md spec/src/architecture.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

123 requirements,94 tasks,7 protected files/4 chapters and15 package boundaries pass. Other Rust/UI aggregate lanes out of scope.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush
- crates/jj-store-cli/src/git_bridge/transport.rs#subprocess_stream_limit_accepts_boundary_and_rejects_overflow
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

Continue GB4 fault/ref-concurrency and GB6 credentials/real-RustFS/browser gates. No complete bridge claim.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


