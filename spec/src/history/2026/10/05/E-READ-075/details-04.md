---
schema: 1
id: E-READ-075
---

# E-READ-075 details 4

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
pnpm spec:first
```

**Notes**

1.72s; 0 tests passed. .local/bounded-reads/performance/kind-task-spec-first-final2-075.log
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

2.92s; 0 tests passed. .local/bounded-reads/performance/kind-task-spec-first-schema-075.log
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

5.15s; 0 tests passed. .local/bounded-reads/performance/kind-task-spec-validate-075.log
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
pnpm test:contracts
```

**Notes**

6.98s; 0 tests passed. .local/bounded-reads/performance/kind-task-type-contracts-075.log
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
pnpm test:contracts
```

**Notes**

3.85s; 0 tests passed. .local/bounded-reads/performance/kind-task-type-contracts-final-075.log
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
pnpm exec prettier --write scripts/contracts.types.ts
```

**Notes**

2.47s; 0 tests passed. .local/bounded-reads/performance/kind-task-type-format-075.log
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
pnpm web:check
```

**Notes**

23.12s; 0 tests passed. .local/bounded-reads/performance/kind-task-web-check-075.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Separate strict kind routes accept only explicit operation and
  operations/copies kind. Start returns202; status and bodyless cancel
  return200. Current Read/epoch gates all calls and only the owner may cancel.
  Same owner/repository/epoch/operation/kind running starts coalesce. Public
  bounded progress retains no IDs, cursor, source or credentials. Graph task
  contracts remain compatible.
- One reserved serving preparation worker covers native graph, local graph and
  kind tasks with non-queuing admission before source capture; no independent
  pool or queue multiplies background work. Interactive file capacity remains
  usable. Original checked historical root, total
  scan/read/byte/result/allocation policy and original preparation deadline
  survive background windows. Per-window source checks remain fresh.
- Supervised jobs own sessions and credentials only during active work. Pending
  I/O cancellation drops the window future while retaining its conservative work
  charge and no cursor/ID progress. Canceling becomes Canceled after actual
  exit; expiry needs no client polling. Teardown cancels actual workers and
  completed timers. Reports expire independently after completion, retain only
  bounded metadata, and restart returns task-reset. Expired jobs retain no
  unregistered retention timer. Scoped freshness hints occur at
  settlement/expiry, not each scan window.
- "New baseline fails one unresolved kind_tasks import before the API exists; no
  functional test runs. Intermediate compiler integration errors (child
  visibility, overbroad parameter visibility edit, identifier-only route
  registration and test Key conversion) were repaired. Initial formatter lookup
  failed because the new routes subdirectory was missing, then succeeded after
  creating it. Initial spec-first fails two mapped chapter requirements; updated
  CLI/governance chapters repair that gate. The generated consumer initially
  reports five TypeScript diagnostics because request DTOs are inline, not
  schema components; reference the actual route request type and preserve
  unsupported kind/budget rejection. The first HTTP suite reports two stale test
  expectations: 119 paths versus the correct122 after three additive routes, and
  required-field order; correct both exact expectations without changing the
  wire contract. All final substantive targets pass before commit."
- Lifecycle and transport evidence uses real native in-memory publication and
  verified records, distinct from scale, filesystem/RustFS, installed CLI or
  release qualification. No milestone is promoted.
- Preserve all515 previous evidence entries and their historical text verbatim.
  Main remains on E-READ-071 during this slice; the user explicitly requested
  landing E-READ-072 through this completed slice into main, focused
  verification there, and continued work in the retained temporary workspace. No
  dev service, installed executable, fixture, Linux source, publication or
  migration changes.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Integrate kind preparation into app recovery and remote RPC/CLI consumers;
  these separate tasks are now callable through native HTTP, but existing graph
  controls have not yet invoked them.
- Selected copy family RPC/client and applicable native mutation capability
  providers; indexed get_copy_records DAG range/path queries; larger copy graph
  chunk/disk support beyond conservative64MiB.
- General filtered log continuation preserving upstream indirect/missing edges,
  uncached/ambiguous evolution, indexed code/local application search, oversized
  comparison/change grouping and extreme-filter candidates.
- Complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification and
  Help/generated-reference/source-review refresh plus inherited
  installation-reference drift remain open.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified task/API slice with Jujutsu, land E-READ-072 through E-READ-075 into clean main, run focused combined source verification there, and continue app/remote integration in the retained workspace. Full qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

