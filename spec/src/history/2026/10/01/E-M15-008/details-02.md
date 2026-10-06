---
schema: 1
id: E-M15-008
---

# E-M15-008 details 2

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
cargo test -p jj-store-service --test collab_pagination --locked (before)
```

**Notes**

Target absent; /tmp/jj-collab-pages-before.log.
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
cargo fmt -p jj-store-service
```

**Notes**

Repeated after production, regression and schema name edits.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service --all-targets --locked
```
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
cargo test -p jj-store-service --test collab_pagination --locked (initial)
```

**Notes**

Six focused cursor regressions; then added real-token and absent-domain regressions.
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
cargo test -p jj-store-service --test collab_pagination --test native_collab --locked
```

**Notes**

Eight pagination and nine native service regressions. Publication preserves old captured selections; exact query/actor/page bindings, tamper/size rejection, fixed expiry/restart/reset, capacity/object/byte/deadline budgets, real revocation, history/library membership and fail-fast concurrency. /tmp/jj-collab-pages-test-final.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --all-targets --locked -- -D warnings (initial)
```
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
cargo check -p jj-store-cli -p jj-store-rpc --all-targets --locked
```

**Notes**

Direct native read consumers compile.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-service --all-targets --locked -- -D warnings (final schema names)
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-service -- --check
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/api.md spec/src/collaboration.md spec/src/milestones.md
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Mapped mutations.md chapter missing; added explicit no-authority/no-publication and current revocation contract.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/api.md spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first (final)
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate (final)
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress (final)
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/api.md spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- HTTP/RPC/native CLI exposure and generated public contract acceptance,
  optional capture binding, reducer 3, canonical cross-source timeline/search,
  released shared UI and jj-acp integration remain pending.
- Native code still rejects Thread/Reaction writes until reducer 3. MemoryStore
  cursor/service evidence does not establish real RustFS, cold processes,
  browsers or runner acceptance. Root aggregate and unrelated Rust/UI suites are
  out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Expose the qualified pager through HTTP and native RPC, then native CLI; preserve reducer-3 and runner gates.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/collab_pagination.rs#resumed_pages_keep_all_source_roots_after_publication_and_bind_viewer_query_and_size
- crates/jj-store-service/tests/collab_pagination.rs#fixed_expiry_restart_and_capacity_require_explicit_reset_without_fallback
- crates/jj-store-service/tests/collab_pagination.rs#actor_aggregation_and_exact_target_remain_bound_across_reaction_pages
- crates/jj-store-service/tests/collab_pagination.rs#history_and_emoji_pages_keep_pinned_membership_and_query_kind
- crates/jj-store-service/tests/collab_pagination.rs#object_byte_deadline_and_configuration_budgets_fail_closed
- crates/jj-store-service/tests/collab_pagination.rs#concurrent_queries_fail_fast_without_waiting_for_pin_capacity
- crates/jj-store-service/tests/collab_pagination.rs#resumed_pages_reauthorize_current_tokens_and_never_store_credentials
- crates/jj-store-service/tests/collab_pagination.rs#absent_domains_stay_absent_and_final_pages_do_not_consume_pin_capacity
<!-- /evidence-field -->

