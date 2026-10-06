---
schema: 1
id: E-UI-024
---

# E-UI-024

<!-- evidence-field:id markdown -->
## Id

E-UI-024
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.15
- M8a.4
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

- crates/jj-store-cli/tests/native_lifecycle/server_search.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/search.rs
- crates/jj-store-server/src/search_config.rs
- crates/jj-store-server/tests/configuration.rs
- scripts/dev-demo.mjs
- spec/src/adapters.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/configuration.rs#search_configuration_is_explicit_bounded_and_contains_no_credentials
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_runs_recovers_and_restores_after_cache_loss
- crates/jj-store-cli/tests/server_executable.rs#local_trusted_executable_bootstraps_registers_and_serves_without_tokens
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
live GET /v1/bootstrap and /v1/repos/{repository}/search/status at http://127.0.0.1:7460/
```

**Notes**

The running LocalTrusted demo advertised repository search false and returned search_not_configured, reproducing the disabled command without relying on fixture state.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked
```

**Notes**

The access-specific regression initially produced two compiler diagnostics because search token_environment was still a mandatory string.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked
```

**Notes**

The focused configuration contract accepts credential-free LocalTrusted entries, retains authenticated JJ_STORE_SEARCH_TOKEN_ references and rejects either access mode's credential form in the other.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_runs_recovers_and_restores_after_cache_loss --locked
```

**Notes**

The existing authenticated search worker still indexes, advances, snapshots, restarts and restores through its bearer session.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable --locked -- --ignored --exact local_trusted_executable_bootstraps_registers_and_serves_without_tokens
```

**Notes**

Two focused iterations corrected the acceptance itself: search is repository-specific rather than a bootstrap capability, and a real searchable file must be published before asserting a file hit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable --locked -- --ignored --exact local_trusted_executable_bootstraps_registers_and_serves_without_tokens
```

**Notes**

The real embedded-RustFS LocalTrusted process reaches a current Tantivy generation, advertises repository search, returns a tokenless file result and reports doctor search state ready.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-server -p jj-store-cli --tests --locked -- -D warnings; node --check scripts/dev-demo.mjs; pnpm check:boundaries; pnpm check:architecture
```

**Notes**

Both affected Rust packages are warning-free, the development script parses, all 11 package boundaries pass and all 196 governed web modules retain their budgets. Search configuration was extracted so each modified production source remains at or below 300 lines.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

Cold readiness initially timed out because Node fetch supplied an incomplete browser Fetch Metadata tuple and received access_denied. The probe now sends the exact Origin plus same-origin metadata, records its last response and shuts down the owned server on failure.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The checkout-owned LocalTrusted demo started with repository 07199c19c07a5834554e3ed194710518, no credentials, embedded RustFS and a current search generation before reporting ready.
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
live API and in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

Repository capabilities advertise LocalTrusted search, position 25 is indexed without staleness or degradation, native returns two main-scope file hits, and the browser command displays operation, commit and file results for native.
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
pnpm spec:first; pnpm check:progress; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Same-change governance maps nine protected files to seven updated canonical chapters, validates all 53 task records and confirms Rust plus Prettier formatting without running unrelated aggregate suites.
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

Rust 1.98.1; jj-lib 0.45.1; Tantivy 0.26.2; embedded rustfs 1.0.0 with default features disabled; Node.js 22 or newer.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Format-2 search entries now require token_environment only for authenticated access and omit it for LocalTrusted access. Local workers use a stable installation-derived read-only principal. Native objects, manifest, WAL, receipts, IDs, HTTP routes and browser storage formats are unchanged. Visual baselines were skipped by user direction.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


