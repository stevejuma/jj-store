---
schema: 1
id: E-M13-007
---

# E-M13-007 details 3

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
pnpm spec:first
```

**Notes**

17 protected files and 7 canonical chapters pass.
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
pnpm check:progress
```

**Notes**

Draft evidence used unsupported in-progress result; corrected to the qualified product result before final bookkeeping.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo clippy -p jj-agent --all-targets -p jj-store-jj --test local_evidence -p jj-store-wal --test agent_provenance --test agent_catalog -p jj-store-service --test local_agent --test local_agent_pages -p jj-store-cli --test local_agent_provenance --test local_agent_recovery --test agent_capture_spool --locked -- -D warnings
```

**Notes**

Affected libraries and exact consumers pass with no warnings after clone cleanup.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Updated evidence ledger formatted.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Draft pass retained an unfinished bookkeeping gap; cleared after all product and lint gates completed.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/local-jj.md spec/src/publication.md spec/src/mutations.md spec/src/jj-integration.md spec/src/cli.md spec/src/milestones.md
```

**Notes**

Changed canonical documents pass formatting.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-wal --locked --test agent_provenance -p jj-store-cli --test local_agent_provenance
```

**Notes**

Final clone-cleanup verification: 1 shared native projector and 7 local provider cases pass.
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
pnpm check:progress
```

**Notes**

102 tasks pass after draft bookkeeping fixes.
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

- User-facing provenance CLI and automatic hook code associations remain
  incomplete; local ReviewStore and M13.3 completion are separate gates.
- Native storage intentionally rejects portable capsules until a native carrier
  is qualified.
- Filesystem server/UI, conversion, real RustFS and unrelated root aggregates
  remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ/jj-lib/jj-cli 0.45.1; gix 0.87.1; Git 2.55.0; macOS ARM64; disposable Git repositories and explicit development binary.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive portable_association/portable_checkpoint_version variants retain AgentObject V1 framing and semantic catalog kinds. Portable relation subjects retain profile-aware identities. Local AgentStore descriptor V2 adds canonical selected evidence inventory; V1 remains readable. Native V1 record bytes, manifests and WAL remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit qualified portable provenance provider, then integrate user-facing provenance CLI and shared ReviewStore.
<!-- /evidence-field -->

