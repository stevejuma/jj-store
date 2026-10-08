---
schema: 1
id: E-CHAT-BODY-013
---

# E-CHAT-BODY-013 details 2

<!-- evidence-field:remaining list -->
## Remaining

- Cold development logs retain 7 ResizeObserver-loop notifications. Browser
  assertions pass; runtime diagnostic removal remains unqualified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11. Existing Design Core/Mira installation unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No captured fixture changes, live agent prompt protocol/parser, backend dispatch, persistence or API/ACP changes. Presentation reply contexts retain stable thread IDs and comment ID arrays; stored replyTo records remain available while source views filter them.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One owned UI-only comment/reply surface, catalog simulation and direct tests. No Rust, aggregate repository lanes, desktop inspection or visual-baseline comparisons/updates. Cold acceptance uses a private Storybook and Chromium without replacing the user preview on 6048.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the updated main-thread reference. Keep native IME, live agent integration, desktop and visual approval as independent gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 specify transcript-only answer rendering,
  persistent quoted reply context, same-discussion reply grouping and clickable
  double ticks. JST-API-001 explicitly unchanged because all delivery remains
  catalog simulation.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress; visual baselines unapproved and independent
  live, native IME, adapter, permission/cancellation and desktop gates remain
  open.

```
<!-- /evidence-field -->

