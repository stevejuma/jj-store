---
schema: 1
id: E-M21-002
---

# E-M21-002 details 3

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
pnpm exec prettier --check web/src/lib/controller/native-data-source.ts web/src/lib/controller/http-local-code-data-source.ts web/tests/vite/local-stack-history.test.ts spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md docs/site/technical/local.md docs/site/reference/transport.md docs/maintenance/coverage.json
```

**Notes**

Source coverage review changed coverage.json formatting; formatted that file only. reads-format.log
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
pnpm exec prettier --check web/src/lib/controller/native-data-source.ts web/src/lib/controller/http-local-code-data-source.ts web/tests/vite/local-stack-history.test.ts spec/src/api.md spec/src/cli.md spec/src/compatibility.md spec/src/documentation.md spec/src/jj-integration.md spec/src/local-jj.md docs/site/technical/local.md docs/site/reference/transport.md docs/maintenance/coverage.json
```

**Notes**

Final changed-file format check passes; ledger excluded. reads-final-format.log
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
pnpm spec:first
```

**Notes**

17 protected files/six chapters pass; reads-spec-first-final.log
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
pnpm spec:validate
```

**Notes**

Five validators, 23 chapters, 173 requirements pass; reads-spec-validate-final.log
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
pnpm check:progress
```

**Notes**

Updated bounded-read evidence and preserved historical payloads; reads-progress.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Lazy real adapter/controller injection, durable
  preparation/publication/checkout, combinations, exact file/hunk and graph
  commands, linked native execution and cold complete lifecycle remain. This
  bounded-read slice is verified; M21.2 is incomplete.
- Five inherited Clippy diagnostics recorded in contract slice remain unchanged;
  no full Clippy pass claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue lazy operation-pinned presentation reads in descendant pxztpulz, then integrate the real application lifecycle and durable local execution.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope markdown -->
## Out of scope

Root aggregate lanes, full Rust workspace tests, package publication, installed-service changes, screenshot comparisons and visual baselines.
<!-- /evidence-field -->

