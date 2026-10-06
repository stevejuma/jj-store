---
schema: 1
id: E-M15-002
---

# E-M15-002 details 2

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
cargo test -p jj-store-wal --test collab_persistence (before)
```

**Notes**

Test target absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-wal
```
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
cargo test -p jj-store-wal --test collab_persistence --locked
```

**Notes**

9 focused regressions pass, including every-domain authorization, legacy omission, frozen WorkStore origin compatibility, unsupported code-repository origins and format-5 preservation. Native memory-store evidence only.
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
cargo test -p jj-store-jj --test local_metadata --locked
```

**Notes**

13 real filesystem metadata tests including additive batch atomicity, legacy receipts, stock Git/JJ GC, hard exit and unknown outcomes. New batch domain bytes are carrier fixtures, not complete local typed-service acceptance.
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
cargo test -p jj-store-wal --locked
```

**Notes**

66 tests pass. Escalated because shared manifest/WAL/recovery/domain publication affects independent existing code/AgentStore/ReviewStore/WorkStore/Git consumers. Final additive authorization/legacy omission tests rerun in the exact 6-test target.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-jj --all-targets --locked -- -D warnings (initial)
```

**Notes**

Larger RepositoryDomains made AlreadyEnabled variant too large; boxed the variant, then updated its exact existing test constructor.
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
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-jj --all-targets --locked -- -D warnings (constructor repair)
```

**Notes**

Existing test required the new boxed constructor; corrected without weakening assertions.
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
cargo clippy -p jj-collab -p jj-store-wal -p jj-store-jj --all-targets --locked -- -D warnings
```
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
cargo check -p jj-store-service -p jj-store-replication -p jj-store-rpc --all-targets --locked
```

**Notes**

Direct Rust consumer compilation; no full consumer acceptance claim.
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
cargo test -p jj-collab --test domain --locked
```

**Notes**

13 retained domain regressions.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-collab -p jj-store-wal -p jj-store-jj
```
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters, 140 requirements.
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
pnpm check:progress
```

**Notes**

111 tasks; same-change protected evidence.
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
pnpm check:boundaries
```

**Notes**

16 packages.
<!-- /run -->

<!-- run -->
### Run 15

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

Boundary checker change also maps to spec-governance.md; added exact M15 authority/lane scope.
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
cargo clippy -p jj-store-wal --all-targets --locked -- -D warnings
```
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
pnpm spec:first
```

**Notes**

20 protected files; 8 canonical chapters.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/publication.md spec/src/replication.md spec/src/spec-governance.md spec/src/milestones.md scripts/check-boundaries.mjs
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Local typed graph carrier/services/commands and complete local asset retention
  acceptance remain pending; lower-level batch metadata is implemented.
- Native portable capsule carrier remains unsupported; explicit native
  operation/commit anchors are available.
- Central target membership/visibility resolver, service/API/RPC/CLI, reducer 3,
  shared released UI, search and real-RustFS/cold full-flow qualification remain
  pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

New native persistence plus additive local carrier envelope. Full relevant WAL suite escalated for shared authority infrastructure; focused JJ metadata/domain suites and direct Rust consumer checks. Root aggregate and UI/browser lanes remain out of scope.
<!-- /evidence-field -->

