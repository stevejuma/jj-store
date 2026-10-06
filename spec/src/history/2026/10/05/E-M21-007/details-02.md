---
schema: 1
id: E-M21-007
---

# E-M21-007 details 2

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
cargo test --locked -p jj-store-service --test working_tree_scan
```

**Notes**

Before implementation the test binary could not import working_tree; working-tree-before.log.
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
cargo test --locked -p jj-store-service --test working_tree_scan
```

**Notes**

Five initial source regressions pass; working-tree-first.log.
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
cargo test --locked -p jj-store-service --test working_tree_scan --test local_working_copy --test stack_workspace_contracts
```

**Notes**

19 tests pass: six scan, four existing Save, nine stack contracts; working-tree-tests-final.log. Binary bytes, modes, sorted names, ignored/untracked obstacles, nontraversed symlinks, hardlink/socket/unsafe rejection, workspace replacement, cumulative limits/depth/deadline/verify cancellation and late external edits included.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service --lib -- -D warnings
```

**Notes**

working-tree-check.log: unchanged jj-store-jj dependency diagnostics (manual_range_patterns in local_browser/git_read/extras.rs; map_entry in local_browser/references/prepared.rs; too_many_arguments in query/cache/resident.rs and query/index.rs). Byte-identical parent source verified; broader strict lint remains incomplete.
<!-- /run -->

<!-- run -->
### Run 5

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

Same diagnostics reproduce at their owning crate; working-tree-jj-lint-baseline.log. Assigned to existing bounded-read helper source; no lint exception or diagnostic suppression added.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy --locked -p jj-store-service --no-deps --lib -- -D warnings
```

**Notes**

Owning-service inspection after excluding dependency lint finds two additional unchanged-parent diagnostics: large_enum_variant in reads.rs OpenRead and manual_is_multiple_of in stack_workspace/selection.rs. working-tree-service-lint.log. This is not a replacement passing lane or a waiver; strict lint qualification remains an explicit gap.
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
cargo check --locked -p jj-store-service --lib
```

**Notes**

Affected package compiles; working-tree-package-check.log. Does not replace the failing strict lint lanes.
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

17 package boundaries pass; working-tree-boundaries.log. No upstream JJ import added to the service.
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
cargo fmt --all
```

**Notes**

Formatted changed Rust source/tests; no unrelated Rust changes introduced.
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
cargo fmt --all -- --check
```

**Notes**

working-tree-format-rust.log.
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
pnpm docs:review services "Private physical scan fingerprints bounded descriptor-relative regular files and symlink targets, includes checkout obstacles and verifies two passes. It adds no wire route, JJ snapshot, executor or Unstaged provider projection."
```

**Notes**

Reviewed source coverage and updated service fingerprint; working-tree-docs-review.log.
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
pnpm docs:check
```

**Notes**

139 files, nine diagrams, valid links/anchors; working-tree-docs.log.
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
pnpm spec:first
```

**Notes**

Eight protected files/four canonical chapters; working-tree-spec-first.log. Final ledger-inclusive recheck follows.
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
pnpm spec:validate
```

**Notes**

Five validators, 23 chapters, 173 requirements; working-tree-spec.log. Final ledger-inclusive recheck follows.
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
pnpm exec prettier --write docs/site/technical/architecture.md docs/implementation-plan.md spec/src/local-jj.md spec/src/mutations.md spec/src/architecture.md spec/src/documentation.md docs/maintenance/coverage.json spec/src/verification.md
```

**Notes**

Changed documentation formatted through targeted invocations; milestones.md excluded.
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
pnpm exec prettier --check docs/implementation-plan.md docs/maintenance/coverage.json docs/site/technical/architecture.md spec/src/architecture.md spec/src/documentation.md spec/src/local-jj.md spec/src/mutations.md spec/src/verification.md
```

**Notes**

working-tree-format-docs.log passes.
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
pnpm spec:first
```

**Notes**

Final ledger-inclusive gate passes: eight protected files/five chapters; working-tree-spec-first-final.log.
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
pnpm spec:validate
```

**Notes**

Final validation passes five validators/23 chapters/173 requirements; working-tree-spec-final.log.
<!-- /run -->
<!-- /evidence-field -->

