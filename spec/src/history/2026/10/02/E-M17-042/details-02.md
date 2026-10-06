---
schema: 1
id: E-M17-042
---

# E-M17-042 details 2

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
cargo test -p jj-store-server --lib process::tests --locked > /tmp/jj-store-agent-capture-m172-foreground-before.log 2>&1
```

**Notes**

Initial fixture compile fails because PolicyFile has no write method. Used the actual bounded TokenPolicy encode contract with a private fixture file; no production rule changed.
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
cargo test -p jj-store-server --lib process::tests --locked > /tmp/jj-store-agent-capture-m172-foreground-before-corrected.log 2>&1
```

**Notes**

Genuine before regression: first fresh-process drop case reports state lock released while capture storage remained alive. One parent failure; its helper registration alone passes. No implementation was edited before this proof.
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
cargo test -p jj-store-server --lib process::tests --locked > /tmp/jj-store-agent-capture-m172-foreground-after.log 2>&1
```

**Notes**

An edit script stopped on a guessed nonexistent CLI heading before implementation changed, so this command repeats the genuine before lock failure. It is not an after-implementation result. Corrected the heading from the owning chapter and resumed the edit; incidental absent-module inspection guesses are not product evidence.
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
cargo fmt --all
```

**Notes**

Pinned formatting passes after extraction and each subsequent source/test correction.
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
cargo test -p jj-store-server --lib process::tests --locked > /tmp/jj-store-agent-capture-m172-foreground-lock-after.log 2>&1
```

**Notes**

Exact foreground target passes: one main-process behavior entry plus helper registration dispatches four fresh-process cases for prepared drop, run cancellation, completed shutdown and preparation failure. All preserve actual lock/client lifetime invariants; disabled capture creates no agent-capture directory.
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
cargo test -p jj-store-server --lib --locked > /tmp/jj-store-agent-capture-m172-foreground-lib-final.log 2>&1
```

**Notes**

Complete owning server library passes all 20 entries, including seven storage ownership tests, eight executor faults, two listener bounds, replication outage and the foreground behavior/helper registration. This covers shared foreground/native storage and executor consumers; not a whole Rust workspace run. One helper is a registration-only entry, with four actual fresh-process cases.
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
cargo test -p jj-store-cli --test server_executable --locked > /tmp/jj-store-agent-capture-m172-foreground-cli-cold.log 2>&1
```

**Notes**

15 ordinary fresh-executable cases pass; three real RustFS/deployment ignores are not passes.
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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --exact > /tmp/jj-store-agent-capture-m172-foreground-filesystem.log 2>&1
```

**Notes**

Exact direct filesystem runtime consumer passes from development output; no native storage or capture state is created.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test configuration --test embedded_storage --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-foreground-native.log 2>&1
```

**Notes**

Seven runtime entries (six behavior entries and helper registration) and 16 configuration cases pass; storage helper dispatches five fresh-process cases. Two separately built real RustFS cases are ignored.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-foreground-check.log 2>&1
```

**Notes**

Owning server and direct CLI all-target checks pass.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-foreground-clippy.log 2>&1
```

**Notes**

Owning server and direct CLI strict all-target lint passes.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-foreground-rustfmt.log 2>&1
```

**Notes**

Pinned main and embedded manifest Rust formatting passes.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-foreground-doc-check.log 2>&1
```

**Notes**

Initial document formatting reports three changed documents requiring formatting. No product failure; repaired with owning formatter before final gates.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-foreground-boundaries.log 2>&1
```

**Notes**

17 package boundaries pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-foreground-architecture.log 2>&1
```

**Notes**

719 web module architecture audit passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-foreground-doc-format.log 2>&1
```

**Notes**

Formats all four changed documents after the initial three-document style failure.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-foreground-spec-validate.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass; repeated after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-foreground-spec-first.log 2>&1
```

**Notes**

Nine protected paths and three owning chapters pass; repeated after final sealing.
<!-- /run -->
<!-- /evidence-field -->

