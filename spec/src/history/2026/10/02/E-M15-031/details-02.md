---
schema: 1
id: E-M15-031
---

# E-M15-031 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test collab_pagination materialization --locked
```

**Notes**

Before implementation: directory loaded Reaction root; three concurrent reads loaded 12 domain records versus 4 for one read. /tmp/jj-collab-perf-before.log.
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
cargo test -p jj-store-service --test collab_pagination --locked
```

**Notes**

Final 14/14: root reuse/scoped reads, refreshed manifest envelope, coalescing, epoch/cursor reset, count/byte/deadline bounds, TTL/eviction, corruption and failed-load retry, cancellation, changed roots, viewer filtering and revoked tokens. /tmp/jj-collab-perf-service-final.log. Intermediate compile repairs: Review Arc membership initialization, Cache visibility, private test codec calls. Initial limit error mapping regression 1/11 repaired without changing assertions.
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
cargo test -p jj-store-service --test collab_pagination --test native_collab --test agent_sessions --test content_search --locked
```

**Notes**

31/31 at that stage; final pager expanded to14 separately. Direct full-pin consumers retain membership, exact revisions, actor filtering, content scope and current authorization. /tmp/jj-collab-perf-service-qualified.log.
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
cargo test -p jj-store-wal --test agent_catalog --test review_catalog --test collab_persistence --locked
```

**Notes**

21/21; detached immutable memberships retain native audit/cold closure and persistence behavior. /tmp/jj-collab-perf-wal.log.
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
cargo test -p jj-store-rpc --test collab --locked
```

**Notes**

3/3 transport, exact target/image/receipt and reconstruction gates. /tmp/jj-collab-perf-rpc.log.
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
cargo test -p jj-store-http collab --locked
```

**Notes**

3 contract tests; this name filter does not exercise Thread HTTP handlers. Actual handler acceptance follows. /tmp/jj-collab-perf-http.log.
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
cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance --locked
```

**Notes**

2/2 actual HTTP router acceptance: roots, signed cursors, exact revisions, images, auth, limits and unknown identity;184 unrelated lifecycle tests excluded. /tmp/jj-collab-perf-http-acceptance.log.
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
cargo clippy -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc --lib --locked -- -D warnings
```

**Notes**

Pre-existing redundant_locals at jj-store-jj/src/local_browser/operations.rs:202. Exact source is unchanged in @-. /tmp/jj-collab-perf-rust-check.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc --lib --locked --no-deps -- -D warnings
```

**Notes**

Pre-existing collapsible_if diagnostics in service/content_search/extract.rs:25,82,110 under Rust1.98.1. Parent source verified; no lint suppressions or unrelated source changes. /tmp/jj-collab-perf-rust-focused-check.log.
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
cargo check -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc --lib --locked
```

**Notes**

Owning Rust library checks pass. /tmp/jj-collab-perf-package-check.log.
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
cargo fmt --all -- --check; pnpm exec prettier --write spec/src/api.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/mutations.md spec/src/publication.md
```

**Notes**

Focused source/document formatting, /tmp/jj-collab-perf-format.log and doc-format.log.
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
cargo build -p jj-store-cli --bin jj-store --locked
```

**Notes**

Explicit development server built; embedded storage binary reused. /tmp/jj-collab-perf-server-build.log.
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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-perf-server-main node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Owned main service restarted85963-&gt;471/storage561;3633ms. Repo snapshots, directory, thread/messages, fixture and installation unchanged. Served prior built UI hash95a71cd6 matches; search ready. /tmp/jj-collab-perf-restart.log.
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
node /tmp/jj-thread-read-benchmark.mjs /tmp/jj-thread-perf-server-benchmark.json
```

**Notes**

5 sequential samples per endpoint in the unchanged dev profile, server already warm from restart preservation reads. Median directory51ms/detail40ms/timeline58ms/reaction56ms/emoji45ms, versus902/915/903/1064/1009ms baseline. First Reaction materialization134ms and first emoji42ms; these are initial-domain samples, not a fully cold browser/server measurement. No writes.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Mapped canonical chapters and ledger checks pass; focused document formatting. /tmp/jj-collab-perf-{spec-first,spec-validate,progress,ledger-format}.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test collab_pagination one_entry_cache_still_serves_queries_that_need_two_domain_roots --locked
```

**Notes**

Before capacity correction: one retention slot with two initializing target domains returned Limit. /tmp/jj-collab-perf-capacity-before.log.
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
cargo test -p jj-store-service --test collab_pagination one_entry_cache_still_serves_queries_that_need_two_domain_roots --locked; cargo test -p jj-store-service --test collab_pagination --locked; cargo check -p jj-store-service --lib --locked
```

**Notes**

Extra roots now validate request-locally without entering a saturated cache, retaining existing per-pin/admission bounds. Exact capacity regression and final15/15 pagination pass; service check passes. /tmp/jj-collab-perf-capacity-{after,check}.log. No format/transport changes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Frontend parallel loading, targeted refresh, query caching/cancellation,
  bounded reaction reads and indexed replies are implemented in E-M15-032; final
  live rebuilt-main UI timing awaits the shared development runtime choice.
- Clippy parent diagnostics remain explicit; package checks and focused behavior
  gates pass. No full-suite qualification or visual baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust read materialization service plus direct HTTP/RPC and data-only WAL selections. The WAL selection readers and both transport consumers justify their focused direct-consumer checks. Root aggregates, full workspace/lifecycle suites, publication format changes and unrelated UI story suites are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Immutable source graphs and directory/exact-target indexes are shared by repository+epoch+exact RecordRef. Bounded single-flight OnceCells contain no sessions, credentials or viewer results. Agent/Review membership selections detach from coordinators and reattach the current reader. Complete snapshot envelopes/auth remain fresh. Existing signed cursor, WAL/native/HTTP/RPC formats unchanged.
<!-- /evidence-field -->

