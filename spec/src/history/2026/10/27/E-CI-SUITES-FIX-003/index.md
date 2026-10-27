---
schema: 1
id: E-CI-SUITES-FIX-003
---

# E-CI-SUITES-FIX-003

<!-- evidence-field:id markdown -->
## Id

E-CI-SUITES-FIX-003
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

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-OBJ-004
- JST-DOC-002
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-object/src/s3/deletion.rs
- docs/maintenance/coverage.json
- scripts/validation/Dockerfile
- scripts/validation/prepare.mjs
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/27/E-CI-SUITES-FIX-003/index.md
- spec/src/history/2026/10/27/E-CI-SUITES-FIX-003/SUMMARY.nav.md
- spec/src/history/2026/10/27/index.md
- spec/src/history/2026/10/27/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies
- crates/jj-store-object/src/s3/deletion.rs#a_foreign_listed_key_is_rejected_before_any_delete
- crates/jj-store-object/src/s3/deletion.rs#unknown_put_outcomes_block_erasure_before_list_or_delete
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
pnpm check
```

**Notes**

Before. Docker aggregate check lane on the integrated trunk (run jj-store-validation-79c8fb5f-b26b-4c54-ab63-7c106b9c9674, old full image 486d1bdc34eb): the check:embedded-rustfs step failed before any lint because the Pulsar v6.9.0 build script panicked with 'Could not find protoc'. The full validation image never provisioned the protocol compiler that the hosted CI embedded-runtime producer explicitly installs and reports (ci-qualification-build.yml protobuf-compiler + protoc --version) and that the owning distribution chapter requires before compilation. No product or test source involved.
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
pnpm validate:images
```

**Notes**

Repaired scripts/validation/Dockerfile by adding protobuf-compiler to the full stage's existing apt provisioning line, exactly mirroring the hosted producer. Both fingerprinted images rebuilt on the omarchy daemon: full jj-store-validation:full-1d81b62d40c9b3ad62a0 (sha256:3ad5c9927513ccc0a2ab117c5d46df8323d8c25ac1e74e755c02e0de265d7819) and ui jj-store-validation:ui-f549aaa79262ad85c483 (sha256:5a3681e99bcd0d7b97d9a0882f4a53ff4ad672d965b091f383f7d2358d6fe66d). The ui stage retagged from fully cached layers; the full stage rebuilt only from the changed apt layer through the Rust tooling layers. Runner-owned cache volumes reseed under the new image fingerprint.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

Docker aggregate check lane rerun with the rebuilt image (run jj-store-validation-cd6c24f0-31c6-4279-b6a8-7288f21b85f8): the image repair is proven — check:embedded-rustfs passed with Pulsar v6.9.0 compiling against the provisioned protocol compiler — and the lane reached its final strict workspace clippy step, which then failed on two pre-existing collapsible_if findings in the jj-store-object S3 deletion test harness (crates/jj-store-object/src/s3/deletion.rs:197,300, lib test target under clippy 1.99.0 -D warnings). The code landed 2026-10-16 in c43a8761 and no clippy --all-targets lane had run over the unpushed trunk since; it is unrelated to this slice's changes.
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
pnpm dev:cargo -- clippy --locked -p jj-store-object --all-targets -- -D warnings; pnpm dev:cargo -- test --locked -p jj-store-object; rustfmt --edition 2024 --check crates/jj-store-object/src/s3/deletion.rs
```

**Notes**

Applied clippy's exact let-chain collapse at both test-harness sites (no semantic change; JST-OBJ-004 deletion refusals, fencing and disabled-destructive-delete behavior untouched). Native macOS proof: crate clippy --all-targets -D warnings clean, crate suite 10 passed / 0 failed including all three deletion mock-server tests, rustfmt clean.
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
pnpm docs:review publication "<recorded review reason>" && pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

The deletion.rs edit invalidated the publication documentation-review fingerprint (JST-DOC-002 operating exactly as designed; the aggregate docs:check step failed closed with 'Documentation review required: publication'). Re-reviewed the three topic pages (docs/site/technical/publication.md, docs/site/technical/storage.md, docs/site/concepts/snapshots.md) against the test-only style change: none references the deletion test harness and every publication/storage/snapshot statement remains accurate as written. Recorded the checkpoint, restored prettier formatting (2-line diff) and revalidated natively.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test
```

