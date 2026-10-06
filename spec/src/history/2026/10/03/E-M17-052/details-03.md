---
schema: 1
id: E-M17-052
---

# E-M17-052 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-http --lib contract::capture_tests::capture_management_contract_is_installation_local_and_actor_free --locked -- --exact > /tmp/jj-store-agent-capture-m172-rescan-http-contract-after.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.07s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-http --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-rescan-http-library-after.log 2>&1
```

**Notes**

Explicit serial profile, unchanged within-scenario concurrency, original worker deadlines and observation bounds. test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.83s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo check -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-rescan-check.log 2>&1
```

**Notes**

All direct server/service/HTTP/CLI targets compile.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo clippy -p jj-store-server -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-rescan-lint.log 2>&1
```

**Notes**

All direct server/service/HTTP/CLI targets pass strict lint.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm contracts:generate > /tmp/jj-store-agent-capture-m172-rescan-contract-generation.log 2>&1
```

**Notes**

Regenerates 88 paths, 67 native and 21 local routes, preserving existing contracts. One additive rescan route and cached discovery/degradation fields.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm contracts:check > /tmp/jj-store-agent-capture-m172-rescan-contracts.log 2>&1
```

**Notes**

All 166 actual registered HTTP/SSE responses conform, including 19 rescan responses and all prior captures. Seven contract-tool tests and TypeScript contract fixture compile pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm web:check > /tmp/jj-store-agent-capture-m172-rescan-web-check.log 2>&1
```

**Notes**

Generated contract direct web consumer: zero errors and zero warnings.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rescan-ledger-format.log 2>&1
```

**Notes**

Formats new single-ledger entry without altering prior 306 parsed entries.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-rescan-spec-first.log 2>&1
```

**Notes**

Protected implementation and shared runner map to changed owning canonical chapters.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-rescan-progress.log 2>&1
```

**Notes**

All 129 tasks and 307 entries validate, including unchanged prior 306 entries.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-rescan-rust-format.log 2>&1
```

**Notes**

Pinned Rust and unchanged embedded-runtime formatting pass.
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
pnpm exec prettier --check scripts/contracts.mjs schemas/openapi.json web/src/lib/api/generated.ts spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rescan-format-check.log 2>&1
```

**Notes**

All direct changed scripts/generated artifacts/canonical documents pass formatting; repeated at final sealing.
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
pnpm exec prettier --write spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-rescan-final-format.log 2>&1
```

**Notes**

Final sealing passes after product, registered capture, generated consumer and ledger result recording; document/progress checks repeated after recording these completed commands.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-rescan-final-validate.log 2>&1
```

**Notes**

Final sealing passes after product, registered capture, generated consumer and ledger result recording; document/progress checks repeated after recording these completed commands.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-rescan-final-first.log 2>&1
```

**Notes**

Final sealing passes after product, registered capture, generated consumer and ledger result recording; document/progress checks repeated after recording these completed commands.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-rescan-final-progress.log 2>&1
```

**Notes**

Final sealing passes after product, registered capture, generated consumer and ledger result recording; document/progress checks repeated after recording these completed commands.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-rescan-final-boundaries.log 2>&1
```

**Notes**

Final sealing passes after product, registered capture, generated consumer and ledger result recording; document/progress checks repeated after recording these completed commands.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-rescan-final-architecture.log 2>&1
```

**Notes**

Final sealing passes after product, registered capture, generated consumer and ledger result recording; document/progress checks repeated after recording these completed commands.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1. Native MemoryStore, real private filesystem and actual loopback transport/fresh processes. No real RustFS or live harness acceptance.
<!-- /evidence-field -->

