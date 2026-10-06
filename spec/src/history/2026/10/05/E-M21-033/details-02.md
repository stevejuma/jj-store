---
schema: 1
id: E-M21-033
---

# E-M21-033 details 2

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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans installation::retained_tree_files_install_with_durable_progress_and_unchanged_native_metadata -- --exact
```

**Notes**

native-completion-before.log: Existing physical-only baseline passes; it leaves original native metadata and has no completion method.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 22
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo check --locked -p jj-store-jj
```

**Notes**

native-completion-first-check.log: 22 compilation diagnostics: sibling-module privacy and u64/u32 mode comparison. Repaired without weakening guards. No tests ran.
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

native-completion-check-repair.log: Owning package check passes after controlled internal visibility and mode conversion repairs.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans completion::native_tree_completion_and_exact_replay_preserve_installed_inodes -- --exact
```

**Notes**

native-completion-first-test.log: Test fixture referenced a non-public upstream executable-bit type. Replaced with public decoded FileType, actual filesystem mode and stock-JJ consumption assertions. No tests ran.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans completion::
```

**Notes**

native-completion-expanded.log: Test fixture accessed a private ExecBit field. Repaired using the same public/native checks, without production API changes. No tests ran.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans completion::native_tree_completion_and_exact_replay_preserve_installed_inodes -- --exact
```

**Notes**

native-completion-exact-repaired.log: Exact real-native/scanner completion and replay pass; stock JJ consumes the resulting checkout without a spurious commit rewrite.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

native-completion-first-lint.log: One large-enum diagnostic. Private recovery variants now own boxed payloads.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans completion::
```

**Notes**

native-completion-functional.log: One missing-comma parser diagnostic during the enum repair, also reported by cargo fmt; fixed immediately. No tests ran.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans completion::
```

**Notes**

native-completion-functional-repaired.log: Eight completion parents pass. Subsequent frame re-acknowledgement hardening and the ninth missing-evidence/order parent are included in the final family below.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_checkout_plans
```

**Notes**

native-completion-final-plans.log: 43 plan/inventory/installation/native-completion parents pass; all five isolated helpers are invoked by passing parents. Includes nine new native completion parents and four abrupt native boundaries.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --test stack_tree_checkout --test stack_unchanged_checkout --test stack_head_publication --test stack_source_recovery --test stack_foreground --test stack_reword_preparation
```

**Notes**

native-completion-provider-regressions.log: 94 direct provider parents pass: original foreground/reword, head publication, source/extra/index recovery, tree planning and unchanged completion.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-jj --lib signed_native_milliseconds_match_upstream_duration_truncation
```

**Notes**

native-completion-unit.log: One exact native timestamp regression passes, including sub-millisecond negative timestamps, negative seconds/nanos and overflow/invalid nanosecond bounds.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-service --test working_tree_inventory --test working_tree_checkout
```

**Notes**

native-completion-service-regressions.log: 13 service inventory and registered guard parents pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-cli --test stack_registered_preparation
```

**Notes**

native-completion-registered-regressions.log: 12 registered preparation/private execution parents pass.
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
CARGO_INCREMENTAL=0 cargo test --locked -p jj-store-server --test stack_local_execution
```

**Notes**

native-completion-runtime-regressions.log: Nine existing unchanged-tree private runtime parents pass.
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
CARGO_INCREMENTAL=0 cargo clippy --locked -p jj-store-jj -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --no-deps -- -D warnings
```

**Notes**

native-completion-lint-final.log: Owning/direct-consumer all-target strict type/lint passes.
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
cargo fmt --all
```

**Notes**

source formatting: Repeated after source/test edits; final pinned format lane below passes. No unrelated source was changed.
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

native-completion-rust-format.log: Pinned Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

