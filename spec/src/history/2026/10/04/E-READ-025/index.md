---
schema: 1
id: E-READ-025
---

# E-READ-025

<!-- evidence-field:id markdown -->
## Id

E-READ-025
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

enclosing Jujutsu change zzqxsxts
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Performance-first continuation and prepared local historical point membership
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; Node26.9.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP schema, source/index format, companion/native publication, authority or capacity changes. Existing immutable prepared generations now serve selected scoped point reads; generation absence alone selects the bounded ordinary reader.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/prepared_points.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-service/src/local_index_tasks.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-JJ-005
- JST-API-001
- JST-GOV-003
- JST-GOV-007
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
jj --no-pager st; jj describe -m "Reuse prepared local membership for scoped historical reads"
```

**Notes**

Continue clean zzqxsxts in existing bounded-reads workspace on integrated main f5cd62d4. Main and other workspaces untouched. Latest user asks performance delivery before comprehensive qualification; plan and canonical execution scope updated.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_historical_points_skip_ancestry_and_preserve_content_and_conflicts -- --nocapture (before)
```

**Notes**

New exact regression reproduces Invalid for prepared CommitSummary, after proving the cold deep summary hits its four-commit visit limit. Six ordinary response oracles include a three-term conflict. prepared-points-before.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_historical_points_skip_ancestry_and_preserve_content_and_conflicts -- --nocapture (after)
```

**Notes**

1pass/9.05s. Six selected summary/path/file/diff/changed-path/conflict responses match ordinary results under four-commit visit admission. Later commits in every selector and mismatched file IDs rejected. Source index corruption and prepared segment removal do not trigger repair; extra heads unchanged. prepared-points-after.log.
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

1pass/7.33s. Actual isolated filesystem-only HTTP: cold summary works, ready summary preserves exact snapshot/object with at most three metadata nodes and less work than cold, valid substituted view fails corrupt_history with no ordinary fallback. Owned server stopped and disposable fixture removed. prepared-points-http.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_local_ -- --nocapture
```

**Notes**

3pass/18.89s. Direct prepared-reader regressions preserve log/conflict edges, hidden rewrites/evolution and 10,000 reverse-reference labels. This is a focused subset, not the complete local-browser or lifecycle lane. prepared-history-regressions.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http --lib --locked
```

**Notes**

Affected libraries and compiled dependencies pass in28.82s; CLI direct consumer compiles through the exact tests. package-check.log.
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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli; cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli -- --check; pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md spec/src/spec-governance.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/mutations.md spec/src/cli.md
```

**Notes**

Focused production/test Rust and canonical/plan formatting pass. format-rust.log/format-docs.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check docs/implementation-plan.md spec/src/architecture.md spec/src/spec-governance.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/mutations.md spec/src/cli.md
```

**Notes**

8 protected files/8 canonical chapters;5validators/23chapters/172requirements;138tasks;focused formatting pass. spec-first-025.log/spec-025.log/progress-025.log/format-docs-check.log. Complete plan verification and Help fingerprints stay pending by explicit user scope.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Performance delivery remains open: prepared visible change/reference pages
  and operation lookup; native visibility/annotations, immutable
  growth/eviction; indexed deep/copy/search/RPC reads; collaboration/search
  projections and lightweight visible repository cards."
- "User-requested deferred qualification: latest Linux/native large fixtures,
  isolated RSS/release speed, deep pages/capacities,
  restart/cancel/revocation/publication/fault races, comprehensive
  contracts/cold browser/real-RustFS and bundled Help/source-review refresh.
  Existing comprehensive integration evidence remains E-READ-024 and is not
  rerun for this delivery."
- Cold deep point reads still use bounded edge proof when no generation is ready
  and may return explicit resource limits; this delivery reuses ready membership
  without raising ordinary limits. Complete plan/milestone acceptance remains
  pending.
- Root aggregates, registry-release acceptance, visual baseline approval and
  unrelated runtime/deployment lanes are out of scope. Original dev
  services/Linux/main workspace preserved; no backend rebuild/restart or source
  migration.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the focused point-read delivery, then add prepared local visible change/reference windows in this same temporary workspace. Complete combined verification belongs to the later qualification slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Performance implementation first with exact correctness/package/governance checks; separate later complete qualification as explicitly requested by the user.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_points.rs#prepared_historical_points_skip_ancestry_and_preserve_content_and_conflicts
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_details_preserve_hidden_rewrites_without_history_preflight
- crates/jj-store-cli/tests/local_browser/prepared_details.rs#prepared_local_visibility_preserves_divergence_offsets_and_ten_thousand_bookmarks
- crates/jj-store-cli/tests/local_browser/preparation.rs#prepared_local_log_preserves_conflicts_and_filtered_native_edges
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


