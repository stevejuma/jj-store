---
schema: 1
id: E-CI-IMAGE-WEB-PLAYWRIGHT-001
---

# E-CI-IMAGE-WEB-PLAYWRIGHT-001

<!-- evidence-field:id markdown -->
## Id

E-CI-IMAGE-WEB-PLAYWRIGHT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/publish-main-image.yml
- scripts/ci-workflows.test.mjs
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/14/E-CI-IMAGE-WEB-PLAYWRIGHT-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-workflows.test.mjs#successful main builds publish bounded source
  and rolling image tags
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
GitHub Actions run 43616733741 job web on f70b88172eb0f9e36597d1ff6fac39fd6b41b67d
```

**Notes**

`pnpm web:build` reached documentation diagram rendering and failed because
Playwright Chromium `1243` was not installed on the hosted runner.
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
node --test scripts/ci-workflows.test.mjs
```

**Notes**

The new workflow regression failed before implementation because no Chromium
installation step preceded `pnpm web:build`; seven of eight tests passed.
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
node --test scripts/ci-workflows.test.mjs
```

**Notes**

All eight workflow regressions passed after installing the lockfile-matched
Chromium browser, including its hosted system dependencies, before web build.
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
pnpm check:progress
pnpm exec spec-validator search "JST-ARCH-005" --json
pnpm exec spec-validator doctor --json
pnpm exec spec-validator validate --only governance,summary,fileLimits,verification --json
pnpm exec spec-validator first --base main --head @ --json
pnpm exec prettier --check <changed files>
```

**Notes**

Progress validated 186 tasks and 1,045 evidence records. QMD search returned the
owning public-distribution chapter, doctor passed with the existing wrapped-script
warning, four validators reported no findings, spec-first passed, and all changed
files matched repository formatting.
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

Node 24.19.0 locally and 22.23.4 on the failed runner; Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No source, package, archive, image-tag or persisted-data format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The main-image web job installs the lockfile-matched Chromium runtime before
`web:build` stages documentation diagrams. Architecture and index publication
remain gated on that exact source-bound web artifact.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Merge the repair, rerun main-image publication for merged main, then qualify
the final release source after the dependency stack lands.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 still requires an immutable, qualified browser and Help
  artifact before image publication. The workflow now installs the runtime
  already required by documentation diagram rendering.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Local workflow regression coverage passes;
  hosted GHCR publication and the final qualified release remain open.
```
<!-- /evidence-field -->
