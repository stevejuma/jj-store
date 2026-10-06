---
schema: 1
id: E-M17-008
---

# E-M17-008 details 2

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
cargo test -p jj-store-capture --test codex_transcript codex_transcript_reader_accepts_only_exact_bound_product_surface --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-reader-before.log 2>&1
```

**Notes**

Before source implementation: exact header/surface regression fails once at unavailable reader seam.
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
cargo test -p jj-store-capture --test codex_transcript codex_transcript_reader_accepts_only_exact_bound_product_surface --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-reader.log 2>&1
```

**Notes**

Exact bound CLI/Desktop header regression passes.
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
cargo test -p jj-store-capture --test codex_transcript --locked > /tmp/jj-store-agent-capture-m172-codex-reader-expanded.log 2>&1
```

**Notes**

Seven initial source tail, generation, partial, byte-bound and resume tests pass.
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
cargo test -p jj-store-capture --test codex_transcript rewrite_between_prefix_verification_steps_cannot_skip_changed_history --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-prefix-before.log 2>&1
```

**Notes**

Same-size rewrite in an already verified portion between prefix steps was incorrectly skipped.
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
cargo test -p jj-store-capture --test codex_transcript rewrite_between_prefix_verification_steps_cannot_skip_changed_history --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-prefix-final.log 2>&1
```

**Notes**

Exact rewrite-between-verification-steps regression passes after snapshot restart fix.
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
cargo test -p jj-store-capture --lib --test codex_transcript --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-codex-reader-final.log 2>&1
```

**Notes**

Eight source tests, nine durable progress tests and one lease test pass.
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
cargo test -p jj-store-capture --test codex_transcript codex_transcript_reader_accepts_only_exact_bound_product_surface --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-reader-exact-final.log 2>&1
```

**Notes**

Final exact initial source regression passes.
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
cargo test -p jj-store-capture --lib --test codex_transcript --test reader_progress --locked > /tmp/jj-store-agent-capture-m172-codex-runtime-final.log 2>&1
```

**Notes**

Final runtime surface passes: nine source tests, nine progress tests and one lease test. Record count limit retains every later record.
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
cargo test -p jj-store-server --test configuration --locked > /tmp/jj-store-agent-capture-m172-codex-config.log 2>&1
```

**Notes**

All 15 configuration tests pass, including separate exact Desktop pin and retained enabled-worker startup guard.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --locked > /tmp/jj-store-agent-capture-m172-codex-check.log 2>&1
```

**Notes**

Affected runtime and direct consumer packages check.
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
cargo clippy --no-deps -p jj-store-capture -p jj-store-server -p jj-store-cli --lib --test codex_transcript --test reader_progress --test configuration --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-codex-clippy.log 2>&1
```

**Notes**

Strict affected library and direct source/progress/configuration test clippy passes.
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
cargo fmt --all
```

**Notes**

Rust formatting applied.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/compatibility.md
```

**Notes**

Changed canonical chapters formatted.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm check:boundaries
```

**Notes**

All 17-package ownership boundaries pass.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Evidence ledger formatted.
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
pnpm check:progress
```

**Notes**

129 tasks validate; all 262 parent evidence entries preserved.
<!-- /run -->
<!-- /evidence-field -->

