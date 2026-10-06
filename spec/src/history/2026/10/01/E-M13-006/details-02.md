---
schema: 1
id: E-M13-006
---

# E-M13-006 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked --test portable
```

**Notes**

Before state: portable agent provenance acceptance target missing.
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
cargo fmt -p jj-agent -p jj-store-model
```

**Notes**

Repeated after edits.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-agent --locked
```

**Notes**

Compiled; two unused imports warned and removed before final lint.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked --test provenance
```

**Notes**

Three native provenance regressions pass after shared facts extraction.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked --test portable --test provenance
```

**Notes**

Four portable and three native provenance tests pass. Frozen V1 association bytes retained.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-agent --locked
```

**Notes**

All 24 tests pass across activity, deltas, indexes, model, normalizers, portable and native provenance; no doc-test cases.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test agent_catalog -p jj-store-service --test agent_checkpoints --test local_agent -p jj-review --test portable
```

**Notes**

3 native catalog, 1 native checkpoint, 7 local provider and 3 portable review regressions pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent --all-targets -p jj-store-model --lib --locked -- -D warnings
```

**Notes**

Affected model libraries and complete agent targets pass without warnings.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/identity.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Changed documents formatted.
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
cargo fmt -p jj-agent -p jj-store-model -- --check
```

**Notes**

Affected Rust formatting passes.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final ledger formatted.
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
pnpm spec:first
```

**Notes**

8 protected files and 4 canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

21 chapters and 129 requirements pass.
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

15 package boundaries pass.
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
pnpm check:progress
```

**Notes**

102 tasks pass evidence bookkeeping.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/identity.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Changed canonical documents pass formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Standalone portable models only; local selected capsules,
  association/checkpoint persistence and CLI still incomplete.
- ReviewStore, filesystem server/UI and conversion remain incomplete; unrelated
  UI/RustFS/root aggregates out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ/jj-lib 0.45.1; macOS ARM64; isolated local-jj workspace.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive standalone CodeAssociationV2 and AgentCheckpointVersionV2 with profile-bound evidence and shared non-code facts V1. Native V1 codecs, field order, record envelopes and identities unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish model gates and commit, then integrate portable provenance with local retained evidence and stale-safe catalogs.
<!-- /evidence-field -->

