---
schema: 1
id: E-M18-007
---

# E-M18-007

<!-- evidence-field:id markdown -->
## Id

E-M18-007
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
- JST-LOCAL-001
- JST-UI-006
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Captured message occurrence times in agent response transcripts
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change upuyqosu
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean main upuyqosu/73ed9ca2 above session timing slice slzksssp/72ebebc7. Existing captured session 41ffb0d2c68320aeb63a8713fe38d6a6 has 13 messages with no API occurrence field; UI displays sequence numbers. Exact native regression fails with four unknown times instead of two known/two unknown times. Native timestamps already exist in immutable sanitized observation payloads.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; existing Chrono 0.4.45 and Svelte 5.57.1. Existing Codex CLI 0.156.1/Desktop 0.159.2 qualified normalizer profiles retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional occurred_at_millis fields on AgentActivity.Message and AgentTranscriptChunk; HTTP version 1 retained. Exact selected normalized event IDs match sanitized observations. Explicit observation time wins over qualified RFC3339 recovery. Chunks additionally match current source-generation-1 identity/sequence and exact selected message content under a 4096-event bound. Missing/malformed/unmatched/unsupported evidence stays unknown. Native objects, local companion metadata, capture/recovery identities, cursor framing and receipt/publication formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/fixtures/AgentPromptFixture.svelte
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/agent_session_timing.rs
- crates/jj-store-service/src/collaboration/agent_transcript_timing.rs
- crates/jj-store-service/src/collaboration/agent_transcripts.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/local_repository/browser_agents/transcripts.rs
- crates/jj-store-service/tests/agent_sessions/timing.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/local_agent/timing.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/ui/community-message/CommunityMessageMeta.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/tests/collaboration-presentation.test.ts
- spec/src/milestones.md
- spec/src/compatibility.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-007 details 2](details-02.md)
- [E-M18-007 details 3](details-03.md)
