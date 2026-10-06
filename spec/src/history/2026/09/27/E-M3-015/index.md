---
schema: 1
id: E-M3-015
---

# E-M3-015

<!-- evidence-field:id markdown -->
## Id

E-M3-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

foreground-shutdown-regression
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/process.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/server_executable.rs
- spec/src/cli.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/server_executable.rs#cold_server_registers_shutdown_before_reporting_listener
- crates/jj-store-cli/tests/server_executable.rs#cold_search_worker_polls_failed_storage_and_stops_with_foreground_server
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Existing search-worker shutdown acceptance exited with SIGTERM. Listening output preceded polling the async handler-registration function.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_search_worker_polls_failed_storage_and_stops_with_foreground_server -- --exact
```

**Notes**

1 focused retry passed before the fix, confirming timing sensitivity rather than resolving the ordering defect.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_registers_shutdown_before_reporting_listener --locked -- --exact
```

**Notes**

New immediate-termination regression fails before the source fix. Both native run and serve-local are covered by eight cold starts each.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_registers_shutdown_before_reporting_listener --locked -- --exact
```

**Notes**

Immediate termination passes for all 16 cold run/serve-local starts after eager handler registration.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable --locked
```

**Notes**

11 executable tests pass including the original search-worker regression; 1 external RustFS case remains intentionally ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Final 18 lanes pass: 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, 369 Rust tests (6 external-service/credential tests ignored), rustfmt, contracts, governance, boundaries and mdBook.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Final aggregate passes: governance, storage harness self-tests, contracts, browser unit tests, 19 catalog unit tests, 34 stories, cold catalog and 369 Rust tests; 6 external-service/credential tests ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Static Storybook, existing browser bundle/manifest and all locked Rust workspace binaries build successfully.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
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

JJ 0.45.1; Rust 1.97.1; existing locked dependencies.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire or storage changes. Unix shutdown handlers now install synchronously before the listening event; enclosing Jujutsu change is the evidence reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


