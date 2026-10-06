---
schema: 1
id: E-M9-015
---

# E-M9-015

<!-- evidence-field:id markdown -->
## Id

E-M9-015
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
- crates/jj-agent/src/capture_cursor.rs
- crates/jj-agent/src/lib.rs
- crates/jj-agent/src/normalize.rs
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_files.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/agent_capture_spool.rs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_spool.rs#sanitized_capture_and_prepared_publication_survive_restart
- crates/jj-store-cli/tests/agent_capture_spool.rs#capture_spool_rejects_concurrent_owners_and_changed_retries
- crates/jj-store-cli/tests/agent_capture_spool.rs#capture_spool_enforces_byte_bounds_and_rejects_corruption
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_capture_spool --locked
```

**Notes**

The test-first compile reported the missing durable cursor and capture-spool contracts.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_capture_spool --locked
```

**Notes**

The first implementation required the lockfile to record jj-store-cli's direct ownership of the existing jj-agent workspace package.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_capture_spool --offline
```

**Notes**

The first compiled regression found a test-local fixture name shadow; no production contract was changed to resolve it.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_capture_spool --locked
```

**Notes**

All three focused spool regressions pass: sanitized restart recovery and exact prepared identity, exclusive ownership and immutable retries, plus byte-bound and corruption rejection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-agent --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test agent_capture_spool --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-agent -p jj-store-cli --test agent_capture_spool --locked -- -D warnings; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo fmt --all -- --check
```

**Notes**

All 20 AgentStore model/normalizer tests and three capture-spool regressions pass; the affected model and CLI test targets are warning-free and formatted. The spool modules contain 276 and 51 physical lines and the regression contains 179.
<!-- /run -->

<!-- run -->
### Run 6

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

Canonical validation passed, then spec-first required the CLI-owned durable journal to be documented in both architecture.md and cli.md; both ownership mappings were added.
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
pnpm check:boundaries; pnpm test:governance; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm spec:build; pnpm exec prettier --check spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All 13 package boundaries and 26 governance tests pass; 85 requirements validate, nine protected files map to four canonical chapters, all 60 progress tasks are consistent, the book builds and changed prose is formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Owned Codex, Cursor and OpenCode discovery plus setup/status commands
- Released registry acceptance for @lapismd/agent-redaction
- Pinned real-harness capability matrix and smoke tests in M9.6
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; capture spool format v1; AgentStore capture projection schema v1; exact normalizer profiles from E-M9-008.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

CaptureCursorV1 retains one source generation and accepted position. The user-restricted locked spool atomically persists a sanitized batch before acknowledging its cursor, then the exact AgentDeltaRequest before publication. Exact retries survive restart; unacknowledged atomic temporary files are discarded, while changed retries, corrupt or unexpected files, concurrent ownership and unprepared or mismatched completion fail closed.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


