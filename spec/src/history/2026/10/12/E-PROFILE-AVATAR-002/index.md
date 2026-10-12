---
schema: 1
id: E-PROFILE-AVATAR-002
---

# E-PROFILE-AVATAR-002

<!-- evidence-field:id markdown -->
## Id

E-PROFILE-AVATAR-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M26
- M26.1
- M26.2
- M26.4
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-SEC-008
- JST-API-010
- JST-UI-021
- JST-UI-016
- JST-OBJ-006
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-application/ComputeResources.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/components/store-application/SettingsNavigation.stories.svelte
- catalog/components/store-application/StoreProfileSettings.stories.svelte
- catalog/fixtures/ProfileSettingsFixture.svelte
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/directory.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/directory/images.rs
- crates/jj-store-server/src/directory/mod.rs
- crates/jj-store-server/src/directory/mutation.rs
- crates/jj-store-server/src/directory/tests.rs
- crates/jj-store-server/tests/directory_api.rs
- crates/jj-store-service/src/directory.rs
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- schemas/openapi.json
- scripts/dev-ui-config.mjs
- scripts/dev-ui.test.mjs
- spec/src/current/contracts/api/05-bootstrap-and-browser-admission.md
- spec/src/current/contracts/architecture/05-object-foundation.md
- spec/src/current/contracts/cli/02-cli-and-server-commands-continued-2.md
- spec/src/current/contracts/compatibility/37-intentional-thread-formats.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/mutations/06-durable-command-service.md
- spec/src/current/contracts/security/11-installation-profiles-and-emoji.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M26/index.md
- spec/src/history/2026/10/12/E-PROFILE-AVATAR-002/SUMMARY.nav.md
- spec/src/history/2026/10/12/E-PROFILE-AVATAR-002/details-02.md
- spec/src/history/2026/10/12/E-PROFILE-AVATAR-002/details-03.md
- spec/src/history/2026/10/12/E-PROFILE-AVATAR-002/details-04.md
- spec/src/history/2026/10/12/E-PROFILE-AVATAR-002/index.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
- web/src/lib/api/directory-client.ts
- web/src/lib/api/generated.ts
- web/src/lib/controller/directory-actions.svelte.ts
- web/src/lib/controller/directory-model.ts
- web/src/lib/controller/directory.svelte.ts
- web/src/lib/ui/community-avatar-picker/CommunityAvatarPicker.svelte
- web/src/lib/ui/store-settings/StoreComputeSettings.svelte
- web/src/lib/ui/store-settings/StoreCustomEmojiSettings.svelte
- web/src/lib/ui/store-settings/StoreDirectoryField.svelte
- web/src/lib/ui/store-settings/StoreNumericSettingRow.svelte
- web/src/lib/ui/store-settings/StoreProfileSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.css
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-settings/emoji-settings.css
- web/src/lib/ui/store-settings/profile-settings.css
- web/tests/real/profiles-emoji.spec.ts
- web/tests/vite/directory.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/directory/images.rs#large_jpeg_avatars_resize_and_keep_aspect_ratio
- crates/jj-store-server/src/directory/images.rs#resized_jpeg_avatars_apply_exif_orientation
- crates/jj-store-server/src/directory/images.rs#oversized_emoji_rasters_normalize_and_keep_transparency
- crates/jj-store-server/src/directory/images.rs#source_file_over_two_mib_is_normalized_and_small_sources_keep_bytes
- crates/jj-store-server/src/directory/images.rs#avatar_decode_and_source_limits_reject_before_publication
- crates/jj-store-server/src/directory/tests.rs#avatar_upload_receipts_survive_lost_acknowledgement_and_keep_source_identity
- crates/jj-store-server/src/directory/tests.rs#oversized_emoji_uploads_normalize_before_explicit_library_save_and_restart
- crates/jj-store-server/tests/directory_api.rs#binary_avatar_upload_normalizes_large_sources_replays_and_admits_identity_first
- crates/jj-store-http/src/contract/tests.rs#avatar_upload_contract_is_binary_personal_and_receipt_bound
- web/tests/vite/directory.test.ts#emoji selection stays local through refresh
  and cancel without server writes
- web/tests/real/profiles-emoji.spec.ts#cold local and enrolled profiles and
  emoji survive real object store restart
- web/tests/numeric-setting.test.ts#numeric validation rejects empty,
  fractional, unsafe and out-of-range values without rounding
- scripts/dev-ui.test.mjs#HMR proxy preserves the public origin and streams
  every versioned API route
<!-- /evidence-field -->

