---
schema: 1
id: E-M4-016
---

# E-M4-016 details 2

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
pnpm format:rust:check
```

**Notes**

Before change: pinned Rust 1.97.1 rustfmt policy and normal/spec integration verified; formatting passes.
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
pnpm contracts:check
```

**Notes**

Before change: required generated contract checker command absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-http --all-targets
```

**Notes**

Initial scaffolding: contract test module not yet added (E0583).
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib contract::tests --locked
```

**Notes**

Initial tests exposed omitted properties on the empty query schema. Generator now handles an empty deny-unknown-field object explicitly.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib contract::tests --locked
```

**Notes**

Fixture wrongly expected a named transparent RefTarget schema. Assertion now checks its actual inline array in NativeRemoteRef.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib contract::tests --locked
```

**Notes**

3 contract shape/inventory tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

Exact lifecycle filter omitted nested module names and selected zero tests. Checker rejected that run; corrected to content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Expected missing same-change ledger/update diagnostics before adding this evidence entry.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate
```

**Notes**

OpenAPI 3.1 validates without external references; generated JSON and TypeScript written deterministically.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

Generated artifacts match; 25 actual HTTP/SSE serialized responses conform. Five Node schema tests and TypeScript consumer acceptance pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

58 requirements, package boundaries, progress, rustfmt, regenerated contract conformance and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

242 Rust tests pass / 6 external tests ignored; 98 native lifecycle tests. 16 governance tests, 3 storage harness self-tests, 5 contract tests and TypeScript consumer checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production workspace build succeeds with schema traits and typed capabilities/detail/error responses.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 10 lanes pass, including generated contract conformance, rustfmt, repeated full Rust tests, mdBook and mapped spec-first.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after source, generated artifact, specification and ledger formatting.
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

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; schemars 1.2.2; openapi-typescript 7.13.0; swagger-parser 13.1.0; Ajv 8.20.0; TypeScript 5.9.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

OpenAPI 3.1 / JSON Schema 2020-12 plus generated TypeScript read contracts. No native persistence, identity, RPC or HTTP wire-shape changes. Enclosing Jujutsu change is the version-control reference. M4.2 remains In progress for browser implementation and real SSE deployment qualification; E-M4-015 retains the pending shared Docker restart approval and live isolated harness. M5, M6 and M8a remain unfinished.
<!-- /evidence-field -->

