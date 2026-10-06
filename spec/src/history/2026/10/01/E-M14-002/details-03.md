---
schema: 1
id: E-M14-002
---

# E-M14-002 details 3

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
cargo fmt -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-cli -- --check
```

**Notes**

Rust formatting passed.
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
pnpm exec prettier --write scripts/contracts.mjs spec/src/api.md spec/src/cli.md spec/src/security.md spec/src/verification.md spec/src/milestones.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md
```

**Notes**

Changed scripts and canonical chapters formatted; commands were run in two batches.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

Specification validation and progress passed; corrected spec-first passed with 20 protected files and seven canonical chapters.
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

<!-- evidence-field:scope markdown -->
## Scope

Service contracts, CLI-owned bounded discovery, server-owned registry/lifecycle and HTTP admission are one local-management behavior. Generated-contract changes justify the complete contract capture lane; unchanged RustFS is reused through its explicit development runtime. All cargo commands set CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target to reuse the build cache without duplicating it; sources stay in the isolated settings workspace. Root aggregates, full Rust workspace, unrelated UI suites and release qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds device-local workspace-links.json v1 with installation identity, monotonic revision and bounded verified folder bindings. Adds optional local_workspace_links capability and versioned GET/list plus POST/link DTOs. Native object/WAL/receipt/agent formats unchanged. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; macOS ARM64; existing pinned embedded RustFS runtime; pnpm 10.32.1; locked Axum 0.8 and Svelte toolchain.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete M14.2 folder-link Settings presentation, then M14.3 agent setup and queue recovery.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Git-backed browser management remains gated on M13.4.
- No unlink, agent setup, queue worker, editable Search, storage maintenance or
  Sync UI is delivered by this backend slice.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/workspace-*.log in jj-store-local-settings; cold fixture used temporary directories and owned child cleanup.
<!-- /evidence-field -->

