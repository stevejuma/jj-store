---
schema: 1
id: E-READ-019
---

# E-READ-019

<!-- evidence-field:id markdown -->
## Id

E-READ-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xnnmsulm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Separate native index construction policy and checked-root task capability
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Persistence/wire shapes unchanged. Trusted construction limits separate from interactive limits; loaded index ceiling4million, inline1000unchanged. Native task running timeout1hour; completed-report TTL5minutes from exit.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/preparation.rs
- crates/jj-store-service/src/index_tasks.rs
- crates/jj-store-service/src/reads/preparation.rs
- crates/jj-store-service/src/reads.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-004
- JST-PUB-002
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#explicit_preparation_has_its_own_work_budget
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#preparation_policy_enforces_its_own_limits
- crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#preparation_retains_the_root_without_a_page_membership_limit
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_running_deadline_is_independent_of_completed_retention
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_expiry_cancels_pending_storage_without_status_polling
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
cargo test -p jj-store-cli --test native_lifecycle --locked explicit_preparation_has_its_own_work_budget -- --nocapture
```

**Notes**

Before: preparation inherited interactive max_work and failed Limit. Source audit also found50kcommit/10sdefaults, growing100kmembership set and task TTL shared with construction.
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
cargo test -p jj-store-cli --test native_lifecycle --locked explicit_preparation_has_its_own_work_budget -- --nocapture
```

**Notes**

1passed; explicit task succeeds while original interactive work limit still rejects page.
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
cargo test -p jj-store-cli --test native_lifecycle --locked preparation_ -- --nocapture
```

**Notes**

14passed; substring also included unchanged command/mutation preparation tests. Separate trusted policy rejects excess ceilings and enforces tighter bounds; original page membership limit remains in force.
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_task_acceptance:: -- --nocapture
```

**Notes**

6passed; running task survives completed-report TTL, cancels pending body before terminal status and later expires. Existing dedup/current revocation/restart/expiry/worker tests retained.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

27passed; full affected graph module justified by changed default graph ceiling and shared preparation/read/cache consumers. Includes pinned upstream bytes, quotas, corruption, concurrent warm reads and task cancellation.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

Affected packages/direct consumers checked. Existing linker unwind-size advisory remains.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Protected query/read change required mutations ownership chapter; added non-publication and unchanged command-budget contract.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/api.md spec/src/publication.md spec/src/cli.md spec/src/mutations.md; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; cargo fmt --all -- --check
```

**Notes**

167requirements,17packages and formatting pass.
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
pnpm check:progress
```

**Notes**

136task evidence bookkeeping passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- New ceilings are admission policy, not large-scale qualification. Native
  resident index admission still occurs after decoding, global native/local
  memory accounting and immutable generation lifecycle remain next. No
  UI/wire-shape changes; UI/contract generation/root aggregate lanes out of
  scope.
- Native preparation retains current session only during the bounded job, with
  immutable checked catalog root and1024positive membership cache entries.
  Authorization and misses have10sper-observation limits; expiry/cancellation
  cannot repin or publish. Million-object catalog/throughput and source-memory
  measurements remain pending.
- Linux full preparation probe from E-READ-018 remains running; no result
  claimed. Remaining local full-preflight endpoints,
  history/prefix/copy/search/RPC, collaboration projections/cards and
  Linux/native2m/10krefs/deep-page/near-capacity/cache/restart/race/real-RustFS
  acceptance remain open. Original services unchanged; inherited
  registry/full-contract gates retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit native preparation policy, then add predecode/global resident memory admission and complete remaining bounded reads/scale qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


