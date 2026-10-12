---
schema: 1
id: E-BUILD-STORAGE-001
---

# E-BUILD-STORAGE-001

<!-- evidence-field:id markdown -->
## Id

E-BUILD-STORAGE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3.3
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-CLI-004
- JST-DESK-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- README.md
- package.json
- scripts/browser-fixtures.mjs
- scripts/build-storage-inventory.mjs
- scripts/build-storage-lock.py
- scripts/build-storage-policy.mjs
- scripts/build-storage.mjs
- scripts/build-storage.test.mjs
- scripts/contracts.mjs
- scripts/desktop.mjs
- scripts/dev-demo-services.mjs
- scripts/dev-ui-api.mjs
- scripts/dev-ui.acceptance.mjs
- scripts/docs/references.mjs
- scripts/install-cli.mjs
- scripts/managed-cargo.mjs
- scripts/managed-cargo.test.mjs
- scripts/profile-acceptance.mjs
- spec/src/current/contracts/architecture/01-architecture.md
- spec/src/current/contracts/cli/03-requirements.md
- spec/src/current/contracts/cli/39-development-build-storage.md
- spec/src/current/contracts/cli/SUMMARY.nav.md
- spec/src/current/contracts/cli/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M17/history/M17.2/index.md
- spec/src/current/progress/M17/index.md
- spec/src/current/progress/M3/history/M3.3/index.md
- spec/src/current/progress/M3/index.md
- spec/src/history/2026/10/12/E-BUILD-STORAGE-001/SUMMARY.nav.md
- spec/src/history/2026/10/12/E-BUILD-STORAGE-001/index.md
- spec/src/history/2026/10/12/E-BUILD-STORAGE-001/details-02.md
- spec/src/history/2026/10/12/E-BUILD-STORAGE-001/details-03.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/build-storage.test.mjs#storage thresholds and cleanup order retain
  active temporary targets
- scripts/build-storage.test.mjs#dry run reports cleanup and preserves cache and
  application state
- scripts/build-storage.test.mjs#symlink roots and protected cache entries
  refuse deletion
- scripts/build-storage.test.mjs#managed build leases block cleanup while
  independent targets can build
- scripts/build-storage.test.mjs#native Cargo locks and running target
  executables refuse cleanup
- scripts/build-storage.test.mjs#Cargo-selected first-party cleanup retains
  dependency artifacts and can be repeated
- scripts/build-storage.test.mjs#temporary target completion is explicit and
  automatic cleanup prioritizes it
- scripts/build-storage.test.mjs#cleanup rejects missing Cargo markers and
  preserves running executables
- scripts/build-storage.test.mjs#temporary retirement dry run preserves
  lifecycle and unknown roots are rejected
- scripts/build-storage.test.mjs#interrupted cache removal releases leases and a
  retry completes
- scripts/build-storage.test.mjs#an oversized protected target does not evict
  useful idle caches when free space is healthy
- scripts/managed-cargo.test.mjs#managed Cargo pins targets before the test
  separator and rejects unknown directories
- scripts/managed-cargo.test.mjs#two managed workspaces reuse unchanged builds
  and keep independent executables
- scripts/managed-cargo.test.mjs#failed maintenance prevents compilation and
  preserves a live target
- scripts/managed-cargo.test.mjs#repository Rust build entry points use the
  managed wrapper
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; cargo/rustc 1.98.1; jj 0.45.1; Node 26.9.0; pnpm 10.32.1; Python 3.14.7. No compiler/toolchain upgrade or installed-tool replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version-1 device-local target registry and JSON storage report; POSIX build/maintenance leases outside checkouts. Native repository identities, publication, WAL, API and persisted application formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned build tooling across root, desktop, embedded RustFS, local-agent and compute Cargo roots plus registered JJ and temporary workspaces. Defaults retain incremental compilation and debug=0, bound maintenance at 40 GiB/target and 80 GiB/family with 30 GiB free. Completed temporary targets and oversized idle caches are cleaned first. Shared leases, native Cargo locks, cache markers, canonical paths, protected-state scans, ps executable identity and lsof consumers refuse unsafe cleanup. Postflight preserves just-built output and checks failed builds too. An unfixable live oversized target does not evict useful idle caches when free space is healthy.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-CLI-004 and its owning build-storage chapter add bounded managed
  development maintenance. JST-DESK-001 executable and private-state custody
  remain unchanged; no app API, Rust product behavior or native storage format
  changed.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M3.3 remains Done for existing CLI contracts; M17.2 remains In progress
  with independent macOS host, platform and package gates. This slice verifies
  device-local development tooling only; it does not qualify additional
  storage/platform/release behavior. The enclosing Jujutsu change is the
  evidence revision.

```
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

When the owned main development target is idle, clean its accumulated package artifacts and rebuild through managed Cargo. Retire completed temporary targets and clean idle workspace build roots before forgetting workspaces.
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-BUILD-STORAGE-001 details 2](details-02.md)
- [E-BUILD-STORAGE-001 details 3](details-03.md)
