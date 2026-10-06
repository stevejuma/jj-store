---
schema: 1
id: E-M15-013
---

# E-M15-013

<!-- evidence-field:id markdown -->
## Id

E-M15-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-bound-capture-transport-worker
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vkzvsskr
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 5d09d9e5. Native capture preparation has no binding input; captured spool entries cannot carry bindings, and native drain rejects bound returned intent.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; real Unix gRPC over disposable MemoryStore; filesystem spool reopen. No installed binary, live service or runner changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive canonical ThreadBindingV1 bytes on protobuf field 7, empty for legacy unlinked capture. Optional captured spool field omitted when absent. Returned actor/time and full AgentDeltaRequest are journaled without refreshing explicit observed Thread or assignment; existing WAL request and receipt bytes remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_intent.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- crates/jj-store-cli/tests/agent_capture_spool.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/agent/binding_tests.rs
- crates/jj-store-rpc/src/agent.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/collab.rs
- crates/jj-store-service/src/native_agent.rs
- crates/jj-store-service/src/transport.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-013 details 2](details-02.md)
- [E-M15-013 details 3](details-03.md)
