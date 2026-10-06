---
schema: 1
id: E-M13-001
---

# E-M13-001 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-jj --locked --test local_metadata
```

**Notes**

Before state: no local_metadata test target or implementation exists.
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
pnpm install --frozen-lockfile --offline
```

**Notes**

544 cached packages; no lockfile changes. Existing development pnpm hook resolves Community/Design Core locally; no publication or registry claim.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-jj --locked
```

**Notes**

Initial compile: missing Write trait, missing SHA-1 argument, and moving a Drop-owned buffer. All three corrected.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-jj --locked --test local_metadata
```

**Notes**

Initial filesystem gate: 8 tests pass, including killed subprocesses and stock Git GC. Follow-up closure audit and stock JJ compatibility still pending.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-jj --locked --test local_metadata
```

**Notes**

Final expanded gate: 11 tests pass including SHA-1 closure validation, missing nested objects, JJ conflict-header terms, stock JJ import/undo/GC, Git fsck/GC and killed subprocesses.
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
cargo fmt -p jj-store-jj
```

**Notes**

Pinned toolchain and tracked formatter policy; no unrelated source diffs.
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
pnpm exec prettier --write AGENTS.md spec-validator.config.mjs spec/src/SUMMARY.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/architecture.md spec/src/spec-governance.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

Changed canonical documents formatted; verification rows were then moved into the existing table.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-jj --test local_metadata --locked -- -D warnings
```

**Notes**

One redundant closure in the new module; corrected without suppressing diagnostics.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

New verification rows initially appended after the table were parsed as prose; moved into the canonical table.
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
pnpm spec:first
```

**Notes**

8 protected files and 5 canonical chapters pass.
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
pnpm check:progress
```

**Notes**

102 tasks pass evidence bookkeeping; not product proof.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass; no dependency additions.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

Canonical verification table repaired and formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-jj --test local_metadata --locked -- -D warnings
```

**Notes**

Owning package and exact test target pass with no warnings.
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

4 validators, 21 chapters and 129 requirements pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-jj --locked --test native_codec
```

**Notes**

8 retained native identity, malformed-data, frozen-envelope and full-field regression tests pass.
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
cargo fmt -p jj-store-jj -- --check
```

**Notes**

Owning package formatting passes.
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
pnpm exec prettier --check AGENTS.md spec-validator.config.mjs spec/src/SUMMARY.md spec/src/local-jj.md spec/src/jj-integration.md spec/src/architecture.md spec/src/spec-governance.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

All changed canonical documents/configuration pass formatting.
<!-- /run -->
<!-- /evidence-field -->

