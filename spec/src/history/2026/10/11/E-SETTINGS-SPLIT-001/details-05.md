---
schema: 1
id: E-SETTINGS-SPLIT-001
---

# E-SETTINGS-SPLIT-001 final checks

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-ownership.test.ts web/tests/vite/production-store-settings.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/managed-code-index.test.ts web/tests/vite/kind-preparation.test.ts
```

**Notes**

38 pass, including the new exact-index ownership test: code-read cancellation neither aborts nor duplicates its task. Independent owner replacement still rejects old replies. /tmp/jj-settings-final-index-ownership-after.log.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits
```

**Notes**

Three document validators pass (3596 chapters, 181 requirements before this final run page). Earlier skeleton-only run reported a missing Summary entry; adding scoped date/record navigation resolves it. No authored budget changed. /tmp/jj-settings-final-doc-validation.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Only inherited diagnostics remain: missing .pnpmfile.cjs, the malformed installed-package evidence file/test reference, invalid run result and invalid result under E-NPM-REGISTRY-001. New Settings file/evidence/history coverage is complete; removed picker component is recorded as removed_files. /tmp/jj-settings-final-progress.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The same five inherited evidence-integrity diagnostics prevent a trustworthy spec-first change set. No new Settings contract/evidence diagnostic remains. /tmp/jj-settings-final-spec-first.log.
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
pnpm exec prettier --write <changed Markdown, JSON, TypeScript, Svelte and CSS paths>; pnpm exec prettier --check <same paths>
```

**Notes**

All changed authored files are formatted. No root formatting, baseline update or dependency override.
<!-- /run -->
<!-- /evidence-field -->
