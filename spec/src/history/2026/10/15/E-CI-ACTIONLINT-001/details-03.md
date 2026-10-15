---
schema: 1
id: E-CI-ACTIONLINT-001
---

# E-CI-ACTIONLINT-001 details 3

<!-- evidence-field:next markdown -->
## Next

Inspect hosted governance and fast on the updated signed head before merge; full hosted/release qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-005 keeps the same pinned workflow linter, checksum
  enforcement, existing Ubuntu amd64 runner, independent stages and required
  passing checks. This repairs an unsupported installation mechanism without
  relaxing gates. JST-GOV-003/004/011 evidence remains append-only with explicit
  milestone review.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Installer qualification does not establish full
  CI, native/archive/container/registry publication or other release gates.

```
<!-- /evidence-field -->
