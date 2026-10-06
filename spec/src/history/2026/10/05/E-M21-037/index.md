---
schema: 1
id: E-M21-037
---

# E-M21-037

<!-- evidence-field:id markdown -->
## Id

E-M21-037
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

unzloutz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Owned public command recovery contracts and validated existing-state journal projections
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1/Git SHA-1 profile; version-1 service contracts. Real registered setup/execution acceptance uses isolated folders and explicit test/development binaries.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive owned public result/status/pending/cancellation-identity contracts, 128-command and eight-MiB response bounds. Existing private journal version 6, combination version 1 and native/companion/WAL formats are unchanged. No HTTP route or generated transport change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation/setup.rs
- crates/jj-store-server/src/stack_command_journal/public.rs
- crates/jj-store-server/src/stack_command_journal.rs
- crates/jj-store-server/tests/stack_command_journal/public.rs
- crates/jj-store-server/tests/stack_command_journal.rs
- crates/jj-store-service/src/stack_workspace/mod.rs
- crates/jj-store-service/src/stack_workspace/receipt.rs
- crates/jj-store-service/src/stack_workspace/recovery.rs
- crates/jj-store-service/tests/stack_command_recovery_contracts.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-MUT-001
- JST-CLI-004
- JST-UI-020
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Existing service protocol contracts pass 9 tests. New recovery-contract tests initially fail compilation with 17 missing-type diagnostics; new server projection tests fail compilation with 12 missing-method diagnostics. After implementation 5 recovery parents pass and one schema test fails because it reads definitions rather than the emitted $defs. The lookup is corrected without weakening schema/private-data assertions.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Stable repository/physical/JJ-workspace/profile scope independent of live generations; exact original ID/digest/preparation binding, preview/receipt linkage and strict public serialization. Existing journal status/pending project only owned public results with actor checks, bounded unique discovery, terminal exclusion and reopen/read-only evidence. Actual registered setup verifies projection while retained private execution exists and proves these reads leave code/disk state unchanged. Receipt pending predicate is centralized with unchanged semantics. Cancellation validation grants no execution authority.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-037 details 2](details-02.md)
- [E-M21-037 details 3](details-03.md)
- [E-M21-037 details 4](details-04.md)
