---
schema: 1
id: E-READ-079
---

# E-READ-079

<!-- evidence-field:id markdown -->
## Id

E-READ-079
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

rwzxwqwr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused-regression
<!-- /evidence-field -->

<!-- evidence-field:versions list -->
## Versions

- JJ 0.45.1
- Rust 1.98.1
- macOS ARM64
<!-- /evidence-field -->

<!-- evidence-field:formats list -->
## Formats

- Standard JJ default index format 6; no authoritative or wire-format change
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

First approved simplification stage: prove standard JJ warm graph reuse and incremental native construction before lifecycle integration.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-005
- JST-LOCAL-004
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/managed_code.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/managed_code.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/code_index/source.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/code_index.rs
- crates/jj-store-jj/src/query/code_index/native.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- docs/implementation-plan.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/managed_code.rs#warm_code_index_does_not_walk_commit_bodies
- crates/jj-store-cli/tests/local_browser/managed_code.rs#standard_code_construction_is_read_only_when_source_index_is_absent
- crates/jj-store-cli/tests/local_browser/managed_code.rs#linux_standard_index_reuse_and_history_pages
- crates/jj-store-cli/tests/native_lifecycle/managed_code.rs#standard_native_index_extends_indexed_ancestor
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-079 details 2](details-02.md)
- [E-READ-079 details 3](details-03.md)
