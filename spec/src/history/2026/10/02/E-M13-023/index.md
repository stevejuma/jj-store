---
schema: 1
id: E-M13-023
---

# E-M13-023

<!-- evidence-field:id markdown -->
## Id

E-M13-023
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yyknxszk
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Isolated local-completion workspace on vtqozuvm/bb24714d. Physical Save absent; local code mutations false. Exact new filesystem regression cannot compile before implementation. Default workspace and live demo retained.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ0.45.1; Git SHA-1; Rust1.98.1; rustix1.1.5; Svelte5.57.1; Bits UI2.18.1. Existing documented temporary Design Core source validation from E-M13-022 retained; npm manifest/pnpm lockfile unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive WorkingFile/WorkingFileSelection physical device DTOs and two LocalTrusted APIs, independent working_copy_edit capability. Repository/generation/path/workspace/fingerprint bound to admitted physical files. Save changes no JJ heads/bookmarks or companion refs. No native or portable stored identity/receipt changes. Concurrent browser saves serialize; arbitrary external writers cannot participate in an OS filesystem CAS. Atomic rename uses a private same-filesystem temporary under .jj, inaccessible through the API and excluded from JJ snapshots. Enclosing change yyknxszk is revision reference.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/working_copy.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/local_browser/working_copy.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/working_copy.rs
- crates/jj-store-service/tests/local_working_copy.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/working-copy-types.ts
- web/src/lib/projects/repository-file-stats.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/store-working-copy/StoreWorkingFileEditor.svelte
- web/src/lib/ui/store-working-copy/working-copy-editor.svelte.ts
- web/tests/real/local-browser.spec.ts
- web/tests/vite/local-browser.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_working_copy.rs#physical_save_is_atomic_stale_checked_and_preserves_recorded_state
- crates/jj-store-service/tests/local_working_copy.rs#working_files_reject_unsafe_paths_symlinks_special_and_unbounded_content
- crates/jj-store-service/tests/local_working_copy.rs#workspace_replacement_and_file_replacement_are_stale
- crates/jj-store-service/tests/local_working_copy.rs#concurrent_browser_saves_accept_only_one_original_selection
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- web/tests/vite/local-browser.test.ts#physical_save_preserves_selection_and_never_retries_or_advances_code
- web/tests/real/local-browser.spec.ts#physical working copy saves retain drafts
  and leave recorded state unchanged
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-023 details 2](details-02.md)
- [E-M13-023 details 3](details-03.md)
- [E-M13-023 details 4](details-04.md)
