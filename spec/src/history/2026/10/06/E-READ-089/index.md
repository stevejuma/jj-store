---
schema: 1
id: E-READ-089
---

# E-READ-089

<!-- evidence-field:id markdown -->
## Id

E-READ-089
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-API-005
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded local physical admission cache with fresh read boundaries
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wmpqkuqp
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact real HTTP fixture: 40 files across three unrelated edits caused 238 physical registration/folder inspections. Assertion &lt;=20 failed. Same-profile Linux quick metadata root12.475s,arch9.336s,block22.591s; names1.241-1.474s.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Share up to256 process-local validated folder entries for inner checks, max age1s; preserve full request/preparation boundary validation and independent object membership. Registration persistence invalidates before and after replacement; existing physical watcher hints invalidate entries. Expiry and mandatory boundary validation cover lost events. Shared refresh suppresses concurrent duplicate inspection; no native permission, wire, source-state or UI change. Exact tests precede complete direct local_browser binary and affected five-package all-target checks; root aggregates, UI, native and RustFS suites out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust1.98.1; JJ/jj-lib0.45.1; existing pinned dependencies; unoptimized test/development profile.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive internal Access.check with full-authorize default and owned service FolderAdmission cache. Existing HTTP DTOs/cursors, source JJ/Git metadata, manifests, WAL, registry persistence and credentials unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/admission_cache.rs
- crates/jj-store-cli/tests/local_browser/managed_http.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/local_browser/binding.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/budget.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-server/src/local_repositories/watcher.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-service/src/local_browser/admission/tests.rs
- crates/jj-store-service/src/local_browser/admission.rs
- crates/jj-store-service/src/local_browser.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/admission_cache.rs#local_metadata_reads_bound_registration_inspections
- crates/jj-store-cli/tests/local_browser/admission_cache.rs#cached_inner_checks_cannot_bypass_final_physical_validation
- crates/jj-store-cli/tests/local_browser/admission_cache.rs#registration_persistence_invalidates_shared_admission_synchronously
- crates/jj-store-service/src/local_browser/admission/tests.rs#invalidation_during_validation_does_not_publish_a_stale_entry
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-089 details 2](details-02.md)
