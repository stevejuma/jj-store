---
schema: 1
id: E-REUSABLE-AGENTS-011
---

# E-REUSABLE-AGENTS-011 details 2

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
node .local/reusable-conversations/history-wire-before.mjs
```

**Notes**

Before: direct JavaScript JSON parsing demonstrably rounds u64::MAX; retaining the inner JSON as text preserves the exact integer token.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p tauri-plugin-jj-store --features tauri --locked
```

**Notes**

Initial --locked attempt refused newly promoted production dependency metadata before compilation. Resolved offline; no registry version changed.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p tauri-plugin-jj-store --features tauri --offline
```

**Notes**

Resolved promoted production dependency metadata offline; no existing registry version changed.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p tauri-plugin-jj-store --features tauri --locked
```

**Notes**

Seven tests pass before adding schema equality: four real-JJ lifecycle/admission cases, strict frame regression and two wire regressions.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p tauri-plugin-jj-store --no-default-features --locked
```

**Notes**

Final headless eight tests pass, including exact checked-schema equality.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p tauri-plugin-jj-store --features tauri --all-targets --locked -- -D warnings
```

**Notes**

Strict native transport all-target lint passes, including schema example and test.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p tauri-plugin-jj-store --no-default-features --all-targets --locked -- -D warnings
```

**Notes**

Strict headless all-target lint passes.
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
CARGO_INCREMENTAL=0 pnpm --silent dev:cargo -- run -p tauri-plugin-jj-store --example schema --quiet --locked
```

**Notes**

Generated owned public schema; 4,015 lines / 85,920 bytes.
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
node --test scripts/governance.test.mjs scripts/local-history-features.test.mjs scripts/ci*.test.mjs scripts/release.test.mjs scripts/docs.test.mjs
```

**Notes**

94 relevant governance/CI/release/docs regressions pass.
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
pnpm docs:review history-bridge 'Reviewed strict public inner frames, bounded version-one JSON text request/result envelopes, full-width integer preservation, generated Rust schema equality and unchanged typed service admission, permissions, source pins and lifecycle.'
```

**Notes**

Owning source review refreshed.
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
node scripts/docs.mjs render
```

**Notes**

144 files / 26,691,126 bytes; nine offline diagrams, links and anchors valid.
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
node scripts/check-boundaries.mjs
```

**Notes**

25 package boundaries pass.
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
node scripts/package-check.mjs
```

**Notes**

26 Cargo packages pass contained-input, overview, license and versioned dependency checks.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Initial authored formatting check identified four unformatted Markdown/coverage files; corrected with Prettier.
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
node .local/reusable-conversations/client-format.mjs
```

**Notes**

Final authored formatting passes.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- fmt -p tauri-plugin-jj-store -- --check
```

**Notes**

Owning Rust formatting passes.
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
pnpm spec:validate
```

**Notes**

Initial fresh evidence navigation omitted one child SUMMARY include. Added the exact include without regenerating historical evidence.
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

18 protected files / five owning canonical chapters pass.
<!-- /run -->
<!-- /evidence-field -->

