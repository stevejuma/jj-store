---
schema: 1
id: E-M13-013
---

# E-M13-013

<!-- evidence-field:id markdown -->
## Id

E-M13-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-browser-registration
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/local_browser.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_browser.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#local_folder_inspection_preserves_uninitialized_metadata_and_dirty_files
- crates/jj-store-cli/tests/local_browser.rs#registration_replays_across_restart_and_removal_preserves_metadata
- crates/jj-store-cli/tests/local_browser.rs#registration_recovers_after_initialization_and_rejects_identity_replacement
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- local_folder_inspection_preserves_uninitialized_metadata_and_dirty_files --exact
```

**Notes**

Before implementation: missing local browser modules; three compiler diagnostics.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

Initial two focused real filesystem tests pass; interrupted registration acceptance added next.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

All three real filesystem admission/restart/replay/removal tests pass, including linked workspace inspection and identity replacement.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test workspace_management --locked
```

**Notes**

Both existing native folder admission regressions pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-cli -p jj-store-service -p jj-store-server --locked
```

**Notes**

Focused package checks pass.
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
pnpm spec:first
```

**Notes**

7 protected files, 6 owning canonical chapters.
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
pnpm check:progress
```

**Notes**

Generated root dependency symlink entered evidence inventory; replaced with an ignored actual node_modules directory and corrected evidence.
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
pnpm check:boundaries
```

**Notes**

All 15 package boundaries pass.
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
pnpm spec:validate
```

**Notes**

21 canonical chapters, 132 requirements validate.
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
pnpm check:progress
```

**Notes**

Corrected evidence inventory passes; generated dependencies remain ignored.
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
pnpm exec prettier --write spec/src/local-jj.md spec/src/api.md spec/src/ui.md spec/src/cli.md spec/src/mutations.md spec/src/milestones.md; rustfmt --edition 2024 affected Rust files
```

**Notes**

Changed canonical chapters, ledger and Rust files formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Locked JJ 0.45.1, Git SHA-1, Rust toolchain; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private local-repositories.json version 1. Existing companion ref/records unchanged. Enclosing Jujutsu change srqxrtpr.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish registration gates, then local read provider and filesystem runtime.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Folder admission and durable registration provider only; transport/runtime/UI remain subsequent slices. Root aggregates and unrelated product lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Filesystem runtime, local query APIs, browser integration and cold mixed
  acceptance remain pending.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


