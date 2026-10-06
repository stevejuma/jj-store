---
schema: 1
id: E-M12-002
---

# E-M12-002 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate && pnpm check:progress && pnpm spec:first && pnpm check:boundaries
```

**Notes**

20 chapters/123 requirements; 94 tasks; protected changes and 15 package boundaries passed.
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
cargo clippy -p jj-store-jj -p jj-store-wal --all-targets --locked -- -D warnings && cargo fmt --all --check
```

**Notes**

Final production and expanded-test targets passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/git_bridge.rs#exact_signed_git_parent_and_native_child_survive_cache_loss_copy_and_cold_recovery
- crates/jj-store-jj/tests/git_bridge.rs#git_tree_modes_gitlinks_merge_order_and_deterministic_export_survive_rebuild
- crates/jj-store-jj/tests/git_bridge.rs#lossy_git_metadata_and_incomplete_object_closure_are_rejected
- crates/jj-store-jj/tests/git_bridge.rs#git_domain_keeps_workstore_origins_frozen_and_native_signatures_unsupported
- crates/jj-store-jj/tests/git_bridge.rs#git_catalog_rejects_changed_mappings_missing_closure_and_cross_repository_snapshots
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

GB2 authorized service/RPC, local bindings/private cache and atomic fetch. GB4 must qualify the reducer before code repositories with origin ledgers activate Git; WorkStore homes remain local. Real RustFS and transport not qualified by GB1.
<!-- /evidence-field -->

