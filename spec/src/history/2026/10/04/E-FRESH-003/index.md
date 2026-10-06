---
schema: 1
id: E-FRESH-003
---

# E-FRESH-003

<!-- evidence-field:id markdown -->
## Id

E-FRESH-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Scoped application feed, committed monitors and semantic state-owner notices
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; notify 8.2; macOS ARM64; protocol V1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Application SSE V1 ready/changed/reset and scoped process-local jse1 cursor; native manifest/WAL/object and durable repository SSE formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/application_events.rs
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs
- crates/jj-store-http/src/application_events/authorization.rs
- crates/jj-store-http/src/application_events/monitors.rs
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/application_events/tests.rs
- crates/jj-store-http/src/application_events.rs
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-http/src/search.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/capture_management/settings.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_runtime/freshness.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/local_repositories/watcher.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-server/src/replication/freshness.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-server/src/search_management/mod.rs
- crates/jj-store-server/src/server_settings.rs
- crates/jj-store-server/src/storage_management/mod.rs
- crates/jj-store-server/src/sync_management.rs
- crates/jj-store-server/src/workspaces.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/events.rs
- crates/jj-store-service/src/freshness/native.rs
- crates/jj-store-service/src/freshness.rs
- crates/jj-store-service/src/index_tasks.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_browser.rs
- crates/jj-store-service/src/local_index_tasks.rs
- crates/jj-store-service/src/local_management.rs
- crates/jj-store-service/src/search_management.rs
- crates/jj-store-service/src/server_settings.rs
- crates/jj-store-service/src/storage_management.rs
- crates/jj-store-service/src/sync_management.rs
- crates/jj-store-tantivy/src/paging.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/adapters.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
- spec/src/compatibility.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/application_events/tests.rs#shared_installation_monitor_replays_scoped_changes_and_unchanged_renewal_is_read_free
- crates/jj-store-service/src/freshness.rs#unchanged_witness_does_not_notify_and_replay_overflow_requires_reset
- crates/jj-store-cli/tests/native_lifecycle/application_events.rs#application_events_share_committed_observation_keep_checkpoint_narrow_and_reauthorize_delivery
- crates/jj-store-server/src/capture_runtime/freshness.rs#scheduling_only_capture_changes_are_not_semantic_freshness
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
cargo test -p jj-store-http --lib --locked
```

**Notes**

18 library and contract cases passed after integrating bounded-read follow-ups.
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
cargo test -p jj-store-server --lib freshness --locked
```

**Notes**

2 semantic capture/sync producer suppression tests passed; linker emitted inherited compact-unwind warning.
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
cargo test -p jj-store-cli --test native_lifecycle application_events --locked
```

**Notes**

Actual two-subscriber native feed, external committed publication within five seconds, narrow checkpoint Storage event and current revocation passed.
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
cargo test -p jj-store-cli --test native_lifecycle index_tasks --locked
```

**Notes**

5 exact preparation/retention/cancellation regressions passed after retirement timer lifecycle fix.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks_acceptance -- --nocapture
```

**Notes**

5 local preparation/retention/cancellation regressions passed. Corrected the previously misnamed local_index_tasks filter; exact integrated repeat recorded in E-FRESH-005.
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
pnpm contracts:check
```

**Notes**

257 actual HTTP/SSE responses conformed; 7 contract tool tests passed. Before integrating later DTO-preserving read-cache commits.
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
cargo clippy -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-rpc -p jj-store-tantivy --no-deps --lib --locked -- -D warnings -A clippy::large_enum_variant
```

**Notes**

Affected integrated library clippy passes with the one inherited service large-enum exemption and no dependency linting. This is not an unqualified strict-clippy pass.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --lib freshness --locked
```

**Notes**

New audience regression used a nonexistent constructor. Fixed to PrincipalId::new without changing the assertions.
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
cargo test -p jj-store-service --lib freshness --locked
```

**Notes**

2 final hub tests pass, including independent principal audiences.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Final integrated lint/hub and cold browser/real-RustFS/filesystem evidence
  pending; M20 remains In progress. Root aggregate lanes out of scope under
  targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete scoped browser consumers and cold idle/renewal/replay qualification.
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zrmqwtuqxqun
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


