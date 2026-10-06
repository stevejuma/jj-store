---
schema: 1
id: E-READ-080
---

# E-READ-080 details 4

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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli -- --check
```

**Notes**

.local/bounded-reads/managed/rust-format-slice-final.log: affected Rust files formatted.
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
pnpm exec prettier --check catalog/components/store-application/StoreKindPreparation.stories.svelte catalog/fixtures/KindPreparationFixture.svelte scripts/contracts.mjs spec/src/api.md spec/src/architecture.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/spec-governance.md web/src/lib/controller/{http-local-index-data-source,http-store-data-source,http-store-write-data-source,native-data-source,production-index-preparation,production-read-services,production-store-index}.ts web/src/lib/controller/managed-index-recovery.svelte.ts web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte web/tests/vite/managed-code-index.test.ts
```

**Notes**

.local/bounded-reads/managed/format-slice-final.log; later documentation and compatibility additions were formatted explicitly.
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
pnpm check:progress
```

**Notes**

.local/bounded-reads/managed/progress-slice-final.log: 139 tasks; bookkeeping only.
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
pnpm exec prettier --check spec/src/milestones.md spec/src/cli.md spec/src/compatibility.md spec/src/mutations.md spec/src/ui.md docs/implementation-plan.md
```

**Notes**

.local/bounded-reads/managed/format-ledger-slice-final.log: formatted ledger and mapped chapters.
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

.local/bounded-reads/managed/canonical-ledger-final.log: 5 validators, 23 chapters, 172 requirements after E080.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Reuses the pinned standard JJ builder and per-operation associations proven in
  E079. All bookmarks share one graph namespace; metadata/reference/content
  reads retain their independent bounded paths. No new repository watcher,
  search bootstrap or authority projection.
- One shared native/local queue has at most 64 retained entries and 64 pending
  sources. Foreground exact selections remain; background single-head selections
  coalesce. Construction uses the existing reserved preparation worker, a total
  one-hour enqueue deadline, cancellation-aware authorization and bounded Busy
  backoff. Teardown cancels; readiness adoption rechecks source authority.
- Managed cache reservations share existing slots, repository/global bytes and
  file-count limits. Active construction reserves its ceiling; inactive
  namespaces charge actual files, survive restart, and permit bounded eviction.
  Source segments are checksum-validated; repair occurs only in the private
  copy. Process-resident handles use existing LRU admission.
- Existing monitors retain first-open activation without browser/SSE leases.
  Startup discovers retained bindings and reconnects those monitors. Code
  witnesses alone enqueue upkeep. Already captured local heads are reused;
  transient Busy/deadline failures retain readiness and unfinished activation
  retries. Background local observations leave foreground headroom when capacity
  exceeds one; legacy freshness checks yield on Busy instead of adding pressure.
- Core tasks reuse exact-operation DTOs and fresh native epoch/local
  registration authorization. Additive LocalTrusted-only routes preserve
  authenticated principal-scoped and advanced preparation paths. Browser
  recovery uses the owning Svelte effect, retries the retained ordinary
  files/branches view once, excludes advanced views, and does not spin after
  cancellation/failure. Setup explains automatic maintenance.
- "Full local consumption exposed a compatibility defect: while core preparation
  was pending, point reads skipped an already ready advanced index. The exact
  failing regression passes after restoring that fallback. Parallel local
  qualification also exposed foreground contention from background metadata
  observers; the final admission correction is covered by a deterministic worker
  regression and the original cold flow. Assertions remain unchanged."
- "Initial implementation/fixture failures were corrected: duplicate
  prepare_inner, local/native Stats conversion, private process Result alias,
  16-byte principal identity, graph discriminator/required fields, generated
  capability drift, snapshot union typing and reactive fixture warning.
  Architecture caught a 303-line root controller; the effect moved into a
  dedicated helper without relaxing limits. The first contract run encountered a
  transient intermediate private-field compile error while a fix was being
  written; repeat the full registered capture lane against frozen final Rust
  source."
- All 520 earlier ledger entries remain verbatim and JSON-identical. Preserve
  failed-run logs. Inherited macOS linker unwind-table warning remains. No
  source repair, Linux metadata write, fixture reset/migration, installed CLI
  replacement, package publication/push, unrelated daemon restart or
  visual-baseline change.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Land accumulated E077-E080 into clean main, advance the clean 7470 preview
  source and cold-verify the original Linux app with its existing
  registration/fixture. Keep this temporary workspace open.
- Advanced search/evolution/copy/deep-query optimization and authenticated
  automatic maintenance remain deferred. Native Linux-sized full cold
  construction is not qualified; owned admission may reject standard builder
  allocation.
- Complete scale/RSS/latency/fault/RustFS/release qualification and visual
  baselines remain deferred. Focused checks do not promote the M13 milestone.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified managed lifecycle/UI slice, land into main and record original-dev-app cold acceptance separately.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

