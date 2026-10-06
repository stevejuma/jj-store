---
schema: 1
id: E-M12-004
---

# E-M12-004 details 3

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/executable_native.rs#native_git_fetch_tracks_refs_repeats_without_code_operation_and_survives_cache_loss
- crates/jj-store-cli/src/git_bridge/device/tests.rs#approved_endpoints_reject_credentials_helpers_and_ssh_options
- crates/jj-store-cli/src/git_bridge/device/tests.rs#private_state_rejects_symlinks_and_public_permissions
- crates/jj-store-cli/src/git_bridge/transport.rs#cache_monitor_rejects_excess_bytes_and_symlinks
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

Continue GB3 durable selected push and recovery. M12.3 remains incomplete pending code-origin activation in GB4. Expanded peer races, cancellation, credentials, real RustFS and end-to-end/browser gates remain later acceptance work. UI and unrelated Rust aggregate lanes were out of scope under targeted policy.
<!-- /evidence-field -->

