---
schema: 1
id: E-READ-082
---

# E-READ-082

<!-- evidence-field:id markdown -->
## Id

E-READ-082
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

xrspplor
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Retain freshness witnesses during observation pressure
<!-- /evidence-field -->

<!-- evidence-field:versions list -->
## Versions

- JJ/jj-lib 0.45.1
- Rust 1.98.1
- macOS ARM64
<!-- /evidence-field -->

<!-- evidence-field:formats list -->
## Formats

- No API, schema, index or authoritative format change
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Narrow HTTP monitor correction exposed by cold Linux acceptance. Busy/deadline/resource admission retains the last witness and existing retry; confirmed access and unavailable-source errors retain their ordinary notice path.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/application_events/monitors.rs
- crates/jj-store-http/src/error.rs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/application_events/monitors.rs#observation_pressure_does_not_invent_repository_changes
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
cargo test -p jj-store-http application_events::monitors::tests::observation_pressure_does_not_invent_repository_changes --locked -- --exact
```

**Notes**

.local/bounded-reads/managed/monitor-pressure-before.log: original behavior invents an unavailable witness for Busy.
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
cargo test -p jj-store-http application_events --locked
```

**Notes**

.local/bounded-reads/managed/monitor-pressure-final.log: all 4 event tests pass, including the exact regression and shared-monitor/finite-stream/restart authorization tests.
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
cargo check -p jj-store-http --locked
```

**Notes**

.local/bounded-reads/managed/monitor-check.log: affected HTTP package check.
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
pnpm check:architecture
```

**Notes**

.local/bounded-reads/managed/monitor-architecture.log: 993 modules.
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
pnpm spec:validate
```

**Notes**

.local/bounded-reads/managed/monitor-canonical.log: canonical requirements validate.
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
cargo fmt -p jj-store-http
```

**Notes**

Affected Rust files formatted.
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
pnpm spec:first && pnpm check:progress && cargo fmt -p jj-store-http -- --check && pnpm exec prettier --check spec/src/api.md spec/src/architecture.md spec/src/milestones.md
```

**Notes**

.local/bounded-reads/managed/monitor-governance-final.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Original 7470 trace showed repeated successful snapshot/graph/commit/diff
  calls at an unchanged exact selection. The live SSE trace showed repository
  witnesses oscillating between registration generation and unavailable while
  background admission was Busy. This feedback loop repeatedly reset selected
  comparison loading. Busy is pressure, not evidence that repository state
  changed.
- Both initial and retained monitor observations now use the concrete error
  classification. Existing access/identity invalidation and unavailable notices
  are preserved. No additional watcher, timer, queue, route or projection.
- UI tests/types remain E081; no changed UI or public DTO. Full contract
  capture, unrelated UI/Rust tests and release/RustFS/visual qualification are
  out of scope. Backend rebuild, landing and final live acceptance are recorded
  in the following integration evidence.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Final landing and live stable Linux browser proof pending. Advanced
  search/evolution/copy/deep-query, authenticated automatic maintenance and full
  scale/fault/RustFS/release/visual qualification stay deferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land this focused monitor slice, replace only preview backend, then verify stable Linux browsing and index reuse.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


