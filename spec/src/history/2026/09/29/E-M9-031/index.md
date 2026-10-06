---
schema: 1
id: E-M9-031
---

# E-M9-031

<!-- evidence-field:id markdown -->
## Id

E-M9-031
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

collaboration-cli-and-live-hints
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/presentation.rs
- crates/jj-store-cli/src/review_commands.rs
- crates/jj-store-cli/src/review_presentation.rs
- crates/jj-store-cli/tests/review_commands.rs
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/review.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/review.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/review_mutations.rs
- crates/jj-store-service/tests/review_threads.rs
- docs/cli/review.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/review_threads.rs#review_command_wakes_idle_subscription_at_accepted_position
- crates/jj-store-rpc/tests/review.rs#local_peer_reads_and_replays_receipt_backed_review_commands
- crates/jj-store-cli/tests/review_commands.rs#review_commands_require_explicit_receipt_and_concurrency_identities
- crates/jj-store-cli/tests/review_commands.rs#review_list_show_and_status_preserve_full_stable_ids
<!-- /evidence-field -->

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
cargo check -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

The test-first compile required an offline lockfile refresh after the existing workspace jj-review package became a direct RPC dependency.
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
cargo check -p jj-store-rpc -p jj-store-cli --offline
```

**Notes**

The first implementation compile found that the shared review command request was deserialize-only; deriving the matching serialized public contract fixed the RPC boundary.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

The next compile identified the intentionally missing authored CLI help document and production CLI jj-review dependency edge; both were added without changing package ownership. The RPC uses the service/WAL facade rather than depending on jj-review directly.
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
cargo test -p jj-store-service --test review_threads review_command_wakes_idle_subscription_at_accepted_position --locked; cargo test -p jj-store-rpc --test review --locked; cargo test -p jj-store-cli --test review_commands --locked
```

**Notes**

The exact live-hint regression, LocalTrusted native RPC acceptance and both CLI grammar regressions pass. The accepted ReviewStore position reaches an already idle subscription; list/detail, durable status and identical replay retain complete typed identities; missing concurrency inputs are rejected.
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
cargo clippy -p jj-store-service --test review_threads --locked -- -D warnings; cargo clippy -p jj-store-rpc --lib --test review --locked -- -D warnings; cargo clippy -p jj-store-cli --lib --test review_commands --locked -- -D warnings; cargo fmt --all -- --check
```

**Notes**

The directly affected service, RPC and CLI targets are warning-free and all Rust is formatted. Production modules contain at most 296 physical lines and focused tests at most 306.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm check:architecture; pnpm spec:validate; pnpm spec:first
```

**Notes**

All 13 package boundaries, 200 governed web modules and 89 requirements passed; the same-change gate then required mapped architecture, JJ integration and mutation chapter updates for the new dependency, RPC and service contracts.
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
cargo test -p jj-store-service --test review_threads review_command_wakes_idle_subscription_at_accepted_position --locked; cargo test -p jj-store-rpc --test review --locked; cargo test -p jj-store-cli --test review_commands --locked
```

**Notes**

The final post-extraction run passes the one live-hint test, one LocalTrusted RPC test and two CLI grammar tests.
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
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed documentation>
```

**Notes**

All 13 package boundaries, 89 requirements, 19 protected files across six canonical chapters, 61 progress tasks and changed-document formatting pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; native RPC v1; ReviewStore page/detail and command/status JSON contracts v1; cursor token jsr1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds four typed native RPC methods carrying bounded canonical JSON for shared ReviewStore service models and the jj review list/show/create/reply/edit/tombstone/hide/resolve/reopen/status family. Commands retain explicit request IDs and exact optimistic-concurrency identities. Committed commands reuse repository-only process-local wake hints; selected WAL replay remains authoritative.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


