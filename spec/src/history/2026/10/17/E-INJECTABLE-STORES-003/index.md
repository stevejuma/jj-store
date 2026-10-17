---
schema: 1
id: E-INJECTABLE-STORES-003
---

# E-INJECTABLE-STORES-003

<!-- evidence-field:id markdown -->
## Id

E-INJECTABLE-STORES-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M32
- M32.1
- M32.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-006
- JST-LOCAL-001
- JST-LOCAL-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

selected-store-participant-admission
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lmwxmnno.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Continued the existing participant draft on parent b2a6eb74. Initial focused factory run passed3core and7Git tests. Before this slice, replacing a public store field could retain a stale built-in participant, and live cross-domain sources were loaded eagerly. Two additional regressions qualify replay with a disabled current source and independently named compatible registrations.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-core/src/stores/factories.rs
- crates/jj-store-core/src/stores/mod.rs
- crates/jj-store-core/src/stores/sources.rs
- crates/jj-store-core/src/stores/traits.rs
- crates/jj-store-core/tests/factories.rs
- crates/jj-store-core/tests/independent_stores.rs
- crates/jj-store-git/README.md
- crates/jj-store-git/src/factories.rs
- crates/jj-store-git/src/repository/collab_commands.rs
- crates/jj-store-git/tests/factories.rs
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/progress/M32/history/M32/index.md
- spec/src/current/progress/M32/history/M32.1/index.md
- spec/src/current/progress/M32/history/M32.2/index.md
- spec/src/current/progress/M32/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-git/tests/factories.rs#committed_reaction_replays_without_the_current_source_store
- crates/jj-store-git/tests/factories.rs#reactions_reject_replaced_thread_sources_before_staging
- crates/jj-store-core/tests/independent_stores.rs#each_store_is_independently_replaceable_by_an_external_implementation
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
pnpm dev:disk
```

**Notes**

Free33.9GiB, root target34.2GiB; authorized dev stack remains stopped pending final restoration.
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
pnpm spec:search -- "JST-ARCH-006"
```

**Notes**

Current spec-validator rejects the extra separator; direct script invocation follows.
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
pnpm spec:search "JST-ARCH-006"
```

**Notes**

QMD native modules have a Node ABI mismatch. Used the reported rg fallback and read the owning architecture/M32 sources; no dependency reinstall.
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
pnpm dev:cargo -- test -p jj-store-core -p jj-store-git --test factories --locked --message-format=short
```

**Notes**

Initial focused baseline:3core and7Git factory tests passed.
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
pnpm dev:cargo -- test -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

18tests passed in the complete core/Git suites;0doctests. Escalated because factory selection and source admission are shared by both packages. Covers all five independent external stores plus Git load/reopen/replay, source replacement/disablement and atomic binding.
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
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Strict lint for all affected library/test targets.
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
rustfmt --edition2024 --config skip_children=true <changed Rust files>
```

**Notes**

Formatted the changed Rust surface.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

First draft used bare test names and omitted the newly added task-history paths. Replaced references with file#test and refreshed the changed file inventory.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Second draft used file-only test references; completed them with exact named test anchors.
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
pnpm check:progress && pnpm spec:first && pnpm spec:validate
```

**Notes**

162tasks/853evidence records,10protected files/1owning chapter,7validators/4110chapters/206requirements passed.
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
pnpm dev:cargo -- test -p jj-store-service --test local_collab --test local_agent_binding --locked --message-format=short
```

**Notes**

Direct service consumer regressions: bound capture and the complete local collaboration family passed; no full service suite claimed. Log participants-service.log.
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

Pinned Rust1.99, JJ0.45.1, Git2.55.0; current0.1.0libraries.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No disk/transport changes. Existing Git ref, original request identities, canonical bytes and receipts remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Seals actual selected participants after loading, exposes immutable handles, and requires provider-owned runtime capability plus fresh source admission for compound commands. Live Git source domains are opened only when needed; retained historical dependency inventories retain full audits. Replay checks original receipts before source compatibility.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement native factories, service/CLI factory routing and independent consumer qualification. Restore retained Vite dev stack after validation. Unrelated UI/root aggregate lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-006 gains private selected-participant admission and explicit
  shared-provider compatibility. JST-LOCAL-001/003 authority, canonical bytes,
  actor provenance and original-receipt replay remain unchanged. Updated
  architecture chapter governs the additive API contract.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M32/M32.1/M32.2 remain In progress for end-to-end provider
  qualification; native factories and consumer migration remain M32.3/M32.4.
  This completed composition slice does not complete those gates.

```
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages


