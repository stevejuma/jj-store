---
schema: 1
id: E-M9-018
---

# E-M9-018

<!-- evidence-field:id markdown -->
## Id

E-M9-018
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

- Cargo.toml
- Cargo.lock
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_sanitization.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/agent_capture_spool.rs
- crates/jj-store-cli/tests/agent_kingfisher.rs
- docs/cli/agent.md
- docs/licenses/NOTICE.md
- docs/upstream-lock.md
- package.json
- pnpm-lock.yaml
- scripts/check-progress.mjs
- scripts/governance.test.mjs
- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- scripts/agent-capture-sanitizer.mjs
- scripts/agent-capture-sanitizer.test.mjs
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- scripts/local-package-pnpmfile.test.mjs#local package hook uses available
  agent redaction source
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_kingfisher.rs#kingfisher_redacts_secrets_and_normalizes_workspace_paths
- crates/jj-store-cli/tests/agent_kingfisher.rs#kingfisher_cached_scan_child
- crates/jj-store-cli/tests/agent_kingfisher.rs#invalid_or_oversized_hook_payload_is_omitted_without_source_details
- crates/jj-store-cli/tests/agent_capture_spool.rs#sanitized_capture_and_prepared_publication_survive_restart
- crates/jj-store-cli/tests/agent_setup.rs#setup_command_selects_one_explicit_integration
- scripts/local-package-pnpmfile.test.mjs#local package hook uses available
  Design Core source
- scripts/local-package-pnpmfile.test.mjs#local package hook retains registry
  range when source is absent
<!-- /evidence-field -->

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
cargo test -p jj-store-cli --test agent_kingfisher --locked
```

**Notes**

Three focused regressions pass. A cold medium-rule cache build plus scan took 94.37 seconds; the child-process cache reuse scan took 3.55 seconds and stayed within the installed five-second hook budget. Secrets and canonical workspace paths are removed, while invalid and oversized inputs return only the fixed omitted state.
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
cargo test -p jj-store-cli --test agent_setup --locked; cargo clippy -p jj-store-cli --test agent_kingfisher --test agent_setup --locked -- -D warnings; node --test scripts/local-package-pnpmfile.test.mjs; cargo fmt --all -- --check
```

**Notes**

All four existing setup regressions and both remaining local-package hook regressions pass; the affected CLI and focused tests are warning-free and the Rust workspace is formatted.
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
pnpm install --lockfile-only --ignore-scripts
```

**Notes**

The lockfile no longer contains the superseded JavaScript redaction package or its transitive dependency closure.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first
```

**Notes**

Package boundaries and the canonical requirements passed; spec-first identified the missing spec-governance mapping update for removal of the JavaScript sanitizer tooling.
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
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

The mapped chapter passed after the governance update; the progress checker then exposed that historical evidence could not represent two removed files and one retired named test.
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
node --test scripts/governance.test.mjs scripts/local-package-pnpmfile.test.mjs; pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed files>
```

**Notes**

All 23 focused governance and local-package regressions pass. Thirteen package boundaries, 85 canonical requirements, 16 protected-file mappings, all 60 progress tasks and changed-file formatting pass. Historical evidence remains verifiable through explicit removed-file and retired-test records.
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
cargo test -p jj-store-cli --test agent_capture_spool --locked; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

All three durable spool regressions pass with the Kingfisher policy identity. The upstream source/license record, 19 protected-file mappings, 85 requirements and all 60 progress tasks pass their focused checks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Wire jj agent hook ingestion into the durable capture spool and publication
  recovery
- Add drift-safe owned removal after ingesting setup is qualified
- Execute pinned real-harness capability and smoke tests in M9.6; Cursor remains
  unsupported on the current host
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; MongoDB Kingfisher v2.7.0 at source revision 63dccba231b6b01990c84319d25f92f49428a431 under Apache-2.0; scanner default features disabled; matching Vectorscan fork pinned at the same revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Sanitization policy mongodb-kingfisher@2.7.0/builtin-medium/v1 accepts one bounded JSON object, canonicalizes workspace paths, scans only in memory with Base64 decoding disabled, merges finding byte ranges, emits canonical JSON with [REDACTED] substitutions and fails closed to a fixed omitted state. Real setup prewarms a user-restricted compiled-rule cache; no raw temporary input file or JavaScript sanitizer remains.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


