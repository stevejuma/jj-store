---
schema: 1
id: E-COLUMN-HOVER-001
---

# E-COLUMN-HOVER-001 details 2

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
requirements:
  - JST-UI-004
  - JST-UI-006
reason: Catalog waits now distinguish loading headers from mounted production
  views. Typed independent fixture navigation and layout v1 persistence are
  unchanged. The generic hover/content lifecycle contract is updated and
  verified upstream under DC-SHA-036.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
tasks:
  - M4.13
reason: M4.13 remains Deferred; prepared-archive/local consumer evidence does
  not satisfy its future Design Core registry-release gate.

```
<!-- /evidence-field -->

<!-- evidence-field:source_evidence markdown -->
## Source evidence

Design Core spec/records/column-canvas-hover-previews.md. Final default archive SHA256 e6b658cedea6e8812fe0a7f350f171c55b6297dc9fdf0862b03d56c82e31766c. Final source follow-up commit ea8fd10d; default source and retired temporary workspace trees are identical. Source workspace was forgotten and moved to Trash after all owned test servers stopped; final logs are preserved in default Design Core .local/column-preview.
<!-- /evidence-field -->

<!-- evidence-field:live_acceptance markdown -->
## Live acceptance

Before installation, 750 ms native dwell on the collapsed revisions rail produced no preview. The final archive opens a populated 480 px Op Log preview; controls remain interactive and Escape/pointer departure dismiss while the durable column stays collapsed. Rechecked after an independent main-demo restart with the same installed guard/defaults. Screenshot: ignored .local/column-preview/oplog-hover-preview.jpg.
<!-- /evidence-field -->

