---
schema: 1
id: E-CI-HEAVY-001
---

# E-CI-HEAVY-001

<!-- evidence-field:id markdown -->
## Id

E-CI-HEAVY-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-LOCAL-001
- JST-LOCAL-002
- JST-LOCAL-003
- JST-GOV-003
- JST-GOV-004
- JST-GOV-011
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uvupsmop
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused qualification of the CLI-owned legacy AgentStore regression only. PR #1 merged at its green b71f2c4c head through normal protected merging into 615822b93de172e7bae13c3c77262018769ad48a. Complete manual qualification dispatched at that immutable merge revision. User deferred compilation-cache improvements; experiment 38712300595 was canceled before a measured comparison completed. Normal CI retains rust-cache alone. The prior heavy shard failure is an obsolete V1 AgentStore compatibility assertion, contrary to the current-only decision recorded by E-M17-034. This slice updates the CLI-owned regression without altering implementation, schemas, publication rules or migration support. Focused verification covers the changed provenance binary and the direct descriptor decoder tests; unrelated local suites remain out of scope. Full hosted qualification remains mandatory and is tracked separately. After reproducing the deterministic failure locally and validating its focused repair, the initial main qualification attempt was canceled before heavy compilation to avoid known-failing work; complete qualification will restart at the follow-up PR revision.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_agent_provenance.rs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M13/index.md
- spec/src/history/2026/10/15/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-HEAVY-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-CI-HEAVY-001/index.md
- spec/src/history/2026/10/15/E-CI-HEAVY-001/details-02.md
- spec/src/history/2026/10/15/E-CI-HEAVY-001/details-03.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_agent_provenance.rs#legacy_local_agent_descriptor_is_rejected_without_rewrite
- crates/jj-store-service/src/local_repository/agent_descriptor.rs#rejects_legacy_agent_descriptor_without_in_memory_upgrade
- crates/jj-store-service/src/local_repository/agent_descriptor.rs#current_agent_descriptor_requires_exact_version_identity_and_canonical_bytes
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-cli/tests/local_agent_provenance.rs#original_local_agent_descriptor_remains_readable_before_v2_publication
<!-- /evidence-field -->

