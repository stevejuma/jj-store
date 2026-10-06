---
schema: 1
id: E-UI-090
---

# E-UI-090 details 5

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
pnpm catalog:test (final)
```

**Notes**

43 catalog/controller files /238 tests pass; complete direct unit consumer surface. /tmp/rich-search-catalog-unit-final.log
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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte (final)
```

**Notes**

Two keyboard and accessibility production-component cases pass. /tmp/rich-search-stories-final.log
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
pnpm web:build (final)
```

**Notes**

Production assets rebuilt, only inherited CSS parser/chunk-size warnings. /tmp/rich-search-web-build-final.log
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
cargo test --locked -p jj-store-jj query::language (final)
```

**Notes**

Three exact language cases pass including bare bookmark mine, mine() callable and builtin_log workspace context. Unrelated integration binaries contain only filtered tests. /tmp/rich-search-language-tests-final.log
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
cargo fmt -p jj-store-jj -p jj-store-http -p jj-store-cli -- --check
```

**Notes**

Owning packages formatted. /tmp/rich-search-rust-format-final.log
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
pnpm exec prettier --write (changed consumer files via explicit subprocess arguments)
```

**Notes**

Corrected formatter argument list passes. /tmp/rich-search-final-format2.log
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md; pnpm check:progress
```

**Notes**

120 tasks; evidence bookkeeping passes. /tmp/rich-search-ledger-format.log and progress.log
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries (inherited before state)
```

**Notes**

Existing jj-store-service production jj-review dependency is explicitly authorized by architecture.md, but unchanged audit map rejects it. Parent manifest and audit reproduce the mismatch; no production dependency added. /tmp/rich-search-boundaries-final.log
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test --test-name-pattern=service_review_models_preserve_core_layer_boundaries scripts/governance.test.mjs (before)
```

**Notes**

Exact new regression fails with inherited invalid-layer report. /tmp/rich-search-boundary-before.log
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
node --test --test-name-pattern=service_review_models_preserve_core_layer_boundaries scripts/governance.test.mjs (after)
```

**Notes**

One regression passes after canonical model-map alignment; reverse core/publication dependencies still rejected. /tmp/rich-search-boundary-after.log
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
pnpm check:boundaries (final)
```

**Notes**

16 owned packages pass; upstream JJ and persistence boundaries remain intact. /tmp/rich-search-boundaries-final2.log
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
pnpm test:governance
```

**Notes**

42 passed. Wider governance suite required because the shared static dependency audit and contract consumer check changed. /tmp/rich-search-governance-final.log
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check (isolated first)
```

**Notes**

Generated artifacts and runtime response validation passed, seven contract unit tests passed, then compile-time consumer rejected the new structured error union in IndexTask. Consumer narrowing repaired. /tmp/rich-search-isolated-contracts.log
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
pnpm test:contracts (after typed diagnostic narrowing)
```

**Notes**

Seven validation tests and TypeScript consumer acceptance pass; strings retain old behavior and invalid_query exposes its diagnostic message. /tmp/rich-search-contract-types-fixed.log
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
cargo test --locked -p jj-store-cli --test local_browser
```

**Notes**

All7 local-folder reader/registration/history cases pass. /tmp/rich-search-local-browser-final.log
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
jj restore --from @- pnpm-lock.yaml; pnpm install --lockfile-only --ignore-scripts --ignore-pnpmfile
```

**Notes**

Portable registry lock generated with CodeMirror autocomplete and Lezer highlight1.2.5; no checkout specifiers/checksum. Explicit assertion passes. Source validation node_modules remains separate. /tmp/rich-search-portable-lock-final.log
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
pnpm web:check; pnpm catalog:check; pnpm exec tsx --test web/tests/store-route-codec.test.ts (final)
```

**Notes**

Zero errors/warnings,131 catalog modules and11 URL codec tests. /tmp/rich-search-web-check-final.log, catalog-check-final.log, route-tests-final.log
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
pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:first; pnpm spec:validate (latest)
```

**Notes**

738 modules, ownership pass,56 protected files /9 chapters and146 requirements. Subsequent governance source map requires final rerun. /tmp/rich-search-architecture-final2.log and *-final.log
<!-- /run -->
<!-- /evidence-field -->

