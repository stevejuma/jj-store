---
schema: 1
id: E-M17-010
---

# E-M17-010 details 3

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
cargo check -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-results-check.log 2>&1
```

**Notes**

Affected models/projector/providers/runtime/server/CLI checks pass without changing wire contracts.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-results-cli.log 2>&1
```

**Notes**

All 21 directly affected native/local spool/publication/recovery tests pass. Two child drainer fixture entrypoints are excluded from parent enumeration.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test transcript_sanitization --locked > /tmp/jj-store-agent-capture-m172-results-runtime.log 2>&1
```

**Notes**

Three native sanitizer/normalizer acceptance tests plus explicitly launched fresh-process cached-scanner child pass; the child is ignored only in parent enumeration.
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
cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-capture --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-results-clippy.log 2>&1
```

**Notes**

Strict all-target lint exposed inherited items_after_test_module in content_search/cursors.rs. Parent bytes were confirmed identical before the mechanical relocation; no lint was waived.
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
cargo fmt -p jj-store-service
```

**Notes**

Moved the unchanged cursor test module after production items and formatted; production/test statements remain identical.
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
cargo test -p jj-store-service --lib content_search::cursors::tests::capacity_restart_and_query_deadline_fail_closed_without_eviction --locked -- --exact > /tmp/jj-store-agent-capture-m172-results-cursor-test.log 2>&1
```

**Notes**

Exact moved cursor regression passes, preserving capacity, restart and expiration behavior without eviction.
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
cargo clippy -p jj-agent -p jj-store-wal -p jj-store-service -p jj-store-capture --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-results-clippy-final.log 2>&1
```

**Notes**

Original strict all-target lint command passes after mechanical inherited-layout repair.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/compatibility.md spec/src/milestones.md
```

**Notes**

Formatted the canonical chapters and ledger; prior evidence remains structurally identical.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-results-progress-initial.log 2>&1
```

**Notes**

129 tasks pass with all 264 parent evidence entries retained and final scope gates explicit.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final ledger formatting after recording direct consumer results and the repaired inherited lint failure.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-results-format-final.log 2>&1
```

**Notes**

Final pinned Rust formatting passes including cursor-module relocation.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/local-jj.md spec/src/mutations.md spec/src/compatibility.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-results-doc-format.log 2>&1
```

**Notes**

All changed canonical chapters and the final evidence ledger pass Prettier.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-results-progress-final.log 2>&1
```

**Notes**

Final pass: 129 tasks with M17.2/aggregate still In progress, 265 evidence entries and complete parent preservation.
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
pnpm check:progress
```

**Notes**

Same-change validation after final command evidence is recorded.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Final JSON evidence serialization expanded short arrays; the final ledger required another Prettier pass. Product validation remains passing.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Corrected final JSON evidence layout without changing prior evidence values.
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
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Corrected final ledger formatting passes.
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

Rust 1.98.1; JJ 0.45.1; existing Codex CLI 0.156.1 and Desktop 0.159.2 native envelope profiles; synthetic sanitized fixtures. Native recovery uses MemoryStore; local recovery uses disposable real Git companion objects.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Transient typed ToolResultObservation and normalization candidates have no persistence encoding. Function/custom-tool outputs resolve exact source/session/tool aliases; receiving a response is Unknown and preserves existing terminal evidence. Existing native IDs, hook bytes, object kinds, event variants, manifest/companion formats, prepared requests, receipts and wire schemas remain unchanged.
<!-- /evidence-field -->

