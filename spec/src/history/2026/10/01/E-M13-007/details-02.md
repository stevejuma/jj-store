---
schema: 1
id: E-M13-007
---

# E-M13-007 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --locked --test local_agent_provenance
```

**Notes**

Before state: provider acceptance target missing. Exact source fixture placed at the CLI-owned upstream boundary, preserving service dependency rules.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance
```

**Notes**

Before state: no CLI-owned source-evidence acceptance target.
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
cargo fmt -p jj-agent -p jj-store-wal -p jj-store-jj -p jj-store-service
```

**Notes**

Affected libraries formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-service --locked
```

**Notes**

Provider compiles; unused RecordRef import warned and removed before final lint.
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
cargo fmt -p jj-agent -p jj-store-service -p jj-store-cli
```

**Notes**

Portable wire and CLI-owned direct-provider fixture formatted.
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
cargo fmt -p jj-store-wal
```

**Notes**

Native shared-projector regression formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance
```

**Notes**

Initial five cases pass: retained source archives, stale corrections/replay, fail-closed evidence, checkpoint ownership and concurrent shared relations. Additional relation audit and V1/unknown tests require final rerun.
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
cargo fmt -p jj-agent -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-cli
```

**Notes**

Final affected source and additional regressions formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance
```

**Notes**

Seven final provider cases pass including V1 compatibility and ref-lock unknown outcome; no source op or checkout publication.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked
```

**Notes**

All 24 shared model tests pass, including additive portable variants and frozen native V1 bytes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/local-jj.md spec/src/publication.md spec/src/mutations.md spec/src/jj-integration.md spec/src/cli.md spec/src/milestones.md
```

**Notes**

Changed canonical chapters and ledger formatted.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test agent_provenance --test agent_catalog -p jj-store-service --test local_agent --test local_agent_pages --test agent_sessions --test agent_relations --test agent_checkpoints -p jj-store-jj --test local_evidence --test local_metadata
```

**Notes**

37 tests pass: 1 shared native projector, 3 native catalog, 7 native service read, 9 local provider/page, 6 capsule and 11 metadata/GC/recovery tests.
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
cargo fmt -p jj-store-cli
```

**Notes**

Repeated after strengthening pinned checkpoint tool-version assertion.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance --test local_agent_recovery --test agent_capture_spool
```

**Notes**

7 final provenance, 2 ordered local recovery and 3 shared spool tests pass. Checkpoint rejects same-kind tool-version substitution before publication.
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

21 chapters and 129 requirements pass.
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

15 package boundaries pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent --all-targets -p jj-store-jj --test local_evidence -p jj-store-wal --test agent_provenance --test agent_catalog -p jj-store-service --test local_agent --test local_agent_pages -p jj-store-cli --test local_agent_provenance --test local_agent_recovery --test agent_capture_spool --locked -- -D warnings
```

**Notes**

One clone-on-Copy diagnostic in portable relation projection; removed redundant clone without altering value semantics.
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
cargo fmt -p jj-agent -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-cli -- --check
```

**Notes**

Affected Rust packages pass formatting.
<!-- /run -->
<!-- /evidence-field -->

