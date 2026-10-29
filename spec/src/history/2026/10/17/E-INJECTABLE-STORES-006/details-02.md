---
schema: 1
id: E-INJECTABLE-STORES-006
---

# E-INJECTABLE-STORES-006 details 2

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
pnpm dev:target:register -- --target .local/build-targets/store-consumer --reason "Qualify independent collaboration library consumer and feature dependency profiles"
```

**Notes**

Registered isolated build target before qualification; no main/independent workspace target sharing.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --offline --message-format=short
```

**Notes**

First independent resolution selected uncached schemars_derive1.2.3; offline command failed before compilation. Retried normal public-registry access with the independent lock; root lock remained unchanged.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --locked --message-format=short
```

**Notes**

stock: 1passed,0failed. embedding-stock-2.log. Stock JJ creates a real upstream Git workspace; plain Git disables JJ; native example includes all five domains and exact prepared Agent/Thread replay.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --locked --no-default-features --message-format=short
```

**Notes**

plain Git: 1passed,0failed. embedding-git.log. Stock JJ creates a real upstream Git workspace; plain Git disables JJ; native example includes all five domains and exact prepared Agent/Thread replay.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --locked --no-default-features --features native --message-format=short
```

**Notes**

native without JJ: 2passed,0failed. embedding-native-only.log. Stock JJ creates a real upstream Git workspace; plain Git disables JJ; native example includes all five domains and exact prepared Agent/Thread replay.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --locked --features native --message-format=short
```

**Notes**

combined stock JJ/native: 2passed,0failed. embedding-combined.log. Stock JJ creates a real upstream Git workspace; plain Git disables JJ; native example includes all five domains and exact prepared Agent/Thread replay.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- run --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --locked --features native
```

**Notes**

Executable consumer completed both Git and native scenarios: independent store composition and original receipt replay passed. embedding-run.log.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --all-targets --locked --no-default-features -- -D warnings
```

**Notes**

plain Git: strict all-target lint passed for the independent consumer. embedding-clippy profile logs.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --all-targets --locked -- -D warnings
```

**Notes**

stock JJ: strict all-target lint passed for the independent consumer. embedding-clippy profile logs.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --all-targets --locked --no-default-features --features native -- -D warnings
```

**Notes**

native without JJ: strict all-target lint passed for the independent consumer. embedding-clippy profile logs.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --target-dir .local/build-targets/store-consumer --all-targets --locked --features native -- -D warnings
```

**Notes**

combined: strict all-target lint passed for the independent consumer. embedding-clippy profile logs.
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
cargo tree --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --edges normal --prefix none --format {p} --no-default-features
```

**Notes**

Active normal graph git: 164unique package names. No application service/server/HTTP/RPC, capture, search, replication or runtime package. Plain Git/stock omit native/WAL/object; plain Git/native-without-JJ omit jj-lib/JJ adapter. No forbidden dependency. Raw Cargo tree is read-only; no explicit cleanup ran concurrently.
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
cargo tree --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --edges normal --prefix none --format {p}
```

**Notes**

Active normal graph stock: 234unique package names. No application service/server/HTTP/RPC, capture, search, replication or runtime package. Plain Git/stock omit native/WAL/object; plain Git/native-without-JJ omit jj-lib/JJ adapter. No forbidden dependency. Raw Cargo tree is read-only; no explicit cleanup ran concurrently.
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
cargo tree --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --edges normal --prefix none --format {p} --no-default-features --features native
```

**Notes**

Active normal graph native-only: 239unique package names. No application service/server/HTTP/RPC, capture, search, replication or runtime package. Plain Git/stock omit native/WAL/object; plain Git/native-without-JJ omit jj-lib/JJ adapter. No forbidden dependency. Raw Cargo tree is read-only; no explicit cleanup ran concurrently.
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
cargo tree --manifest-path crates/jj-store-jj/tests/fixtures/store-consumer/Cargo.toml --locked --edges normal --prefix none --format {p} --features native
```

**Notes**

Active normal graph combined: 297unique package names. No application service/server/HTTP/RPC, capture, search, replication or runtime package. Plain Git/stock omit native/WAL/object; plain Git/native-without-JJ omit jj-lib/JJ adapter. No forbidden dependency. Raw Cargo tree is read-only; no explicit cleanup ran concurrently.
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
node --test --test-name-pattern="independent store consumer" scripts/managed-cargo.test.mjs
```

**Notes**

Exact new Cargo-root regression passed1 before full shared tooling suite.
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
node --test scripts/build-storage.test.mjs scripts/managed-cargo.test.mjs
```

**Notes**

Complete directly affected shared build-storage tooling suite passed16/16. Independent root selects its own target and rejects sharing the main target. embedding-storage-tools.log.
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
pnpm docs:check
```

**Notes**

First full documentation check compiled current docs_reference through managed Cargo, then detected stale installation/type references for the already-committed deletion contract. No product test failed.
<!-- /run -->
<!-- /evidence-field -->

