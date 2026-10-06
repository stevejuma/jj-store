---
schema: 1
id: E-READ-092
---

# E-READ-092

<!-- evidence-field:id markdown -->
## Id

E-READ-092
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-005
- JST-UI-019
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Capability-gated progressive Blame UI with fixture-backed lifecycle evidence
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pvlzsxrt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main and retained bounded-reads workspace initially shared the clean E-READ-091 parent. Blame had no route, controls or presenter. The user separately authorized the next UI slice after the reader comparison blocker. Initial exact regression failed because blame-model did not exist.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository-owned file panel controls, file_view hash restoration, progressive grouped source and bounded virtual chunks, real-origin commit routes, query-observer lifecycle and retained progress. Production has no Blame provider or HTTP route, so Blame is disabled; restored URLs explain unavailability and keep source. No upstream, Rust, backend, server restart or generated wire contract changes.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; repository pnpm 10.32.1, Svelte 5, Vitest 4.1.11, Storybook 10.5.7, Playwright Chromium; released Design Core public tokenizeAsync and TokenLine APIs; unchanged jj-lib 0.45.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

file_view=source|preview|blame on a recorded Files selection. Optional internal UI provider, not an HTTP contract. UI progress uses one-based end-exclusive ranges with deduplicated real commit summaries. Session cache scope is host-owned and includes source/access generation and algorithm; only latest cumulative progress survives unobserved batches for the existing ten-minute cache lifetime.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreBlame.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/fixtures/StoreBlameFixture.svelte
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/jj-integration.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/blame-query.ts
- web/src/lib/controller/blame-types.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/production-file-reader.svelte.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-fields.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/controller/store-stacks-navigation.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte
- web/src/lib/ui/community-projects/StoreFileDisplayControls.svelte
- web/src/lib/ui/store-blame/StoreBlameGroup.svelte
- web/src/lib/ui/store-blame/StoreBlameView.svelte
- web/src/lib/ui/store-blame/StoreFileBlame.svelte
- web/src/lib/ui/store-blame/blame-model.ts
- web/tests/store-blame.test.ts
- web/tests/vite/blame-query.test.ts
- web/tests/vite/store-route-history.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/store-blame.test.ts#file display routes restore and reject invalid
  contexts
- web/tests/store-route-codec.test.ts#native operation route hash round-trips
  every stable selection
- web/tests/vite/blame-query.test.ts#observers coalesce, release independently,
  resume progress and reuse completion
- web/tests/vite/store-route-history.test.ts#route history restores, pushes and
  replaces native browser hashes
- web/tests/vite/store-stacks-navigation.test.ts#focused tabs retain independent
  selections and return to the exact repository route
- web/tests/vite/production-file-reader.test.ts#files load the selected commit
  metadata independently of graph history
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-092 details 2](details-02.md)
