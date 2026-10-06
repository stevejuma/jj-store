---
schema: 1
id: E-M15-006
---

# E-M15-006 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (final)
```

**Notes**

111 tasks; placeholder fixed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (final)
```

**Notes**

4 protected files, 3 canonical chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native code writes still require qualified reducer 3; current native command
  acceptance is WorkStore homes, plus explicit code rejection. This is not full
  native code, RustFS, cold-process or browser acceptance.
- Transport cursor capacity/expiry/query binding, HTTP/RPC/CLI exposure,
  optional capture binding, reducer 3, canonical cross-source timeline/search,
  released shared UI and runner gates remain pending. Root aggregate pnpm lanes
  and unrelated Rust/UI suites are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish the service slice; implement bounded transport continuations and public command boundaries, then qualify reducer 3 before claiming native code collaboration.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/native_collab.rs#absent_reads_do_not_select_domains_or_change_code_and_limits_fail_closed
- crates/jj-store-service/tests/native_collab.rs#commands_derive_actor_replay_receipts_and_keep_status_scoped_to_current_identity
- crates/jj-store-service/tests/native_collab.rs#counts_filter_actors_before_pagination_and_pins_keep_exact_source_roots
- crates/jj-store-service/tests/native_collab.rs#history_continuations_are_thread_members_and_revision_feedback_never_moves
- crates/jj-store-service/tests/native_collab.rs#library_and_asset_reads_retain_original_versions_and_reject_unselected_assets
- crates/jj-store-service/tests/native_collab.rs#bounded_pins_and_current_token_revocation_apply_to_reads_and_receipt_status
- crates/jj-store-service/tests/native_collab.rs#replicated_code_commands_remain_gated_without_changing_authority
- crates/jj-store-service/tests/native_collab.rs#imported_conflicted_actor_slots_have_no_ordinary_value_or_hidden_count
- crates/jj-store-service/tests/native_collab.rs#service_deadline_keeps_unknown_request_identity_for_exact_retry
<!-- /evidence-field -->

