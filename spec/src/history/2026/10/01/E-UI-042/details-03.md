---
schema: 1
id: E-UI-042
---

# E-UI-042 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds. Log .scratch/editor-focus-spec-build.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/collaboration-story-assertions.ts spec/src/api.md spec/src/milestones.md spec/src/ui.md spec/src/verification.md web/src/lib/ui/community-projects/StoreFileEditor.svelte web/src/lib/ui/editor-focus.css web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte web/src/lib/ui/store-mutations/StoreMutationFields.svelte web/tests/browser/editor-focus.spec.ts
```

**Notes**

All 14 changed production, catalog, regression and canonical files pass formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Changed stories remain visual-pending; visual baseline approval is separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated UI suites; Rust/native contract generation, real-RustFS and shared
  release qualification are out of scope under the targeted UI-only policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing local dependency setup unchanged; no sibling or package manifest edits; registry-only release qualification is not claimed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Static application host stylesheet scopes native editable-content focus outline and shadow removal to file-editor contents and Community composer Mira content. Toolbar/button focus styles and editing/submission semantics remain intact. Actual production asset CSP is applied only to isolated preview HTML in the focused browser regression; no shared fixture/configuration, CSP policy, DTO, native identity, object, manifest, WAL, receipt or credential format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused served-policy editor/composer regression and direct-consumer story coverage; visual approval remains separate.
<!-- /evidence-field -->

