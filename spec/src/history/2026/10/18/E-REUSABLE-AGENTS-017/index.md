---
schema: 1
id: E-REUSABLE-AGENTS-017
---

# E-REUSABLE-AGENTS-017

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-017
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-LOCAL-003
- JST-LOCAL-004
- JST-DOC-001
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/code_index/operation_reads.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/tauri-plugin-jj-store/README.md
- crates/tauri-plugin-jj-store/tests/admission.rs
- crates/tauri-plugin-jj-store/tests/operations/mod.rs
- docs/maintenance/reusable-agent-workflows-progress.md
- docs/site/technical/local.md
- packages/jj-history/src/generated.ts
- packages/jj-history/src/validate.ts
- packages/jj-history/tests/client.test.mjs
- packages/jj-history/tests/types.ts
- scripts/history-contracts.mjs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/jj-integration/14-bounded-application-graphs.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-017/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-017/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p tauri-plugin-jj-store --test admission --locked --target-dir .local/build-targets/public-release operations::prepared_operation_pages_and_parent_deltas_preserve_source_and_pin -- --exact
```

**Notes**

Initial test module path produces E0583; move it to the normal integration-root operations/mod.rs, without a path attribute. Before production changes the exact regression then fails with IndexRebuildRequired. Logs oplog-producer-before.log and oplog-producer-before-query.log.
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
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 pnpm test:desktop:api
```

**Notes**

Notes consumer cold native API: admission/backend cases pass, operation graph fails index_rebuild_required after an external source advance. This reveals the producer allowlist gap; no consumer fallback is added. repository-views-native-api.log in Notes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm --filter @lapismd/jj-history build
node --test --test-name-pattern="operation differences" packages/jj-history/tests/client.test.mjs
```

**Notes**

A real native difference frame is accepted, but missing/unknown status tags are also accepted because reference siblings are ignored. One exact client case fails before the generator/validator correction; oplog-client-before.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p tauri-plugin-jj-store --test admission --locked --target-dir .local/build-targets/public-release operations::prepared_operation_pages_and_parent_deltas_preserve_source_and_pin -- --exact
```

**Notes**

Exact query regression passes. Original pin, parent pages/deltas, future-operation refusal, invalid parent refusal, root and dirty source preservation pass. A strengthened corrupted-source-index proof follows in the final record; oplog-producer-after.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p tauri-plugin-jj-store --locked --target-dir .local/build-targets/public-release
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features local-history --test local_history --locked --target-dir .local/build-targets/public-release
```

**Notes**

All nine plugin cases and the independent real-JJ local-history case pass. Exact origin/window/vault/generation admission, cancellation/drain and checked schema remain covered. oplog-producer-tests.log and oplog-local-history-tests.log.
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
node scripts/history-contracts.mjs --write
pnpm --filter @lapismd/jj-history test
pnpm --filter @lapismd/jj-history check
```

**Notes**

All eight client cases and strict built-export/type consumers pass. Referenced enum tags are retained in generated intersections and validated alongside their payload; malformed tags fail closed. No schema/wire change. oplog-client-after.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test -p tauri-plugin-jj-store --locked --target-dir .local/build-targets/public-release
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 pnpm dev:cargo -- clippy -p tauri-plugin-jj-store --all-targets --features tauri --locked --target-dir .local/build-targets/public-release -- -D warnings
```

**Notes**

Final nine plugin cases and strict Tauri all-target Clippy pass with OperationChanges. The strengthened real-JJ case deliberately corrupts source-derived indexes and verifies they remain byte-identical while pinned operations, exact deltas and groups still read successfully. Logs oplog-producer-final-tests.log, oplog-producer-final-clippy.log and oplog-producer-index-proof.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm progress:history
pnpm check:progress
pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book
pnpm exec spec-validator first
```

**Notes**

Initial task/nav generation rewrites unrelated projections and causes fresh-evidence coverage failures in progress/spec-first. Five selected validators pass (4276 chapters, 207 requirements). Restore unrelated projections, retain M33/M33.4 and new evidence navigation, and format before final document consumers. One combined failed documentation attempt; no product failure waived.
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
pnpm check:progress
pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book
```

**Notes**

Second documentation pass exposed a source-qualified record incorrectly retaining consumer gaps, an unrelated M33.1 formatting diff, and a 403-line governance page. Scope remaining gaps to the separate consumer task, restore M33.1 and keep governance within 400 lines. Final rerun follows.
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
pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book
pnpm exec spec-validator first
```

**Notes**

Five validators pass after restoring the 400-line governance limit (4264 chapters, 207 requirements); spec-first identifies the newly required jj-integration chapter missing from evidence inventory. Add that owning chapter and all final files to this record. Logs oplog-spec-4.log and oplog-first-4.log.
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
pnpm check:progress
pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book
pnpm exec spec-validator first
```

**Notes**

Final progress: 171 tasks and 893 evidence records; five validators: 4264 chapters and 207 requirements; spec-first: 12 protected files and five canonical chapters. Formatting runs after generation; unrelated generated changes restored. Logs oplog-progress-qualified.log, oplog-spec-qualified.log and oplog-first-qualified.log. No root aggregate Rust/UI/service tests are needed for this focused producer boundary.
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

Enclosing Jujutsu change qsrlrzvs, based on 3f14cf91. Rust 1.99.0, pnpm 10.34.5, Node 26.9.0; macOS ARM64; stock JJ 0.45.1 profile.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing requests/responses, status tags, exact operation identities and integer wire encodings are unchanged. The prepared code-index reader admits only basic unfiltered operation graphs, exact parent deltas and grouped revision changes. Other advanced queries retain IndexRebuildRequired. Generated client types now retain existing reference-sibling tags; malformed frames are refused.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

NOTES-05F producer prerequisite. Keep original prepared pin/physical generation/fingerprint checks and scoped worker/drain. Parent-edge proof admits requested historical operations; bounded operation/view and commit-parent reads compute deltas/groups without source indexes/trees or a full-repository fallback. Source allowlist/decoder exact cases precede plugin and local-history direct consumers. Root UI/server aggregates are out of scope; Notes UI and cold native qualification are separate. No installed apps, user data, release or baseline changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete Notes Column Canvas/AppShell body, identifiers, expanded file/diff UI and cold consumer acceptance. NOTES-05F remains the sole Active task. Other advanced queries and managed conversation/restart remain separate gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-003/004 operation pinning and read-only authority now
  explicitly qualify basic operations from prepared code-index handles.
  JST-ARCH-007 records the narrow Tauri capability and faithful existing JSON
  Schema reference constraints. Existing schemas, permissions and source
  authority remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33 and M33.4 retain In progress and original gates. This is a producer
  prerequisite for the requested NOTES-05F view; Notes UI/native operation
  acceptance and managed conversation/restart remain open.

```
<!-- /evidence-field -->

