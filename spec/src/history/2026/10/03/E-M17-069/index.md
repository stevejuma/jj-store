---
schema: 1
id: E-M17-069
---

# E-M17-069

<!-- evidence-field:id markdown -->
## Id

E-M17-069
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

configured-codex-whole-server-startup
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change omzuxolm
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/server_executable/configured_capture/helpers.rs
- crates/jj-store-cli/tests/server_executable/configured_capture.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/src/process/capture/state.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/process/tests/capture_factory/controls.rs
- crates/jj-store-server/src/process/tests/capture_factory/foreground.rs
- crates/jj-store-server/src/process/tests/capture_factory/startup.rs
- crates/jj-store-server/src/process/tests/capture_factory.rs
- crates/jj-store-server/tests/configuration.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/server_executable/configured_capture.rs#cold_configured_codex_capture_initializes_publishes_and_preserves_pause
- crates/jj-store-server/src/process/tests/capture_factory/startup.rs#server_capture_initializes_only_absent_state_and_preserves_existing_policy
- crates/jj-store-server/tests/configuration.rs#unsupported_capture_reader_fails_before_server_state_creation
- crates/jj-store-cli/tests/server_executable.rs#cold_unsupported_capture_config_check_and_startup_preserve_private_state
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-server/tests/configuration.rs#enabled_capture_startup_fails_before_state_creation_until_workers_are_connected
- crates/jj-store-cli/tests/server_executable.rs#cold_capture_config_check_and_startup_gate_preserve_private_state
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-069 details 2](details-02.md)
- [E-M17-069 details 3](details-03.md)
- [E-M17-069 details 4](details-04.md)
