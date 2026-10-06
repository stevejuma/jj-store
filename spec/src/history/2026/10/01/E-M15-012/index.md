---
schema: 1
id: E-M15-012
---

# E-M15-012

<!-- evidence-field:id markdown -->
## Id

E-M15-012
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

native-atomic-capture-provider
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uvpzptzm
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Native capture selects only AgentStore. New bound_capture target absent. Resume isolated workspace at cbaaba4d; current owned provider work has no overlapping user edits.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; native MemoryStore CAS fault fixture, no live RustFS or installed runtime changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentDeltaRequest additive optional thread_binding omitted when absent; frozen unlinked canonical bytes preserved. Explicit ThreadBindingV1 observed version/assignment plus stable actor/request/session link ID and prepared acceptance time. Native manifest format 5 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/capture_binding.rs
- crates/jj-collab/src/lib.rs
- crates/jj-store-cli/src/agent_capture_intent.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/tests/agent_capture_spool.rs
- crates/jj-store-rpc/src/agent/binding_tests.rs
- crates/jj-store-rpc/src/agent.rs
- crates/jj-store-wal/src/publication/agent_delta.rs
- crates/jj-store-wal/src/publication/agent_thread_binding.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/bound_capture.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/publication.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/bound_capture.rs#capture_cas_race_reapplies_disjoint_changes_but_rejects_changed_assignment
- crates/jj-store-wal/tests/bound_capture.rs#bound_capture_selects_both_roots_once_preserves_lifecycle_and_replays_after_recovery
- crates/jj-store-wal/tests/bound_capture.rs#stale_or_denied_binding_cannot_select_any_agent_edits
- crates/jj-store-wal/tests/bound_capture.rs#unknown_binding_retains_identity_and_changed_retry_is_rejected
- crates/jj-store-wal/tests/bound_capture.rs#unlinked_encodings_are_unchanged_and_bindings_require_one_exact_session
- crates/jj-store-rpc/src/agent/binding_tests.rs#delta_codec_retains_explicit_binding_and_unlinked_prepare_rejects_it
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-012 details 2](details-02.md)
- [E-M15-012 details 3](details-03.md)
