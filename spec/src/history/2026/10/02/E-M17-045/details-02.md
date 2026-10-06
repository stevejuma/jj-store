---
schema: 1
id: E-M17-045
---

# E-M17-045 details 2

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
cargo fmt --all
```

**Notes**

Formatting applied to inherited running-loss scaffold before its initial compilation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests::running_loss --locked > /tmp/jj-store-agent-capture-m172-running-loss-before.log 2>&1
```

**Notes**

Scaffold compilation fails with nine diagnostics: missing test-only faults module/discovery field and private delivery resources, plus verifier methods resolving the crate Result alias. This is not a demonstrated product behavior failure. Corrected fixture result types and added private cfg(test) fault plumbing.
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
cargo fmt --all
```

**Notes**

Pinned formatting applied after canonical and test-only fault changes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::tests::running_loss --locked > /tmp/jj-store-agent-capture-m172-running-loss-after.log 2>&1
```

**Notes**

Exact four-case fresh-process matrix passes in 15.99 seconds. Two registered entries include one no-op subprocess helper. Actual running tasks panic after qualification/recovery; lock/storage lifetime, exact unknown Prepared intent, current admission withholding and receipt replay are verified.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_runtime --test capture_supervisor --locked > /tmp/jj-store-agent-capture-m172-running-loss-consumers.log 2>&1
```

**Notes**

59 direct entries pass: 25 owning-library, seven runtime, 27 supervisor. Four no-op helper registrations are not independent behavior tests. Cold factory runs all 16 subprocess cases and compiles its own scanner cache; library completes in 96.13 seconds. Runtime 23.37 seconds and supervisor 49.10 seconds.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-running-loss-check.log 2>&1
```

**Notes**

Affected server and direct CLI all-target compilation passes.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-running-loss-clippy.log 2>&1
```

**Notes**

Strict affected server and direct CLI all-target lint passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-running-loss-doc-format.log 2>&1
```

**Notes**

All four canonical documents formatted; final evidence sealing repeats formatting.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-running-loss-rustfmt.log 2>&1
```

**Notes**

Pass; canonical and ledger checks are repeated after final sealing.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-running-loss-boundaries.log 2>&1
```

**Notes**

Pass; canonical and ledger checks are repeated after final sealing.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-running-loss-architecture.log 2>&1
```

**Notes**

Pass; canonical and ledger checks are repeated after final sealing.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-running-loss-spec-validate.log 2>&1
```

**Notes**

Pass; canonical and ledger checks are repeated after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-running-loss-spec-first.log 2>&1
```

**Notes**

Pass; canonical and ledger checks are repeated after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-running-loss-progress.log 2>&1
```

**Notes**

Pass; canonical and ledger checks are repeated after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-running-loss-doc-check.log 2>&1
```

**Notes**

Initial document check passes before recording its run; later sealing formatting drift is separately recorded.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-running-loss-doc-check.log 2>&1
```

**Notes**

Final JSON serialization expands compact arrays in E-M17-045, so milestones formatting fails. Applied the existing formatter; no product behavior failure.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-running-loss-doc-check.log 2>&1
```

**Notes**

Sealing format drift is repaired; all four documents pass. Final serialization is formatted before repeating checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1. Native MemoryStore commit/receipt recovery, external-S3 lifetime client without requests, fresh subprocesses and real private filesystem locks.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public or durable format change. Current AgentStore, exact Prepared request/actor/receipt and source cursor unchanged. Fault gates, discovery gate field and delivery admission setter exist only under cfg(test).
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ywvkzyko
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

