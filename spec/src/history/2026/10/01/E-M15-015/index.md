---
schema: 1
id: E-M15-015
---

# E-M15-015

<!-- evidence-field:id markdown -->
## Id

E-M15-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

parent-workspace-rebase-qualification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vlyrzmqt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Isolated clean Thread/Reaction tip 320afb54; main parent 98506452, change omyyllum, with active unfinished main work excluded. Rebase of 14 implementation changes plus empty head produced conflicts across capture spool, route registration and canonical evidence. Exact bound_enqueue_ target was absent; route-inventory regression failed once with 66 actual paths versus 58 expected. Parent local-settings M14 independently collided with Thread M14 task and evidence IDs. All 211 parent evidence entries and 14 original Thread entries are retained, with only the Thread milestone/evidence namespace renumbered. Pre-qualification parent-only file comparison matched all 183 paths; the lease correction intentionally updates the owning capture helper afterward.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; pnpm 10.32.1; CARGO_INCREMENTAL=0. Native format 5 and local companion envelope 2 unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Retain parent M14 settings scope/evidence; renumber Thread/Reaction tasks and their 14 evidence entries to M15 without changing acceptance gates. Preserve compatibility lease, separate mutation lock and native/local drain leases. Bound and unbound enqueue share mutation/queue limits. Combined generated HTTP inventory contains 66 paths. StateLease explicitly unlocks shared and exclusive locks on Drop to prevent inherited open descriptions prolonging a released lease; no persistent bytes or public API changed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_files.rs
- crates/jj-store-cli/src/credentials.rs
- crates/jj-store-cli/src/folder_picker_tests.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/state_lease.rs
- crates/jj-store-cli/tests/agent_capture_spool.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/routes.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/replication.md
- spec/src/spec-governance.md
- spec/src/verification.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-015 details 2](details-02.md)
- [E-M15-015 details 3](details-03.md)
- [E-M15-015 details 4](details-04.md)
