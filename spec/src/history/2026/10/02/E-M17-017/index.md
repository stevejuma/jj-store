---
schema: 1
id: E-M17-017
---

# E-M17-017

<!-- evidence-field:id markdown -->
## Id

E-M17-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

stable-installation-capture-actor-and-in-process-native-publication-boundary
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/service_agent_publication.rs
- crates/jj-store-server/src/capture_authority.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_authority.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/security.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_authority.rs#capture_actor_is_stable_separate_and_disabled_preparation_reads_no_identity
- crates/jj-store-server/tests/capture_authority.rs#native_publisher_requires_current_admission_and_retains_receipt_actor_and_code_heads
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_authority --locked > /tmp/jj-store-agent-capture-m172-authority-before.log 2>&1
```

**Notes**

One missing authority interface establishes the before state. A second error is a fixture binding shadowing its helper; renamed before implementation verification.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Repeated focused formatting after the service adapter, authority and native publication fixture.
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
cargo test -p jj-store-server --test capture_authority --locked > /tmp/jj-store-agent-capture-m172-authority-actor.log 2>&1
```

**Notes**

Exact stable/distinct actor, disabled no-state/no-identity-read, permission and private-identity checks pass.
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
cargo test -p jj-store-server --test capture_authority --locked > /tmp/jj-store-agent-capture-m172-authority-native.log 2>&1
```

**Notes**

Two authority/native publication tests pass. Native service receipts retain the actor and exact replay; read-only requests are denied and code heads remain unchanged.
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
cargo test -p jj-store-server --test capture_authority --test capture_activity --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-authority-server.log 2>&1
```

**Notes**

All five direct authority/activity/directory admission test cases pass.
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
cargo test -p jj-store-cli --test agent_capture_publication --test agent_capture_spool --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-authority-cli.log 2>&1
```

**Notes**

21 direct shared spool/publication/local-recovery tests pass; native publication subprocess helpers remain invoked by parent tests.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-authority-check.log 2>&1
```

**Notes**

Affected runtime/server and direct CLI consumer all-target checks pass.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-authority-clippy.log 2>&1
```

**Notes**

Strict all-target lint passes with no suppression.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-authority-format.log 2>&1
```

**Notes**

Rust and embedded-runtime formatting pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-authority-boundaries.log 2>&1
```

**Notes**

17-package dependency boundaries pass; no dependency was added.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/security.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-authority-doc-write.log 2>&1
```

**Notes**

Canonical documents formatted; partial requirement rows remain Planned and milestones remain In progress.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-authority-progress.log 2>&1
```

**Notes**

129 tasks validate; 272 evidence structures preserve all 271 parent entries.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-authority-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-authority-first.log 2>&1
```

**Notes**

Five protected files and four canonical chapters pass spec-first.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/security.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-authority-doc-format.log 2>&1
```

**Notes**

All changed canonical documents pass formatting.
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

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1 header and constant sanitized gap. Real owned native service/WAL/receipt publication over MemoryStore; no real RustFS, live harness or installed daemon qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-017 details 2](details-02.md)
