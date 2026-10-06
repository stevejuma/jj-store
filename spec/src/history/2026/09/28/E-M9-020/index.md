---
schema: 1
id: E-M9-020
---

# E-M9-020

<!-- evidence-field:id markdown -->
## Id

E-M9-020
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/agent.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/agent_publication.rs
- crates/jj-store-service/src/guard.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/native_agent.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-wal/src/publication.rs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-rpc/tests/agent_publication.rs#local_peer_prepares_and_replays_exact_agent_publication
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-rpc --test agent_publication --locked
```

**Notes**

The test-first compile proved that NativeClient had no prepare or publish AgentStore RPC methods.
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
cargo test -p jj-store-rpc --test agent_publication --locked
```

**Notes**

The first complete transport reached publication and exposed that production guards still inherited the fail-closed default for all domain writes; the guard now requires the current write principal and exact request actor.
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
cargo test -p jj-store-rpc --test agent_publication --locked
```

**Notes**

The LocalTrusted Unix RPC prepares a server-principal-owned delta, commits and replays its receipt, rejects a changed principal and preserves a storage-unknown outcome until the same request commits.
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
cargo clippy -p jj-store-service -p jj-store-rpc --lib --test agent_publication --test local_trusted --locked -- -D warnings
```

**Notes**

The affected service and RPC libraries plus both focused Unix transport targets are warning-free. New production modules contain 249 and 63 physical lines; the regression contains 176.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm spec:first
```

**Notes**

The first same-change gate required the RPC and service changes to update their mapped jj-integration and mutations chapters in addition to the collaboration chapter.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/milestones.md spec/src/mutations.md spec/src/verification.md
```

**Notes**

All 89 requirements, 14 protected files across six canonical chapters and all 61 progress tasks validate; every changed specification file is formatted.
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
cargo test -p jj-store-rpc --test agent_publication --test local_trusted --locked
```

**Notes**

Both focused RPC regressions pass, covering agent prepare/publish/replay/unknown outcomes and retained owner-only bearer-free Unix admission.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

The first package audit rejected direct jj-agent and WAL dependencies from RPC plus a direct jj-agent dependency from service. Collaboration types are now re-exported through the existing WAL and service facades instead of weakening the layer policy.
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
cargo test -p jj-store-rpc --test agent_publication --offline; pnpm check:boundaries
```

**Notes**

The exact agent publication regression still passes through the allowed facade types, and all 13 package dependency boundaries pass.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm spec:first
```

**Notes**

After the dependency correction, spec-first required the WAL facade re-export to be recorded in its mapped publication chapter.
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
cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-rpc --lib --test agent_publication --test local_trusted --locked -- -D warnings; cargo test -p jj-store-rpc --test agent_publication --test local_trusted --locked; cargo test -p jj-store-service --test authorization --locked; cargo fmt --all -- --check
```

**Notes**

The final allowed WAL facade, service guard and RPC transport are warning-free and formatted. Both focused RPC tests and all eight service authorization regressions pass.
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
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/milestones.md spec/src/mutations.md spec/src/publication.md spec/src/verification.md
```

**Notes**

All 13 package boundaries, 89 requirements, 14 protected files across seven canonical chapters and 61 progress tasks pass; all changed specifications are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Drain captured and prepared entries in source-sequence order through the exact
  native RPC request
- Add drift-safe owned integration removal after ingesting setup is qualified
- Execute pinned real-harness capability and smoke tests in M9.6; Cursor remains
  unsupported on the current host
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; native RPC package jj_store.native.v1; AgentStore delta schema v1; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds bounded canonical JSON AgentCapturePrepareRequest and AgentDeltaPublishRequest messages. Prepare derives the current peer principal, server activation time and pinned AgentStore root. Publish accepts only the exact validated AgentDeltaRequest and returns a typed committed receipt or unknown identity without changing native object, manifest or AgentStore identities.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


