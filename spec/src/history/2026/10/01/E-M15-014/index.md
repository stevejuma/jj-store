---
schema: 1
id: E-M15-014
---

# E-M15-014

<!-- evidence-field:id markdown -->
## Id

E-M15-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.2
- M15.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-atomic-bound-capture
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change poovpwwu
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 847e5093. Local capture selects only Agent; local prepared capture rejects any explicit binding. No local bound provider regression exists.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; real disposable colocated Git-backed JJ folders, companion ref CAS and stock Git GC. No installed service, checkout migration or live RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive local BoundCapture enum variant preserves existing Capture JSON tags, field order and intent/receipt bytes. Companion envelope 2 selects Agent and Thread descriptors together and retains exact newly staged Agent session dependencies. Original optional spool binding selects the complete local prepared request; matching receipts precede source and Thread preconditions.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_capture_intent.rs
- crates/jj-store-cli/src/local_agent_publication.rs
- crates/jj-store-cli/tests/local_agent_recovery.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_thread_binding.rs
- crates/jj-store-service/src/local_repository/collab_external.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/local_agent_binding.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-014 details 2](details-02.md)
