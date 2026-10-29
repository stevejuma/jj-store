---
schema: 1
id: E-PACKAGE-LICENSE-002
---

# E-PACKAGE-LICENSE-002

<!-- evidence-field:id markdown -->
## Id

E-PACKAGE-LICENSE-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-28
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
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/28/E-PACKAGE-LICENSE-002/index.md
- spec/src/history/2026/10/28/E-PACKAGE-LICENSE-002/SUMMARY.nav.md
- spec/src/history/2026/10/28/index.md
- spec/src/history/2026/10/28/SUMMARY.nav.md
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
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first --head HEAD --base HEAD^
```

**Notes**

After E-PACKAGE-LICENSE-001 added LICENSE.md/NOTICE.md under `jj-history-service` and `jj-vault`, Spec-first required the mapped chapters `26-reusable-agent-workflows.md` and `32-automatic-vault-backup.md` because an unchanged-contract confirmation could not cover those packaging paths alone.
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
pnpm docs:review; pnpm exec spec-validator first --head @ --base release/jj-history-trusted-publish@origin; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Recorded that publishable history-service and vault crates must ship Apache-2.0 LICENSE.md/NOTICE.md without changing runtime admission, backup or publication authority. Spec-first against the license tip parent and docs:review both pass.
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

No package or protocol version changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Canonical packaging prose only; no DTO, lockfile or public API change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Close the Spec-first gap left by E-PACKAGE-LICENSE-001: map the new crate LICENSE/NOTICE files to their owning architecture and local-jj chapters so the tip can pass mapped Spec-first without claiming an unqualified unchanged confirmation for those paths.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Confirm hosted PR Spec-first, tooling and Pages are green on the release tip.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updates JST-ARCH-005 packaging expectations in the reusable-agent-workflows
  and automatic-vault-backup chapters so publishable jj-history-service and
  jj-vault crates explicitly require LICENSE.md/NOTICE.md. Runtime admission,
  backup workers and publication authority remain as previously specified.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Documenting required license files for already
  landable crates does not promote a publication or visual gate.

```
<!-- /evidence-field -->
