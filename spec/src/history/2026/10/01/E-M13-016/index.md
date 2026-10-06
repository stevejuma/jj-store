---
schema: 1
id: E-M13-016
---

# E-M13-016

<!-- evidence-field:id markdown -->
## Id

E-M13-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-browser-workstore-transport
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser/work.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-service/src/local_repository/browser_work.rs
- crates/jj-store-service/src/local_repository/selection.rs
- crates/jj-store-service/src/local_repository/work.rs
- crates/jj-store-service/src/local_repository/work_pages.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/work/browse.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/tests/local_work.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_work.rs#local_browser_browse_uses_complete_root_and_explicit_committed_pin
- crates/jj-store-service/tests/local_work.rs#forged_continuations_cannot_read_staged_metadata_candidates
- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- crates/jj-store-service/tests/local_work.rs#bound_local_browser_commands_cannot_follow_replaced_repository_identity
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-016 details 2](details-02.md)
