---
schema: 1
id: E-READ-073
---

# E-READ-073 details 3

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
cargo test -p jj-store-service --lib --locked reads::preparation::tests
```

**Notes**

0.4s; 1 tests passed. .local/bounded-reads/performance/copy-family-final-preparation-073.log
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
cargo test -p jj-store-cli --test native_rpc --locked
```

**Notes**

15.26s; 22 tests passed. .local/bounded-reads/performance/copy-family-final-rpc-073.log
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
cargo test -p jj-store-cli --test native_lifecycle --locked
```

**Notes**

93.36s; 231 tests passed. .local/bounded-reads/performance/copy-family-final-lifecycle-073.log
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked
```

**Notes**

56.24s; 0 tests passed. .local/bounded-reads/performance/copy-family-final-packages-073.log
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
pnpm check:boundaries
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-boundaries-073.log
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
pnpm check:architecture
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-architecture-073.log
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
pnpm spec:first
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-spec-first-073.log
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
pnpm spec:validate
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-spec-validation-073.log
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
pnpm format:rust:check
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-rust-format-073.log
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
pnpm check:progress
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-progress-073.log
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
pnpm exec prettier --check spec/src/jj-integration.md spec/src/mutations.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

Completed; 0 tests passed. .local/bounded-reads/performance/copy-family-document-check-073.log
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
pnpm check:progress
```

**Notes**

5.76s; 0 tests passed. .local/bounded-reads/performance/copy-family-ledger-progress-073.log
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/jj-integration.md spec/src/mutations.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md spec/src/milestones.md
```

**Notes**

3.58s; 0 tests passed. .local/bounded-reads/performance/copy-family-ledger-format-073.log
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/mutations.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md
```

**Notes**

.local/bounded-reads/performance/copy-family-document-format-073.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- 'Exact before target fails once with Other(Limit("native inventory result")):
  five related copies amid96 unrelated copy roots and96 files exceeded the
  original full-inventory matching bound. After delivery selects exact ancestors
  and their descendants, excluding an unrelated parent branch of a merge
  descendant. Cold and warm backend delivery read exactly five native copy
  bodies; warm requests read zero inventory descriptors.'
- The original checked manifest stays explicit for browser, historical-operation
  and independent preparation providers. Cache keys include process-local
  source/repository scope, epoch and inventory root. Shared64MiB allocation
  leases follow graph/checked-root owners beyond eviction; eight shared permits
  and32 fixed five-minute root slots constrain concurrency and retention.
  Same-root construction gates hold no credentials or source handles.
- Complete additions share an unchanged graph or admit and clone only prior copy
  adjacency before extension/sealing. Nonretained baselines restart cold with
  spent descriptors and original deadline retained. Cold construction remains
  the ordinary two-million-descriptor bound; body hydration is absent from
  projection construction. Matching family nodes/work/scratch have separate
  fresh admission.
- Acknowledged staged copy parents and reverse children are append-only,
  admitted before upload and insertion. Captured sequence excludes later writes
  without whole-stage cloning. Exact repeated writes are idempotent; unknown
  uploads and foreign sessions do not enter family selection. An overlapping
  committed/staged identity still discovers committed descendants. Ordered body
  terms and placeholder boundaries retain upstream semantics.
- Compatibility providers still use their bounded complete-inventory path.
  Native MutationAccess implementations that do not provide the new source
  capability and remote RPC clients remain on compatibility delivery; this slice
  does not claim all command or RPC copy endpoints are optimized.
- The new pin test initially counted three warm requests against the one-request
  sixteen-read helper allowance. Reset counters between each request without
  weakening the existing bound; the same target passes. Two initial compiler
  issues (nonexistent direct service tokio_util import and wrong test Budget
  module path) were repaired. Inherited macOS linker unwind-size warning is not
  a functional failure.
- Preserve all513 prior ledger evidence objects and their original text
  verbatim. Main stays on the already verified E-READ-071 landing; current
  follow-up is only in the retained temporary workspace. No installed
  executable, dev service, fixture, Linux source, credentials, publication or
  migration changed.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Resumable cold native kind preparation beyond two million descriptors remains
  the highest-priority delivery target. Selected relation construction beyond
  conservative shared64MiB admission may need chunk/disk support.
- Deliver selected copy-family RPC/client and applicable native mutation
  capabilities; index get_copy_records DAG range/path selection instead of broad
  ancestor/tree hydration.
- General filtered-log continuation preserving indirect/missing edges,
  uncached/ambiguous evolution, indexed code/local application search and
  oversized comparison/change grouping/extreme-filter candidates remain.
- Complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification and
  Help/generated-reference/source-review refresh remain deferred. Inherited
  installation-reference drift stays open. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified E-READ-073 source/backend family slice with Jujutsu and continue resumable cold native kind preparation in the retained workspace. Do not repeat the completed E-READ-071 main landing. Full qualification stays separate.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

