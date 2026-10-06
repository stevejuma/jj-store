---
schema: 1
id: E-READ-055
---

# E-READ-055

<!-- evidence-field:id markdown -->
## Id

E-READ-055
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

enclosing Jujutsu change oqwzuzwn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared local operation graph admission without selected view or commit authority
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib/jj-cli0.45.1;native format-2;index format-6;local Git SHA-1 browser profile;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence/public DTO/schema/route/cursor/RPC change. Operation graph selection uses the existing canonical operation fingerprint from JSTLOP2; code readers preserve their combined operation/view seal. Current registration access, repository/path binding, generation and bounded current-head membership remain mandatory. Plain and metadata-filtered operations reuse ready handles without constructing scoped commit grants; only separately admitted selected views may cross the read-only operation store. Verification escalation: Request.can_use_prepared_index changes local provider/HTTP dispatch for all operation graph kinds, so the complete local browser consumer suite runs with the owning JJ units and affected package checks. Unrelated UI/native suites and complete qualification stay out of scope for this local slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/prepared_operation_admission.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-JJ-005
- JST-API-002
- JST-API-004
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this slice, land E-READ-053/054/055 in main, verify combined native/local readers, retain bounded-reads and continue indexed deep operation/log/evolution continuations.
<!-- /evidence-field -->

<!-- evidence-field:commands list -->
## Commands

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_operation_admission.rs#prepared_operation_pages_admit_fresh_metadata_without_any_view_or_git_reads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-055 details 2](details-02.md)
- [E-READ-055 details 3](details-03.md)
