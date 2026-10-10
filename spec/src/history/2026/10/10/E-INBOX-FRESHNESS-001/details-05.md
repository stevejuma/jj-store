---
schema: 1
id: E-INBOX-FRESHNESS-001
---

# E-INBOX-FRESHNESS-001 details 5

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Add E-INBOX-FRESHNESS-001 and explicit lifecycle review to M24/M25 while
  both remain In progress. This read-ownership evidence does not close live
  first-load queueing, preparation event coverage, independent
  provider/adapter/platform gates or visual approval.

```
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- web/tests/vite/production-automatic-refresh.test.ts#a missed repository
  publication reloads without an SSE or a manual action
- web/tests/vite/capture-freshness.test.ts#visible capture loads polls coalesces
  and stops on closure
- web/tests/vite/agent-discussions.test.ts#discussion outage fallback runs
  independently and stops when the view closes
<!-- /evidence-field -->

