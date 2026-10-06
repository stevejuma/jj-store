---
schema: 1
id: E-M18-008
---

# E-M18-008

<!-- evidence-field:id markdown -->
## Id

E-M18-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.4
- M9.5
- M18.8
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-002
- JST-COLLAB-004
- JST-COLLAB-005
- JST-API-001
- JST-API-006
- JST-LOCAL-001
- JST-UI-006
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Recorded tool durations, padding before tool icons and simplified session times
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rvkzvqxz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean main rvkzvqxz/1163ba3e above upuyqosu/f6f69419; preview clean. Both existing exec tools in selected session lack duration fields. Exact native duration regression fails once. Initial tool-group stories fail twice for missing inset/duration. User then asks to remove Started prefix; its exact Agent response story fails once. Subsequent user inspection identifies zero row padding before tool icons despite an 8px outer-group inset; strengthened icon regression captures that boundary.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Chrono 0.4.45; Svelte 5.57.1; qualified Codex CLI0.156.1/Desktop0.159.2 profiles retained; released Design Core ToolCalls stats extension used.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional nonnegative duration_millis in AgentToolActivity version1. Exact selected event observations provide ordered call/result endpoints or timed hook lifecycle endpoints; source/profile/sequence and monotonic times must match. Missing, malformed, ambiguous, over-bound evidence stays absent. Zero remains known. Unknown/failed outcomes stay unchanged. No immutable object, source cursor, capture recovery, receipt or native/local publication format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentToolGroups.stories.svelte
- catalog/components/store-application/agent-sidebar-play.ts
- catalog/fixtures/agent-tool-groups.ts
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/agent_session_timing.rs
- crates/jj-store-service/src/collaboration/agent_tool_timing.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/tests/agent_sessions/timing.rs
- crates/jj-store-service/tests/local_agent/timing.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTools.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/tests/collaboration-presentation.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-008 details 2](details-02.md)
- [E-M18-008 details 3](details-03.md)
