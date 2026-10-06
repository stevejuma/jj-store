---
schema: 1
id: E-READ-037
---

# E-READ-037

<!-- evidence-field:id markdown -->
## Id

E-READ-037
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change soxlmmqu
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Reuse local WorkStore immutable roots, projections and exact-query keys across reopened facades
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; stock JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted format, public schema, transport envelope, domain capacity or publication change. Every browse/activity/detail request retains full committed companion and fresh Work-blob admission before lookup. Keys bind repository, Work object and SHA-256 bytes. Process-local retention:32 roots/256MiB/five-minute idle expiry/eight exact-query key sets per root. Cold derivation is serialized outside the map lock; conservative decoding/projection/key allowances are charged before allocation under512MiB live admission. Root/key eviction retains charges until final readers exit; oversized retention can return request-local data. These modeled allowances are not measured RSS qualification.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/local_repository/browser_work.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository/work_cache/data.rs
- crates/jj-store-service/src/local_repository/work_cache/memory.rs
- crates/jj-store-service/src/local_repository/work_cache/tests.rs
- crates/jj-store-service/src/local_repository/work_cache.rs
- crates/jj-store-service/src/local_repository/work_pages.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/work/browse.rs
- crates/jj-store-service/src/work/projection.rs
- crates/jj-store-service/src/work.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WORK-005
- JST-WORK-009
- JST-LOCAL-002
- JST-LOCAL-004
- JST-GOV-003
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
cargo test -p jj-store-service --lib --locked local_work_first_pages_share_data_across_repository_handles -- --nocapture (before)
```

**Notes**

1fail/0.41s: independently reopened handles rebuild unchanged root data. local-root-before-037.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib --locked local_work_first_pages_share_data_across_repository_handles -- --nocapture (initial implementation)
```

**Notes**

Compilation exposed private re-export and mutable-iterator errors; no tests executed. Corrected visibility and iterator binding. local-root-after-037.log.
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
cargo test -p jj-store-service --lib --locked local_work_first_pages_share_data_across_repository_handles -- --nocapture (corrected)
```

**Notes**

1pass/0.36s. Two private-interface warnings were subsequently removed by narrowing constructor visibility. local-root-after-fixed-037.log.
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
cargo test -p jj-store-service --lib --locked local_repository::work_cache:: -- --nocapture (initial and final)
```

**Notes**

7pass/1.42s then8pass/1.93s. Cross-facade root/key Arc reuse, selected-window hydration, exact rebuild after query eviction, expiry/root/byte caps, request-local fallback, live charge retention, tiny admission/failed-build cleanup, warm lookup during cold-build lease, duplicate construction, missing blobs/tighter source limits and fresh envelopes/distinct roots. local-root-units-037.log/local-root-units-final-037.log.
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
cargo test -p jj-store-service --test local_work --test work_store --locked -- --nocapture
```

**Notes**

11local/5native pass (3.49s/0.13s). Existing pinned publication/history, thread detail, receipt replay/GC, staged/foreign selection, bound identity, browse/filter/group and native cache consumers. local-root-service-037.log.
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
cargo test -p jj-store-service --lib --locked work:: -- --nocapture
```

**Notes**

2pass/0.01s shared aggregate and selected-window summary consumers. local-root-shared-units-037.log.
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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture (initial)
```

**Notes**

1pass/14.10s actual disposable filesystem HTTP fixture. Registration, explicit pins, signed continuation/binding/query admission and existing code/collaboration reads. Inherited oversized __eh_frame linker warning remains. local-root-http-037.log.
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
cargo check -p jj-store-service -p jj-store-cli --lib --locked
```

**Notes**

Affected service/CLI and direct transitive consumers compile/25.35s. local-root-check-037.log.
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
rustfmt --edition 2024 changed service files; rustfmt --edition 2024 --check changed service files; cargo fmt -p jj-store-service -- --check; pnpm exec prettier --write spec/src/architecture.md spec/src/local-jj.md spec/src/mutations.md spec/src/projects.md docs/implementation-plan.md
```

**Notes**

Targeted Rust/canonical formatting passes. local-root-format-037.log/local-root-prettier-037.log.
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
pnpm exec spec-validator validate --only governance; node scripts/check-boundaries.mjs; pnpm check:architecture
```

**Notes**

Owning governance1validator/23chapters/172requirements,17package dependency boundaries and972module architecture pass. local-root-governance-037.log/local-root-boundaries-037.log/local-root-architecture-037.log.
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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --nocapture (final after map-lock cleanup changes)
```

**Notes**

1pass/13.84s on final production code; inherited oversized __eh_frame linker warning remains. local-root-http-final-037.log. Total unique focused functional targets:27pass/0fail (8cache units,2shared units,11local service,5native service,1HTTP).
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --lib --locked -- -D warnings
```

**Notes**

Supplemental strict lint qualification stops in unchanged jj-store-jj before checking this slice: range/map-entry, three existing wide signatures and large Tables enum. Owning JJ query source is byte-identical to the parent (no JJ paths in current diff); this is an inherited qualification gap, not the exact cache regression. local-root-clippy-037.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --lib --no-deps --locked -- -D warnings
```

**Notes**

Supplemental service-only strict lane finds existing large OpenRead enum in unchanged src/reads.rs:83; no cache-specific diagnostics. Not described as passing. local-root-clippy-service-037.log. Full lint qualification and owning cleanup remain in later verification work.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check spec/src/architecture.md spec/src/local-jj.md spec/src/mutations.md spec/src/projects.md spec/src/milestones.md docs/implementation-plan.md; jj --no-pager diff --git plus added-line whitespace validation
```

**Notes**

138-task evidence bookkeeping,12protected paths/5mapped chapters,5validators/23chapters/172requirements and whitespace pass. local-root-ledger-format-037.log/local-root-progress-037.log/local-root-spec-first-037.log/local-root-spec-037.log/local-root-final-037.patch. Final ledger formatter/progress/spec-first and targeted Prettier check also pass (local-root-ledger-format-final-037.log/local-root-progress-final-037.log/local-root-spec-first-final-037.log/local-root-doc-check-037.log); final package format passes (local-root-format-final-037.log). Focused delivery; no milestone promotion.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-037 details 2](details-02.md)
