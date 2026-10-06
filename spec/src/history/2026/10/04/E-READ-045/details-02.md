---
schema: 1
id: E-READ-045
---

# E-READ-045 details 2

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
cargo test -p jj-store-cli --test local_browser --locked prepared_filter_windows_preserve_unicode_substrings_and_hydrate_only_selected_targets -- --nocapture (before)
```

**Notes**

0pass/1fail,11.14s tests: prepared nonempty reference filters return Invalid. local-filter-before-045.log.
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
cargo check -p jj-store-jj --locked (first)
```

**Notes**

One compile diagnostic: redb readonly builder belongs to Database; corrected constructor. One unused variable/import warning removed before final checks. local-filter-check-first-045.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_filter_windows_preserve_unicode_substrings_and_hydrate_only_selected_targets -- --nocapture (first)
```

**Notes**

1pass/11.34s tests;216 ordinary/prepared windows. local-filter-exact-first-045.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared::filters::tests -- --nocapture (initial and batched)
```

**Notes**

3pass each; sparse/dense masks, quota refusal, selection/checksum/cancellation/work/byte admission. Batching avoids per-document database posting writes. local-filter-units-045.log and local-filter-units-batched-045.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_filter_windows_preserve_unicode_substrings_and_hydrate_only_selected_targets -- --nocapture (stronger fixture first)
```

**Notes**

Fixture used unsupported @~10 revision syntax; changed to JJ repeated-parent syntax without product/assertion changes. local-filter-exact-final-045.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_filter_windows_preserve_unicode_substrings_and_hydrate_only_selected_targets -- --nocapture (batched/final)
```

**Notes**

Final1pass/15.54s tests (66.54s with build);288 windows across12 filters,3kinds,2directions,4anchors. Whole-string contextual Unicode casing, short/sparse/name/description filters, hidden rewrites, divergence and removed bookmark terms match ordinary reader under4history visits. Later operation is excluded; source/prepared index segments are unavailable after admission; final selected divergent page works with an unrelated Git commit object missing. Final page5metadata records,6663charged bytes,709work. local-filter-exact-batched-045.log, local-filter-consumer-00-045.log and local-filter-final-00-045.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture (expanded cross-block fixture first)
```

**Notes**

8pass/1fail: expanded9000-row/three-block test charged228work, exceeding unchanged200bound because intersection still read a fully excluded anchor block. Fixed production masking before intersections; assertion retained. local-filter-consumer-01-045.log.
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
cargo test -p jj-store-jj --lib --locked filter_seeks_compressed_windows_and_validates_selection_checksum_and_budgets -- --nocapture
```

**Notes**

1pass/2.93s;9000 lexical rows, sparse cross-block forward/backward seeks below the original200work assertion, dense/short and Unicode text matches, exact-candidate exclusion, quota/checksum/source-pin/work/byte/cancellation failures. local-filter-cross-block-final-045.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture (final)
```

**Notes**

9pass; filter, operation identity/fingerprint/prefix, compact table and pre-growth scratch consumers. local-filter-final-01-045.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture (first)
```

**Notes**

9pass/1fail: new changed-query cursor test incorrectly expected409; existing HTTP digest rejection is400 invalid_request. Corrected only this new assertion after checking owning route/error contract. local-filter-final-02-045.log.
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
cargo test -p jj-store-cli --test local_browser --locked local_point_http_uses_ready_membership_and_only_falls_back_when_cold -- --nocapture (fixed)
```

**Notes**

1pass/8.26s tests. Actual cold filtered rebuild-required pin, preparation retry, signed forward/backward filtered windows and changed-query rejection, plus existing selected-point/prefix/comparison behavior. local-filter-http-final-045.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli; pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/architecture.md spec/src/compatibility.md spec/src/cli.md docs/implementation-plan.md
```

**Notes**

Affected source/doc formatting during implementation: local-filter-format[-final/-last/-batched/-expanded/-complete/-admission]-045.log and local-filter-doc-format-045.log. Final formatting/check recorded below.
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

17package boundaries pass. local-filter-boundaries-045.log.
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
pnpm check:ui-ownership
```

**Notes**

1ownership validator passes. local-filter-ownership-045.log.
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
pnpm spec:first
```

**Notes**

12protected files/6canonical chapters pass before final ledger. local-filter-spec-first-045.log.
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
pnpm spec:validate
```

**Notes**

5validators/23chapters/172requirements pass before final ledger. local-filter-spec-045.log.
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
rustc --version; target/debug/jj --version (initial probe)
```

**Notes**

Rust1.98.1 reported. Custom CLI rejects --version spelling with exit2; corrected subcommand succeeds but emits an empty version suffix, so this entry records the owning libraries' Cargo pin, not a runtime CLI version proof. No executable replacement.
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
target/debug/jj version
```

**Notes**

Read-only corrected probe exits0; output jj with empty version suffix. local-filter-version-045.log. Exact jj-lib/jj-cli0.45.1 pins separately inspected in Cargo.toml.
<!-- /run -->
<!-- /evidence-field -->

