---
schema: 1
id: E-M4-013
---

# E-M4-013

<!-- evidence-field:id markdown -->
## Id

E-M4-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-index-preparation-and-service-tasks
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs
- crates/jj-store-jj/examples/query_qualification.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-service/src/index_tasks.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/pagination.rs
- crates/jj-store-service/src/reads.rs
- docs/evidence/m4-index-preparation-audit.md
- docs/evidence/m4-index-preparation-budget.json
- scripts/qualify-query-resources.py
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_is_explicit_snapshot_bound_and_disposable
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_cancels_pending_work_and_retains_shared_slot_until_exit
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#graph_cursor_rebuild_reports_original_snapshot_after_later_publication
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_report_pinned_completion_restart_and_current_revocation
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_deduplicate_bound_cancel_and_drop_running_jobs
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_retention_expiry_and_failures_are_explicit
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_expiry_cancels_pending_storage_without_status_polling
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_preparation_
```

**Notes**

Before: 17 compile diagnostics identify absent explicit preparation, cancellation and inline-threshold interfaces. No tests ran; this is a missing-feature baseline.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_preparation_
```

**Notes**

Both native preparation fixtures pass after implementation: exact snapshot/cache loss/hard budget and cancellation/shared worker lifecycle.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_tasks_
```

**Notes**

Before service implementation: unresolved index_tasks import; no tests ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_tasks_
```

**Notes**

Initial three service task cases pass for ready/failed/canceled, retention, restart/revocation, coalescing, bounded execution and service drop.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_rebuild_reports_original_snapshot_after_later_publication
```

**Notes**

Before pagination integration: rebuild error loses the original cursor snapshot after a later publication.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_rebuild_reports_original_snapshot_after_later_publication
```

**Notes**

Exact regression passes with original complete snapshot in rebuild-required response and original cursor resume after preparation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_
```

**Notes**

All 7 preparation/task/cursor cases pass, including expiry without status polling.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_tasks_report_pinned_completion_restart_and_current_revocation
```

**Notes**

Expanded access fixture also rejects valid same-principal tokens for a different native repository on status and cancel.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
```

**Notes**

Optimized native graph fixture built with explicit cold admission/preparation.
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
python3 scripts/qualify-query-resources.py --scenario graphs --binary target/release/examples/query_qualification --output docs/evidence/m4-index-preparation-budget.json
```

**Notes**

Linear and wide-head histories each contain 50,000 commits. Cold admission, preparation and 8 prepared/warm graph reads pass. Whole process: 5.71 seconds elapsed, 5.22 CPU seconds, 296026112 peak RSS bytes. Scope excludes service, RustFS/network and browser costs.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, package boundaries, progress, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

216 Rust tests pass / 6 external cases intentionally ignored. All 78 native lifecycle cases, 15 governance tests and 3 storage-harness self-tests pass; no unresolved failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 216 Rust tests, rustfmt, mdBook and mapped spec-first checks.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-013 details 2](details-02.md)
