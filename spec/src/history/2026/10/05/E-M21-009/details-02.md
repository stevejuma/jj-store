---
schema: 1
id: E-M21-009
---

# E-M21-009 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test working_tree_current
```

**Notes**

working-current-before.log: expected missing current API/types before implementation; no current checkout proof existed.
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
cargo check -p jj-store-jj --lib
```

**Notes**

working-current-dependency-check.log: current provider compiles. Lockfile adds only the existing rustix dependency to the package; no version upgrades. Prost moved from dev to runtime dependency.
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
cargo test --locked -p jj-store-jj --test working_tree_current --test working_tree_baseline
```

**Notes**

working-current-first.log: initial seven tests pass. working-current-label-tests.log repeats seven after exact tree labels and directory binding implementation; working-current-expanded.log passes nine after race/revocation, sparse bounds and label mismatch regressions.
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
cargo check --locked -p jj-store-service --lib
```

**Notes**

working-observer-check.log: completed observation owner compiles after removing unfinished placeholder.
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
cargo test --locked -p jj-store-service --test working_tree_projection --test working_tree_scan --test local_working_copy --test stack_workspace_contracts
```

**Notes**

working-current-service-first.log: 23 focused tests pass, including sparse-excluded tracked obstacles without false deletions and outside-sparse candidate refusal.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-service --test working_tree_observer -- --ignored
```

**Notes**

working-observer-real-first.log: four fresh disposable real colocated JJ fixtures pass, including actual sparse set, current physical edits/complete ignore rules, metadata/policy races/revocation and canceled worker admission retention. No HTTP/browser or executor acceptance is claimed.
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
cargo fmt -p jj-store-jj -p jj-store-service
```

**Notes**

Changed Rust source/tests formatted. Later cargo fmt -p jj-store-service formats the test-only release-on-panic guard.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-service --test working_tree_observer -- --ignored
```

**Notes**

working-observer-real-final.log: four pass after adding fail-safe fixture gate release. TempDir owns cleanup; real code/metadata/files remain byte-identical for ordinary observations.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/target/debug/jj cargo test --locked -p jj-store-service --test working_tree_local real_folder_comparison_preserves_operations_metadata_and_unsnapshotted_files -- --ignored --exact
```

**Notes**

working-current-local-baseline.log: existing fresh physical/baseline regression passes after labels extension; no source publication or physical writes.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-jj --lib -- -D warnings
```

**Notes**

working-current-lint.log: same four existing git_read/extras.rs, references/prepared.rs, query/cache/resident.rs and query/index.rs diagnostics. Owning files byte-identical to e687b039; no new lint suppression.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service --no-deps --all-targets -- -D warnings
```

**Notes**

working-current-service-lint.log: same reads.rs large enum and stack_workspace/selection.rs manual multiple diagnostics, reproduced in lib/lib-test. Owning files byte-identical to e687b039. Strict lint is not qualified or waived.
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
cargo check --locked -p jj-store-jj -p jj-store-service --all-targets
```

**Notes**

working-current-package-check.log: both affected packages and targets compile; no workspace aggregate.
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
pnpm check:boundaries
```

**Notes**

working-current-boundaries.log: 17 boundaries pass; service imports owned JJ interfaces only.
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
cargo fmt --all -- --check
```

**Notes**

working-current-format-rust.log and working-current-format-rust-final.log pass; no ledger formatting.
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
pnpm docs:review local "Private current WC observation proves exact checkout trees and sparse scope, captures bounded global/repository ignore policy and verifies repeated metadata and heads without initializing state. No scan transport or executor is enabled."
```

**Notes**

working-current-docs-local-review.log: updated owning manual/source review.
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
pnpm docs:review services "Private service observation joins repeated current WC metadata/policy/head witnesses to two-pass physical inventory under an eight-second deadline and bounded admission retained through canceled blocking scans. Sparse excluded paths remain obstacles, no transport, snapshot or execution capability."
```

**Notes**

working-current-docs-service-review.log: updated service source/manual review.
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
pnpm docs:check
```

**Notes**

working-current-docs.log: source-derived CLI example build, explicit review coverage, nine Mermaid diagrams, strict manual build and 139-file link/anchor validation pass.
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
pnpm exec prettier --write docs/implementation-plan.md docs/site/technical/architecture.md docs/site/technical/local.md spec/src/local-jj.md spec/src/mutations.md spec/src/jj-integration.md spec/src/architecture.md spec/src/documentation.md spec/src/ui.md spec/src/verification.md
```

**Notes**

working-current-format-docs.log: targeted changed docs only; milestones.md excluded.
<!-- /run -->
<!-- /evidence-field -->

