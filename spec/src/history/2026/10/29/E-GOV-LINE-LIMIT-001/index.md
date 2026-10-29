---
schema: 1
id: E-GOV-LINE-LIMIT-001
---

# E-GOV-LINE-LIMIT-001

<!-- evidence-field:id markdown -->
## Id

E-GOV-LINE-LIMIT-001
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

- JST-ARCH-005
- JST-GOV-003
- JST-GOV-010
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
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
pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

The governance page is 400 physical lines. File-limit validation accepts it, and the Dependabot rollup and git-only publication boundary remain.
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
pnpm check:architecture && pnpm check:ui-ownership && pnpm check:progress && pnpm spec:first --base 753e9a8006ebf3100350d3fd6453287961fa4496 --head @ && CI_EVENT=pull_request CI_PR_AUTHOR=stevejuma GITHUB_HEAD_REF=chore/cli-git-and-cargo-rollup JJ_STORE_CHECK_BASE=753e9a8006ebf3100350d3fd6453287961fa4496 JJ_STORE_CHECK_REVISION=@ node scripts/changeset-check.mjs
```

**Notes**

The specification and release-policy checks from the failing pull-request job pass on this change. Actionlint is unchanged because no workflow file changed.
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

No package version change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change. The governance page stays within the 400-line bound.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Restore the 400-line bound on spec governance without dropping the CLI publication boundary or the combined Dependabot Cargo update.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Re-run the specification and release-policy job on the updated pull request.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005, JST-GOV-003 and JST-GOV-010 keep the same behavior.
  The governance page only returns to the 400-line bound.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. The line-limit repair does not complete
  release qualification.

```
<!-- /evidence-field -->


