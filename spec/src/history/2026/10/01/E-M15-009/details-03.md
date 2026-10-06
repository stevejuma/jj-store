---
schema: 1
id: E-M15-009
---

# E-M15-009 details 3

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
pnpm exec prettier --check spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Focused document formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Full RPC package suite is required because the shared NativeStore protobuf service, common server construction and typed client are changed. Existing RPC routes and direct service/CLI consumers are the complete impact surface; unrelated Rust/UI/root aggregate lanes remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- HTTP exposure/generated browser contracts, native CLI executable acceptance,
  optional capture binding, reducer 3, canonical cross-source timeline/search,
  released shared UI and jj-acp integration remain pending.
- Native code writes stay gated on reducer 3; current transport mutation
  acceptance uses WorkStore homes. Reconstructing a gRPC server over the same
  MemoryStore is transport receipt/cursor evidence, not process-restart,
  persistent RustFS, cold browser or real runner qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Expose HTTP and native CLI with focused cold executable/generated contract acceptance, preserving reducer-3 and runner gates.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-rpc/tests/collab.rs#unix_rpc_keeps_exact_feedback_assets_pages_and_receipts_across_transport_reconstruction
- crates/jj-store-rpc/tests/collab.rs#raw_rpc_refuses_actor_injection_ambiguous_rasters_and_limits_without_publication
- crates/jj-store-rpc/src/collab/codec_tests.rs#binary_response_checks_exact_identity_metadata_and_single_byte_carrier
<!-- /evidence-field -->

