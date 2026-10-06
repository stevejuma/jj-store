---
schema: 1
id: E-M3-009
---

# E-M3-009 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native
```

**Notes**

Six cases pass; two OS-facility tests ignored in normal lane. Shared terminal fixture remains compatible.
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
node --test scripts/governance.test.mjs
```

**Notes**

15 cases pass, including new server composition boundary regression.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-server-rustfs.json
```

**Notes**

Nine real-storage probes pass on the final implementation. Immutable report retained; no cleanup errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

One Clippy collapsible-if diagnostic in server policy lock file validation. Flattened the conditional without changing behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification, boundaries, progress, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

119 Rust tests, 15 governance tests and 3 storage-harness self-tests pass. Five external tests are ignored in the normal lane; server RustFS acceptance ran separately.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Workspace and both development executables build.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All nine lanes pass, including full Rust suite, rustfmt, mapped spec-first and mdBook.
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

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Init and repository token CLI commands, recoverable attachment and complete
  ordinary-command semantic parity remain.
- Shared TLS deployment and non-Unix serve-local handoff qualification remain
  release gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; pinned RustFS digest in retained m3-server-rustfs report.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New server TOML configuration V1. Existing native object, token policy, WAL, manifest, receipt and RPC V1 formats unchanged. Operator policy files contain verifiers only; credentials stay in OS facility or environment.
<!-- /evidence-field -->

