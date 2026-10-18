---
schema: 1
id: E-FILE-HISTORY-001
---

# E-FILE-HISTORY-001

<!-- evidence-field:id markdown -->
## Id

E-FILE-HISTORY-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-LOCAL-004
- JST-JJ-003
- JST-JJ-005
- JST-DOC-002
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/file_predicates.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/native.rs
- crates/tauri-plugin-jj-store/src/file_history.rs
- crates/tauri-plugin-jj-store/src/lib.rs
- crates/tauri-plugin-jj-store/src/service.rs
- crates/tauri-plugin-jj-store/tests/admission.rs
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs
- crates/tauri-plugin-jj-store/tests/operations/mod.rs
- docs/maintenance/coverage.json
- docs/maintenance/reusable-agent-workflows-progress.md
- docs/site/technical/local.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/21-filesystem-browser-reads.md
- spec/src/current/contracts/local-jj/13-rich-history-query-delivery.md
- spec/src/current/progress/M33/checkpoints.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/18/E-FILE-HISTORY-001/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-FILE-HISTORY-001/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/tauri-plugin-jj-store/tests/file_history/mod.rs#file_history_matches_jj_across_path_batches_merges_and_deletion
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs#file_history_retains_native_conflicted_merge_semantics
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs#file_history_keeps_spaces_quotes_and_pattern_characters_literal
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
pnpm dev:clean -- --dry-run; pnpm dev:target:retire -- --target .local/build-targets/public-release --apply
```

**Notes**

User explicitly authorized retiring this protected target. Ordinary managed maintenance subsequently removed the eligible idle jj-store-forge-stacks target; free space reached 31.1 GiB. Running backend PID 22952 stayed protected. Sources, .local/dev-demo and native review data were preserved; no reset or deployment.
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
pnpm dev:cargo -- check -p tauri-plugin-jj-store --no-default-features; pnpm dev:cargo -- test -p tauri-plugin-jj-store --test admission --no-default-features
```

**Notes**

Initial affected package check and five existing admission cases pass. Logs file-history-check.log and file-history-native-tests.log.
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
pnpm dev:cargo -- test -p tauri-plugin-jj-store --no-default-features
```

**Notes**

Default parallel package attempt: revocation_preserves_other_windows_and_failed_preparation_is_observable saw Status::Failed(Busy) with shared resource pressure. This is retained as an unresolved parallel qualification limitation, not declared inherited or repaired. Full package serial qualification follows; explicit multithreaded lifecycle cases remain enabled.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p tauri-plugin-jj-store --no-default-features -- --test-threads=1
```

**Notes**

Final expanded oracle attempt used a raw quoted filename as the stock-JJ positional fileset; JJ correctly rejected the invalid CLI input. Repaired the oracle with root-file plus JSON literal encoding. Native implementation unchanged. Earlier two oracle cases had passed.
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
pnpm dev:cargo -- test -p tauri-plugin-jj-store --test admission file_history --no-default-features -- --test-threads=1; pnpm dev:cargo -- test -p tauri-plugin-jj-store --no-default-features -- --test-threads=1
```

**Notes**

All three real JJ oracle cases pass, then all twelve package cases pass (eight admission, one frames, one schema, two wire). Covers &gt;70 unrelated commits/multiple 64-commit changed-path batches, clean/conflicted merges, deletion, exact pagination/order, spaces/quotes/fileset-looking paths, retained pins after external writes, revoked reads and preserved source/index/head/dirty bytes. Final logs file-history-oracle-final.log and file-history-package-qualified.log.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features local-history --test local_history; pnpm dev:cargo -- clippy -p tauri-plugin-jj-store -p jj-store-jj --no-default-features --features jj-store-jj/local-history --all-targets -- -D warnings
```

**Notes**

Focused local-history case passes; final strict affected-package Clippy passes. Original query wire/schema and permission tests remain unchanged.
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
pnpm docs:review local 'Reviewed exact native file predicates under complete upstream changed-path coverage, bounded preflight and retained operation pins. Local manual documents derived preparation without source files, source index, heads or refs mutation.'
pnpm docs:review history-bridge 'Reviewed typed path-index-required read retry through private admitted changed-path batches at the original handle. Existing window/vault authorization, cancellation, generation, preparation timeout, schemas and empty default permissions remain intact.'
node scripts/docs.mjs render
```

**Notes**

Strict manual render initially refused the inherited history-client fingerprint. Parent/current source fingerprints both fd34e912f7b445c7efa525836b5310f715b6ab65d352aedea62af5ffe52b9787, proving no change in this slice. Reviewed unchanged client against its existing manual and acknowledged that source independently.
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
pnpm docs:review history-client 'Reviewed unchanged parent/current history-client source fingerprint fd34e912f7b445c7efa525836b5310f715b6ab65d352aedea62af5ffe52b9787 against existing typed-client manual. Constraint-preserving decoding, bounded exact integers, immutable scope and explicit handle closure remain documented; file predicates add no client schema or API.'
node scripts/docs.mjs render
```

**Notes**

Strict manual build passes: nine Mermaid diagrams in both themes without network, 144-file / 26716957-byte validated closure with links and anchors. No CLI/OpenAPI generation was required: their contracts are unchanged. An earlier evidence:new regenerated 566 navigation files; 547 unrelated outputs were explicitly restored before retaining only the new date/M33.4 navigation.
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
pnpm dev:cargo -- fmt --all -- --check; pnpm check:progress; pnpm exec spec-validator validate --only governance,fileLimits,verification,summary,book; pnpm exec spec-validator first
```

**Notes**

Rust formatting, 171-task/906-record progress, five scoped validators and 13-file/four-chapter spec-first pass. Initial evidence test references lacked #name and were repaired to exact Rust test names. Consumer cold ordinary app restores File history, reads three matching Note.md revisions, opens 7cb493e5 at retained bd3e4eca pin without false toast, and retains dirty note bytes; E-REPO-017 owns final UI evidence.
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

Enclosing Jujutsu change uvrmkunv on f1cb2225. JJ 0.45.1, Rust 1.99.0, Node 24.10.0, pnpm 10.32.1. Notes consumer enclosing mmtysxqu on 41f8c649; normal macOS ARM64 build.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identities, source objects, Git/JJ metadata, public version-one text frames, checked JSON schema, default permissions and cache format are unchanged. Existing prepared changed-path coverage is extended only in private derived storage.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Prepared local-history readers admit native files() only after complete upstream changed-path coverage. Preflight charges work/bytes/memory and checks current authorization/cancellation/deadline. The Tauri service extends private coverage at the original handle through existing blocking workers, preparation permit/cache admission and timeout. Only the typed path_index_required read condition retries. diff_lines() and cross-operation predicates remain disabled. Notes status spans preparation and the query; renderer never scans commits to approximate file history.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement requested Notes file-blame toolbar/view as NOTES-05I8; retain separate managed conversation/restart and broader native/platform/provider gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-ARCH-007/JST-LOCAL-004/JST-JJ-003/005: owning local-JJ, JJ
  integration and architecture chapters now qualify metadata-only native file
  predicates through exact retained pins. JST-DOC-002/003 manual and checklist
  describe derived preparation, not source mutation or broader native
  acceptance. No public schema, grant, publication, receipt or identity contract
  changes."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33/M33.1-M33.4 remain In progress; M33.5-M33.8 Planned. NOTES-05I
  remains the single Active umbrella. Native file-filter parity is a narrow
  capability slice and does not complete managed AgentStore/ThreadStore chat or
  restart gates; broader consumer/provider/platform and visual baselines remain
  separate.

```
<!-- /evidence-field -->

