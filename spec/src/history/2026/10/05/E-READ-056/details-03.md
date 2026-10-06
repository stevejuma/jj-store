---
schema: 1
id: E-READ-056
---

# E-READ-056 details 3

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
pnpm spec:first --file crates/jj-store-cli/tests/local_browser/preparation.rs --file crates/jj-store-cli/tests/local_browser/prepared_operation_admission.rs --file crates/jj-store-cli/tests/local_browser/prepared_operations.rs --file crates/jj-store-cli/tests/local_browser/prepared_points_http.rs --file crates/jj-store-cli/tests/local_browser.rs --file crates/jj-store-cli/tests/native_lifecycle/graphs.rs --file crates/jj-store-jj/src/local_browser/graphs.rs --file crates/jj-store-jj/src/local_browser/mod.rs --file crates/jj-store-jj/src/local_browser/operation_store.rs --file crates/jj-store-jj/src/local_browser/preparation/graph.rs --file crates/jj-store-jj/src/query/cache/constructed.rs --file crates/jj-store-jj/src/query/cache.rs --file crates/jj-store-jj/src/query/graphs.rs --file crates/jj-store-jj/src/query/index/secondary.rs --file crates/jj-store-jj/src/query/index.rs --file crates/jj-store-jj/src/query/language/operations.rs --file crates/jj-store-jj/src/query/mod.rs --file crates/jj-store-jj/src/query/operation_neighbors.rs --file crates/jj-store-jj/src/query/prepared/mod.rs --file crates/jj-store-jj/src/query/prepared/operations/children/tests.rs --file crates/jj-store-jj/src/query/prepared/operations/children.rs --file crates/jj-store-jj/src/query/prepared/operations.rs --file crates/jj-store-jj/src/query/projection.rs --file docs/implementation-plan.md --file spec/src/api.md --file spec/src/architecture.md --file spec/src/cli.md --file spec/src/jj-integration.md --file spec/src/local-jj.md --file spec/src/milestones.md
```

**Notes**

.local/bounded-reads/performance/main-operation-spec-stack-056.log; Spec-first gate passed: 24 protected file(s), 6 canonical chapter(s).
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
pnpm exec prettier --check docs/implementation-plan.md spec/src/architecture.md spec/src/milestones.md
```

**Notes**

All changed integration documents use Prettier style; main-operation-docs-format-final-056.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Combined main verification passes200 distinct Rust tests:36 complete local
  browser,43 complete native graph/cache/preparation,8 native
  reference/admission,57 JJ,35 service and21 HTTP units; exact1 local and3
  native regressions rerun separately. Two existing local large-fixture tests
  intentionally ignored. Affected JJ/service/HTTP/RPC/CLI checks, Rust format
  and package boundaries pass. No milestone promotion. Complete
  scale/RSS/latency/fault/cold-browser/RustFS/release and inherited
  Help/generated-reference/source-review refresh remain deferred. Existing
  generated installation-reference drift remains a recorded failing docs:check
  qualification gate. No dev service, installed executable, Linux source,
  fixture or unrelated workspace change.
- Indexed deep operation/log/evolution continuations; predecessor/copy/code
  search lookups; retained oversized comparisons/change groups/extreme filter
  candidate continuations; paged native RPC and remaining local application
  search/index remain feature delivery.
<!-- /evidence-field -->

