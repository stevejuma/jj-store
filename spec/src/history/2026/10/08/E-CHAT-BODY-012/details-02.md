---
schema: 1
id: E-CHAT-BODY-012
---

# E-CHAT-BODY-012 details 2

<!-- evidence-field:scope markdown -->
## Scope

One owned chat fold widget and its direct unit/Storybook/cold browser consumers. No Rust, repository aggregates, unrelated architecture audit or visual-baseline comparison/update. The cold run owns an ephemeral Storybook process and isolated Chromium; it leaves the 6048 user preview running.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the reversible controls in the running reference. Retain independent native IME, live adapter, desktop and visual qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 now specify a retained Show less control and
  reversible local disclosure state. JST-API-001 is explicitly unchanged because
  folds do not mutate captured text, comments, requests or dispatch.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This UI-only acceptance does not close live,
  native IME, desktop or visual gates; baselines remain unapproved.

```
<!-- /evidence-field -->

