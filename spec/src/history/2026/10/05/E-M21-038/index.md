---
schema: 1
id: E-M21-038
---

# E-M21-038

<!-- evidence-field:id markdown -->
## Id

E-M21-038
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

pvzywxwv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded owned current-workspace discovery and authenticated selected-combination catalog projection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Version-1 owned discovery/catalog contracts; current supported Git SHA-1/JJ 0.45.1 profile. Native identity serialization is covered as a contract only, not native provider qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive service DTOs and schema bounds: 64 selected combinations, 4096 stack members per combination and eight-MiB complete responses. Private combination version 1, journal version 6, code/native/companion/WAL and transport inventory formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation/setup.rs
- crates/jj-store-server/src/stack_combinations/public.rs
- crates/jj-store-server/src/stack_combinations.rs
- crates/jj-store-server/tests/stack_combinations/public.rs
- crates/jj-store-server/tests/stack_combinations.rs
- crates/jj-store-service/src/stack_workspace/discovery.rs
- crates/jj-store-service/src/stack_workspace/mod.rs
- crates/jj-store-service/tests/stack_workspace_discovery_contracts.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/milestones.md
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

Existing recovery/protocol family passes 15 parents. New discovery binary initially fails compilation with 22 missing-contract diagnostics. First implementation run exposes one test-only E0502 borrow diagnostic; duplicate membership is cloned before the mutation without weakening assertions. No product runtime failure was observed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Repository/current-binding/exact-operation discovery input, full observed selection and absent/ready/stale/pending states separate from historical receipts. Selected catalog scope, active generation/code linkage, profile-preserving identities, unique names/IDs/membership and complete resource limits. Existing authenticated combination state projects only selected names/targets/merge/code/stacks with a pending-coordination flag, never proposed state or retention. Metadata-owner tests cover empty/pending/selected/corrupt/foreign/reopen and unchanged reads separately from actual registered setup projection matched to a real observed native checkout. Runtime joins and all production capabilities remain absent.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-038 details 2](details-02.md)
- [E-M21-038 details 3](details-03.md)
