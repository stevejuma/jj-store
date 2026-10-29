---
schema: 1
id: E-PACKAGE-GIT-PINS-001
---

# E-PACKAGE-GIT-PINS-001

<!-- evidence-field:id markdown -->
## Id

E-PACKAGE-GIT-PINS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/ci-package-check.test.mjs
- scripts/package-check.mjs
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/29/E-PACKAGE-GIT-PINS-001/index.md
- spec/src/history/2026/10/29/E-PACKAGE-GIT-PINS-001/SUMMARY.nav.md
- spec/src/history/2026/10/29/SUMMARY.nav.md
- spec/src/history/2026/10/29/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci-package-check.test.mjs#package check admits only the published Lapis git tags
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
node --test scripts/ci-package-check.test.mjs
```

**Notes**

The new regression passed: the exact lapis-ai-runtime v0.2.0 tag is admitted,
and a foreign git tag and a branch pin both fail. 1 passed, 0 failed.
Local Python was 3.11.16 because the system python3 is 3.9 and has no
tomllib. Hosted CI uses Python 3.12.
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
node --test scripts/release.test.mjs scripts/ci*.test.mjs scripts/automation-governance.test.mjs && node scripts/package-check.mjs
```

**Notes**

This is the release:check lane. 60 passed, 0 failed. The workspace package
check then reported all 28 Cargo packages have admitted dependencies.
Coordinated metadata stayed 0.1.0.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Progress passed with 186 tasks and 981 evidence records. Spec-first passed
with 2 protected files and 1 canonical chapter. The wider product suite was
not run.
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

lapis-ai git tag v0.2.0 crates 0.2.0; lapis-agent-redaction git tag
lapis-agent-redaction@0.1.0 crate 0.1.0. Local package check used Python
3.11.16; hosted setup uses Python 3.12.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. The package check now
accepts only the four published git tags and still rejects every other git
dependency.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The required tooling gate failed because package-check rejected the git tag
pins already selected by the workspace manifests. The check now admits those
exact tags and the regression rejects a foreign repository and a branch pin.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the fix onto the open pin pull request and wait for the required check.
Full release qualification stays with M30.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 package check and JST-GOV-002 shared AI client text now
  admit only the published lapis-ai v0.2.0 and lapis-agent-redaction 0.1.0
  git tags. Every other git dependency still fails. This does not publish
  jj-store or switch the pins to crates.io.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. Admitting the published tags in the package
  check does not complete Cargo, native bundle, Pages, or GHCR release
  qualification.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages
