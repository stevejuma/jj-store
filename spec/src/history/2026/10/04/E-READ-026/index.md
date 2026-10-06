---
schema: 1
id: E-READ-026
---

# E-READ-026

<!-- evidence-field:id markdown -->
## Id

E-READ-026
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oyukmsxr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared local visible-change/reference keyset windows with page-only target hydration
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; Node26.9.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing JJ format6, Git/companion/native identities and publication formats unchanged. No HTTP schema, cursor or task-contract change. The immutable resident generation retains an ordered visible change identity projection; ordinary query limits and source authorization remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-cli/tests/local_browser/prepared_references.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation/visibility.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-jj/src/local_browser/references/prepared.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-JJ-005
- JST-API-001
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
jj --no-pager st; jj describe -m "Page local change references from prepared visible identities"
```

**Notes**

Clean follow-up on E-READ-025 in the retained bounded-reads workspace. Main and other workspaces unchanged. Acceptance target: selected visible-reference windows without whole-closure hydration.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_reference_windows_seek_visible_changes_before_hydrating_targets -- --nocapture (before)
```

**Notes**

Exact regression reproduces Invalid for the prepared reference reader; cold Changes fails under four-commit history admission. Ordinary reference oracle preserves divergent changes and conflicted bookmarks. prepared-references-before.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_reference_windows_seek_visible_changes_before_hydrating_targets -- --nocapture (first after)
```

**Notes**

1pass/14.20s.18forward/backward/deep/arbitrary-anchor/exact windows match ordinary records and continuations under four-commit admission, with at most six metadata nodes. Later commits excluded; different embedded operation rejected; source-index corruption/disposable segment loss do not trigger repairs. prepared-references-after.log.
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
cargo test -p jj-store-cli --test local_browser --locked local_point_http_uses_ready_membership_and_only_falls_back_when_cold -- --nocapture
```

**Notes**

1pass/8.50s. Existing focused filesystem HTTP regression extended with cold Changes409/exact snapshot, ready one-row page with at most three metadata nodes, signed forward/previous cursor round-trip, independent cold summary and corrupt-ready rejection. Owned isolated server stopped. prepared-references-http.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture (initial admission review)
```

**Notes**

6pass/23.13s. Direct prepared history/content/reference/HTTP consumers, hidden rewrites/prefix offsets and 10,000 reverse labels. After this pass, review conservatively increased projection allocation admission and restricted divergent-target preallocation checks to distinct change targets. The exact six-test subset is rerun for those final admission changes, not expanded to the full suite. prepared-read-regressions-026.log.
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
cargo check -p jj-store-jj -p jj-store-http --lib --locked
```

**Notes**

Affected libraries and direct compiled service consumers pass in14.94s. CLI/server composition compiles through the exact tests. package-check-026.log.
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
cargo fmt -p jj-store-jj -p jj-store-http -p jj-store-cli; cargo fmt -p jj-store-jj -p jj-store-http -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/cli.md; pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/cli.md
```

**Notes**

Focused Rust and canonical chapter formatting pass. format-rust-026.log/format-docs-026.log/format-docs-check-026.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture (final admission)
```

**Notes**

6pass/27.62s after final conservative384bytes/indexed-commit admission and distinct divergent-target preallocation check. Exact subset includes HTTP recovery/cursor round-trip, selected point content/conflicts, keyset references, hidden rewrites and 10,000 labels. prepared-read-regressions-026-final.log. This is not complete browser/integration or scale qualification.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

9protected files/5canonical chapters;5validators/23chapters/172requirements;138tasks. spec-first-026.log/spec-026.log/progress-026.log. Historical evidence bytes preserved; no milestone promotion. Comprehensive verification remains deferred.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete large-scale qualification remains deferred under the
  performance-first plan. Visibility plus ordered change projection
  conservatively reserves384bytes/indexed commit before growth, plus exact
  head/name charges, within the shared1GiB allowance. This is an allocation
  estimate, not isolated RSS or latest Linux/native preparation qualification.
- Nonempty description/reference filtering and an embedded operation different
  from the outer selection retain existing complete bounded dispatch. Indexed
  operation prefix/comparison, copy/search/native RPC and remaining deep
  continuations remain performance work.
- Native visibility/annotations, immutable additional-operation growth and
  shared eviction, root-bound collaboration/search projections and lightweight
  visible repository-card windows remain implementation slices.
- Comprehensive contracts/cold browser, Linux/native large fixtures,
  restart/revocation/publication/fault pressure, real-RustFS,
  release/registry-only qualification and bundled Help/source-review refresh are
  deferred to the separate qualification slice. Root aggregates and visual
  baseline approval are out of scope; original dev services/Linux/main workspace
  remain untouched.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue prepared operation/prefix/comparison and indexed filtering, native visibility/cache reuse, collaboration/search/card performance in this temporary workspace. Complete combined qualification belongs to its separate later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Deliver ordered prepared local Changes/All windows and existing HTTP recovery with exact regression/package/governance checks. No plan or milestone completion is claimed.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_references.rs#prepared_reference_windows_seek_visible_changes_before_hydrating_targets
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-cli/tests/local_browser/prepared_points.rs#prepared_historical_points_skip_ancestry_and_preserve_content_and_conflicts
- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_visibility_preserves_divergence_offsets_and_ten_thousand_bookmarks
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


