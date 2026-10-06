---
schema: 1
id: E-M17-038
---

# E-M17-038 details 3

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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-generations-check-final.log 2>&1
```

**Notes**

All owning capture/server and direct CLI consumer targets compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-generations-lint-final.log 2>&1
```

**Notes**

All owning and direct consumer targets pass strict lint.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-generations-doc-format.log 2>&1
```

**Notes**

Changed canonical chapters, partial verification evidence and ledger formatted.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-generations-spec-validate.log 2>&1
```

**Notes**

Canonical specification and governed ownership validate.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-generations-spec-first.log 2>&1
```

**Notes**

Spec-first passes: 21 protected files and three canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-generations-progress.log 2>&1
```

**Notes**

All 129 tasks validate; prior 292 entries preserved and full M17/M17.2 remain In progress.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-generations-rust-format.log 2>&1
```

**Notes**

Final owning Rust files pass format check.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_source --test capture_pipeline --test capture_recovery --test capture_parking --test capture_reanchor --test capture_resume --locked > /tmp/jj-store-agent-capture-m172-generations-server-rerun.log 2>&1
```

**Notes**

All 54 direct native server cases pass with original default test concurrency, deadlines and assertions when repeated without simultaneous other qualification suites. Includes 24 supervisor cases and unchanged blocked-directory fault. The prior timeout is preserved, not converted into a pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-server
```

**Notes**

An initially created tests/capture_pipeline directory changed implicit inline fixture-module resolution; rustfmt could not resolve the existing fixture. Moved the new helper under existing capture_source, updated its explicit path and removed only the empty temporary directory. No assertion weakened. Subsequent formatting passes.
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
cargo fmt -p jj-store-server
```

**Notes**

Corrected module placement formats successfully.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline progress_capacity:: --locked > /tmp/jj-store-agent-capture-m172-generations-progress-capacity-before.log 2>&1
```

**Notes**

Actual new-lookup regression: exact independent-handle physical capacity refusal was hidden by Service("capture source progress unavailable"). Queue grew through four real sanitized reader batches, beyond the pre-existing smaller handle limit.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_pipeline progress_capacity:: --locked > /tmp/jj-store-agent-capture-m172-generations-progress-capacity-after.log 2>&1
```

**Notes**

Exact capacity regression passes after all new automatic progress lookup maps preserve the existing typed spool error. Limit/required totals are exact; accepted pending bytes and source progress are unchanged.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_source --test capture_pipeline --test capture_recovery --test capture_parking --test capture_reanchor --test capture_resume --locked > /tmp/jj-store-agent-capture-m172-generations-server-sealed.log 2>&1
```

**Notes**

55 final direct native server cases pass across seven binaries, including all 24 supervisor cases and the independent-handle capacity lookup regression. Default concurrency and original deadlines/assertions unchanged; no simultaneous other product suite.
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
cargo check -p jj-store-server --all-targets --locked > /tmp/jj-store-agent-capture-m172-generations-check-sealed.log 2>&1
```

**Notes**

Final server targets compile after capacity mapping/test additions. Capture and CLI production remain unchanged since their complete passing lanes.
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
cargo clippy -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-generations-lint-sealed.log 2>&1
```

**Notes**

Final server targets pass strict lint after all production and test additions.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-generations-rust-format-sealed.log 2>&1
```

**Notes**

Final affected production/tests pass Rust formatting.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-generations-boundaries-sealed.log 2>&1
```

**Notes**

Final source dependency boundaries pass.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-generations-architecture-sealed.log 2>&1
```

**Notes**

Final source architecture checks pass; every changed production module is below 300 lines.
<!-- /run -->
<!-- /evidence-field -->

