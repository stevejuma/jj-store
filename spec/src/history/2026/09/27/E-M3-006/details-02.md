---
schema: 1
id: E-M3-006
---

# E-M3-006 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native
```

**Notes**

Before implementation: executable_native target absent; no cases executed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Initial compile used GlobalArgs.no_pager instead of GlobalArgs.early_args.no_pager; corrected.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native
```

**Notes**

Initial three tests: two passed, one help-rendering assertion failed because wrapped Markdown code spans retained backticks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native custom_help_completion_build_info_and_ordinary_git_commands
```

**Notes**

Exact rendering regression passes after handling split markers.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native attach_rejects_read_only_access_before_creating_local_state
```

**Notes**

Denied attachment created a physical directory before authorization.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native attach_rejects_read_only_access_before_creating_local_state
```

**Notes**

Write authorization now precedes local filesystem changes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native custom_terminal_paging_and_color_policy
```

**Notes**

Missing shell-wrapped pager lost the result instead of falling back to stdout.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native custom_terminal_paging_and_color_policy
```

**Notes**

Direct pager process with quoted arguments retains output on spawn failure.
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
env -u JJ_STORE_TOKEN DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native -- --ignored
```

**Notes**

Both real macOS OS-facility tests pass: direct roundtrip and separate-process login/status/logout. Generated credential entries deleted.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:rust:check
```

**Notes**

Formatting drift in final pager refactor; formatter gate rejected it before cargo checks.
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
pnpm format:rust && pnpm format:rust:check
```

**Notes**

Workspace rustfmt applied and exact formatting gate passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native
```

**Notes**

Final six normal executable tests pass after pager refactor; two OS-facility tests have separate explicit passing evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Clippy all targets, rustfmt, specification, package boundaries and progress checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

93 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Four explicitly ignored external tests: two RustFS tests and two OS-facility tests; OS tests passed in the separate recorded lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Development jj executable built; installed jj unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All nine lanes pass, including rustfmt, mapped spec-first, mdBook and executable acceptance.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Standalone server run/configuration/diagnostics, operator provisioning,
  repository creation, token administration and serve-local remain.
- Attachment recovery after uncertain publication, full ordinary-command
  semantic parity and real executable RustFS acceptance remain M3 gates.
- Loopback executable tests use the production RPC/service with memory storage;
  they do not establish RustFS deployment conformance.
- macOS OS credential integration is verified; Linux and Windows platform
  acceptance remains release hardening.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; keyring 3.6.3; jj-nostr command reference ab864f64
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Introduces credential-free connection record format 1 pinned to jj-native-0.45.1-v1. OS account keys bind canonical endpoint and repository. Native objects, WAL, manifest, RPC and token-policy V1 unchanged.
<!-- /evidence-field -->

