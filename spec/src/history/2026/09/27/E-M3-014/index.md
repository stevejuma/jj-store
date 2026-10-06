---
schema: 1
id: E-M3-014
---

# E-M3-014

<!-- evidence-field:id markdown -->
## Id

E-M3-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
- M3.2
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

standalone-tls-deployment
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/server_executable.rs
- scripts/rustfs-contract.py
- spec/src/cli.md
- spec/src/security.md
- spec/src/compatibility.md
- crates/jj-store-cli/src/commands.rs
- docs/cli/capabilities.md
- docs/evidence/m3-tls-server-rustfs.json
- docs/evidence/m3-gate-audit.md
- spec/src/verification.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/server_executable.rs#cold_tls_server_checks_trust_hostname_and_plaintext
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext
```

**Notes**

Before fixture: zero matching tests; no positive standalone TLS acceptance.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext
```

**Notes**

New negative plaintext assertion stopped at TCP/channel establishment. Tonic can return before the TLS peer rejects the HTTP/2 preface; fixture must drive an RPC and assert transport failure.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext
```

**Notes**

Plaintext RPC rejected as transport Cancelled rather than Unavailable. Both are valid pre-application connection failures; fixture accepts those two transport categories and excludes application authorization/errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext
```

**Notes**

1 passed. Explicit trust succeeds; untrusted certificate, wrong hostname and plaintext RPC are rejected. Fresh standalone server starts and terminates.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-tls-server-rustfs.json
```

**Notes**

All 9 probes pass. Actual CLI init/attach, token administration and cache-free process/RustFS recovery use TLS with generated localhost certificate and disabled development override; owned resources removed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Regular test lane passed, but repeated TLS negative test returned Unknown with a local tonic transport/h2 FRAME_SIZE_ERROR source. Numeric status depends on timing. Final fixture asserts a local tonic transport error source (not a decoded application status) instead.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Prettier found line wrapping in canonical CLI and security chapters; Rust formatting passed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext
```

**Notes**

5 consecutive runs pass after asserting the local tonic transport error source, independently of numeric h2/gRPC status.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All functional lanes and mdBook passed; mapped spec-first detected missing governance chapter update for the RustFS harness change. Added explicit TLS report/transport scope guidance.
<!-- /run -->

<!-- run -->
### Run 10

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
### Run 11

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

136 Rust cases pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass.
<!-- /run -->

<!-- run -->
### Run 12

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
### Run 13

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

All 9 lanes pass after the governance mapping update, including repeated full Rust suite, rustfmt, mdBook and spec-first.
<!-- /run -->

<!-- run -->
### Run 14

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

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; generated localhost TLS certificate; pinned RustFS image.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence, RPC or application format changes. M3 native CLI/server gate qualified on current Unix host and pinned single-node TLS topology; M4 bounded reads/HTTP/UI, M5 typed application mutations, M6 search and M8a deployment/platform release hardening remain open.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


