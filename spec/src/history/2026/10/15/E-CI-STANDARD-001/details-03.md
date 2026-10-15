---
schema: 1
id: E-CI-STANDARD-001
---

# E-CI-STANDARD-001 details 3

<!-- evidence-field:remaining list -->
## Remaining

- Hosted standard required check on this slice; manual dispatch after the
  workflow reaches default branch. Existing heavy failure remains unresolved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 22.23.3 and 24.10.0; pnpm 10.32.1; JJ 0.45.1; actionlint 1.7.12. Existing Rust 1.99 and Nextest 0.9.148 pins unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GitHub workflow dispatch/reusable inputs replace full with all_checks for standard checks; heavy inputs expose only ref. No public Rust API, repository format or persistent data change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Verify the hosted standard check, then implement affected-crate selection and caching. M30 remains In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 explicitly separates standard merge checks from full manual
  and release qualification. All selected gates fail closed; release still
  requires every heavy lane.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Existing heavy, native, storage, browser,
  package and publication qualifications are not promoted.

```
<!-- /evidence-field -->

