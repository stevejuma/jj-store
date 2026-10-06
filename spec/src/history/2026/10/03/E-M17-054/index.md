---
schema: 1
id: E-M17-054
---

# E-M17-054

<!-- evidence-field:id markdown -->
## Id

E-M17-054
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_commands/capture.rs
- crates/jj-store-cli/src/agent_commands/capture/client.rs
- crates/jj-store-cli/src/agent_commands/types.rs
- crates/jj-store-cli/tests/capture_cli.rs
- crates/jj-store-cli/tests/capture_cli/fixture.rs
- crates/jj-store-cli/tests/capture_cli/transport.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/capture_cli.rs#disabled_capture_status_requires_no_repository_or_device_state
- crates/jj-store-cli/tests/capture_cli.rs#capture_controls_preserve_exact_receipt_and_current_policy
- crates/jj-store-cli/tests/capture_cli/transport.rs#capture_unknown_responses_retain_original_command_without_redirect_or_retry
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-054 details 2](details-02.md)
- [E-M17-054 details 3](details-03.md)
