---
schema: 1
id: E-M17-004
---

# E-M17-004 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent_binding bound_diagnostic_capture_retains_thread_intent_without_inventing_session --locked -- --exact
```

**Notes**

Before implementation: bound diagnostic-only capture fails with Corrupt and cannot retain binding or source progress.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent_binding bound_diagnostic_capture_retains_thread_intent_without_inventing_session --locked -- --exact
```

**Notes**

Initial implementation compiler error: one dependency match branch returned a set where the existing contract requires a vector. Corrected.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-service --test local_agent_binding bound_diagnostic_capture_retains_thread_intent_without_inventing_session --locked -- --exact
```

**Notes**

Concurrent main-workspace compilation replaced shared target artifacts with older model/WAL metadata. Not product regression evidence. Subsequent commands use this workspace isolated target.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cp -cR /Users/stevejuma/code/lapismd/jj-store/target /Users/stevejuma/code/lapismd/jj-store-agent-capture/target
```

**Notes**

APFS clone retained dependencies but concurrent ephemeral RPC object files disappeared during copy. All 16 cloned workspace packages were then cleaned before rebuilding; no shared target or other workspace source changed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo clean -p jj-collab -p jj-agent -p jj-store-model -p jj-review -p jj-work -p jj-store-jj -p jj-store-object -p jj-store-wal -p jj-store-cli -p jj-store-replication -p jj-store-rpc -p jj-store-service -p jj-store-adapters -p jj-store-server -p jj-store-http -p jj-store-tantivy
```

**Notes**

Cleaned isolated workspace package outputs after dependency clone; removed 28070 files, 33.5 GiB logical bytes.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test local_agent_binding bound_diagnostic_capture_retains_thread_intent_without_inventing_session --locked -- --exact
```

**Notes**

Initial isolated provider regression passes: one atomic gap link, no synthetic session/assignment, reopening and later valid bound capture.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test bound_capture bound_diagnostic_gap_selects_atomic_evidence_and_preserves_unknown_restart_receipt --locked -- --exact
```

**Notes**

Unknown outcome and cold receipt replay pass with exact bound diagnostic evidence; code heads and session/assignment catalogs are unchanged.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test bound_capture stale_denied_or_misleading_bound_gap_selects_neither_root --locked -- --exact
```

**Notes**

Receipt/evidence mismatch, denied Thread authorization and stale Thread version each select neither root.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:generate
```

**Notes**

Regenerated OpenAPI and TypeScript for additive capture_gap Thread target; no new route or capture wire framing.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all
```

**Notes**

Rust source/test formatting.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/collaboration.md spec/src/jj-integration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/api.md spec/src/compatibility.md spec/src/ui.md web/src/lib/ui/store-threads/thread-context.ts web/tests/vite/thread-context.test.ts
```

**Notes**

Changed canonical chapters and presentation helper/test formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test local_agent_binding bound_diagnostic_capture_retains_thread_intent_without_inventing_session --locked -- --exact
```

**Notes**

Final source/projection continuity and archived Thread qualification passes after Git gc and reopening; later valid bound session captures proceed.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/thread-context.test.ts
```

**Notes**

Test fixture expected null Thread ID although route codec correctly omits absent optional fields. Corrected to assert nullish/absent; no product route change needed.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-collab -p jj-store-wal --locked > /tmp/jj-store-agent-capture-m171-wal.log 2>&1
```

**Notes**

121 tests passed, zero failed/ignored. Complete relevant domain/WAL suite justified by shared graph closures and publication command authority changes. MemoryStore/storage-contract tests are not real RustFS qualification.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/thread-context.test.ts web/tests/vite/production-threads.test.ts > /tmp/jj-store-agent-capture-m171-ui.log 2>&1
```

**Notes**

24 tests passed: explicit gap directory route and direct Thread-controller compatibility.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check > /tmp/jj-store-agent-capture-m171-web-check.log 2>&1
```

**Notes**

Svelte type check: zero errors and warnings with regenerated gap union.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```

**Notes**

Owned UI source boundary passed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-rpc --test collab diagnostic_gap_binding_survives_preparation_publication_and_transport_reconstruction --locked -- --exact > /tmp/jj-store-agent-capture-m171-rpc-exact.log 2>&1
```

**Notes**

Real isolated Unix gRPC preparation, publication, exact gap-link history and receipt replay across transport reconstruction passed; MemoryStore authority, not persistent RustFS.
<!-- /run -->
<!-- /evidence-field -->

