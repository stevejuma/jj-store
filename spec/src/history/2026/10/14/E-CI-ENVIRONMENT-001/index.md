---
schema: 1
id: E-CI-ENVIRONMENT-001
---

# E-CI-ENVIRONMENT-001

<!-- evidence-field:id markdown -->
## Id

E-CI-ENVIRONMENT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-GOV-002
- JST-GOV-004
- JST-LOCAL-004
- JST-LOCAL-005
- JST-MUT-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

kltptywl
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/working_tree/walk.rs
- scripts/check-progress.mjs
- scripts/governance.test.mjs
- scripts/managed-cargo.test.mjs
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/governance.test.mjs#progress_checks_merge_against_every_parent
- scripts/managed-cargo.test.mjs#two managed workspaces reuse unchanged builds
  and keep independent executables
- crates/jj-store-service/tests/working_tree_inventory.rs#inventory_retains_ignored_symlink_binary_and_unicode_rows_without_content
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [CI environment evidence](details-02.md)
- [CI environment evidence](details-03.md)
