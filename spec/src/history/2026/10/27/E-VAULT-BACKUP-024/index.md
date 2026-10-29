---
schema: 1
id: E-VAULT-BACKUP-024
---

# E-VAULT-BACKUP-024

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-024
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
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/27/E-VAULT-BACKUP-024/index.md
- spec/src/history/2026/10/27/E-VAULT-BACKUP-024/SUMMARY.nav.md
- spec/src/history/2026/10/27/index.md
- spec/src/history/2026/10/27/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/indexed_read_admission.rs#native_query_waits_for_the_publisher_then_reads_the_retained_generation
- crates/jj-vault/tests/indexed_read_admission.rs#sustained_publisher_contention_remains_a_bounded_busy_error
- crates/jj-store-cli/tests/executable_native.rs#native_git_bookmark_publication_matches_pinned_upstream_scope
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs#markdown_deltas_and_bound_task_cli_survive_loaded_and_fresh_sessions
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
pnpm check
```

**Notes**

Docker aggregate check lane on the integrated trunk with the rebuilt protobuf-provisioned image: PASS, exit 0 (run jj-store-validation-cfd26a9d-2d04-4460-82eb-2be0988a8c85): all steps passed — documentation including both re-recorded review checkpoints, spec validation (8 validators, 4478 chapters, 211 requirements), boundaries (27 packages), architecture, progress over the fresh records (186 tasks, 967 records), rustfmt, embedded-rustfs strict clippy, contracts, web/catalog checks and strict workspace clippy -D warnings.
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
pnpm dev:cargo -- test --workspace --locked
```

**Notes**

Native macOS ARM64 workspace Rust suite on the platform-qualified profile (Git 2.55.0, jj 0.45.1 — the M12 GB6 environment): 679 passed / 1 failed / 11 ignored across 68 completed targets before cargo fail-fast. The platform-qualified families pass completely: executable_native 22 passed / 0 failed / 3 ignored including all eight native_git_* tests that the container Git 2.43.0 cannot admit, and agent_execution 22 passed / 0 failed / 2 ignored with the repaired context_delivery staging and the intermittent metadata_recovery ok. The single failure is pre-existing and unrelated to this slice: rpc_tls_checks_trust_and_server_hostname panics with the rustls 0.23.45 process-level CryptoProvider ambiguity (native_rpc 27/1 in the workspace run and identically in an isolated -p rerun), a cross-platform rustls feature-resolution defect: an isolated Docker rerun on Linux (run jj-store-validation-e9ee4131-4d29-43df-9464-b537ca81aae2) reproduces the identical panic at 27 passed / 1 failed, so the ambiguity is not macOS-specific. Its Cargo.lock lineage (f58b9363, 2026-10-21) predates the slice and touches no code this work changed; resolving it requires an owner decision on the canonical rustls provider (exactly one of ring/aws-lc-rs) or explicit process-level provider installation, so it is assigned to the native RPC/TLS dependency family and reported rather than guessed here. The abort left later packages (including jj-vault) unreached in this run; jj-vault remains Docker-qualified at 33/33 with the 4/4 admission regression in E-VAULT-BACKUP-023.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --workspace --locked --no-fail-fast
```

**Notes**

Complete native macOS workspace enumeration (331 targets): 2482 passed / 5 failed / 61 ignored. Breakdown of the five: (1) rpc_tls_checks_trust_and_server_hostname — the cross-platform rustls CryptoProvider defect characterized above; (2-3) query::cache::reservation::tests reservations_share_native_repository_total_and_lifetime_admission and stale_reserved_generations_are_discarded_before_reuse — load-sensitive Busy-deadline timing flakes, isolated rerun 3 passed / 0 failed; (4) bound_unpublished_rejection_discards_only_the_matching_pending_update (jj-store-server stack_combinations) — load-sensitive flake, isolated rerun 1 passed / 0 failed; (5) contract_inventory_covers_security_methods_media_and_task_bodies — deterministic stale 202/88 pins against the deliberately landed 204-path surface, repaired and proven in E-CI-SUITES-FIX-003 (crate suite 50/0/1). All five predate this slice and none touches the read-admission contract; the three flakes and the rustls defect remain separately owned and reported.
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
pnpm docs:review http "<recorded review reason>" && pnpm exec prettier --write docs/maintenance/coverage.json && node scripts/docs.mjs check
```

**Notes**

