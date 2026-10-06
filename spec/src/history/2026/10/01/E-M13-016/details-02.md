---
schema: 1
id: E-M13-016
---

# E-M13-016 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-http --locked
```

**Notes**

Mechanical blocking helper edit supplied state to two direct spawn_blocking calls; corrected both.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_work --locked -- local_browser_browse_uses_complete_root_and_explicit_committed_pin --exact
```

**Notes**

Complete root counts/filtering, pinned later publications, changed cursor query and unrelated history rejection pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

HTTP/CLI actor and receipt parity, later metadata publication, explicit old pin, stale writes and signed filter binding pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_work --test work_store --locked
```

**Notes**

All 9 local and 4 native WorkStore tests passed, including shared complete-root browse and code authorization boundaries.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked
```

**Notes**

All 13 HTTP unit/contract tests pass with explicit retained native and additive local route inventories.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

All 6 local browser fixtures passed.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Separate local WorkStore models and strict pin bodies generated.
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
pnpm web:check
```

**Notes**

Zero errors and warnings in generated contract consumer.
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
pnpm spec:first
```

**Notes**

CLI and mutation canonical chapters required by mapped tests/runtime; updated both.
<!-- /run -->

<!-- run -->
### Run 10

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

Generated public contracts also require compatibility chapter; updated it.
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
pnpm spec:first && pnpm check:progress && pnpm check:boundaries && pnpm spec:validate
```

**Notes**

14 protected files, 7 canonical chapters, 111 tasks, 15 packages and 132 requirements validated.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

108 actual native/local HTTP/SSE responses conform; all 6 contract validation tests and contract TypeScript checks pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_work --locked -- bound_local_browser_commands_cannot_follow_replaced_repository_identity --exact
```

**Notes**

Wrong bound identity rejects a command, status and pinned browse without companion publication.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_work --test work_store --locked
```

**Notes**

All 10 local and 4 native WorkStore tests pass after identity binding.
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
pnpm spec:first
```

**Notes**

16 protected files and 8 owning canonical chapters pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

Final filesystem HTTP/CLI receipt, actor, pin, replay and stale tests pass after admission and identity hardening.
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

JJ 0.45.1, Git SHA-1, macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive explicit local WorkStore pin bodies and jlw1 process-signed continuations; CLI cursor format and native snapshot codecs unchanged. Enclosing Jujutsu change xpovpkru.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement independently pinned AgentStore presentation and local freshness, then the profile-aware frontend.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cross-layer WorkStore transport. Escalation: the complete-root browse projection is shared by native and local providers, so qualify both service WorkStore test binaries; local admission and route inventory affect all HTTP routes, so run the HTTP library tests. Local runtime fixture captures new responses through the retained contract harness, so run the complete capture set. Root aggregates, RustFS and unrelated UI interaction lanes are out of scope for this slice.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- M13.4 remains In progress. Agent endpoints, frontend integration and final
  cold mixed qualification remain subsequent slices.
- Code writes, search, reviews, conversion and synchronization stay disabled.
<!-- /evidence-field -->

