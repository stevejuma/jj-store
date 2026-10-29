---
schema: 1
id: E-MIRA-012-PIN-001
---

# E-MIRA-012-PIN-001

<!-- evidence-field:id markdown -->
## Id

E-MIRA-012-PIN-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-005
- JST-ARCH-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- spec/src/current/contracts/architecture/06-browser-presentation-ownership.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/29/E-MIRA-012-PIN-001/index.md
- spec/src/history/2026/10/29/E-MIRA-012-PIN-001/SUMMARY.nav.md
- spec/src/history/2026/10/29/index.md
- spec/src/history/2026/10/29/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
npm view @lapismd/mira@0.1.2 version dist.integrity --json
pnpm install --lockfile-only --ignore-pnpmfile
```

**Notes**

npm reports @lapismd/mira 0.1.2 with integrity
sha512-7QQsCm8q1T6XnbszhK+g9u5VMlLLu60ECT3iru8oTAncT8z2orLd5PLxUcZnFfCKVp/ivSjD691YkLhtGKYagw==.
The lock specifier is ~0.1.2 and the resolved package is registry 0.1.2 with
that integrity. Editor and plugin packages were not changed. pnpm 10.32.1.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Progress passed with 186 tasks and 980 evidence records. Spec-first passed
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

pnpm 10.32.1. @lapismd/mira moved from ~0.1.1 to ~0.1.2. The lockfile resolves
0.1.2 from the npm registry.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. The registry lockfile now
selects published Mira 0.1.2 for the existing composer contract.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Consume published Mira 0.1.2 in the jj-store manifest and lock. Design Core
stays at ^0.7.0. Editor and plugin packages stay on their current ranges.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push this onto the open pin pull request. Full release qualification stays
with M30.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-API-005 browser UI and JST-ARCH-005 packaging consume published
  Mira ~0.1.2. The browser presentation ownership chapter and lockfile now
  require that range for the composer contract. Design Core ^0.7.0 is unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. Consuming published Mira 0.1.2 does not complete
  Cargo, native bundle, Pages, or GHCR release qualification.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages
