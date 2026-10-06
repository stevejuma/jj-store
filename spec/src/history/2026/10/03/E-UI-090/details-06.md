---
schema: 1
id: E-UI-090
---

# E-UI-090 details 6

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
pnpm contracts:check (final)
```

**Notes**

Generated artifacts match;151 actual HTTP/SSE responses conform,7 validation tests and TypeScript consumer acceptance pass. /tmp/rich-search-contracts-final2.log
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
node scripts/browser-fixtures.mjs; cargo build --locked -p jj-store-http --example browser_preview
```

**Notes**

Captured native fixture responses and prebuilt isolated production-asset host. /tmp/rich-search-isolated-fixtures.log and preview.log
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/history-query.spec.ts (cold second)
```

**Notes**

Local profile passed; native completion test used broad /mutable/ matcher and matched four distinct aliases. Replaced with exact mutable() alias assertion. /tmp/rich-search-cold-browser2.log
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/history-query.spec.ts (cold third)
```

**Notes**

Local profile passed; native clear assertion counted the CodeMirror placeholder as document text. Asserted actual empty-editor placeholder, absent clear action and cleared accepted URL instead. Query success wait also now requires the exact applied text. /tmp/rich-search-cold-browser3.log
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/history-query.spec.ts (cold final)
```

**Notes**

Both native and filesystem UI profiles pass from a fresh production-asset host under unchanged STRICT_CSP. Native includes real keyboard completion, static highlighting, invalid draft retention, frozen operation/time, clear, operation grammar and axe; local verifies pinned custom grammar transport. Browser transports use explicit deterministic fixtures; actual query semantics are separately proved by Rust/local/HTTP tests. /tmp/rich-search-cold-browser4.log
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress (final after governance updates)
```

**Notes**

All protected source/test/schema/chapter changes mapped;146 requirements,120 tasks. /tmp/rich-search-spec-first-final2.log, spec-validate-final2.log, progress-final.log
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
pnpm exec prettier --check (all changed consumer source/docs/schema files)
```

**Notes**

All checked files formatted. /tmp/rich-search-format-check-final.log. Final browser/ledger formatting repeated separately.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-only installation and qualification require a Design Core release
  containing custom-language mode. Source package and consumer acceptance are
  verified; no package publishing requested or performed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused native/local graph/query/HTTP/frontend contract surface; complete source Design Core nonvisual lanes required because the generic public component spans repositories. Root jj-store aggregates, unrelated Rust/runtime/RustFS suites and Design Core screenshots/baseline updates are out of scope; visual tests explicitly excluded by user. No milestone completion or release/deployment claim. Shared static boundary-audit repair ran all42 governance tests; no additional production dependency or storage format change. All functional qualification failures are resolved. Disposable source/static/preview hosts were cleaned up. No root/full-Rust/real-RustFS or registry-only full-flow claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Release the qualified Design Core extension through the normal workflow, refresh the consumer registry range/lockfile if needed, and rerun registry-only acceptance. Retain explicit deferrals for visual baselines and wider M13 work.
<!-- /evidence-field -->

