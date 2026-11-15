---
schema: 1
id: E-ORIGINAL-REMOTE-005
---

# E-ORIGINAL-REMOTE-005

<!-- evidence-field:id markdown -->
## Id

E-ORIGINAL-REMOTE-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.9
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-LOCAL-008
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/working_tree/current.rs
- crates/jj-store-service/src/working_tree/observer.rs
- crates/jj-store-service/src/working_tree/status.rs
- crates/jj-store-service/tests/working_tree_observer.rs
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/current/contracts/jj-integration/03-existing-native-read-behavior.md
- spec/src/current/contracts/local-jj/18-bounded-stack-history-browsing.md
- spec/src/current/progress/M34/index.md
- spec/src/current/verification/local-03.md
- spec/src/current/contracts/mutations/05-typed-native-command-preparation.md
- spec/src/current/contracts/documentation/13-native-remotes-source-qualification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/working_tree_observer.rs#status_accepts_metadata_only_heads_but_not_a_different_working_copy_commit
- crates/jj-store-jj/tests/working_tree_current.rs#current_read_proves_checkout_tree_sparse_and_captured_base_policy_without_writes
- crates/jj-store-jj/tests/working_tree_baseline.rs#baseline_reads_exact_workspace_at_selected_operation_without_publication
<!-- /evidence-field -->

