---
schema: 1
id: E-M0-002
---

# E-M0-002

<!-- evidence-field:id markdown -->
## Id

E-M0-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M0
- M0.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-26
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

compiled-native-integration
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.toml
- Cargo.lock
- crates/jj-store-cli/examples/upstream_custom_backend.rs
- crates/jj-store-jj/tests/compatibility.rs
- crates/jj-store-jj/proto/probe.proto
- fixtures/native-profile/m0-hashes.json
- scripts/smoke-upstream-cli.py
- docs/licenses/NOTICE.md
- scripts/rustfs-contract.py
- docs/evidence/m0-rustfs-cold.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/compatibility.rs#pinned_identity_roots_and_predecessor_presence
- crates/jj-store-jj/tests/compatibility.rs#explicit_bounded_reads_preserve_divergent_heads
- crates/jj-store-jj/tests/compatibility.rs#native_factory_preserves_request_context_across_rpc
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test compatibility
```

**Notes**

Before crate creation: package absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test compatibility
```

**Notes**

2 passed; initial upstream hash fixture deliberately empty before recording pinned reference vectors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test compatibility
```

**Notes**

3 tests; reference hash fixture established, request-context RPC and read-only divergent operation probes pass.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build -p jj-store-cli --example upstream_custom_backend
```
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
Isolated upstream_custom_backend --help/init-jit/describe/log/op log/evolog smoke; reproduced by scripts/smoke-upstream-cli.py
```

**Notes**

6 commands; upstream Git-backed extension proof only.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```
<!-- /run -->

<!-- run -->
### Run 7

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
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
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
python3 scripts/smoke-upstream-cli.py
```

**Notes**

6 commands
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
python3 scripts/rustfs-contract.py --run --output .scratch/m0-rustfs-cold.json
```

**Notes**

8 real probes, including truncated body; fresh disposable container/volume.
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

jj-cli/jj-lib 0.45.1, Rust 1.97.1, tonic 0.14.6, protobuf compiler pinned in Cargo.lock
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Test-only probe protobuf; native profile hash vectors; production wire format not implemented.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


