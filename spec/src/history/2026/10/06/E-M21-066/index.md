---
schema: 1
id: E-M21-066
---

# E-M21-066

<!-- evidence-field:id markdown -->
## Id

E-M21-066
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

server-free local Stacks CLI and native engine interoperability
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change otwqqtrz: Add a server-free local Stacks command family
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Expose full existing ordinary-local Stacks functionality through jj-store stack: inspection, workspace/branch/content/graph/conflict/history actions, exact file/hunk pins, native snapshots, no-snapshot inspection and nonmutating dry runs.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/stack_commands/args.rs
- crates/jj-store-cli/src/stack_commands/context.rs
- crates/jj-store-cli/src/stack_commands/mutations.rs
- crates/jj-store-cli/src/stack_commands/queries.rs
- crates/jj-store-cli/src/stack_commands/selection.rs
- crates/jj-store-cli/src/stack_commands.rs
- crates/jj-store-cli/tests/stack_cli.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/preview.rs
- crates/jj-store-jj/src/local_stacks/read.rs
- docs/cli/server-commands.md
- docs/cli/server.md
- docs/cli/stacks.md
- docs/maintenance/coverage.json
- docs/site/cli/index.md
- docs/site/cli/jj-store.md
- docs/site/guides/browsing.md
- docs/site/technical/local.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; macOS ARM64; pnpm 10.32.1; no package publication or installed executable replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing version-1 TOML, native graph model, command DTOs and HTTP schemas unchanged. Transient version-1 mixed-source CLI selection input reuses operation/revision/Selection fields. Text/JSON results include exact pins. CLI exit 3 identifies applied-operation attention; no retry.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

target/debug/jj-store stack --help rejected the unrecognized subcommand (exit 2). Native previews used load_at_head, which could reconcile concurrent operation heads. No Stacks CLI existed.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_cli.rs#stack_help_and_completions_are_discoverable_without_a_repository
- crates/jj-store-cli/tests/stack_cli.rs#inspection_dry_run_and_invalid_arguments_preserve_native_state
- crates/jj-store-cli/tests/stack_cli.rs#workspace_membership_and_settings_keep_native_merge_wip_and_checkout
- crates/jj-store-cli/tests/stack_cli.rs#topology_cards_aliases_partial_application_and_native_history_agree
- crates/jj-store-cli/tests/stack_cli.rs#selected_files_and_hunks_preserve_unselected_work_and_reject_stale_input
- crates/jj-store-cli/tests/stack_cli.rs#graph_branch_and_content_actions_use_native_rewrites
- crates/jj-store-cli/tests/stack_cli.rs#linked_workspace_and_relative_paths_use_its_own_local_configuration
- crates/jj-store-cli/tests/stack_cli.rs#grouped_unapply_preserves_shared_explicit_members
- crates/jj-store-cli/tests/stack_cli.rs#text_conflict_resolution_and_native_undo_redo_retain_work
- crates/jj-store-cli/tests/stack_cli.rs#concurrent_heads_and_ambiguous_selectors_do_not_reconcile_during_inspection
- crates/jj-store-cli/tests/stack_cli.rs#movement_and_abandonment_preserve_other_applied_branches
- crates/jj-store-cli/tests/stack_cli.rs#diff_continuations_integrated_choices_and_reconciliation_are_exact
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-066 details 2](details-02.md)
- [E-M21-066 details 3](details-03.md)
