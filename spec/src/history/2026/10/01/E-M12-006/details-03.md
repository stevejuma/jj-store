---
schema: 1
id: E-M12-006
---

# E-M12-006 details 3

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
cargo fmt --all && cargo clippy -p jj-store-wal -p jj-store-jj -p jj-store-replication -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write AGENTS.md docs/cli/commands.md spec/src/cli.md spec/src/git-interop.md spec/src/publication.md spec/src/jj-integration.md spec/src/mutations.md spec/src/replication.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

Tracked guide scope update also owns spec-governance.md; added explicit qualified scope and remaining gates.
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
cargo test -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-rpc --locked
```

**Notes**

Post-admission relevant owning and direct receiver suites passed; 2 real RustFS tests ignored. Native CLI exact gate already rerun. No unrelated UI or full workspace aggregate lanes.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/spec-governance.md spec/src/milestones.md && pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

21 protected files/8 owning chapters; 20 chapters/123 requirements;94 tasks;15 package boundaries.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/git_bridge.rs#git_origin_activation_requires_owned_validation_and_retains_selected_facts
- crates/jj-store-jj/tests/git_bridge.rs#git_origins_keep_causal_conflicts_replay_and_cold_copy_without_transport
- crates/jj-store-jj/tests/git_bridge.rs#git_origin_rejects_forged_mapping_and_frozen_bookmark_claims
- crates/jj-store-jj/tests/git_bridge.rs#format_four_seed_retains_git_and_legacy_origin_digest_shape
- crates/jj-store-cli/tests/executable_native.rs#native_git_push_uses_exact_leases_retains_parent_and_recovers_without_repush
- crates/jj-store-rpc/tests/replication.rs#authenticated_git_origin_transfer_is_data_only_and_emits_committed_freshness
- crates/jj-store-service/tests/git_bridge.rs#git_push_executor_cannot_be_spoofed_or_changed_by_another_writer
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

GB4 data-only reducer slice is qualified and enables code Git activation. Remaining M12.5 fault/cancellation matrix and GB5 bookmark selectors, remote lifecycle and atomic multi-ref parity, then GB6 real RustFS external merge, approved credentials, hard process lifetime and cold browser freshness. No complete-bridge or real-RustFS claim from this slice.
<!-- /evidence-field -->

