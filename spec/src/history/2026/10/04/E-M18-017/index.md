---
schema: 1
id: E-M18-017
---

# E-M18-017

<!-- evidence-field:id markdown -->
## Id

E-M18-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18
- M18.2
- M18.8
- M9.4
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wwutzuqw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Configured active Cursor/OpenCode capture and native timing projection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Cursor Desktop3.22.7 fixture profile; real Cursor CLI2026.09.26-dd393fe; OpenCode1.18.30; JJ0.45.1; current AgentStore only. Default Cursor wrapper auto-updated to2026.10.01-e373342 and was correctly refused. Real Desktop qualification remains pending.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Five enabled product profiles; cursor-jsonl/v1 plus metadata-only bounded lifecycle binding; opencode-snapshot/v1 finalized assistant records; unchanged immutable AgentStore identity, sanitized evidence, spool and publication receipts. OpenCode native integer times are read projections, through the owned WAL facade.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
- crates/jj-agent/src/normalize.rs
- crates/jj-agent/src/normalizers/cursor.rs
- crates/jj-agent/src/normalizers/cursor_transcript.rs
- crates/jj-agent/src/normalizers/mod.rs
- crates/jj-store-capture/src/codex_activity.rs
- crates/jj-store-capture/src/codex_discovery/scanner.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/codex_transcript/poll.rs
- crates/jj-store-capture/src/codex_transcript/progress.rs
- crates/jj-store-capture/src/codex_transcript/recovery/identity.rs
- crates/jj-store-capture/src/cursor_binding.rs
- crates/jj-store-capture/src/cursor_binding/pending.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/profile.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/codex_recovery.rs
- crates/jj-store-capture/tests/cursor_transcript.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_commands/types.rs
- crates/jj-store-cli/src/agent_hook.rs
- crates/jj-store-cli/src/agent_probe.rs
- crates/jj-store-cli/src/agent_setup.rs
- crates/jj-store-cli/src/agent_setup_templates.rs
- crates/jj-store-cli/tests/agent_hook.rs
- crates/jj-store-cli/tests/agent_integrations.rs
- crates/jj-store-cli/tests/agent_setup.rs
- crates/jj-store-jj/src/query/language/identity.rs
- crates/jj-store-server/src/capture_config.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/delivery.rs
- crates/jj-store-server/src/capture_supervisor/discovery.rs
- crates/jj-store-server/src/capture_supervisor/discovery/observations.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/src/capture_supervisor/sources.rs
- crates/jj-store-server/src/capture_supervisor/status.rs
- crates/jj-store-server/src/capture_supervisor/status/harness.rs
- crates/jj-store-server/src/opencode_source/ingress.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/tests/capture_factory/startup.rs
- crates/jj-store-server/src/storage_management/journal.rs
- crates/jj-store-server/src/storage_management/mod.rs
- crates/jj-store-server/tests/capture_runtime/draining.rs
- crates/jj-store-server/tests/cursor_supervisor.rs
- crates/jj-store-server/tests/opencode_supervisor.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/collab_reads/materialization.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_session_timing.rs
- crates/jj-store-service/src/collaboration/agent_tool_timing.rs
- crates/jj-store-service/src/collaboration/opencode_timing.rs
- crates/jj-store-service/tests/agent_sessions/timing.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases.rs
- docs/capture-preview.md
- docs/cli/agent.md
- docs/implementation-plan.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/ui.md
- web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-017 details 2](details-02.md)
- [E-M18-017 details 3](details-03.md)
