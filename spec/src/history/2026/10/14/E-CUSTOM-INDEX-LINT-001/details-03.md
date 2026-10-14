---
schema: 1
id: E-CUSTOM-INDEX-LINT-001
---

# E-CUSTOM-INDEX-LINT-001 details 3

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
pnpm dev:cargo -- test -p jj-store-jj --test local_stacks --locked --target-dir .local/build-targets/custom-index -- --exact multisided_text_conflicts_offer_in_app_resolution_and_preserve_other_paths
```

**Notes**

native-conflict-regression: 1 passed, 0 failed, 0 ignored across 1 binaries. See scope for focused lane boundaries.
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
pnpm dev:cargo -- test -p jj-store-jj --lib --test local_stacks --test local_evidence --test native_codec --test compatibility --locked --target-dir .local/build-targets/custom-index -- --test-threads=1
```

**Notes**

native-tests-serial: 141 passed, 0 failed, 3 ignored across 5 binaries. Serial execution avoids global transaction/resource contention; 3 existing ignored tests remain separately governed.
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
pnpm dev:cargo -- test -p jj-store-service --lib --test workflow_store --test work_store --test content_search --test collab_pagination --test agent_sessions --test review_threads --test agent_checkpoints --test agent_relations --test native_collab --test local_work --test local_collab --test local_agent --test local_agent_pages --locked --target-dir .local/build-targets/custom-index
```

**Notes**

service-tests: 170 passed, 0 failed, 0 ignored across 14 binaries. See scope for focused lane boundaries.
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
pnpm dev:cargo -- test -p jj-store-wal --test workflow_store -p jj-store-rpc --test work --test collab --test review --test application_mutations -p jj-store-http --lib --locked --target-dir .local/build-targets/custom-index
```

**Notes**

transport-tests: 49 passed, 1 failed, 1 ignored across 1 binaries. See scope for focused lane boundaries. Pre-existing path-count fixture expected 196 while both checked-in OpenAPI and source expose 200; corrected counts and final transport lane passes.
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
pnpm dev:cargo -- test -p jj-store-http --lib --locked --target-dir .local/build-targets/custom-index -- --exact contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies
```

**Notes**

http-contract-regression: 1 passed, 0 failed, 0 ignored across 1 binaries. See scope for focused lane boundaries.
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
pnpm dev:cargo -- test -p jj-store-wal --test workflow_store -p jj-store-rpc --test work --test collab --test review --test application_mutations -p jj-store-http --lib --locked --target-dir .local/build-targets/custom-index
```

**Notes**

transport-tests-final: 86 passed, 0 failed, 1 ignored across 8 binaries. See scope for focused lane boundaries.
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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index
```

**Notes**

server-tests: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. Interrupted after concurrent fixture startup failures; three owned cold-process test children terminated.
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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index -- --test-threads=1
```

**Notes**

server-tests-serial: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. Isolation run deliberately stopped after the previously failing OAuth cases passed; no assertion failure in this partial run.
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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index -- --test-threads=4
```

**Notes**

server-tests-bounded: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. Four-worker fixture initialization also exceeded existing startup budgets; interrupted and its owned child terminated.
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
pnpm dev:cargo -- test -p jj-store-server --lib --test configuration --test capture_management --test capture_admission --test remote_agent_configuration --locked --target-dir .local/build-targets/custom-index -- --test-threads=1
```

**Notes**

server-tests-qualified: 0 passed, 0 failed, 0 ignored across 0 binaries. See scope for focused lane boundaries. User-requested pause: interrupted with SIGINT before full completion; partial passing cases do not qualify the full server suite.
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
pnpm dev:cargo -- fmt --all
```

**Notes**

All Rust changes formatted. The previously unused local-stack Apply result is discarded; execution and later refreshed source selection are unchanged.
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

Unchanged native collaboration codecs and object hashes; unchanged HTTP/RPC/OpenAPI/TypeScript shapes. Boxed Rust payloads alter memory layout only. Prepared index disk formats, source IDs, limits and retained pins remain identical.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit user-authorized pre-existing lint cleanup: transparent enum payload boxing, equivalent short-circuit guards/default initialization, array decoding, owned input contexts for query windows, local mutation authority and capture storage lifetime, iterator/map entry use and named aliases. No blanket lint suppression or altered source admission. Relevant library/direct suites expand because shared model types feed service/server/transport and local/native query consumers.

The user explicitly requested stopping full tests and integrating into the main workspace on 2026-10-14. No further Rust qualification was launched. Passed focused lanes remain recorded separately from interrupted or deferred checks. M29 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Resume the deferred qualification when requested. Integration preserves the main workspace UI commits; deployment and production/platform qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-JJ-003/005, JST-LOCAL-003/004/005, JST-COLLAB-004/007,
  JST-API-003/006/011 and JST-CLI-004 are intended unchanged: payload
  indirection, query argument contexts and equivalent guards preserve native
  identities, ordering, source admission, codecs, authorization and serialized
  shapes. Affected model/native/service/transport tests and strict production
  library lint passed. Full server and generated-contract equality remain
  deferred; this pending record does not claim completed unchanged-contract
  qualification."

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

