---
schema: 1
id: E-M13-015
---

# E-M13-015 details 2

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-server -p jj-store-cli --locked
```

**Notes**

New async-trait HTTP dependency requires lock inventory refresh.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-server -p jj-store-cli --offline
```

**Notes**

Offline lock update succeeded; five shadowed body decoder diagnostics were corrected.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo check -p jj-store-server -p jj-store-cli --locked
```

**Notes**

Composes optional native runtime; one unused timeout warning was subsequently removed.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

Fixture JSON timestamp literal needed explicit i64 suffix.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage --exact
```

**Notes**

Filesystem-only listener, read-only inspection, registration/replay, mixed discovery schema and pinned graph read pass without native runtime material.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Existing generator rejected multiple methods on one path; additive registration GET/POST exposed it.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked -- contract::tests::local_contracts_preserve_separate_profiles_and_strict_pinned_queries --exact
```

**Notes**

Reproduced duplicate path failure at exact contract boundary.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked -- contract::tests::local_contracts_preserve_separate_profiles_and_strict_pinned_queries --exact
```

**Notes**

Generator now merges distinct methods and rejects repeated methods.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:generate
```

**Notes**

Separate local schemas and query union generated.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked
```

**Notes**

Existing inventory test expected 40 paths despite committed default schema already containing 52; new local routes bring total to 60.
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
jj file show -r 2994c626 schemas/openapi.json | python3 -c path-count
```

**Notes**

Confirmed committed default contains exactly 52 native paths; kept explicit native/local counts and strengthened local admission assertions.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked
```

**Notes**

All 13 HTTP unit/contract tests pass, including existing native security and identity assertions.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-server --test configuration --test local_installation --locked
```

**Notes**

12 configuration and 6 existing installation tests pass; dedicated filesystem configuration test added afterward.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-server --test local_installation --locked -- filesystem_initialization_is_private_and_rejects_nonlocal_authority --exact
```

**Notes**

Dedicated filesystem initialization, private installation material and non-local rejection pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

104 actual native/local HTTP/SSE responses conform; 6 contract validation tests and contract TypeScript checks pass.
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
pnpm web:check
```

**Notes**

Zero errors or warnings in generated contract consumer.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Mapped local-JJ chapter missing in runtime slice; explicit filesystem contract added.
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
pnpm spec:first
```

**Notes**

22 protected files and 9 canonical chapters passed after mapped chapter update.
<!-- /run -->
<!-- /evidence-field -->

