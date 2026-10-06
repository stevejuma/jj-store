---
schema: 1
id: E-M17-010
---

# E-M17-010 details 2

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
cargo test -p jj-agent --test codex_transcript native_tool_results_enrich_known_calls_without_claiming_success --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-before.log 2>&1
```

**Notes**

Reproduced missing tool-result projection: known shell call stays Pending after a native response.
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
cargo fmt -p jj-agent
```

**Notes**

Initial model formatting before focused test.
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
cargo test -p jj-agent --test codex_transcript native_tool_results_enrich_known_calls_without_claiming_success --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-model.log 2>&1
```

**Notes**

Exact same-batch enrichment passes. An unused import warning was removed before final checks.
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
cargo test -p jj-store-wal --test agent_catalog native_results::result_only_capture_enriches_selected_call_and_preserves_old_root --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-selected-before.log 2>&1
```

**Notes**

Reproduced result-only gap: selected session retains one evidence batch instead of two.
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
cargo fmt -p jj-agent -p jj-store-wal
```

**Notes**

Formatted shared selected-root resolver and model boundary.
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
cargo test -p jj-store-wal --test agent_catalog native_results::result_only_capture_enriches_selected_call_and_preserves_old_root --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-selected.log 2>&1
```

**Notes**

Exact result-only regression passes with stable selected session/tool and prior root reads.
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
cargo test -p jj-agent --locked > /tmp/jj-store-agent-capture-m172-results-model-all.log 2>&1
```

**Notes**

All 34 model tests pass including malformed results, oversized aliases and identity mismatch.
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
cargo test -p jj-store-wal --test agent_catalog --locked > /tmp/jj-store-agent-capture-m172-results-selected-all.log 2>&1
```

**Notes**

All 13 directly affected shared/native projection tests pass before adding recovery/staged-invisibility refinements.
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
cargo test -p jj-store-service --test local_agent native_results::local_native_result_enrichment_retains_selected_calls_receipts_and_pins_across_reopen --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-local.log 2>&1
```

**Notes**

Local Git companion result-only acceptance passes with reopen, original receipt replay and old pinned reads.
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
cargo fmt -p jj-agent -p jj-store-wal -p jj-store-service
```

**Notes**

Final affected package formatting; Codex identity helpers extracted below the production module budget without changing derivation.
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
cargo test -p jj-agent --locked > /tmp/jj-store-agent-capture-m172-results-model-final.log 2>&1
```

**Notes**

All 34 owned model tests pass after identity helper extraction.
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
cargo test -p jj-agent --test codex_transcript native_tool_results_enrich_known_calls_without_claiming_success --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-model-exact-final.log 2>&1
```

**Notes**

Final exact regression passes for both CLI 0.156.1 and Desktop 0.159.2 native envelopes without success inference.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test agent_catalog native_results::result_only_capture_enriches_selected_call_and_preserves_old_root --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-selected-exact-final.log 2>&1
```

**Notes**

Final exact native result acceptance passes including fresh Coordinator recovery over the same MemoryStore.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-results-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 canonical requirements pass.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-results-boundaries.log 2>&1
```

**Notes**

All 17 package ownership boundaries pass.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-results-format.log 2>&1
```

**Notes**

Pinned Rust formatting passes.
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
cargo test -p jj-store-service --test local_agent --test local_agent_binding --test local_agent_pages --locked > /tmp/jj-store-agent-capture-m172-results-local-all.log 2>&1
```

**Notes**

All 18 direct local capture/binding/paging tests pass; real disposable Git companion reopen, pinned history and original receipts retained.
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
cargo test -p jj-store-wal --test agent_catalog --locked > /tmp/jj-store-agent-capture-m172-results-selected-final.log 2>&1
```

**Notes**

All 13 final native/shared capture projection tests pass, including staged-call invisibility and fresh Coordinator recovery.
<!-- /run -->
<!-- /evidence-field -->

