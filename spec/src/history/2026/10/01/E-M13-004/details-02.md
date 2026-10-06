---
schema: 1
id: E-M13-004
---

# E-M13-004 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test agent_catalog
```

**Notes**

Native before state: 3 tests pass; projector/catalog extraction rerun passed 3 tests.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test agent_catalog
```

**Notes**

Extraction initially introduced lifetime-shadow diagnostics; fixed before native 3-test rerun.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_agent
```

**Notes**

Before state: local_agent test target did not exist.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-service --locked
```

**Notes**

Private supported method rejected; replaced with existing public profile validation.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-service --locked
```

**Notes**

Local provider and shared immutable engines compile.
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
cargo fmt -p jj-store-wal -p jj-store-service -p jj-store-jj
```

**Notes**

Affected packages formatted, repeated after test creation.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_agent -p jj-store-wal --test agent_catalog
```

**Notes**

5 local tests pass. Invalid fixture used a valid zero source sequence; corrected to duplicate source order. Native test binary was not reached.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_agent -p jj-store-wal --test agent_catalog
```

**Notes**

Corrected exact fixture: 6 local and 3 native tests pass. Final expanded gate: 7 local and 3 native tests pass, including exact native/local record-byte and ID parity.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_work --test agent_sessions --test agent_relations --test agent_checkpoints
```

**Notes**

Direct consumers: 8 local WorkStore, 5 native agent session, 1 native relation and 1 native checkpoint tests pass. Native evidence uses memory store, not real RustFS.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/collaboration.md spec/src/publication.md spec/src/mutations.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Changed canonical chapters formatted.
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
pnpm spec:validate
```

**Notes**

21 chapters and 129 requirements pass.
<!-- /run -->

<!-- run -->
### Run 12

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

Missing mapped jj-integration chapter for the staging-read addition; owning chapter updated.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass.
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
pnpm check:architecture
```

**Notes**

312 web modules pass; new Rust production modules separately counted at most 265 lines, local tests 411.
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
cargo fmt -p jj-store-jj -p jj-store-wal -p jj-store-service -- --check
```

**Notes**

Affected Rust packages pass formatting.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-store-wal --test agent_catalog -p jj-store-service --test local_agent --test local_work --test agent_sessions --test agent_relations --test agent_checkpoints -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

Affected libraries and exact direct-consumer targets pass without warnings.
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
pnpm exec prettier --write spec/src/jj-integration.md spec/src/milestones.md
```

**Notes**

Mapped chapter and final ledger formatted.
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
pnpm spec:first
```

**Notes**

18 protected files and 7 canonical chapters pass.
<!-- /run -->
<!-- /evidence-field -->

