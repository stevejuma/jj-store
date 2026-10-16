---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 14

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
JJ_STORE_REMOTE_TEST_JJ=$PWD/target/debug/jj-store-jj pnpm dev:cargo -- test --locked -p jj-store-server --lib remote_agents::execution_tests::native_remote_execution_retains_receipts_and_never_replays_after_restart -- --ignored --exact
```

**Notes**

remote-agent-jj-31: exit 0, 59 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-server --lib agent_execution::tooling::install::tests::real_private_codex_install_verifies_pinned_versions_and_cold_selection -- --ignored --exact
```

**Notes**

private-tooling-install-31: exit 0, 17.44 seconds. 
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
pnpm dev:cargo -- test --manifest-path desktop/Cargo.toml --workspace --locked --no-default-features --no-fail-fast
```

**Notes**

desktop-workspace-tests-30: exit 0, 29.87 seconds. 
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
pnpm dev:cargo -- clippy --manifest-path desktop/Cargo.toml --workspace --all-targets --locked --no-default-features -- -D warnings
```

**Notes**

desktop-clippy-30: exit 1, 15.87 seconds. 
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
pnpm dev:cargo -- fmt --manifest-path desktop/Cargo.toml -- --check
```

**Notes**

desktop-format-46: exit 0, 0.37 seconds. 
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
pnpm dev:cargo -- clippy --manifest-path desktop/Cargo.toml --workspace --all-targets --locked --no-default-features -- -D warnings
```

**Notes**

desktop-clippy-30: exit 0, 6.15 seconds. 
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
pnpm dev:cargo -- test --manifest-path desktop/Cargo.toml --workspace --locked --no-default-features --no-fail-fast
```

**Notes**

desktop-workspace-tests-30: exit 0, 29.74 seconds. 
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
Desktop strict Clippy failure review and equivalent short-circuit guard repair
```

**Notes**

desktop-lint-repair-review-46: exit 0. First desktop-clippy-30 attempt failed one collapsible_if lint at desktop/src/installation.rs:54. The exact original command, exit 1 and 15.87-second duration remain in results; its same-name log was replaced by the passing rerun before local log retention was added. No allow attribute or assertion change. Fresh exact Clippy and complete desktop tests passed.
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
pnpm desktop:check
```

**Notes**

desktop-check-46: exit 0, 67.33 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-jj --test rustfs --no-run
```

**Notes**

rustfs-native-precompile-45: exit 0, 8.95 seconds. 
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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/main-qualification-rustfs.json
```

**Notes**

rustfs-native-server-45: exit 1, 24.89 seconds. 
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
node .local/main-qualification-2026-10-16/stage-desktop.mjs
```

**Notes**

desktop-stage-45: exit 0, 1.97 seconds. 
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
JJ_STORE_DESKTOP_TEST_RESOURCES=$PWD/.local/main-qualification-2026-10-16/desktop-resources pnpm dev:cargo -- test --manifest-path desktop/Cargo.toml --locked --no-default-features --test managed_runtime -- --ignored --exact real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
```

**Notes**

desktop-managed-runtime-45: exit 0, 17.19 seconds. 
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:rust:check
```

**Notes**

rust-format-47: exit 0, 3.75 seconds. 
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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/main-qualification-rustfs.json
```

**Notes**

rustfs-native-server-45: exit 1, 16.42 seconds. 
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --workspace --locked --test server_executable
```

**Notes**

server-executable-complete-47: exit 1, 6.32 seconds. 
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

server-executable-clippy-47: exit 1, 81.7 seconds. 
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
pnpm format:rust:check
```

**Notes**

rust-format-47: exit 0, 3.87 seconds. 
<!-- /run -->
<!-- /evidence-field -->

