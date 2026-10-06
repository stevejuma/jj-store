---
schema: 1
id: E-FRESH-001
---

# E-FRESH-001

<!-- evidence-field:id markdown -->
## Id

E-FRESH-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

prpswkts
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Isolated event-freshness workspace and accepted Threads watermark
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ workspace based on f5cd62d4; pnpm10.32.1; Vitest4.1.11; TanStack6.3.0
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire or persistence change. Historical route selections retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/controller/production-store-refresh.ts
- web/tests/vite/automatic-store-refresh.test.ts
- spec/src/api.md
- spec/src/ui.md
<!-- /evidence-field -->

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
jj workspace add /Users/stevejuma/code/lapismd/jj-store-event-freshness --name event-freshness -r f5cd62d4 -m Implement-event-driven-application-freshness
```

**Notes**

Clean isolated working copy; bounded-reads committed follow-up stays independent.
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
pnpm install --frozen-lockfile
```

**Notes**

Inherited local-package hook checksum mismatch. pnpm install --no-frozen-lockfile succeeds under existing Design Core source-validation hook; tracked lock restored, no manifest change or registry qualification.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/automatic-store-refresh.test.ts -t 'successful Threads'
```

**Notes**

Before state: accepted sequence remains1 instead of2.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

22pass/3files. An exploratory snapshot narrowing exposed the old collaboration refresh contract; reverted that change for the cache-ownership slice.
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
pnpm web:check
```

**Notes**

0errors/0warnings.
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
pnpm exec prettier --check web/src/lib/controller/production-store-refresh.ts web/tests/vite/automatic-store-refresh.test.ts spec/src/api.md spec/src/ui.md; pnpm spec:validate; pnpm check:progress
```

**Notes**

Owning spec and same-change ledger validation.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Shared cache leases, application event service/producers, browser migration,
  generated contracts/manual and cold filesystem/real-RustFS qualification
  remain. Root aggregate and visual baseline lanes out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement shared cache ownership and cancellation without changing operation pins.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/automatic-store-refresh.test.ts#a successful Threads refresh
  accepts freshness without moving its code pin
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


