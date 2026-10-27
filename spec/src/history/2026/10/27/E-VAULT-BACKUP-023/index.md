---
schema: 1
id: E-VAULT-BACKUP-023
---

# E-VAULT-BACKUP-023

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-023
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-ARCH-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/27/E-VAULT-BACKUP-023/index.md
- spec/src/history/2026/10/27/E-VAULT-BACKUP-023/SUMMARY.nav.md
- spec/src/history/2026/10/27/index.md
- spec/src/history/2026/10/27/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/indexed_read_admission.rs#native_query_waits_for_the_publisher_then_reads_the_retained_generation
- crates/jj-vault/tests/indexed_read_admission.rs#sustained_publisher_contention_remains_a_bounded_busy_error
- crates/jj-vault/tests/indexed_read_admission.rs#native_query_never_creates_a_missing_projection
- crates/jj-vault/tests/indexed_read_admission.rs#native_query_never_repairs_corrupt_projection_bytes
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
pnpm build
```

**Notes**

Before. Native macOS build of the integrated trunk stopped at the documentation stage: validateReview threw 'Documentation review required: local' because the coverage.json local-topic fingerprint predated the jj-vault search/indexed read-admission sources landed by 52b061c5. One failure; no product compilation error.
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
pnpm docs:review local "<recorded review reason>" && pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

Repair. Reviewed the local topic pages (docs/site/technical/local.md, docs/site/guides/git.md, docs/site/guides/conversion.md) against the read-admission change: their architecture-level history/backup statements remain accurate and the canonical automatic-vault-backup chapter owns the admission detail, so no page content required change. The checkpoint records the specific review reason and refreshed fingerprint (JST-DOC-002 invalidation cleared); prettier restored the committed formatting for a 2-line diff and the native validateReview recheck passed.
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
node scripts/validation.mjs run validation:rust -- test --locked -p jj-vault --test indexed_read_admission -- --nocapture
```

**Notes**

Exact integrated-trunk regression in Docker, run jj-store-validation-fc39194d-2910-4106-b928-f16de8679fc3 (full image, source c61065b9): 4 passed / 0 failed in 5.07s, including the real five-second Busy deadline. Phase 1 recorded 4/4 only on the fork source; this is the first pass on the integrated default-workspace lineage.
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
node scripts/validation.mjs run validation:rust -- check --locked -p jj-vault
```

**Notes**

Docker run jj-store-validation-330c821d-04f6-4cb1-807f-543d6f8ffdb8: owning crate check clean.
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
node scripts/validation.mjs run validation:rust -- build --locked -p jj-vault
```

**Notes**

Docker run jj-store-validation-2b14d6e1-cbb1-4eff-980f-49967f3a3574: owning crate build clean.
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
node scripts/validation.mjs run validation:rust -- clippy --locked -p jj-vault --all-targets -- -D warnings
```

**Notes**

Docker run jj-store-validation-96371fb9-21d9-46d9-a77a-5afd37d33912: zero warnings under -D warnings.
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
node scripts/validation.mjs run validation:rust -- test --locked -p jj-vault
```

**Notes**

Docker run jj-store-validation-d3917b60-20c6-4e8b-b0b7-78a1451b6ba0: full owning crate 33 passed / 0 failed across unit, integration and doc tests, identical to the Phase 1 fork qualification.
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
pnpm build
```

**Notes**

Native macOS ARM64 rerun on the repaired tree: build-storybook, client build, vite web:build (1m33s), mkdocs --strict documentation build (9.33s) staged to web/dist/help, embedded-rustfs build (18m03s) and managed-cargo build --workspace --locked (898 crates) all succeeded; exit 0.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Docker aggregate on the complete slice, run jj-store-validation-7dea0fdf-c37e-4570-b671-8862c502ed72: all steps through storage-harness passed — validate, documentation (including the repaired coverage review), catalog-cold-browser, catalog-check, catalog-unit (190 files; 1208 passed | 1 retained skip), catalog-interaction as four fresh-browser shards (216 files; 1048 tests, 0 failed), catalog-build, browser-check/unit/interaction, contracts, rustfmt, boundaries, progress over the fresh records, governance-tests and storage-harness. The final rust-tests workspace suite passed the agent_execution target completely (22/0/2, including the repaired context_delivery) and then stopped at executable_native (14 passed / 8 failed / 3 ignored): all eight failures are native_git_* refusals at the exact requires-Git-2.55.0 product capability gate against the image Git 2.43.0. That macOS-qualified family (M12 GB6) is a pre-existing environmental gap owned by the validation image toolchain, unrelated to the read-admission slice; the jj-vault family stays qualified by runs 3-7 in this entry and the platform-qualified native workspace suite remains a named next gate.
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
node scripts/check-progress.mjs; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; pnpm exec prettier --check changed files
```