**Notes**

Docker aggregate test lane first attempt (run jj-store-validation-8739ef9a-1876-44f7-a5f0-35a1a0523ecc): the opening test:governance step failed 117 passed / 3 failed of 120 subtests. All three failures are the offline clean-install regressions (conversation-client 'fresh source workspace builds public client exports', history-client 'independent history archive preserves exact integers' and 'fresh source workspace builds independent history entries') whose `pnpm install --offline` could not resolve from the container store. The same three tests pass natively on macOS 3/3, proving the tests and fixtures sound. Cause: scripts/validation/prepare.mjs provisions the registry metadata and locked tarballs (`pnpm store add` plus `cargo fetch`) only when the requested lane is exactly test:governance, while the aggregate `test` lane nests it — the same nested-dispatch class E-CI-SUITES-FIX-002 repaired for Storybook sharding.
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
node --test scripts/conversation-client.test.mjs
```

**Notes**

Native macOS control for the governance failures: 3 tests / 3 passed / 0 failed including the offline fresh-workspace install regression. Repaired scripts/validation/prepare.mjs to provision the offline regressions for the aggregate `test` lane as well as the direct test:governance lane; prettier clean.
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
pnpm check
```

**Notes**

Docker aggregate check lane final rerun on the integrated trunk: PASS, exit 0 (run jj-store-validation-cfd26a9d-2d04-4460-82eb-2be0988a8c85, full image 3ad5c9927513). Every step passed end to end — documentation with mkdocs --strict (7.48s) over both re-recorded review checkpoints, spec validation (8 validators, 4478 chapters, 211 requirements), boundaries (27 packages), architecture, progress over the fresh records (186 tasks, 967 evidence records), rustfmt, embedded-rustfs strict clippy with the provisioned protocol compiler, contracts, web/catalog checks and the strict workspace clippy with -D warnings (41.17s final compile).
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --workspace --locked --no-fail-fast
```

**Notes**

The complete native macOS workspace enumeration (331 targets: 2482 passed / 5 failed / 61 ignored) exposed one deterministic non-flake failure beyond the owned rustls defect: contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies panicked at crates/jj-store-http/src/contract/tests.rs:358 with left 204 vs right 202, reproducible in isolation. Root cause is stale pins, not a contract change: c43a8761 (2026-10-16) deliberately added the confirmed repository-deletion routes /v1/local/repositories/{repo}/delete and /v1/local/repositories/{repo}/deletions/{request_id} and regenerated schemas/openapi.json to 204 paths, but the test exact-count pins stayed at 202 total / 88 local. No full Rust suite had run to completion on any platform since, so the drift stayed latent.
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
pnpm dev:cargo -- test --locked -p jj-store-http --lib; pnpm dev:cargo -- clippy --locked -p jj-store-http --all-targets -- -D warnings; rustfmt --edition 2024 --check crates/jj-store-http/src/contract/tests.rs
```

**Notes**

Aligned the stale inventory pins with the deliberately landed and regenerated 204-path surface: total 202-&gt;204 and /v1/local/ family 88-&gt;90; the non-local count stays 114 and every named-route, operationId-uniqueness and per-route assertion is untouched, so the test stays exactly as strong. Native macOS proof: jj-store-http lib suite 50 passed / 0 failed / 1 ignored, crate clippy --all-targets -D warnings clean (48.73s), rustfmt clean.
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
pnpm docs:review http "<recorded review reason>" && pnpm exec prettier --write docs/maintenance/coverage.json && node scripts/docs.mjs check
```

**Notes**

The contract-pin edit invalidated the http documentation-review fingerprint (JST-DOC-002 firing as designed; the Docker check attempt jj-store-validation-b9cac74d-329e-4a98-8840-244fc942c9b3 failed closed with 'Documentation review required: http'). Re-reviewed the three topic pages (reference/transport.md, reference/security.md, guides/conversion.md): they describe transport/security/conversion behavior, none references the test count pins, and all remain accurate. Recorded the checkpoint, restored prettier formatting, and proved the complete documentation gate natively: validateReview OK, mkdocs --strict built in 7.34s, closure 144 files with links and anchors valid.
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
pnpm check
```

**Notes**

