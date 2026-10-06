---
schema: 1
id: E-M14-017
---

# E-M14-017

<!-- evidence-field:id markdown -->
## Id

E-M14-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change normzvzq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native repository creation without a physical workspace
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Svelte 5.57.1; Vitest 4.1.11; Playwright 1.63.0. Existing local Design Core validation setup; registry-only notification qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native formats unchanged; private installation creation journal version 1; additive LocalTrusted Admin HTTP create/status contracts; browser persists exact credential-free public request only.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-server/src/local_repositories/native_creation.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/tests/native_creation.rs
- crates/jj-store-service/src/local_browser.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/native-repository-manager.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-settings/StoreNativeRepositoryCreate.svelte
- web/src/lib/ui/store-work/StoreProjectsAddMenu.svelte
- web/tests/browser/native-repository-creation.spec.ts
- web/tests/native-repository-manager.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/native_creation.rs#native_creation_recovers_unconfirmed_publication_with_retained_identity
- crates/jj-store-server/tests/native_creation.rs#native_creation_replays_after_restart_without_a_physical_workspace
- crates/jj-store-server/tests/native_creation.rs#native_creation_http_is_local_admin_only_and_captures_contracts
- web/tests/browser/native-repository-creation.spec.ts#cold native creation
  recovers exact intent across entrypoints and reload
- web/tests/browser/native-repository-creation.spec.ts#cold native creation
  keeps name rejection inline and permits a corrected request
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-017 details 2](details-02.md)
- [E-M14-017 details 3](details-03.md)