**Notes**

Record and ledger validation on the committed tree: progress ok (180 tasks; 964 evidence records), Spec-first gate passed with 4 protected files and 0 canonical chapters through the unchanged-contract confirmations in this entry and E-CI-SUITES-FIX-002, Specification validated: 8 validators, 4463 chapters, 208 requirements, and prettier clean on every changed JSON/JS/Markdown file. Rerun after the final run-note edits.
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

Native host macOS ARM64 (node v24.10.0, pnpm 10.32.1, jj 0.45.1, Rust 1.99.0, uv 0.10.0); remote omarchy Linux x86_64 Docker daemon (16 CPUs/62 GiB), lane resources 8 CPUs/24 GiB/2 GiB shm, ssh-cli transport; full image sha256:486d1bdc34eb0f4799bcd2d09ff726e78d737e2991cabf79212dd7029ed60b2e (jj-store-validation:full-585ee5c4eeb3d3866f6b), ui image sha256:9889f7bb0e260a76592ceabb5410c53fe723a097197e616b7929bf3c3d33b174 (jj-store-validation:ui-5f91ab85f47886aab9df). Source: working-copy revision c61065b9 (integrated 52b061c5 plus the same-change repairs); later lane runs record their own snapshot revisions.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored-format, public DTO or API changes; the indexed read constructor remains crate-private, re-confirming the E-VAULT-BACKUP-022 unchanged-format surface on the integrated trunk. coverage.json gains only the refreshed local-topic review reason and fingerprint.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Phase 2 qualification of the integrated history-read-admission slice (52b061c5) on the default workspace trunk: stale local documentation-review checkpoint repaired, exact Docker regression 4/4, crate check/build/clippy and the 33-test crate suite in Docker, the native full build, and the aggregate Docker spec:check lane green through storage-harness with the four-shard Storybook suite. The container workspace Rust suite is blocked only by the separately owned validation-image Git-2.55.0 gap in the macOS-qualified native_git family; the aggregate Docker check/test lanes and the platform-qualified native macOS workspace Rust suite run against the integrated trunk after the preserved thread-files plan lands, per the user landing priority. Sibling record E-CI-SUITES-FIX-002 owns the lane repairs (Storybook shard dispatch and the cross-device bundled-binary test link).
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the preserved thread-files plan on trunk (user priority), then run the aggregate Docker check and test lanes plus the macOS-platform-qualified native workspace Rust suite (Git 2.55.0, jj 0.45.1) against the integrated trunk and record the results as an update to this entry. Report-only decisions for the user remain: the stale main bookmark, the powynzov fork disposition, validation-image Git provisioning and the separately owned metadata_recovery race.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: Verification and documentation-review maintenance only. The bounded
  read-admission contract changed by E-VAULT-BACKUP-022 (JST-LOCAL-007,
  JST-ARCH-007) and the JST-DOC-002 review invalidation operate exactly as
  specified; no requirement text changes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.6 remains Partial at its original complete cold
  application/search/restore gate. Integrated-trunk full-lane qualification does
  not close Desktop, provider, cross-platform, distribution or visual gates.

```
<!-- /evidence-field -->

