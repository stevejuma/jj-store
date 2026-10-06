---
schema: 1
id: E-M6-008
---

# E-M6-008 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked
```

**Notes**

Initial named acceptance cannot compile: server search configuration absent (three diagnostics).
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials
```

**Notes**

Original configuration fixture passes after implementation and lockfile update.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked
```

**Notes**

First lifecycle fixture expansion cannot compile: wrong fixture token field (two diagnostics), plus one unused import warning.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked
```

**Notes**

One passed, one failed: exact ordering was assumed across rebuilt generations with equal-score ties. Corrected comparison preserves all document fields and exact same-generation ordering.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_shutdown_waits_for_retained_worker_contexts --locked
```

**Notes**

Required idle-drain API absent; two compiler diagnostics.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_drop_cancels_workers_and_releases_owned_state --locked
```

**Notes**

Stopped runner retained an apparently healthy last watch value. Pager now marks closed health channels degraded.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked
```

**Notes**

Five lifecycle fixtures pass, including closed health, symlink escape and invalid programmatic limits.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_shutdown_waits_for_retained_worker_contexts --locked
```

**Notes**

Original idle-drain target passes; outstanding callback context delays idle completion.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --locked
```

**Notes**

Seven server configuration tests and two diagnostic transport tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_search --locked
```

**Notes**

Both cold executable fixtures pass, including standalone and serve-local background requests against isolated failing storage.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked
```

**Notes**

Four passed, one failed after adding source verification: fixture expected source body in search response, but the existing owned contract indexes body terms and stores file IDs. Replaced with unique term matching and exact native HTTP file readback, preserving source-content verification.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_runs_recovers_and_rebuilds_after_cache_loss --locked
```

**Notes**

Original rebuild target passes with complete document equivalence, exact same-generation order and native source readback.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_drop_cancels_workers_and_releases_owned_state --locked
```

**Notes**

Original closed-health/drop target passes; stopped runner degrades and releases owned state.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked
```

**Notes**

All five final server-search lifecycle fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked
```

**Notes**

Original configuration acceptance passes again with locked dependencies.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test search_diagnostics --locked
```

**Notes**

Both diagnostic transport fixtures pass, including foreign/invalid/oversized streamed responses and redirect rejection.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked
```

**Notes**

All 27 native extraction/visibility/paging/HTTP/configuration/server projection fixtures pass.
<!-- /run -->

<!-- run -->
### Run 18

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

All ten regular cold executable fixtures pass; one real RustFS qualification remains ignored.
<!-- /run -->
<!-- /evidence-field -->

