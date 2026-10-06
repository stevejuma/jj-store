---
schema: 1
id: E-M10-004
---

# E-M10-004 details 3

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
pnpm web:check
```

**Notes**

Svelte diagnostics accept the generated merged-review contracts with no errors or warnings.
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
pnpm web:test
```

**Notes**

All 54 web unit tests pass against the regenerated collaboration API types.
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
cargo fmt --all && cargo test -p jj-store-replication --test collaboration --test one_way --locked
```

**Notes**

The final five combined receiver cases pass after adding pre-closure authorization and exact authored-root fact selection checks.
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
pnpm format:check
```

**Notes**

A later formatting check found the newly added review CLI paragraph needed Prettier wrapping.
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
pnpm exec prettier --write docs/cli/review.md spec/src/milestones.md && pnpm format:check
```

**Notes**

The review CLI guide was formatted and the complete Rust plus Prettier formatting lane passed.
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
cargo test -p jj-store-replication --locked
```

**Notes**

All nine replication integration cases pass together across one-way transfer, collaboration imports, agent convergence and review convergence.
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
pnpm format:check
```

**Notes**

The final Rust and tracked-text formatting check passes before commit.
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
pnpm spec:validate
```

**Notes**

The final canonical specification check accepts all four validators, 18 chapters and 102 requirements.
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

The final 69-task progress ledger validates before commit.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-replication --test agent_convergence --test review_convergence --locked
```

**Notes**

The first expanded matrix compile found four AgentStore fixture types missing from the WAL publication facade and one reply-parent assertion aimed at the body revision instead of the thread selection. The owned facade now exports the existing types and the assertion follows the canonical thread model.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all && cargo test -p jj-store-replication --test agent_convergence --test review_convergence --locked
```

**Notes**

Both AgentStore cases and the three established ReviewStore cases passed; the two new duplicate checks reused their destination's original apply request identity with reversed origin order and correctly received IntentConflict. The replay checks now use fresh request identities while preserving every origin identity and digest.
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
cargo fmt --all && cargo test -p jj-store-replication --test agent_convergence --test review_convergence --locked
```

**Notes**

Both AgentStore cases and all five ReviewStore cases pass. Every mutable agent catalog converges to the same root under forward and reverse delivery, duplicate replay publishes nothing, tombstone/edit retains both maximal revisions and reply/reply retains both child identities and parent links.
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
cargo test -p jj-store-replication --locked && cargo clippy -p jj-store-wal -p jj-store-replication --all-targets --locked -- -D warnings
```

**Notes**

All 13 replication integration cases and every WAL/replication warnings-as-errors target pass together after the complete convergence matrix.
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
cargo fmt --all && pnpm exec prettier --write spec/src/milestones.md spec/src/replication.md && pnpm format:check && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries
```

**Notes**

Formatting and all 102 requirements validated, then the progress checker exceeded Node's default child-process buffer while reading the now larger parent ledger from Jujutsu. Its Jujutsu reads now use a bounded 16 MiB buffer without weakening any ledger validation.
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
pnpm exec prettier --write scripts/check-progress.mjs spec/src/milestones.md spec/src/replication.md spec/src/spec-governance.md && pnpm test:governance && pnpm format:check && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries
```

**Notes**

All 29 governance tests, Rust and tracked-text formatting, 102 canonical requirements, the 69-task ledger and all 14 package boundaries pass with the bounded progress-checker buffer.
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
pnpm spec:check
```

**Notes**

Validation, cold catalog browser, catalog check/unit/interaction/build and browser check/unit lanes pass; browser-interaction stops when its native lifecycle fixture cannot execute the unavailable cmake program for Kingfisher Vectorscan.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

Specification, package boundary, architecture, progress, formatting and cached embedded RustFS clippy lanes pass; contracts:check stops at the same Kingfisher Vectorscan cmake prerequisite.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test
```

**Notes**

All 29 governance tests, storage harness, embedded RustFS restart and managed local storage acceptance pass; the LocalTrusted executable test then stops at the unavailable cmake prerequisite.
<!-- /run -->
<!-- /evidence-field -->

