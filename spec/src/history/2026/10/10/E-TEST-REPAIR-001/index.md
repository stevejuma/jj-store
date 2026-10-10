---
schema: 1
id: E-TEST-REPAIR-001
---

# E-TEST-REPAIR-001

<!-- evidence-field:id markdown -->
## Id

E-TEST-REPAIR-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-015
- JST-UI-018
- JST-UI-019
- JST-API-001
- JST-COLLAB-034
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/ManagedAgentIdentity.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/repository-directory-story-assertions.ts
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte
- catalog/fixtures/StackWorkspaceProductionFixture.svelte
- catalog/fixtures/stack-workspace/production-http.ts
- spec/src/current/contracts/ui/13-shared-application.md
- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/test-repairs-review.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-02.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-03.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-04.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/controller/production-store-mutations.ts
- web/src/lib/controller/production-store-search-controller.ts
- web/src/lib/conversations/ordered-activity.ts
- web/src/lib/ui/community-projects/repository-directory.css
- web/tests/conversation-ordering.test.ts
- web/tests/vite/conversation-create.test.ts
- web/tests/vite/local-browser-merge.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-execution.test.ts#message times retain first observation
  across chunks and never fabricate legacy times
- web/tests/conversation-ordering.test.ts#each assistant segment retains its
  first valid time without borrowing neighboring times
- web/tests/conversation-ordering.test.ts#legacy final answers use only a valid
  observation time from their own record
- web/tests/vite/local-browser-merge.test.ts#thread_hub_navigation_uses_the_shared_directory_without_connecting
- web/tests/vite/local-browser-merge.test.ts#explicit_thread_navigation_connects_its_native_home
- web/tests/vite/local-browser-merge.test.ts#unsupported_transport_discards_previous_native_thread_detail
- web/tests/vite/local-browser-merge.test.ts#local_thread_capability_comes_from_the_transport
- web/tests/vite/local-browser-merge.test.ts#local_profile_switch_preserves_unknown_native_thread_intent
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- web/tests/vite/local-browser-merge.test.ts#mixed_thread_navigation_selects_a_native_home
- web/tests/vite/local-browser-merge.test.ts#local_profile_hides_threads_and_discards_previous_native_thread_state
<!-- /evidence-field -->

