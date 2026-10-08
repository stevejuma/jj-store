---
schema: 1
id: E-CHAT-LIVE-002
---

# E-CHAT-LIVE-002 details 3

<!-- evidence-field:remaining list -->
## Remaining

- Managed dispatch, durable validated answer links and shared live renderers are
  subsequent slices. No cold browser or live adapter acceptance in this model
  slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust workspace locked dependencies; Node 26.9.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional fields omit absent source selectors; existing whole-message encodings remain unchanged. New Thread Submit/CreateCommentDiscussion commands and CommentBatch link variant.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared collaboration model plus its native/local publication providers and generated direct web contract consumer; aggregate Rust, UI stories and visual baselines out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Publish frozen comment context before managed dispatch and validate exact answer links from committed captures.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-COLLAB-034..036 and JST-API-001 gain exact selectors and atomic
  Submit/CommentBatch contracts.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 stays In progress. Live integration and isolated cold acceptance
  remain open.

```
<!-- /evidence-field -->

