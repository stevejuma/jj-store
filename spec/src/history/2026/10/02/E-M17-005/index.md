---
schema: 1
id: E-M17-005
---

# E-M17-005

<!-- evidence-field:id markdown -->
## Id

E-M17-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

capture-runtime-config-foundation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-capture/Cargo.toml
- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture_files.rs
- crates/jj-store-capture/src/agent_capture_intent.rs
- crates/jj-store-capture/src/agent_publication.rs
- crates/jj-store-capture/src/agent_sanitization.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/local_agent_publication.rs
- crates/jj-store-capture/src/path_policy.rs
- crates/jj-store-capture/src/profile.rs
- crates/jj-store-capture/src/state_lease.rs
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_files.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/src/agent_sanitization.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/local_agent_publication.rs
- crates/jj-store-cli/src/state_lease.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/capture_config.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/config/format-2-example.toml
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/policy.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/tests/configuration.rs
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
- spec-validator.config.mjs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/security.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-cli/src/agent_capture_intent.rs
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-cli/src/state_lease.rs#capture_lease_releases_with_an_inherited_file_description
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/src/state_lease.rs#capture_lease_releases_with_an_inherited_file_description
- scripts/governance.test.mjs#capture_runtime_consumes_owned_interfaces_without_reverse_dependencies
- crates/jj-store-cli/tests/agent_capture_spool.rs#sanitized_capture_and_prepared_publication_survive_restart
- crates/jj-store-cli/tests/local_agent_recovery.rs#bound_local_spool_keeps_original_observation_and_request_across_unknown_restart
- crates/jj-store-server/tests/configuration.rs#automatic_capture_configuration_is_explicit_local_writable_and_bounded
- crates/jj-store-server/tests/configuration.rs#capture_api_config_rejects_credentials_remote_hosts_and_profile_confusion
- crates/jj-store-server/tests/configuration.rs#enabled_capture_startup_fails_before_state_creation_until_workers_are_connected
- crates/jj-store-cli/tests/server_executable.rs#cold_capture_config_check_and_startup_gate_preserve_private_state
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-005 details 2](details-02.md)
- [E-M17-005 details 3](details-03.md)
- [E-M17-005 details 4](details-04.md)
