---
schema: 1
id: E-M10-012
---

# E-M10-012

<!-- evidence-field:id markdown -->
## Id

E-M10-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

review-command-contract-regression-gate
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/contract/review_tests.rs
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/contract/review_tests.rs#collaboration_contract_exposes_receipt_backed_review_commands
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
pnpm spec:check
```

**Notes**

The cold catalog, Storybook, browser, contract and earlier Rust stages passed, but the HTTP review contract test still expected seven commands after resolve_comment_conflict became the eighth canonical variant.
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
cargo test -p jj-store-http --lib contract::review_tests::collaboration_contract_exposes_receipt_backed_review_commands --locked -- --exact
```

**Notes**

The generated OpenAPI contract exposes all eight exact review command tags and requires observed revision IDs for conflict resolution.
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
pnpm spec:check
```

**Notes**

Catalog, Storybook, browser, contracts, Rust tests and mdbook passed; spec-first then required the mapped canonical API chapter for the protected HTTP contract test update.
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
pnpm spec:first
```

**Notes**

The protected HTTP test has canonical API ownership.
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
pnpm spec:validate
```

**Notes**

The canonical specification validates after the HTTP contract mapping.
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
pnpm check:progress
```

**Notes**

The progress evidence mapping validates after the HTTP contract mapping.
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
pnpm spec:check
```

**Notes**

All 18 specification lanes pass with the eight-command review contract test.
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

Rust 1.98.1; schemars generated OpenAPI v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire format change; this test aligns with the existing eight-command ReviewCommandV1 contract.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


