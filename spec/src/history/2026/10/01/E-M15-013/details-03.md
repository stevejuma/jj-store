---
schema: 1
id: E-M15-013
---

# E-M15-013 details 3

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
cargo fmt -p jj-store-service -p jj-store-rpc -p jj-store-cli -- --check
```

**Notes**

Focused Rust formatting passes.
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
pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md && pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Canonical chapters and ledger formatted; focused formatting passes.
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
CARGO_INCREMENTAL=0 cargo clippy -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Strict affected service/RPC/CLI lint passes; /tmp/jj-bound-transport-clippy.log.
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
CARGO_INCREMENTAL=0 cargo check -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked (fixed)
```

**Notes**

Original affected all-target check rerun after forwarding wrapper repair; /tmp/jj-bound-transport-check-fixed.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_capture_publication.rs#bound_capture_journals_before_prepare_and_retries_exact_assignment_after_restart
- crates/jj-store-cli/tests/agent_capture_spool.rs#prepared_thread_binding_survives_restart_and_invalid_binding_is_not_persisted
- crates/jj-store-rpc/tests/collab.rs#explicit_capture_binding_survives_preparation_publication_and_transport_reconstruction
- crates/jj-store-rpc/src/agent/binding_tests.rs#delta_codec_retains_explicit_binding_and_unlinked_prepare_rejects_it
<!-- /evidence-field -->

<!-- evidence-field:escalation markdown -->
## Escalation

Shared native preparation/codec and capture spool touch the service facade, native RPC and both CLI capture providers. Before widening, select the complete RPC package suite, direct Agent/collaboration service consumers, all relevant spool/drain/hook/setup/local recovery CLI consumers, existing unlinked HTTP capture consumer, strict affected package lint and protobuf generation through package compilation. HTTP/OpenAPI schemas, UI and full Rust workspace/root aggregate lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native binding APIs and worker are explicit; ordinary hook/watch callers still
  enqueue unlinked captures. CLI/runner entry points and local bound capture
  remain pending.
- Reducer 3, canonical cross-source timeline/search, released Community channel
  UI template, native real-RustFS and cold process/browser full flow remain
  pending. RPC transport reconstruction is not persistent service restart.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement local atomic Agent/Thread bound capture and retain the complete new session closure through companion metadata ref publication.
<!-- /evidence-field -->

