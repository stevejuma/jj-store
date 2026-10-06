---
schema: 1
id: E-M15-011
---

# E-M15-011 details 2

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
cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance::http_threads_reactions_keep_exact_targets_assets_signed_roots_and_receipts --locked -- --exact (before)
```

**Notes**

Command exits successfully but executes zero tests; absent target is not acceptance evidence.
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
cargo fmt -p jj-store-http -p jj-store-cli
```

**Notes**

Repeated after edits.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 25
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-http --all-targets --locked (initial)
```

**Notes**

One root cause: glob collab_model import brought Error/Result names into the HTTP module. Replaced with explicit imports; /tmp/jj-collab-http-check.log.
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
cargo check -p jj-store-http --all-targets --locked (fixed)
```

**Notes**

/tmp/jj-collab-http-check-fixed.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance --locked (initial)
```

**Notes**

Three fixture root mistakes: expected_message versus expected_revision, replication versus replication_root, and formatting Zeroizing rather than its string view. Corrected; /tmp/jj-collab-http-test.log.
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
cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance --locked (fixed)
```

**Notes**

Both registered transport regressions pass; /tmp/jj-collab-http-test-fixed.log.
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
cargo test -p jj-store-cli --test native_lifecycle http_threads_reactions_acceptance --locked (final)
```

**Notes**

Two tests pass after adding HEAD/range assertions and full route contract capture; /tmp/jj-collab-http-test-final.log.
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
pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/spec-governance.md scripts/contracts.mjs
```

**Notes**

Focused canonical and generator formatting.
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
pnpm exec prettier --write scripts/contracts.test.mjs scripts/contracts.types.ts
```

**Notes**

Contract regression and compile-time acceptance formatted.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:generate (initial)
```

**Notes**

One root OpenAPI error: description was placed on the binary media object instead of requestBody. Moved it; /tmp/jj-collab-http-contracts-generate.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-http --locked (initial)
```

**Notes**

Shared route-inventory assertion expected 40; baseline generated inventory already has 44 and this slice adds 14. Updated the exact count to 58 and asserted M15 routes; /tmp/jj-collab-http-suite.log.
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
cargo test -p jj-store-http --locked (fixed)
```

**Notes**

Complete relevant HTTP suite: 12 library tests plus five asset tests and zero doctests; /tmp/jj-collab-http-suite-fixed.log.
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
cargo test -p jj-store-cli --test native_lifecycle http_ --locked
```

**Notes**

All 38 HTTP consumer tests pass; /tmp/jj-collab-http-consumers.log.
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
cargo clippy -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Strict HTTP and direct CLI lint passes; /tmp/jj-collab-http-clippy.log.
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
pnpm contracts:generate (fixed)
```

**Notes**

OpenAPI validates and both artifacts regenerate; /tmp/jj-collab-http-contracts-generate-fixed.log.
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

Generated contract consumer: zero errors/warnings; /tmp/jj-collab-http-web-check.log.
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
pnpm ui:catalog channel (lapis-community)
```

**Notes**

Read-only source inspection of Community catalog/owning channel conversation. User requires channels as the Thread UI template. No Community edits, package publication or visual acceptance.
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
pnpm check:progress (initial)
```

**Notes**

Recorded zero-test successful command as fail with zero failures, which the ledger rejects. Corrected the command result to pass while retaining explicit absence of acceptance; no qualification claim.
<!-- /run -->
<!-- /evidence-field -->

