---
schema: 1
id: E-CUSTOM-INDEX-001
---

# E-CUSTOM-INDEX-001 details 4

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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index -- --test-threads=1
```

**Notes**

server-tests-serial: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. Isolation run deliberately stopped after the previously failing OAuth cases passed; no assertion failure in this partial run.
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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index -- --test-threads=4
```

**Notes**

server-tests-bounded: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. Four-worker fixture initialization also exceeded existing startup budgets; interrupted and its owned child terminated.
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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index -- --test-threads=1
```

**Notes**

server-tests-qualified: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. User-requested pause: interrupted with SIGINT before full completion; partial passing cases do not qualify the full server suite.
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
pnpm test:governance
pnpm check:boundaries
pnpm spec:validate --only fileLimits
```

**Notes**

Final tooling rerun: 68 governance tests; 19 package boundaries; file limits pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Draft evidence initially omitted the newly cleaned local-stack test file; coverage added before final ledger checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Full server library and configuration/capture lanes paused at the user's
  explicit request; the final serial run ended with SIGINT before completing.
- Final generated-contract equality, CLI consumer check, documentation build and
  spec-first qualification remain deferred.
- Post-cleanup real-RustFS rerun and the latest test-only incremental-cost
  extension remain unexecuted; earlier isolated real-storage and warm-query
  measurements passed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; rustc 1.98.1; JJ 0.45.1; pnpm 10.32.1; redb 4.3.0. Rust runs use CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0 and the registered .local/build-targets/custom-index. Real storage uses the existing pinned embedded RustFS executable from the primary workspace.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New derived schema/row/posting version 1 and derived-index-v1 redb table. Additive custom_data_index_persistence in server configuration format 2, default true/restart-bound. Existing native model, codec, manifest, WAL, receipt, RPC/HTTP and cursor formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native server custom reads share admitted root projections. Persistent secondary candidates accelerate WorkflowStore runs/pending/evidence, WorkStore project/task facets and exact Unicode substring content search. Existing native Thread/Reaction/Agent/Review catalogs remain identity indexes; admitted projections/windows are shared. Review first requests retain a fresh complete provider closure audit. Cold persistence restore still audits source closure and verifies exact row agreement. Memory/disk pressure and failed persistence fall back within the same selected-root contract. No companion migration or model-cap increase.

Debug scoped-query p95 at 100/1,000/4,096/9,500 rows: scan 5,209/56,334/467,791/1,276,833 ns; index 4,875/23,625/143,583/356,125 ns. Exact predicates inspect 1/10/41/95 rows. This is an engine microbenchmark, not an HTTP SLA or cold admission measurement. Warm workflow repetition drops workflow-object reads from 2 to 0. Incremental rows mask exact replacements/removals and compact after four layers.

Resource bounds: 64 disk generations/2 GiB total, 128 MiB per file, 64 MiB derived payload/predecessor retention; Workflow hydration 64 MiB plus current caller inventory. Existing collection capacities remain unchanged. Root aggregate/UI/browser/release lanes are outside this Rust/read-index slice. Wider direct Rust tests are warranted by engine/service/WAL/server and shared enum consumers.

The user explicitly requested stopping full tests and integrating into the main workspace on 2026-10-14. No further Rust qualification was launched. Passed focused lanes remain recorded separately from interrupted or deferred checks. M29 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Resume the deferred qualification when requested. Integration preserves the main workspace UI commits; deployment and production/platform qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ADAPT-008 defines disposable exact-source secondary indexing; owning
  adapters/architecture/CLI/publication/mutation topics describe bounds and
  lifecycle. JST-WF-006/JST-WORK-005/009/JST-COLLAB-004/JST-API-009 preserve
  admission, pins, ordering, source authority and response semantics. No native
  publication or serialized format is replaced.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M29 reviewed In progress at the user-requested verification pause and
  local integration. Completed focused evidence is retained, full
  server/generated-contract/CLI/docs/spec-first gates remain pending.
  M9/M11/M18/M28 and provider/visual qualification are unchanged.

```
<!-- /evidence-field -->

