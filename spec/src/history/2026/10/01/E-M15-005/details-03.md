---
schema: 1
id: E-M15-005
---

# E-M15-005 details 3

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native services/query, HTTP/RPC/CLI, capture binding, reducer 3, shared UI,
  timeline/search, cold real-RustFS and runner gates remain pending. Root
  aggregate pnpm lanes and unrelated UI/Rust workspace suites out of scope.
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Full jj-store-wal suite is required because receipt-first root audits and a private captured-manifest publication guard change the shared domain publisher used by Agent, Review, Work and Git. Shared jj-collab variants also require its suite and focused local companion acceptance. No unrelated Rust workspace or UI aggregate is required.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/collab_commands.rs#native_commands_pin_history_preserve_siblings_and_replay_before_target_checks
- crates/jj-store-wal/tests/collab_commands.rs#source_thread_cas_race_revalidates_visibility_before_reaction_publication
- crates/jj-store-wal/tests/collab_commands.rs#source_review_cas_race_revalidates_moderation_and_keeps_staged_reactions_invisible
- crates/jj-store-wal/tests/collab_commands.rs#manifest_unknown_outcomes_keep_identity_and_replay_after_reopen
- crates/jj-store-wal/tests/collab_commands.rs#native_custom_images_retire_without_rewriting_historical_values_or_assets
- crates/jj-store-wal/tests/collab_commands.rs#native_code_links_require_selected_accepted_identities_and_preserve_absent_domains
- crates/jj-store-wal/tests/collab_commands.rs#native_sessions_remain_optional_and_links_check_selected_membership_and_assignment
- crates/jj-store-wal/tests/collab_commands.rs#disjoint_actor_cas_race_reapplies_original_intent_and_retains_both_slots
<!-- /evidence-field -->

