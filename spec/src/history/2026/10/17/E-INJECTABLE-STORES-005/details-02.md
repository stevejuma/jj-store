---
schema: 1
id: E-INJECTABLE-STORES-005
---

# E-INJECTABLE-STORES-005 details 2

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
pnpm dev:cargo -- test -p jj-store-service --test native_collab --test review_threads --test work_store --locked --message-format=short
```

**Notes**

Before migration:21tests passed (native10/review6/work5), but managed postflight returned1 for target40.1GiB/free29.1GiB. No product test failed.
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
pnpm dev:cargo -- check -p jj-store-service -p jj-store --offline --message-format=short
```

**Notes**

No compilation: disk preflight could not complete locked workspace cleanup after the new service dependency was added. Corrected only service-&gt;native in Cargo.lock, preserving all resolved versions.
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
pnpm dev:disk; pnpm dev:clean -- --dry-run
```

**Notes**

Observed root40.1GiB/free29.0GiB; dryrun permits idle root workspace artifacts. Existing dev stack remains stopped under prior user approval; fixture/staged resources protected.
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
pnpm dev:clean -- --apply
```

**Notes**

First cleanup failed because the new service dependency had not yet been reflected in Cargo.lock. No live process or protected retained target was changed.
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
pnpm dev:clean -- --apply
```

**Notes**

After targeted lock metadata correction, free29.0-&gt;49.4GiB. No reserve override, fixture reset or unrelated process stop. Subsequent validation uses CARGO_INCREMENTAL=0.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-service -p jj-store --locked --message-format=short
```

**Notes**

Selected service/CLI and direct HTTP/RPC/server/capture dependency boundary compiled. consumer-check-2.log.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test local_work factory_facade_replays_sync_receipts_and_keeps_semantic_errors --locked -- --exact
```

**Notes**

Exact new regression passed1: original synchronous receipts replay through the async selected facade, status is identical, and Invalid retains its semantic category.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --test native_collab retained_library_handles_keep_the_session_permission_boundary --locked -- --exact
```

**Notes**

Exact new regression passed1: retained read-only session handles deny execute and receipt status without manifest changes.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-native --test factories unknown_after_apply_retains_original_receipt_and_replays_without_live_sources --locked -- --exact
```

**Notes**

Exact native uncertainty regression passed1 with three after-apply faults: unknown CAS, successful CAS with transport failure, and successful CAS with missing selected control/receipt records. Exact original receipt resolves after readable storage returns.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-service --lib --test local_agent --test local_agent_binding --test local_agent_pages --test local_collab --test local_review --test local_work --test conversion --test conversion_agent_discussion --test native_collab --test review_threads --test work_store --locked --message-format=short
```

**Notes**

151passed,0failed across the complete directly affected service families and service library tests. Wider Rust-only escalation follows shared command composition across CLI/capture/RPC/HTTP/server consumers; unrelated UI/root aggregate/release lanes are out of scope. consumer-service.log.
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/desktop/resources/jj-store-rustfs CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-server --test embedded_storage injectable_native_stores_replay_exact_receipts_after_cold_rustfs_restart --locked -- --ignored --exact --nocapture
```

**Notes**

Real embedded RustFS cold regression passed1 in3.98s: all five domains, exact serialized prepared Agent request with opaque result and atomic Thread binding, process restart, fresh NativeObjects/NativeSession, exact original receipts and unchanged selected manifest. macOS linker emitted its large __eh_frame warning; no test failed. HelperSHA256dc379b958e7b62f156ee5ed8595fdc490ae1240932c8d7b4f6c3de809b9a83e7.
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
node scripts/package-check.mjs
```

**Notes**

24coordinated packages retain contained source inputs, crate overviews and portable registry dependency metadata.
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
pnpm check:boundaries
```

**Notes**

23workspace package boundaries passed, including service-&gt;native provider composition.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress && pnpm spec:first && pnpm spec:validate
```

**Notes**

Progress162/855passed. Spec-first required a current-slice governance chapter for the service dependency allowlist. Updated the actual verification-scope owner; a previous sentence replacement had not matched the formatted source.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-rpc --test agent_publication --test collab --test review --test work --locked --message-format=short
```

**Notes**

7passed across the4directly affected RPC binaries. Agent binding, diagnostic gap, reconstructed transports, actor scoping and exact original receipts passed.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-capture --locked --message-format=short
```

**Notes**

Complete capture suite passed.185observed libtest passes include1printed isolated-child execution;184parent cases passed.3child fixture entries are intentionally ignored by parent discovery and invoked by their acceptance parents. No failure; no new harness or topology qualification claimed.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test local_agent --test local_agent_provenance --test local_agent_provenance_cli --test local_agent_recovery --test local_collab_cli --test local_review_files --test local_work --test native_collab_cli --test review_commands --locked --message-format=short
```

**Notes**

31passed across9affected CLI integration binaries. Exact local/native receipts, portable evidence, recovery, async Work dispatch and native selector behavior passed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store --test local_browser filesystem_runtime_registers_and_reads_without_native_storage --locked -- --exact
```

**Notes**

Cold local-folder HTTP executable regression passed1, including existing local Work/review command and status routes. No native storage required; source repository and companion boundaries preserved.
<!-- /run -->
<!-- /evidence-field -->

