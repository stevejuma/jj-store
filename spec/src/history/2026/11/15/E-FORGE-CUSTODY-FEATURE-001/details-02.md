---
schema: 1
id: E-FORGE-CUSTODY-FEATURE-001
---

# E-FORGE-CUSTODY-FEATURE-001 details 2

<!-- evidence-field:scope markdown -->
## Scope

Hosts injecting encrypted custody can exclude platform keyring linkage. jj-forge and jj-store-service retain native-secrets defaults; workspace engine dependencies do not force that optional backend. Native store methods refuse Unsupported without it. No plaintext or platform fallback.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify and merge the source feature, then update Server pins and disable those two default features. Re-run exact hosted compilation and image acceptance.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Document explicit injected-custody compilation under JST-FORGE-007 while
  retaining default desktop OS custody and fail-closed behavior.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress. This does not promote live provider, packaged
  app, other platform or visual gates. M34 host adoption and cold remotes
  qualification remain open.

```
<!-- /evidence-field -->

