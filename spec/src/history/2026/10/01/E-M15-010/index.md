---
schema: 1
id: E-M15-010
---

# E-M15-010

<!-- evidence-field:id markdown -->
## Id

E-M15-010
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

cold-native-cli
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mlsxmmxs
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Isolated workspace at 91d3d84b. Native collaboration RPC is implemented; native CLI dispatch rejects Thread/Reaction transport and the exact native_collab_cli target is absent.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pnpm 10.32.1; fresh explicit development CLI processes against owner-admitted Unix gRPC over disposable MemoryStore. No installed binary, runner, unrelated service or credentials changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Shared bounded typed intent parsing for local and native providers. Native RPC commands/status/detail and signed --cursor pages; local pages retain --after/--at-metadata and reject native cursors. Original stable request ID retained for uncertain transport outcomes. Persistence and RPC formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_commands/collab.rs
- crates/jj-store-cli/src/agent_commands/collab/intents.rs
- crates/jj-store-cli/src/agent_commands/collab/native.rs
- crates/jj-store-cli/tests/local_collab_cli.rs
- crates/jj-store-cli/tests/native_collab_cli.rs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/mutations.md
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
cargo test -p jj-store-cli --test native_collab_cli --locked (before)
```

**Notes**

Target absent; /tmp/jj-collab-native-cli-before.log.
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

Repeated after edits.
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
cargo check -p jj-store-cli --all-targets --locked (initial)
```

**Notes**

One root diagnostic: local_cursor was mistakenly used in Show, which has no cursor. Removed; /tmp/jj-collab-native-cli-check.log.
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
cargo check -p jj-store-cli --all-targets --locked (fixed)
```

**Notes**

/tmp/jj-collab-native-cli-check-fixed.log.
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
cargo test -p jj-store-cli --test native_collab_cli --test local_collab_cli --locked (final)
```

**Notes**

Native 1 and local 6 cold executable tests pass; /tmp/jj-collab-native-cli-test-final.log.
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

Strict CLI lint passes; /tmp/jj-collab-native-cli-clippy.log.
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
cargo test -p jj-store-cli --test local_agent --test local_agent_provenance_cli --test review_commands --test local_trusted_cli --locked
```

**Notes**

Nine direct dispatch/initialization consumers pass (2+4+2+1); /tmp/jj-collab-native-cli-consumers.log.
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
cargo fmt -p jj-store-cli -- --check
```

**Notes**

Focused Rust formatting passes.
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
pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/local-jj.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Canonical chapters formatted.
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

Five protected files; six canonical chapters.
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

Four validators; 21 chapters; 140 requirements.
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
pnpm check:progress
```

**Notes**

111 tasks; evidence bookkeeping only.
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

16 package boundaries.
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
pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/local-jj.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Focused document formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Shared collaboration dispatch/input parsing affects the native and local providers plus agent/review/LocalTrusted dispatch, so all exact collaboration and nine direct consumer tests were run. Unrelated UI, full Rust workspace and root aggregate lanes remain out of scope under the targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-010 details 2](details-02.md)
