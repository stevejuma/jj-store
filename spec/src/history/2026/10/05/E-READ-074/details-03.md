---
schema: 1
id: E-READ-074
---

# E-READ-074 details 3

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
pnpm check:architecture
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-architecture-074.log
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
pnpm spec:first
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-spec-first-074.log
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
pnpm spec:validate
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-spec-validation-074.log
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
pnpm format:rust:check
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-rust-format-074.log
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
pnpm check:progress
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-progress-074.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/mutations.md spec/src/architecture.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/kind-preparation-document-check-074.log
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
pnpm check:progress
```

**Notes**

4.28s; 0 tests passed. .local/bounded-reads/performance/kind-preparation-ledger-progress-074.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/mutations.md spec/src/architecture.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

3.02s; 0 tests passed. .local/bounded-reads/performance/kind-preparation-ledger-format-074.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- New source APIs begin at a fresh checked root or CommittedRead historical
  selection, advance at most256 descriptors per call, and finish only complete
  results under fresh source/epoch/permission/deadline checks. The initiating
  page result ceiling/lifetime does not replace separately admitted preparation
  limits. Unselected/staged objects and later selections cannot enter the pinned
  traversal.
- A caller-owned Preparation retains checked manifest, source scope, kind,
  cursor, identities, counters and shared allocation lease only. It retains no
  NativeSession, object store, source capability, principal, bearer, mutation
  guard or async job. Eight shared native inventory permits bound active
  windows; the shared64MiB pool charges each cold progress header and admitted
  matching-ID growth. Data owners retain charges after cache eviction. Fixed
  total descriptor/read/encoded-byte/result/allocation/lifetime limits never
  renew across calls.
- "Reserve each bounded window before I/O: at most256 descriptors,1024 record
  attempts and32MiB encoded record allowance. Successful verified windows
  reconcile to reported actual counters. Failed/canceled/unknown windows retain
  conservative charges and do not move cursor or IDs; retries cannot conceal
  spent work. Matching growth is admitted before insertion. Partial finish
  returns IndexRebuildRequired and promotes no complete projection."
- Terminal complete-root traversal moves identities and their existing
  allocation into the ordinary source-scoped kind cache. A concurrent caller
  proving the same immutable root can supply that complete data to a paused
  preparation without further descriptor reads or whole-set comparison under the
  cache lock. Complete owners share allocation; normal GET
  two-million-descriptor ceilings stay unchanged.
- The exact new baseline failed compilation with13 missing method/type
  diagnostics and two unused-import warnings while the new source API was
  absent; no functional tests ran. First implemented acceptance failed once
  because the fixture assumed602 descriptors while native bootstrap actually
  selected605. Derive the exact expected count from the checked original root,
  preserving all three256-sized windows and complete identity assertions; the
  same target then passes.
- Evidence uses verified native in-memory records and test-only selected
  descriptor/policy changes, distinct from guarded publication, transport
  delivery and real-RustFS qualification. A work-counter test crosses the
  ordinary two-million ceiling under the explicit total budget; it does not
  claim a cold two-million-object scale run.
- Preserve all514 prior evidence objects and original ledger text verbatim. Main
  remains on verified E-READ-071; E-READ-072 through E-READ-074 follow-ups
  remain in the retained temporary workspace. No installed executable, dev
  service, fixture, Linux source, publication or migration changes.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Wire callable kind preparation through bounded authenticated
  background-task/HTTP/RPC/client delivery and its request-local
  cancellation/progress controls. Existing graph preparation does not invoke
  this capability; no endpoint is claimed resumable yet.
- Deliver selected family RPC/client and native mutation capabilities, plus
  indexed get_copy_records DAG range/path lookup. Large copy graphs exceeding
  conservative64MiB admission may need chunk/disk support.
- General filtered-log continuation preserving indirect/missing edges,
  uncached/ambiguous evolution, indexed code/local application search and
  oversized comparison/change grouping/extreme-filter candidates remain.
- Complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification,
  Help/generated-reference/source-review refresh and inherited
  installation-reference drift remain open. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified resumable native kind source capability with Jujutsu, then deliver its bounded background-task/transport consumers before indexed copy/path queries. Keep the retained workspace open and main on its already verified landing. Full qualification stays separate.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

