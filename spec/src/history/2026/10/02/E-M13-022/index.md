---
schema: 1
id: E-M13-022
---

# E-M13-022

<!-- evidence-field:id markdown -->
## Id

E-M13-022
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.3
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository review browser and CLI delivery stage
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

vtqozuvm
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Default and new local-completion workspace clean at default committed head 77f91f29. Local provider stores repository reviews but browser transport/screens and CLI dispatch reject the profile. New local_review_browser_pages_and_detail_pin_committed_metadata regression fails compilation with eight missing-method errors.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Git SHA-1, macOS ARM64. Node 26.9.0, pnpm 10.32.1. Local Design Core source b3eb19e7 used temporarily for validation, matching default workspace; registry 0.2.0 lacks six required existing APIs. Dependency manifest and lock remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native review and local companion request/receipt bytes unchanged. Own LocalReviewSnapshot DTO and four local ReviewStore endpoints. Granular repository/actions/code/agent review capabilities. Unused local synchronization field removed; native M10 controls retained. Enclosing Jujutsu change vtqozuvm is revision reference.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- crates/jj-store-cli/src/review_commands.rs
- crates/jj-store-cli/src/review_commands/local.rs
- crates/jj-store-cli/src/review_presentation.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/reviews.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/local_browser/reviews.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/review_presentation.rs
- crates/jj-store-service/src/collaboration/review_replication.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/local_repository/browser_reviews.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/local_repository/review_types.rs
- crates/jj-store-service/tests/local_review.rs
- docs/cli/review.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/http-local-work-data-source.ts
- web/src/lib/controller/http-repository-directory.ts
- web/src/lib/controller/local-repository-directory.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-review-actions.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-repository-types.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/repository-read-types.ts
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-collaboration/StoreRepositoryReviewDialog.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDiscussion.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/tests/real/local-browser.spec.ts
- web/tests/vite/local-browser.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_review.rs#local_review_browser_pages_and_detail_pin_committed_metadata
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/vite/local-browser.test.ts#local_reviews_pin_companion_history_and_confirm_actions_without_advancing_code
- web/tests/vite/local-browser.test.ts#local_review_reads_reject_foreign_companion_snapshots
- web/tests/real/local-browser.spec.ts#filesystem reviews create and reply with
  CLI parity and pinned historical reads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-022 details 2](details-02.md)
- [E-M13-022 details 3](details-03.md)
