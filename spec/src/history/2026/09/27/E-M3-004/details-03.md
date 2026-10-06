---
schema: 1
id: E-M3-004
---

# E-M3-004 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

79 Rust tests, 14 governance tests and 3 storage harness self-tests. No failures. Two real RustFS tests run separately through the isolated harness.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m3-native-session-rustfs.json
```

**Notes**

All ten probes pass against pinned single-node RustFS; actual Rust client races, dropped responses and process-kill/fresh-process cache-free publication recovery retained. Owned container and volume cleaned up. This validates storage/publication conformance, not deployed authentication.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Nine lanes pass before final divergent-protection regression.
<!-- /run -->

<!-- run -->
### Run 5

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

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protection_can_freeze_multiple_existing_divergent_incarnations
```

**Notes**

Final review: one protected commit ID per change ID incorrectly rejected unchanged history with multiple protected divergent incarnations.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protection_can_freeze_multiple_existing_divergent_incarnations
```

**Notes**

Protected change IDs now retain the set of exact protected incarnations; existing divergence remains readable and publishable without permitting a new rewrite.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle
```

**Notes**

Final ten native lifecycle, authentication, semantic-policy, retry and concurrency tests pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Final 80 Rust tests, 14 governance tests and 3 harness self-tests pass. Two external RustFS tests were exercised by the recorded real-storage lane.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All nine lanes, including rustfmt, governed progress, package boundaries, tests, mapped spec-first and mdBook, pass after final regression fix.
<!-- /run -->

<!-- run -->
### Run 13

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

- Native RPC, server executable, CLI commands and OS credential storage are not
  yet integrated.
- Library lifecycle evidence is not full executable CLI/server acceptance or
  browser/API parity.
- Trusted protected-history rules are constructor configuration; server
  configuration and transport administration remain to be connected.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native objects, WAL, manifests and token-policy V1 unchanged. Credential-free client cache binding format jj-store-client-cache-v1 / jj-native-0.45.1-v1. Native request/publication IDs are domain-separated hashes of repository, principal and exact head transition; per-poll runtime binding is process-local.
<!-- /evidence-field -->

