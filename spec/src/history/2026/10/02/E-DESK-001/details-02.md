---
schema: 1
id: E-DESK-001
---

# E-DESK-001 details 2

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
jj workspace add /Users/stevejuma/code/lapismd/jj-store-desktop --name desktop -r @-
```

**Notes**

Clean isolated workspace from main parent 36c92787; existing workspaces and running appearance demo retained.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --lib --locked (before)
```

**Notes**

Desktop manifest absent before implementation.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test desktop_supervision
```

**Notes**

1 admission test passes; real storage test explicitly ignored in this regular invocation.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test --locked -p jj-store-cli --test desktop_supervision -- --ignored --exact desktop_owner_eof_stops_real_service_and_retains_installation
```

**Notes**

Real persistent RustFS: two starts, owner EOF, clean exit, listener/socket removal and unchanged server configuration pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test server_executable
```

**Notes**

14 passed, 3 explicitly ignored; ordinary startup, help/JSON, TLS HTTP1/2, signal-before-ready and state lock all retained.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --lib
```

**Notes**

2 host readiness/navigation tests pass; initial independent lock generated.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 19
hasNote: true
noteEncoding: markdown

```

```shell
cargo check --manifest-path desktop/Cargo.toml --locked (initial latest transitive resolution)
```

**Notes**

Tauri2.11.5 with macros/runtime from2.12 failed inside upstream library; resolved by retaining inspected Buzz-compatible transitive versions, with no dependency patches.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-macros --precise 2.6.3
```

**Notes**

Pin compatible inspected Buzz transitive graph in independent lock.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-codegen --precise 2.6.3
```

**Notes**

Pin compatible inspected Buzz transitive graph in independent lock.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-runtime-wry --precise 2.11.4
```

**Notes**

Pin compatible inspected Buzz transitive graph in independent lock.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-runtime --precise 2.11.3
```

**Notes**

Pin compatible inspected Buzz transitive graph in independent lock.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-utils --precise 2.9.3 (before plugin pin)
```

**Notes**

Newer tauri-plugin required utils2.10.1; pin plugin2.6.3 first.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-plugin --precise 2.6.3
```

**Notes**

Compatible plugin generator pin.
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
cargo update --manifest-path desktop/Cargo.toml -p tauri-utils --precise 2.9.3
```

**Notes**

Compatible utils pin succeeds.
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
cargo check --manifest-path desktop/Cargo.toml --locked (compatible lock)
```

**Notes**

Actual macOS Tauri native host compiles; .scratch/native-check.log.
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
cargo build --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Native debug executable links successfully; /tmp/jj-store-desktop-native-build.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check --locked -p jj-store-cli
```

**Notes**

CLI and direct dependencies pass; /tmp/jj-store-desktop-cli-check.log.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked
```

**Notes**

2 host tests pass; explicit real managed-runtime test remains ignored in regular lane.
<!-- /run -->
<!-- /evidence-field -->

