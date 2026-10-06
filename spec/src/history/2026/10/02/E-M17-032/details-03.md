---
schema: 1
id: E-M17-032
---

# E-M17-032 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --locked > /tmp/jj-store-agent-capture-m172-headroom-cli.log 2>&1
```

**Notes**

All 34 direct native/local publication, spool, recovery and hook cases pass; hook sanitizer warmup took 104 seconds. Existing explicit hook encoding, shared locks and request/receipt identities are preserved.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-headroom-check.log 2>&1
```

**Notes**

Affected capture, server and CLI targets compile; no added dependency or public schema change.
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
cargo clippy -p jj-store-capture -p jj-store-cli -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-headroom-clippy.log 2>&1
```

**Notes**

Affected packages/direct consumers pass strict lint before the final fixture sizing.
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
cargo test -p jj-store-capture --test reader_progress headroom:: --locked > /tmp/jj-store-agent-capture-m172-headroom-bound-exact.log 2>&1
```

**Notes**

Both headroom cases pass after extending native maximum-delta proof to explicit binding and i64::MAX acceptance time.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-headroom-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-headroom-doc-format.log 2>&1
```

**Notes**

Owning canonical chapter formatting.
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
cargo fmt -p jj-store-capture -p jj-store-server -p jj-store-cli -- --check > /tmp/jj-store-agent-capture-m172-headroom-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_parking --locked > /tmp/jj-store-agent-capture-m172-headroom-supervisor-final.log 2>&1
```

**Notes**

Final complete supervisor and parking binaries pass 13 and 5 cases. Together with unchanged directly affected binaries in earlier expanded runs, all 54 direct server capture cases pass after the exact fixture corrections.
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
cargo test -p jj-store-capture --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-headroom-reader-final.log 2>&1
```

**Notes**

Final complete reader progress binary passes all 14 cases, including maximum bound native preparation and local payload duplication.
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
cargo clippy -p jj-store-capture -p jj-store-cli -p jj-store-server --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-headroom-clippy-final.log 2>&1
```

**Notes**

Final affected-package and direct-consumer targets pass strict lint.
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
cargo fmt -p jj-store-capture -p jj-store-server -p jj-store-cli -- --check > /tmp/jj-store-agent-capture-m172-headroom-rust-format-final.log 2>&1
```

**Notes**

Final affected Rust formatting passes; modified production modules remain below 300 lines.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-headroom-ledger-format.log 2>&1
```

**Notes**

Ledger formatting.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-headroom-spec-validate.log 2>&1
```

**Notes**

5 validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-headroom-spec-first.log 2>&1
```

**Notes**

Same-change gate passes: 13 protected files and 3 canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-headroom-progress.log 2>&1
```

**Notes**

All 129 tasks pass bookkeeping, total 287 evidence entries; M17/M17.2 remain In progress.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-headroom-doc-check.log 2>&1
```

**Notes**

All changed documents pass formatting.
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
rustc --version
```

**Notes**

Current toolchain confirmed as Rust 1.98.1 (48a229cea 2026-09-01).
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
jj --version
```

**Notes**

Current JJ confirmed as 0.45.1.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

