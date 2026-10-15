---
schema: 1
id: E-CI-STANDARD-001
---

# E-CI-STANDARD-001 details 2

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
node --test scripts/ci.test.mjs scripts/release.test.mjs scripts/automation-governance.test.mjs
```

**Notes**

23 regressions passed on Node 24.10.0. Linux/Node22 release-tooling run recorded separately.
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
actionlint
```

**Notes**

YAML serialization folded a shell continuation in the existing actionlint installer; restored original command strings and used literal blocks. No installer behavior change.
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
actionlint
```

**Notes**

All workflows passed after preserving literal command bodies.
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

71 tests passed on Node 22.23.3.
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
pnpm test:spec
```

**Notes**

28 tests passed on Node 22.23.3.
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
pnpm release:check
```

**Notes**

23 regressions and all 20 Cargo package containment/version checks passed on Node 22.23.3.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

New workflow regressions exceeded the 500-line limit in ci.test.mjs. Split them into ci-workflows.test.mjs and included all CI regression files in release:check.
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
pnpm check:architecture && pnpm check:ui-ownership && pnpm spec:validate --only governance,summary,fileLimits,verification && pnpm check:progress
```

**Notes**

First three passed; progress caught historical test anchors renamed during refactoring, missing new-file evidence and unrelated navigation whitespace. Restored anchors, scoped navigation generation to the new evidence/day and restored unrelated generated whitespace.
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
pnpm exec prettier --check .
```

**Notes**

Unrelated generated navigation whitespace was unformatted. Restored that churn and formatted only the changed evidence/navigation. The reporting shell also rejected its read-only status variable; subsequent reporting uses no such variable.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm release:check
```

**Notes**

After splitting CI tests, the automation regression still expected the old literal filename. Updated it to require the new CI test glob; both files remain in the release tooling gate.
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
pnpm check:architecture
```

**Notes**

Passed after the test split, historical-anchor preservation, owning-topic updates and scoped evidence repair.
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
pnpm check:ui-ownership
```

**Notes**

Passed after the test split, historical-anchor preservation, owning-topic updates and scoped evidence repair.
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Passed after the test split, historical-anchor preservation, owning-topic updates and scoped evidence repair.
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
pnpm check:progress
```

**Notes**

Passed after the test split, historical-anchor preservation, owning-topic updates and scoped evidence repair.
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
pnpm spec:first
```

**Notes**

Passed after the test split, historical-anchor preservation, owning-topic updates and scoped evidence repair.
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
pnpm format:rust:check
```

**Notes**

Passed after the test split, historical-anchor preservation, owning-topic updates and scoped evidence repair.
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
pnpm release:check
```

**Notes**

Final Node 22 run: 24 tests passed, all 20 package-contained inputs and coordinated metadata passed.
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
pnpm exec prettier --check .
```

**Notes**

Complete formatter gate passed after restoring all unrelated generated pages.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

