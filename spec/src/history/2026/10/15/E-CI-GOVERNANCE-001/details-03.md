---
schema: 1
id: E-CI-GOVERNANCE-001
---

# E-CI-GOVERNANCE-001 details 3

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
jj diff --check
```

**Notes**

Optional whitespace inspection did not execute: installed JJ has no --check option. Used the read-only Git whitespace checker instead; no source or test failure.
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
git diff --check
```

**Notes**

Read-only whitespace inspection passed.
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

Local macOS ARM64; Node 22.23.3 for repaired checks (initial reproduction Node 24.10.0); pnpm 10.32.1; JJ 0.45.1; Git 2.55.0; actionlint 1.7.12. Hosted Ubuntu qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Evidence schema 1 and Git/JJ selected revision readers; repository object formats, native identities and release metadata unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the verified governance-only slice and inspect hosted governance then fast before any merge. Full hosted CI and M30 release qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-GOV-002/003/004/011 still require fresh same-change coverage and
  preserve committed evidence and every merge parent. Git now matches native JJ
  by default; explicit spec-first ranges retain matching path/evidence
  baselines. JST-ARCH-005 stage dependencies, automation exceptions and
  protected merge gates are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This local governance repair does not establish
  complete hosted CI, native, container, registry, Pages publication or release
  qualification.

```
<!-- /evidence-field -->
