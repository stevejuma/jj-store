---
schema: 1
id: E-M10-004
---

# E-M10-004

<!-- evidence-field:id markdown -->
## Id

E-M10-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

collaboration-accepted-fact-convergence
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/catalog.rs
- crates/jj-agent/src/delta.rs
- crates/jj-agent/src/objects.rs
- crates/jj-review/src/commands.rs
- crates/jj-review/src/comments.rs
- crates/jj-review/src/objects.rs
- crates/jj-review/src/replication.rs
- crates/jj-review/src/threads.rs
- crates/jj-store-cli/src/review_commands.rs
- crates/jj-store-replication/tests/agent_convergence.rs
- crates/jj-store-replication/tests/collaboration.rs
- crates/jj-store-replication/tests/one_way.rs
- crates/jj-store-replication/tests/review_convergence.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/collaboration/review_replication.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-wal/src/publication/agent_delta.rs
- crates/jj-store-wal/src/publication/domains.rs
- crates/jj-store-wal/src/publication/replication_agent.rs
- crates/jj-store-wal/src/publication/replication_domains.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_review.rs
- crates/jj-store-wal/src/publication/replication_review_graph.rs
- crates/jj-store-wal/src/publication/review_audit_replication.rs
- crates/jj-store-wal/src/publication/review_command_update.rs
- scripts/check-progress.mjs
- schemas/openapi.json
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/replication.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-replication/tests/collaboration.rs#imported_agent_and_review_facts_reduce_without_replaying_commands
- crates/jj-store-replication/tests/agent_convergence.rs#session_facts_union_and_terminal_conflicts_remain_explicit
- crates/jj-store-replication/tests/agent_convergence.rs#every_mutable_catalog_converges_under_reverse_and_duplicate_delivery
- crates/jj-store-replication/tests/review_convergence.rs#concurrent_comment_edits_retain_all_maximal_revisions_and_converge
- crates/jj-store-replication/tests/review_convergence.rs#explicit_comment_resolution_covers_the_exact_conflict_and_replicates
- crates/jj-store-replication/tests/review_convergence.rs#unseen_reply_and_concurrent_reopen_keep_workflow_open
- crates/jj-store-replication/tests/review_convergence.rs#concurrent_tombstone_and_edit_preserve_both_maximal_revisions
- crates/jj-store-replication/tests/review_convergence.rs#concurrent_replies_preserve_both_children_and_converge
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M10-004 details 2](details-02.md)
- [E-M10-004 details 3](details-03.md)
- [E-M10-004 details 4](details-04.md)
