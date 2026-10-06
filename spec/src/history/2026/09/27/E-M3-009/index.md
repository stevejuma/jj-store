---
schema: 1
id: E-M3-009
---

# E-M3-009

<!-- evidence-field:id markdown -->
## Id

E-M3-009
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

standalone-native-server
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/credentials.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/presentation.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/server_main.rs
- crates/jj-store-cli/tests/fixtures/terminal.py
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/policy.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/tests/configuration.rs
- docs/cli/serve-local.md
- docs/cli/server-commands.md
- docs/cli/server-config.md
- docs/cli/server-doctor.md
- docs/cli/server-operator.md
- docs/cli/server-run.md
- docs/cli/server.md
- docs/evidence/m3-server-rustfs.json
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
- scripts/rustfs-contract.py
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/configuration.rs#configuration_is_bounded_strict_and_resolves_paths_without_creating_state
- crates/jj-store-server/tests/configuration.rs#insecure_shared_endpoints_credentials_and_checkout_control_paths_are_rejected
- crates/jj-store-server/tests/configuration.rs#operator_policy_persists_only_verifiers_and_reload_observes_revocation
- crates/jj-store-server/tests/configuration.rs#operator_updates_serialize_and_state_locks_release_with_the_owner
- crates/jj-store-server/tests/configuration.rs#configuration_and_policy_symlinks_cannot_bypass_private_path_rules
- crates/jj-store-server/tests/configuration.rs#programmatic_configuration_cannot_bypass_private_path_validation
- crates/jj-store-cli/tests/server_executable.rs#standalone_help_configuration_completions_and_build_info_are_consistent
- crates/jj-store-cli/tests/server_executable.rs#operator_commands_provision_list_and_revoke_without_exposing_credentials
- crates/jj-store-cli/tests/server_executable.rs#cold_server_and_serve_local_hold_state_lock_and_stop_on_termination
- crates/jj-store-cli/tests/server_executable.rs#server_rejects_missing_credentials_invalid_tls_and_shared_plaintext_before_listening
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
- crates/jj-store-cli/tests/server_executable.rs#standalone_terminal_paging_color_and_json_contract
- crates/jj-store-cli/src/credentials.rs#competing_operator_provision_cannot_replace_the_first_credential
- scripts/governance.test.mjs#server_composition_cannot_depend_on_cli_or_native_upstream
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M3-009 details 2](details-02.md)
- [E-M3-009 details 3](details-03.md)
