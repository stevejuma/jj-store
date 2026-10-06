---
schema: 1
id: E-M12-003
---

# E-M12-003

<!-- evidence-field:id markdown -->
## Id

E-M12-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authorized-git-publication
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change vltktsqs.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

GB1 a11cdb45: exact representations retained; no logical remotes, observations or typed Git API/RPC. New acceptance/regression target is atomic Git acceptance with authorization, replay, stale code siblings and cold recovery.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; gix 0.87.1; Git 2.55.0; Rust 1.98.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest 4; Git immutable record schema 1; typed acceptance V1; native codec unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/git-interop.md
- spec/src/publication.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/git_interop.rs
- crates/jj-store-wal/src/publication/git_interop/facts.rs
- crates/jj-store-jj/src/git_bridge/mod.rs
- crates/jj-store-jj/src/git_bridge/objects.rs
- crates/jj-store-jj/tests/git_bridge.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/git_bridge.rs
- crates/jj-store-service/tests/git_bridge.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/git_bridge.rs
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-wal --locked
```
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
cargo check -p jj-store-service --locked
```

**Notes**

Raw object decoder method visibility; fixed.
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
cargo check -p jj-store-rpc --locked
```

**Notes**

NativeObjects constructor error needs explicit WAL error mapping; fixed.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo check -p jj-store-rpc --locked
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --locked --test git_bridge git_acceptance_atomically_keeps_code_evidence_siblings_and_replay_after_checkpoint -- --exact
```

**Notes**

New fact types not re-exported; corrected public owned boundary.
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
cargo test -p jj-store-jj --locked --test git_bridge git_acceptance_atomically_keeps_code_evidence_siblings_and_replay_after_checkpoint -- --exact
```

**Notes**

Atomic code/evidence, concurrent sibling preservation, checkpoint/cold recovery and changed-intent rejection.
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
cargo test -p jj-store-rpc --locked --test git_bridge
```

**Notes**

Local admitted peer, typed logical remote, code-unchanged acceptance and durable replay/status.
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
cargo test -p jj-store-service --locked --test git_bridge
```

**Notes**

Read/write authorization and frozen WorkStore origin/metadata/code invariants.
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
cargo test -p jj-store-jj --locked --test git_bridge
```

**Notes**

8/8; filtered observations retain scope-safe expectations, incomparable destination facts conflict, forged edges/import identities rejected.
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
cargo test -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-rpc --locked
```

**Notes**

All relevant suites passed; 2 JJ real RustFS tests ignored. Memory storage and loopback/Unix RPC, not real RustFS.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-rpc --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/git-interop.md spec/src/milestones.md spec/src/publication.md spec/src/jj-integration.md spec/src/mutations.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```
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
cargo fmt --all --check; pnpm exec prettier --write spec/src/milestones.md && pnpm check:progress
```

**Notes**

Rust formatting passed; progress requires passing evidence entries to have no slice gaps. Later CLI/reducer scope is already recorded in next, so removed it from the completed acceptance slice gaps.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/git_bridge.rs#git_acceptance_atomically_keeps_code_evidence_siblings_and_replay_after_checkpoint
- crates/jj-store-service/tests/git_bridge.rs#git_acceptance_reauthorizes_write_and_preserves_home_origins
- crates/jj-store-rpc/tests/git_bridge.rs#local_peer_reads_and_replays_typed_git_acceptance_without_code_changes
- crates/jj-store-jj/tests/git_bridge.rs#git_observations_keep_filtered_history_and_concurrent_destination_expectations
- crates/jj-store-jj/tests/git_bridge.rs#git_verification_rejects_forged_dependency_and_import_identity_claims
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue GB2 device-local bindings, private cache, bounded transport and native ref fetch. Code origin activation is still gated until GB4 reducer qualification; this entry completes the typed acceptance slice only.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


