---
schema: 1
id: E-READ-063
---

# E-READ-063

<!-- evidence-field:id markdown -->
## Id

E-READ-063
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

yxwwpyoo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

prepared-default-log-rank-seeking-native-and-local
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib0.45.1;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authoritative object,WAL,index,cache receipt,cursor,DTO or contract changes. Native reservation196 bytes/indexed commit replaces192; local384-byte reservation retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Complete query library and CLI native_lifecycle/local_browser binaries:shared visible membership serves native/local graphs and reference/prefix consumers,and changed test registrations feed both binaries. Exact before/after first. Package/format/boundary/architecture and changed-document checks only. Root aggregate,WAL/RPC/HTTP/UI/contract generation and complete qualification out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-API-002
- JST-API-004
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/prepared_log_pages.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/prepared_log_pages.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation/visibility.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/visible.rs
- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/src/query/projection/changes.rs
- crates/jj-store-jj/src/query/projection/decorations.rs
- crates/jj-store-jj/src/query/projection/references.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/prepared_log_pages.rs#prepared_all_log_pages_seek_deep_anchors_under_fixed_work
- crates/jj-store-cli/tests/local_browser/prepared_log_pages.rs#prepared_all_log_pages_seek_deep_anchors_under_fixed_work
- crates/jj-store-jj/src/query/prepared/visible.rs#deep_visible_windows_seek_exact_order_under_fixed_work
- crates/jj-store-jj/src/query/prepared/visible.rs#visible_order_rejects_duplicates_work_exhaustion_and_zero_pages
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-063 details 2](details-02.md)
- [E-READ-063 details 3](details-03.md)
