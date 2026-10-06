---
schema: 1
id: E-M15-010
---

# E-M15-010 details 2

<!-- evidence-field:remaining list -->
## Remaining

- HTTP exposure/generated browser contracts, optional atomic capture binding,
  reducer 3, canonical cross-source timeline/search, released shared UI and
  jj-acp consumer qualification remain pending. No ACP runtime linkage or runner
  invocation is introduced.
- Native code writes still reject until reducer 3 is qualified. Acceptance uses
  WorkStore homes and separately proves code rejection. Fresh CLI processes and
  reconstructed RPC transports over MemoryStore establish cold CLI/receipt
  behavior, not server-process restart or real RustFS evidence.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Expose bounded authenticated HTTP surfaces and generated browser contracts, preserving reducer-3 and runner qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_collab_cli.rs#cold_native_cli_replays_receipts_pins_pages_and_preserves_code_without_agent_setup
<!-- /evidence-field -->

