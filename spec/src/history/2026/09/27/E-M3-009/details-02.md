---
schema: 1
id: E-M3-009
---

# E-M3-009 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration
```

**Notes**

Before implementation: package absent; zero cases executed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration
```

**Notes**

Initial five configuration cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration programmatic_configuration_cannot_bypass_private_path_validation
```

**Notes**

Regression showed directly constructed configuration could bypass private control-path checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration programmatic_configuration_cannot_bypass_private_path_validation
```

**Notes**

Config::validate enforces the same boundary for file-loaded and programmatic configuration.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Four CommandError conversion diagnostics; followed by one remaining Ui conversion diagnostic in executable test compilation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable
```

**Notes**

Initial compilation after partial repair: one remaining Ui conversion diagnostic.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable
```

**Notes**

Three cases pass; serve-local fails because custom --config conflicts with upstream global String argument.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination
```

**Notes**

First targeted rerun still had the old --config fixture flag.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination
```

**Notes**

After flag fix, upstream cleanup_guard signal handler exits status 1. Diagnostic rerun reproduced one failure. Unix handoff to paired binary fixes lifecycle ownership.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination
```

**Notes**

Exact cold process/state lock/termination regression passes after Unix exec handoff.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m3-server-rustfs.json
```

**Notes**

Eight storage probes pass; one executable fixture fails: Invalid evolog template keyword description.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m3-server-rustfs-after.json
```

**Notes**

Eight storage probes pass; one executable fixture fails: File show path was relative to fixture cwd, outside selected repository.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m3-server-rustfs-final.json
```

**Notes**

All nine probes pass after correcting pinned evolog template and repository-relative file argument. Server/RustFS kill, empty process/index state, history, binary content and live revocation pass.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --lib competing_operator_provision_cannot_replace_the_first_credential
```

**Notes**

Regression before lock implementation: three unresolved function diagnostics, zero cases executed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --lib competing_operator_provision_cannot_replace_the_first_credential
```

**Notes**

A competing same-endpoint provision cannot reach OS credential mutation; lock releases after completion.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable standalone_terminal_paging_color_and_json_contract
```

**Notes**

Standalone executable passes owned PTY paging/color/no-pager/JSON/fallback checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration
```

**Notes**

All six configuration/policy tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable
```

**Notes**

All five executable tests pass; real RustFS case ignored in this normal lane.
<!-- /run -->
<!-- /evidence-field -->

