---
schema: 1
id: E-M9-023
---

# E-M9-023

<!-- evidence-field:id markdown -->
## Id

E-M9-023
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.2
- M9.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

milestone-closeout
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_publication.rs#captured_entries_publish_in_source_sequence_order
- crates/jj-store-cli/tests/agent_setup.rs#removal_restores_exact_original_and_disables_integration
- crates/jj-store-rpc/tests/agent_publication.rs#local_peer_prepares_and_replays_exact_agent_publication
- crates/jj-review/tests/resolver.rs#exact_identity_and_explicit_insertion_map_without_retargeting_the_anchor
- crates/jj-store-wal/tests/review_commands.rs#create_thread_is_atomic_replayable_and_preserves_sibling_roots
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

The closeout audit required named tests, at least one successful run and evidence before M9.4 can enter in-progress state. M9.4 remains planned until its first implementation slice.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first; pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

All 61 tasks, 89 requirements and the protected milestone chapter validate; M9.2 and M9.3 close with named prior acceptance while M9.4 remains planned for its first implementation evidence.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; ReviewStore schema v1; capture spool format v1; native RPC package jj_store.native.v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence or protocol format changes. M9.2 closes on E-M9-002 through E-M9-022; M9.3 closes on E-M9-010 through E-M9-014 after its M9.2 dependency. LocalTrusted APIs, shared UI and three-harness/cold-copy qualification remain owned by M9.4, M9.5 and M9.6.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


