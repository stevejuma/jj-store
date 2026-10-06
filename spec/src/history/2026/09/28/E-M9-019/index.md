---
schema: 1
id: E-M9-019
---

# E-M9-019

<!-- evidence-field:id markdown -->
## Id

E-M9-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_hook.rs
- crates/jj-store-cli/src/agent_integrations.rs
- crates/jj-store-cli/src/agent_sanitization.rs
- crates/jj-store-cli/src/agent_setup.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/agent_hook.rs
- docs/cli/agent.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_hook.rs#hook_ingestion_survives_restart_orders_sequences_and_never_persists_raw_input
- crates/jj-store-cli/tests/agent_hook.rs#hook_command_selects_one_supported_integration
- crates/jj-store-cli/tests/agent_hook.rs#hook_rejects_disabled_integrations_before_capture_state_exists
- crates/jj-store-cli/tests/agent_integrations.rs#registry_probes_pinned_harnesses_and_persists_explicit_enablement
- crates/jj-store-cli/tests/agent_setup.rs#setup_recovers_owned_state_and_refuses_drifted_configuration
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_hook --locked
```

**Notes**

The test-first compile reported the intentionally missing agent_hook module and ingestion boundary.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_hook --locked
```

**Notes**

The first implementation compile found Rust 1.98's typed TryLockError and the deliberately hidden Hook command's required exhaustive match arm; both were corrected without weakening lock or dispatch behavior.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_hook --locked
```

**Notes**

The initial end-to-end hook regression passed in 92.52 seconds including one cold Kingfisher rule-cache build and three independent hook invocations.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_hook --locked hook_command_selects_one_supported_integration; cargo test -p jj-store-cli --test agent_hook --locked hook_rejects_disabled_integrations_before_capture_state_exists; cargo test -p jj-store-cli --test agent_setup --test agent_integrations --locked
```

**Notes**

The hidden command grammar and pre-capture disabled rejection pass, along with all four setup and all three registry regressions after extraction of the read-only installation-state check.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_hook --locked
```

**Notes**

All three final hook regressions pass in 91.47 seconds. The end-to-end case proves restart-stable source identity, sequences one through eleven including eight concurrent omissions, fixed omission bytes, exact Kingfisher policy and absence of the synthetic raw secret from every durable state file.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test agent_setup --locked; cargo clippy -p jj-store-cli --test agent_hook --test agent_setup --test agent_integrations --locked -- -D warnings; cargo fmt --all -- --check; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed files>
```

**Notes**

All four setup regressions pass after setup readiness was moved ahead of the cold cache build. The affected CLI and tests are warning-free and formatted; 85 requirements, eight protected-file mappings, all 60 progress tasks and changed prose formatting pass. Production modules contain 298 or fewer physical lines and the hook regression contains 178.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Prepare normalized pending captures and retry their exact AgentStore
  publication through the existing receipt path
- Add drift-safe owned integration removal after ingesting setup is qualified
- Execute pinned real-harness capability and smoke tests in M9.6; Cursor remains
  unsupported on the current host
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; capture hook source state v1; MongoDB Kingfisher policy mongodb-kingfisher@2.7.0/builtin-medium/v1; exact normalizer profiles from E-M9-008.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The hidden hook command reads at most four MiB plus one overflow byte, verifies enabled and currently installed device policy without taking the administrative registry lock, sanitizes in memory, reserves a synchronized monotonic source sequence and atomically enqueues one SanitizedObservationBatchV1. Invalid input emits only a fixed omission payload. Reservations survive restart and may leave gaps after interrupted enqueue but are never reused; derived batch and observation IDs are domain-separated by stable source, generation and sequence.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


