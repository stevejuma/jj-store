---
schema: 1
id: E-M3-006
---

# E-M3-006

<!-- evidence-field:id markdown -->
## Id

E-M3-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
- M3.2
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

executable-native-cli
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/build.rs
- crates/jj-store-cli/src/binding.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/credentials.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/main.rs
- crates/jj-store-cli/src/presentation.rs
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-cli/tests/fixtures/terminal.py
- docs/cli/attach.md
- docs/cli/capabilities.md
- docs/cli/commands.md
- docs/cli/credentials.md
- docs/cli/doctor.md
- docs/cli/pagination.md
- docs/cli/status.md
- docs/cli/store.md
- docs/cli/version.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#executable_native_create_edit_describe_log_restart_and_evolution
- crates/jj-store-cli/tests/executable_native.rs#executable_workspaces_keep_local_edits_and_reject_credential_or_binding_mismatch
- crates/jj-store-cli/tests/executable_native.rs#custom_help_completion_build_info_and_ordinary_git_commands
- crates/jj-store-cli/tests/executable_native.rs#custom_color_json_and_metadata_validation_do_not_contact_or_leak_credentials
- crates/jj-store-cli/tests/executable_native.rs#os_credential_facility_roundtrip
- crates/jj-store-cli/tests/executable_native.rs#custom_terminal_paging_and_color_policy
- crates/jj-store-cli/tests/executable_native.rs#attach_rejects_read_only_access_before_creating_local_state
- crates/jj-store-cli/tests/executable_native.rs#executable_login_logout_uses_os_credentials_across_processes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M3-006 details 2](details-02.md)
