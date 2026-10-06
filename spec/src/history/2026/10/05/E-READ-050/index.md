---
schema: 1
id: E-READ-050
---

# E-READ-050

<!-- evidence-field:id markdown -->
## Id

E-READ-050
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

enclosing Jujutsu change opnpqprl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native prepared references at a different embedded operation with retained outer authority
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-cli 0.45.1; native format-2; index format-6; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native, cache receipt/index, WAL, manifest, signed cursor, public DTO/schema or RPC format change. The outer read retains repository, epoch, frozen committed membership and current authorization. A separate explicit reference operation selects only its own generation after point membership admission; prepared data cannot extend access. Existing source-operation preparation and inline bounds remain explicit.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/access/selection.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_embedded.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/architecture.md
- spec/src/milestones.md
- docs/implementation-plan.md
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified native embedded-operation slice, then deliver local embedded-operation membership/preparation recovery before indexed child/deep continuations. Keep main at the verified E-READ-049 landing and retain this workspace for follow-up.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rust-only private native reference selection and directly affected prepared reference/current-access consumers. Exact failing regression first, then existing reference/window/filter families and current committed-read security, affected JJ and direct service/HTTP/RPC/CLI checks and focused format/governance/boundary checks. No generated contracts change; no UI, full workspace/library/transport suites, Linux/two-million fixtures, cold browser/RustFS, RSS/latency, release or visual qualification repeated after E-READ-049 combined-source verification.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-001
- JST-GOV-002
- JST-GOV-003
- JST-GOV-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/reference_embedded.rs#native_embedded_reference_operation_uses_its_own_prepared_window_and_outer_authority
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs#native_prepared_references_select_visible_windows_before_target_reads
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs#workspace_windows_seek_filter_and_resolve_exact_names_without_history
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs#operation_pages_do_not_admit_or_build_a_commit_index
- crates/jj-store-cli/tests/native_lifecycle/reference_filters.rs#native_filtered_references_seek_pinned_text_before_selected_target_reads
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_rechecks_revocation_and_limits_and_rejects_staged_objects
- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_pins_history_and_reports_divergent_heads_without_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-050 details 2](details-02.md)
