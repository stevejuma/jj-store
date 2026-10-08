---
schema: 1
id: E-CHAT-LIVE-004
---

# E-CHAT-LIVE-004 details 2

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; locked Rust workspace; Node 26.9.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional CommentBatch.reply_to, typed ContextBatch.submission, recorded feedback and historical reply fields; absent optional fields preserve legacy encoding.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Conversation model, execution context, native/local read projections and direct generated-contract consumers. No repository aggregate or unrelated Rust lanes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete shared live/recorded/thread renderers, default-setting parity and cold browser qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-034..036 now specify retained immutable submitted revisions,
  exact reply parents and recorded feedback.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This backend slice does not close browser or
  real-adapter gates.

```
<!-- /evidence-field -->

