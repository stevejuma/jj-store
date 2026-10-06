---
schema: 1
id: E-GOV-003
---

# E-GOV-003 details 6

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
pnpm add -D @lapismd/spec-validator@^0.1.4
```

**Notes**

Installed the released registry package; the manifest and lockfile contain a portable semver dependency.
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
node --test scripts/spec/vcs-regression.test.mjs
```

**Notes**

Two actual JJ fixtures without a colocated .git prove pinned explicit revision ranges and all-parent merge coverage.
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
pnpm test:spec
```

**Notes**

19 consumer regressions pass: byte/field fidelity, Unicode and line bounds, delimiter safety, draft cleanup, ID uniqueness, qualified confirmations, resolutions, pinned revisions, registry JJ ranges/merges and book references.
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
pnpm test:governance
```

**Notes**

50 governance tests pass; existing task/dependency/gate, historical mapping, ownership and architecture tests are retained.
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
pnpm check:architecture
```

**Notes**

1163 web/catalog modules pass the existing architecture boundary; product source is unchanged.
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
pnpm docs:check
```

**Notes**

Documentation consumers pass fingerprint/review, Mermaid generation, strict build and closure: 139 files, 15,228,329 bytes.
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
node scripts/spec/migration-audit.mjs
```

**Notes**

615 historical payloads and their order, 142 original task states/dependencies/gates/evidence, 173 original IDs and statements, immutable source hashes and authored budgets are preserved. Only GOV-002/GOV-003 statements change by the accepted governance plan.
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
pnpm spec:search JST-GOV-010 --json
```

**Notes**

Default lexical results stay under current and expose real absolute source paths and line numbers.
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
pnpm spec:search --scope history E-READ-094 --limit 8 --json
```

**Notes**

Explicit history lookup finds the original record plus E-READ-091/E-READ-096 context. Resolution annotations retain the original outcomes.
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
node /Users/stevejuma/code/lapismd/spec-validator/dist/cli.js search --semantic "confirmation that protected refactor leaves contract unchanged" --limit 5 --json
```

**Notes**

Temporary source validation used actual local QMD embeddings; conceptual current retrieval found the owning governance and verification topics.
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
node /Users/stevejuma/code/lapismd/spec-validator/dist/cli.js search --semantic --scope history "cooperative cancellation supersedes hard comparison deadline" --limit 5 --json
```

**Notes**

Actual history collection embedding/vector retrieval stays inside historical sources; no fake QMD result or substituted test.
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
pnpm spec:search --semantic --scope all "unchanged contract confirmation and milestone review" --limit 5 --json
```

**Notes**

Actual registry CLI all-scope embedding/vector retrieval passes. The QMD database and models remain local discovery caches.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/spec/navigation-regression.test.mjs
```

**Notes**

Wide generated navigation exceeded the authored limit, and a literal ## Pages heading inside a command was truncated. Both regressions now pass without changing original evidence payloads.
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
node --test scripts/spec/navigation-regression.test.mjs
```

**Notes**

450-topic navigation remains bounded, complete and byte-stable after repeated regeneration; command headings round-trip exactly.
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
pnpm exec prettier --check scripts/spec/*.mjs scripts/check-progress.mjs spec-validator.config.mjs package.json .qmd/index.yml AGENTS.md spec/src/current/**/*.md spec/src/*.md
```

**Notes**

All changed code and authored current/compatibility documents use the repository formatter. Historical data and immutable archives remain intentionally unformatted.
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
pnpm check:progress
```

**Notes**

143 tasks / 616 records; fresh file coverage, original task gates and validated reviews pass. This proves bookkeeping, not product acceptance.
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
JJ_STORE_CHECK_REVISION=aae49090 pnpm check:progress
```

**Notes**

Pinned pre-migration revision passes using the original 142 tasks and 615 records; later working files are not substituted.
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
JJ_STORE_CHECK_REVISION=potwxpyw pnpm check:progress
```

**Notes**

Selected migration change passes with the split ledger and Markdown evidence.
<!-- /run -->
<!-- /evidence-field -->

