---
schema: 1
id: E-M21-016
---

# E-M21-016 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test stack_operation_archive
```

**Notes**

stack-archive-before.log: absent archive exports/capture and execution retention API/history field establish the missing source-history boundary.
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
cargo test --locked -p jj-store-jj --test stack_operation_archive
```

**Notes**

stack-archive-after.log: six cases pass; the historical-only fixture cannot parse an empty ID because stock JJ does not retain the unreferenced empty --no-edit commit. Exact retries confirm the empty output; the fixture now creates and abandons an actual named current commit. No product assertion weakened.
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
cargo test --locked -p jj-store-jj --test stack_operation_archive history::historical_only_code_remains_reachable_after_gc_and_packed_ref_replay -- --exact
```

**Notes**

stack-archive-history.log: real historical-only commit is absent from selected capsule, retained through Git GC and verified by packed-ref exact replay with unchanged checkout.
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
cargo test --locked -p jj-store-jj --test stack_operation_archive
```

**Notes**

stack-archive-final.log: seven pass after strict original-extra read-only capture and bounds. Full final direct lane subsequently includes parentless non-root validation and actual returned reword ancestry.
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
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-archive-lint.log: one new map-entry diagnostic in source view capture. Replaced contains_key/insert with a vacant entry; no warning suppression. Earlier private-interface warning was repaired by narrowing Budget::authorize visibility.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-jj --test stack_reword_preparation lifecycle::missing_original_extras_fail_before_reword_import_or_candidate_write -- --exact
```

**Notes**

stack-archive-reword-before.log: real preparation still allowed upstream implicit original-extra import. Added strict complete source-history capture before transaction work and complete candidate-history verification.
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
cargo test --locked -p jj-store-jj --test stack_reword_preparation lifecycle::missing_original_extras_fail_before_reword_import_or_candidate_write -- --exact
```

**Notes**

stack-archive-reword-after.log: missing extras now fail Incomplete before import/candidate writes, with byte-identical remaining source metadata, refs, dirty files and checkout.
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
cargo test --locked -p jj-store-jj --test stack_operation_archive --test stack_reword_preparation --test stack_foreground --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-archive-direct.log: initial complete direct boundary has 47 passing integrations. Existing foreground subprocess helper is invoked by its passing parent, not ordinary independent helper invocation.
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
cargo test --locked -p jj-store-jj --lib --test stack_operation_archive --test stack_reword_preparation --test stack_foreground --test stack_preparation_retention --test local_metadata --test local_evidence
```

**Notes**

stack-archive-direct-final.log: intermediate 76 pass. stack-archive-complete.log: final 77 pass, comprising 29 units and 48 direct integrations (7 archives, 7 rewords, 8 foreground, 6 retention, 14 metadata, 6 evidence). Foreground process-loss helper remains explicitly ignored in ordinary invocation but actually runs through its parent.
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
cargo test --locked -p jj-store-cli --test local_browser
```

**Notes**

stack-archive-read-consumer.log: all 28 complete direct local-reader tests pass; two existing external large-repository fixtures intentionally ignored. Runs the shared extra decoder consumers, registered-folder transport, read-only graph/diffs and fresh filesystem runtime. macOS linker unwind-size warnings retained; no native/real-RustFS or UI qualification inferred.
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
cargo check --locked -p jj-store-jj --all-targets
```

**Notes**

stack-archive-check.log, stack-archive-check-final.log and stack-archive-check-complete.log: owning source/all-target checks pass, final includes returned reword ancestry and strict preflight.
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
cargo clippy --locked -p jj-store-jj --all-targets --no-deps -- -D warnings
```

**Notes**

stack-archive-lint-final.log and stack-archive-lint-complete.log: owning strict source/tests lint passes, final includes real reword strict-history integration. No aggregate/dependency/service lint qualification claimed.
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
cargo fmt --all
```

**Notes**

Pinned formatting during implementation; no unrelated source changes.
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
pnpm check:boundaries
```

**Notes**

stack-archive-boundaries.log: upstream APIs stay in owning JJ/CLI boundary; all 17 packages pass.
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
pnpm check:architecture
```

**Notes**

stack-archive-architecture.log: 1090 modules pass; no presentation/checkout-dependency changes.
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
pnpm format:rust:check
```

**Notes**

stack-archive-rust-format.log and stack-archive-rust-format-complete.log: root/runtime formatting pass.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/verification.md docs/implementation-plan.md docs/site/technical/local.md docs/maintenance/coverage.json
```

**Notes**

stack-archive-docs-format-complete.log: changed canonical/manual documents and coverage only; historical milestones.md never formatted. Earlier narrower doc/coverage formatting commands are retained in stack-archive-docs-format*, stack-archive-coverage-format* logs.
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
pnpm docs:review local "Documented complete bounded source operation and historical code archives, strict original-extra read-only capture, exact private upstream materialization, version-2 reserved reachability and immutable legacy slots. Real reword now captures strict source history before writes and returns complete candidate ancestry with unchanged original records; missing extras cannot trigger import. Runtime journal/policy composition, source restoration, publication and checkout remain required."
```

**Notes**

stack-archive-docs-review-complete.log: final source fingerprint covers reword integration. Prior bounded-archive review commands also pass in stack-archive-docs-review.log and stack-archive-docs-review-final.log.
<!-- /run -->
<!-- /evidence-field -->

