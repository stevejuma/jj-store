---
schema: 1
id: E-CRATE-READMES-001
---

# E-CRATE-READMES-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

Before:29tracked crate roots,4missing READMEs. The unchanged vendored Buzz README already supplies upstream overview/usage; its2pre-existing links to unbundled upstream documentation were observed separately.
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
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

Initial overview review found an incorrect new HTTP API overview link. Source inspection also corrected Terminal rather than a nonexistent Engine name in the terminal README. No production code changed.
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
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

An attempted HTTP API path still did not exist. Selected the actual docs/site/reference/http-api.md source; retained upstream vendor documentation rather than asserting its external-source link closure.
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
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

After:29crate roots,0missing README files;0broken first-party relative README links. Existing Git overview and upstream vendored overview remain present. New native/example crates must also receive root READMEs when introduced later.
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
node scripts/package-check.mjs
```

**Notes**

23coordinated packages retain portable registry metadata, contained compilation inputs and required package files.
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
pnpm exec prettier --write <changed Markdown files>
```

**Notes**

Formatted each authored overview, the relocated desktop runbook and owning chapters. Public entry points were checked against their owning source. Rust/UI builds, browser suites and root aggregate lanes are out of scope for prose-only changes; bundled Help sources/tooling and runtime behavior are unchanged.
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
pnpm check:progress
```

**Notes**

The first fresh record was written before its task-history row existed, so that changed history path was absent from files. Regenerated the same uncommitted record after including the task-history change.
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
pnpm spec:first
```

**Notes**

The first source gate surfaced that same missing fresh history coverage; no product or ownership assertion failed.
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
pnpm spec:validate
```

**Notes**

7validators/4106chapters/206requirements passed for the documentation slice.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed Markdown files>
```

**Notes**

The newly appended M32 history row and added milestone paragraph needed normal formatting; formatted those two documents. Other overviews and the relocated runbook were already formatted.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

After fixing fresh history coverage, the map required owning-topic updates for README paths across15area patterns and an explicit owner for the relocated desktop runbook. Added topic-specific unchanged-runtime confirmations and the new documentation/desktop mapping.
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
pnpm exec prettier --check <changed Markdown files>
```

**Notes**

The overview/runbook/ledger formatting repeat passed after formatting the2new ledger blocks.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

A repeat before the new canonical confirmations landed reported the same15area mappings and unmapped desktop runbook. No missing runtime gate was waived.
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
node --test scripts/governance.test.mjs scripts/progress-mapping.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

44passed/0failed. The added source-owner mapping consumes the shared spec/progress/navigation boundary, so these complete focused tooling families are the relevant wider documentation checks.
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
pnpm check:progress
```

**Notes**

162tasks/852records after all current documentation and source-map paths received fresh evidence coverage.
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
pnpm spec:first
```

**Notes**

30protected files/19canonical chapters passed. Every affected crate topic retains an explicit unchanged runtime-contract confirmation; the new standalone runbook has an explicit source owner.
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
pnpm spec:validate
```

**Notes**

7validators/4107chapters/206requirements passed.
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
python3 .local/injectable-stores/check-crate-readmes.py
```

**Notes**

Final29crate roots covered and all first-party relative README links resolve. Four formerly missing helper README files are present.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

