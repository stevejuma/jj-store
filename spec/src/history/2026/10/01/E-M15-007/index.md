---
schema: 1
id: E-M15-007
---

# E-M15-007

<!-- evidence-field:id markdown -->
## Id

E-M15-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

cold-local-cli
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change znxzyuul
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Isolated workspace after 3f33c880; exact local_collab_cli target absent. Typed local provider exists but Thread/Reaction commands are not executable CLI entry points.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; fresh development jj executable processes over disposable colocated, internal and external Git layouts; pnpm 10.32.1. PATH excludes agent runners; no credentials or network model.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Persistence formats unchanged. CLI consumes owned jj-collab intents/results directly; target/value/observed inputs retain full typed identities. Cargo lock adds only internal jj-collab dependency, no external version changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_commands/types.rs
- crates/jj-store-cli/src/agent_commands/collab.rs
- crates/jj-store-cli/tests/local_collab_cli.rs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/local-jj.md
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

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
cargo test -p jj-store-cli --test local_collab_cli --locked (before)
```

**Notes**

Target absent; /tmp/jj-collab-cli-before.log.
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
cargo fmt -p jj-store-cli
```

**Notes**

Run after production edits and after adding executable tests.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 32
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-cli --all-targets (initial)
```

**Notes**

jj_collab glob import shadowed standard Result; fixed explicit standard Result import and nested CLI enum exports.
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
cargo check -p jj-store-cli --all-targets --locked
```

**Notes**

/tmp/jj-collab-cli-check-fixed.log.
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
cargo test -p jj-store-cli --test local_collab_cli --locked
```

**Notes**

Six fresh-process regressions pass; all three Git layouts and sibling workspace. /tmp/jj-collab-cli-test.log.
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
cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Strict CLI lint passes.
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
cargo test -p jj-store-cli --test local_agent --test local_agent_provenance_cli --test review_commands --locked
```

**Notes**

Eight direct dispatch regressions pass, including cold hook sanitization; unrelated suites out of scope.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/local-jj.md spec/src/collaboration.md spec/src/milestones.md
```

**Notes**

Canonical documents formatted.
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
pnpm spec:first
```

**Notes**

Six protected files; five canonical chapters.
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
pnpm spec:validate
```

**Notes**

Four validators; 21 chapters; 140 requirements.
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

111 tasks; evidence bookkeeping only.
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

16 package boundaries pass.
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
cargo fmt -p jj-store-cli -- --check
```

**Notes**

Focused Rust formatting passes.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/local-jj.md spec/src/collaboration.md spec/src/milestones.md
```

**Notes**

Focused documentation formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native transport, signed native continuations, optional capture binding,
  reducer 3, canonical timeline/search, released shared UI and runner
  qualification remain pending. CLI does not invoke or depend on jj-acp.
- Exact local executable tests establish this command boundary only. Real
  RustFS, native code, cold browser and full plan acceptance are not claimed.
  Root aggregate, unrelated Rust/UI and public contract generation lanes are out
  of scope; no HTTP/RPC DTO changed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify bounded native continuation retention and public transports; M15 remains in progress.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-007 details 2](details-02.md)
