---
schema: 1
id: E-M21-066
---

# E-M21-066 details 3

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
pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; pnpm check:architecture
```

**Notes**

spec-first-final.log: 19 protected files/4 chapters; spec-validation-final.log: 5 validators/23 chapters/173 requirements; boundaries.log:17 packages; architecture.log:1163 modules pass.
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
target/debug/jj-store --no-pager stack --help; target/debug/jj-store --no-pager --format=json stack status --no-snapshot
```

**Notes**

after-help.log and main-read-only.log: built executable discovers commands and inspects main without creating Stacks settings or changing graph/checkout. Installed CLI and live services untouched.
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
pnpm docs:check
```

**Notes**

docs-check-final.log: strict manual build succeeds;9 diagrams rendered in both themes;139 files/15,228,329 bytes with valid links/anchors and current source reviews.
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
pnpm check:progress; pnpm spec:first
```

**Notes**

progress-final.log:142 tasks evidence bookkeeping; spec-first-delivery.log:20 protected files/5 chapters. Historical614 entries remain byte-identical; no milestone status promoted.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check changed canonical/manual/review files
```

**Notes**

prettier-check.log: docs review tool expanded coverage.json arrays. Formatted the review file without changing review content.
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
pnpm exec prettier --write docs/maintenance/coverage.json; pnpm exec prettier --check changed canonical/manual/review files; documentation source-review validation
```

**Notes**

coverage-format.log and prettier-final.log pass; all documentation topic source fingerprints validate. Generated CLI reference remains byte-exact to the registered parser;614 historical evidence records preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Focused changed Rust packages and direct CLI/HTTP consumers: 12 cold executable CLI scenarios, 33 existing native Stacks scenarios, one standalone presentation scenario and one actual registered-folder HTTP interoperability scenario. Targeted package checks, documentation, format and governance gates; no full-repository claim.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- Remote fetch/push/PR workflows, interactive terminal UI, manifest-backed stack
  execution, installed CLI replacement, publication, sample graph repair,
  unrelated test suites and screenshot/baseline changes.
- No web/catalog source edits; UI tests/types are out of scope. Existing browser
  styling and controllers remain unchanged. Cold evidence is the real executable
  process suite.
- Strict Clippy -D warnings remains blocked by 21 pre-existing JJ diagnostics in
  byte-identical source files. Advisory Clippy completes with 25 inherited
  diagnostics across JJ/service/HTTP; no diagnostic remains in new or changed
  CLI/reader code.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use the explicitly built target/debug/jj-store stack executable locally. Native jj log/op log provide general history; remote/PR workflows remain separately scoped.
<!-- /evidence-field -->