The contract-pin edit invalidated the http documentation-review fingerprint (JST-DOC-002 firing as designed; the Docker check attempt jj-store-validation-b9cac74d-329e-4a98-8840-244fc942c9b3 failed closed with 'Documentation review required: http'). Re-reviewed the three topic pages (reference/transport.md, reference/security.md, guides/conversion.md): they describe transport/security/conversion behavior, none references the test count pins, and all remain accurate. Recorded the checkpoint, restored prettier formatting, and proved the complete documentation gate natively: validateReview OK, mkdocs --strict built in 7.34s, closure 144 files with links and anchors valid.
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
pnpm check
```

**Notes**

Docker aggregate check lane rerun on the final tree including the contract-pin repair and both re-recorded checkpoints (run jj-store-validation-f1024c81-e984-4762-8d2d-c581a4ce976f): PASS, exit 0 — documentation with all three re-recorded review checkpoints, spec validation (8 validators, 4478 chapters, 211 requirements), boundaries (27 packages), architecture, progress over the fresh records (186 tasks, 967 evidence records at source 20032512), rustfmt, embedded-rustfs strict clippy, contracts, web/catalog checks and the strict workspace clippy with -D warnings over the contract-pin repair. Two prior attempts were stopped by fail-closed gates before their product steps: jj-store-validation-ecb827d1-98fa-4b68-a3d5-b452bb402a0f at the managed-cargo storage preflight (51.0 GiB in the shared target cache volume over its 40 GiB per-target cap, left by the test lane workspace compilation; resolved through runner-owned maintenance — validate:clean --apply with reports retained plus docker volume rm of the rebuildable target volume, warm pnpm/cargo-registry/uv caches kept) and jj-store-validation-b9cac74d at the http documentation review above. All steps pass on the final rerun.
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
pnpm test
```

**Notes**

Docker aggregate test lane on the integrated trunk (run jj-store-validation-42f51c17-b070-408b-a97c-a370ac88c442): every step passed except the final workspace Rust step — test:governance 120 passed / 0 failed (the prepare.mjs provisioning repair proven), storage-harness 8/8, test:embedded-rustfs / test:managed-rustfs / test:local-trusted ignored RustFS gates green with the provisioned protocol compiler, contracts 180/180, web:test, catalog:test (190 files; 1208 passed | 1 retained skip), test:storybook as four fresh-browser shards (216 files; 1048 tests, 0 failed) and catalog:test:browser. The final managed-cargo workspace step stopped at agent_execution 21 passed / 1 failed / 2 ignored: the third occurrence of the intermittent load-sensitive metadata_recovery race (it passes isolated in Docker 2/2, run 20584db0, and passed twice in complete-binary runs: Docker spec:check 7dea0fdf and the native enumeration at 22/0/2). Cargo fail-fast left the executable_native environmental gap unexercised in this attempt; it stays characterized by spec:check run 7dea0fdf (14/8/3, all eight failures at the exact Git-2.55.0 capability gate against image Git 2.43.0). The lane was not rerun after the contract-pin repair because its final Rust step is deterministically blocked by that owned environmental gap; the repaired test is proven natively at crate scope (50/0/1) and the final tree is covered by the check-lane rerun.
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
node scripts/check-progress.mjs; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; pnpm exec prettier --check changed files
```

**Notes**

Record and ledger validation on the final committed tree: progress ok (186 tasks; 967 evidence records), Spec-first gate passed with 5 protected files and 0 canonical chapters through the unchanged-contract confirmations in E-CI-SUITES-FIX-003, Specification validated: 8 validators, 4478 chapters, 211 requirements, native documentation closure green (mkdocs --strict, 144 files, links and anchors valid), rustfmt clean on both edited Rust files and prettier clean on every changed JSON/JS/Markdown file.
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

No format, DTO or API changes; this entry records lane executions only. Committed records E-VAULT-BACKUP-023 and E-CI-SUITES-FIX-002 remain immutable; the aggregate results their Next fields anticipated are recorded here.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Phase 2 aggregate-gate completion on the integrated trunk (52b061c5 read-admission slice, 43f843bd repairs, 641b5fc0 landed thread-files plan): Docker check lane, Docker test lane and the platform-qualified native macOS workspace Rust suite. The Docker test lane's only failures are the eight separately owned native_git environmental refusals; everything else — governance, storage harness, embedded/managed RustFS, local-trusted, contracts, web/catalog units, four-shard Storybook interactions and catalog browser suites — is recorded from the actual run.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

User decisions remain: re-pointing the stale main bookmark, the powynzov fork disposition, validation-image Git 2.55.0 provisioning (or platform-scoping the native_git family) and the separately owned intermittent metadata_recovery race.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: Verification only. The bounded read-admission contract qualified by
  E-VAULT-BACKUP-022/023 (JST-LOCAL-007, JST-ARCH-007) is exercised unchanged by
  the recorded suites; no requirement text, format or authority changes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.6 remains Partial at its original complete cold
  application/search/restore gate. Integrated-trunk aggregate qualification does
  not close Desktop, provider, cross-platform, distribution or visual gates.

```
<!-- /evidence-field -->

