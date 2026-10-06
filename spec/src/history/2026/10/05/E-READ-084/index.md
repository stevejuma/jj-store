---
schema: 1
id: E-READ-084
---

# E-READ-084

<!-- evidence-field:id markdown -->
## Id

E-READ-084
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-API-004
- JST-API-005
- JST-UI-003
- JST-JJ-003
- JST-LOCAL-004
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Selected text-diff admission and accurate unavailable presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mrqotlum
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Linux altera_edac.c is valid UTF-8 without NUL bytes; 62341/62397-byte versions have 2223/2226 guarded lines. A whole-file product of 4948398 exceeded 250000 and returned HTTP200 unavailable/complexity. The UI discarded this reason and displayed Binary diff unavailable. The CLI completed the same comparison in 77ms, with 35 additions and 32 deletions.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One shared private admission helper for native/local selected-file text readers: retain JJ default native hunks for small comparisons; exact leading/trailing boundaries and ordered unique common lines partition longer inputs before cumulative bounded native comparisons. Keep text, output, cancellation, authority and total-work caps. Retain typed unavailable reasons in the selected patch; binary alone gets a binary label, pending patch reads show skeletons and unavailable content remains contextual without duplicate request toasts. Adapt the existing review-anchor consumer; no review feature redesign.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing V1 diff DTOs and reason enum unchanged; no generated contracts, repository/index/native object/WAL/manifest/companion persistence or authority changes. The original content-diff slice okkvprsm/aaa805a2 introduced the app-owned 256KiB and 250000 caps on 2026-09-27. Pinned jj-lib/JJ core 0.45.1 has no corresponding configurable caps; native diff defaults, including its internal histogram behavior, remain untouched.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ/jj-lib/JJ core 0.45.1; rustc 1.98.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
- catalog/fixtures/DiffFixture.svelte
- crates/jj-store-cli/tests/local_browser/scoped_differences.rs
- crates/jj-store-cli/tests/native_lifecycle/diff.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/line_diff.rs
- crates/jj-store-jj/src/line_diff/tests.rs
- crates/jj-store-jj/src/local_browser/diff.rs
- crates/jj-store-jj/src/query/content/diff.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/review-anchor-patch.ts
- web/src/lib/projects/presentation.ts
- web/src/lib/projects/repository-diff.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/tests/production-store-loaders.test.ts
- web/tests/repository-diff.test.ts
- web/tests/vite/production-patch-reader.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_long_file_sparse_edits_remain_text
- crates/jj-store-cli/tests/local_browser/scoped_differences.rs#local_diff_long_file_sparse_edits_remain_text
- crates/jj-store-jj/src/line_diff/tests.rs#small_diffs_retain_jj_default_hunks_and_exact_bytes
- crates/jj-store-jj/src/line_diff/tests.rs#sparse_long_file_edits_with_insertions_and_final_newline_change
- crates/jj-store-jj/src/line_diff/tests.rs#repeated_common_lines_are_trimmed_without_unique_anchors
- crates/jj-store-jj/src/line_diff/tests.rs#expensive_repeated_comparisons_refuse_complete_content
- crates/jj-store-jj/src/line_diff/tests.rs#request_budget_cancellation_propagates_without_a_partial_diff
- web/tests/production-store-loaders.test.ts#selected patch preserves the
  backend unavailable reason
- web/tests/repository-diff.test.ts#only confirmed binary content uses a binary
  label
- web/tests/vite/production-patch-reader.test.ts#selected patch stays pending
  until its typed unavailable result arrives
- web/tests/vite/production-patch-reader.test.ts#obsolete patch responses cannot
  replace a new selection
- web/tests/vite/production-patch-reader.test.ts#failed patch reads clear
  pending state and report through the request service
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-084 details 2](details-02.md)
- [E-READ-084 details 3](details-03.md)
- [E-READ-084 details 4](details-04.md)