Docker aggregate check lane rerun on the final tree including the contract-pin repair and both re-recorded checkpoints (run jj-store-validation-f1024c81-e984-4762-8d2d-c581a4ce976f): PASS, exit 0 — documentation with all three re-recorded review checkpoints, spec validation (8 validators, 4478 chapters, 211 requirements), boundaries (27 packages), architecture, progress over the fresh records (186 tasks, 967 evidence records at source 20032512), rustfmt, embedded-rustfs strict clippy, contracts, web/catalog checks and the strict workspace clippy with -D warnings over the contract-pin repair. Two prior attempts were stopped by fail-closed gates before their product steps: jj-store-validation-ecb827d1-98fa-4b68-a3d5-b452bb402a0f at the managed-cargo storage preflight (51.0 GiB in the shared target cache volume over its 40 GiB per-target cap, left by the test lane workspace compilation; resolved through runner-owned maintenance — validate:clean --apply with reports retained plus docker volume rm of the rebuildable target volume, warm pnpm/cargo-registry/uv caches kept) and jj-store-validation-b9cac74d at the http documentation review above. All steps pass on the final rerun.
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

Native host macOS ARM64 (node v24.10.0, pnpm 10.32.1, jj 0.45.1, Rust 1.99.0, Git 2.55.0); remote omarchy Linux x86_64 Docker daemon (16 CPUs/62 GiB), lane resources 8 CPUs/24 GiB/2 GiB shm, ssh-cli transport. Rebuilt fingerprinted images: full jj-store-validation:full-1d81b62d40c9b3ad62a0 (sha256:3ad5c9927513ccc0a2ab117c5d46df8323d8c25ac1e74e755c02e0de265d7819), ui jj-store-validation:ui-f549aaa79262ad85c483 (sha256:5a3681e99bcd0d7b97d9a0882f4a53ff4ad672d965b091f383f7d2358d6fe66d). Integrated trunk: 43f843bd (Phase 2 repairs) plus 641b5fc0 (landed thread-files plan); lane snapshots record their own working-copy revisions.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Image recipe change only: one apt package added to the full validation stage. No stored format, DTO, lane command, run identity or public API change; image fingerprints rotate through the existing documented mechanism and reseal runner-owned cache volumes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The full local validation image provisions protobuf-compiler through its existing apt line, matching the hosted CI embedded-runtime producer, so check:embedded-rustfs and the embedded-storage test steps compile the Pulsar dependency in the container. Two pre-existing collapsible_if findings in the jj-store-object S3 deletion test harness are collapsed exactly as clippy 1.99.0 directs, unblocking the strict workspace clippy step; deletion semantics, fencing and refusals are untouched. (5) scripts/validation/prepare.mjs provisions the offline clean-install regression metadata for the aggregate test lane as well as the direct governance lane, closing the same nested-dispatch gap class inside the full-image preparation. (6) The jj-store-http contract-inventory test pins are aligned with the deliberately landed 204-path surface (c43a8761 confirmed-deletion routes; regenerated openapi.json), keeping every named-route and uniqueness assertion intact. Companion to E-CI-SUITES-FIX-002 (shard dispatch and cross-device test staging); together they make the documented Docker aggregate lanes executable end to end.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Provisioning Git 2.55.0 in the validation images (or platform-scoping the macOS-qualified native_git family) remains a separate owner decision; until then the container workspace Rust suite retains that owned environmental gap.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "The owning distribution chapter already requires the embedded-runtime
  producer to provision and report the protocol compiler before compiling its
  Pulsar dependency; the local validation image now matches that documented
  hosted behavior. JST-DOC-002 review invalidation operated as specified and the
  publication checkpoint was re-recorded after actual page review. JST-API-001
  contract surface is unchanged: the inventory test now pins the exact 204-path
  surface that c43a8761 landed and openapi.json already records. JST-OBJ-004
  deletion behavior is unchanged: the repair is a clippy-directed style collapse
  inside cfg(test) mock-server code with the owning crate suite green.
  JST-ARCH-005 release qualification, publication authority, receipts, coverage
  semantics and hosted CI selection are untouched."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress at its existing release gates; local
  validation-image maintenance promotes no hosted qualification, publication or
  platform gate.

```
<!-- /evidence-field -->

