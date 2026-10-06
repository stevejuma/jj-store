---
schema: 1
id: E-M21-039
---

# E-M21-039

<!-- evidence-field:id markdown -->
## Id

E-M21-039
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

zmrponqr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Provider-owned stable physical identity read for historical stack recovery admission
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Pinned JJ 0.45.1 Git SHA-1 provider profile. Existing physical-workspace digest version 1 is reused unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire, journal, combination, native checkout, code, companion, WAL or manifest format changes. New provider method returns the existing owned digest.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/working_tree/current.rs
- crates/jj-store-jj/tests/working_tree_current.rs
- crates/jj-store-jj/tests/working_tree_current/identity.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-003
- JST-JJ-004
- JST-UI-020
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Five existing current-reader parents pass. The new exact four-parent identity target initially fails compilation with nine missing-method diagnostics. No initial implementation test failed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the observer-owned path/root/.jj device-and-inode digest through the existing bounded authenticated provider worker. Recheck no-follow directory identity across authorization, bound path bytes and charge directory work. No current code, head, checkout/tree-state, policy or working-file read, initialization or repair. Exact tests cover matching observer identity, damaged/missing current state, unchanged reads, replacement before/during admission, unsafe/missing/oversized paths and access loss at all three authorization boundaries. Registered runtime, store location and command composition are deliberately outside this narrow provider slice. The user requested a pause after the next slice; pause follows verification and commit. Initial patch context misses applied no changes and are not product regression failures. Root aggregate, unrelated Rust/UI/Storybook, transport generation, RustFS, publication and visual lanes are out of scope under the targeted policy.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-039 details 2](details-02.md)
- [E-M21-039 details 3](details-03.md)
