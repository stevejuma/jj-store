---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 15

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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/main-qualification-rustfs.json
```

**Notes**

rustfs-native-server-45: exit 1, 49.56 seconds. 
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
pnpm dev:cargo -- test --workspace --locked --test server_executable
```

**Notes**

server-executable-complete-47: exit 0, 22.88 seconds. 
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
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

server-executable-clippy-47: exit 0, 34.12 seconds. 
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
pnpm format:rust:check
```

**Notes**

rust-format-47: exit 0, 3.8 seconds. 
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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/main-qualification-rustfs.json
```

**Notes**

rustfs-native-server-45: exit 1, 51.11 seconds. 
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
pnpm dev:cargo -- test --workspace --locked --test server_executable
```

**Notes**

server-executable-complete-47: exit 0, 22.97 seconds. 
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
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

server-executable-clippy-47: exit 0, 8.1 seconds. 
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
pnpm format:rust:check
```

**Notes**

rust-format-47: exit 0, 4.22 seconds. 
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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/main-qualification-rustfs.json
```

**Notes**

rustfs-native-server-45: exit 0, 51.17 seconds. 
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
pnpm dev:cargo -- test --workspace --locked --test server_executable
```

**Notes**

server-executable-complete-47: exit 0, 22.84 seconds. 
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
pnpm dev:cargo -- clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

server-executable-clippy-47: exit 0, 8.25 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-rust-corpus.py
```

**Notes**

rust-corpus-coverage-48: exit 0, 0.36 seconds. 
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
pnpm build:embedded-rustfs
```

**Notes**

embedded-build: exit 0, 6.89 seconds. 
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy --manifest-path runtime/local-agent/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

**Notes**

local-agent-clippy: exit 1, 67.24 seconds. 
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
pnpm dev:cargo -- test --manifest-path runtime/local-agent/Cargo.toml --workspace --locked --no-fail-fast
```

**Notes**

local-agent-tests: exit 0, 93.71 seconds. 
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
pnpm dev:cargo -- clippy --manifest-path runtime/compute/Cargo.toml --all-targets --locked -- -D warnings
```

**Notes**

compute-clippy: exit 0, 187.09 seconds. 
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
pnpm dev:cargo -- test --manifest-path runtime/compute/Cargo.toml --locked --no-fail-fast
```

**Notes**

compute-tests: exit 0, 222.51 seconds. 
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy --manifest-path runtime/local-agent/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

**Notes**

local-agent-clippy: exit 1, 25.36 seconds. 
<!-- /run -->
<!-- /evidence-field -->

