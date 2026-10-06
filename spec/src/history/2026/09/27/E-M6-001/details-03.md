---
schema: 1
id: E-M6-001
---

# E-M6-001 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc rpc_stream_protocol_deadlines_and_permits_are_bounded --locked
```

**Notes**

Narrow baseline rerun passes, exposing the nondeterministic nature of the fixture: heads polling can acquire the sole permit before the upload or begin after its short deadline.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc rpc_stream_protocol_deadlines_and_permits_are_bounded --locked
```

**Notes**

After replacing competing polling with server-side upload-admission synchronization and connecting the observer first, exact deadline/permit/release acceptance passes. The 400 ms product deadline and rejection assertions remain unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc --locked
```

**Notes**

All seven tests consuming the native RPC harness pass after synchronization repair.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Final all-target checks pass after RPC synchronization repair.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

312 Rust tests pass, zero failures, six opt-in external tests ignored; governance, contracts, harness and existing browser unit lanes pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace and existing browser assets build.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 lanes pass, including existing cold browser interactions/accessibility, Rust tests, rustfmt, mdBook and spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
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

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New adapter checkpoint V1 with identity, exact selected source reference and durable artifact descriptor under repos/{repo}/adapters/v1/{name}/cursor.json. WAL/native/RPC formats unchanged. Adapter crate and service binding added to committed workspace locks; no UI files changed. Enclosing Jujutsu change is the revision reference. Tantivy and real RustFS/TLS qualification remain separate gates.
<!-- /evidence-field -->

