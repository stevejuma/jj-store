---
schema: 1
id: E-M17-009
---

# E-M17-009 details 2

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
cargo test -p jj-agent --test codex_transcript native_codex_transcripts_project_messages_and_lifecycle_without_inventing_execution --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-normalize-before.log 2>&1
```

**Notes**

Before native parser: no native messages or lifecycle records projected; exact test failed once.
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
cargo test -p jj-agent --test codex_transcript native_codex_transcripts_project_messages_and_lifecycle_without_inventing_execution --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-normalize.log 2>&1
```

**Notes**

Initial exact native parser regression passes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-capture --test transcript_sanitization transcript_sanitization_precedes_durability_and_covers_path_keys_and_values --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-privacy-before.log 2>&1
```

**Notes**

Cold rules compile completed. Test scaffold inspected outer encoded byte arrays rather than decoded payloads, so tag assertion was invalid; corrected test inspection.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test transcript_sanitization transcript_sanitization_precedes_durability_and_covers_path_keys_and_values --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-privacy-keys-before.log 2>&1
```

**Notes**

Corrected decoded-payload regression reproduced a private home path surviving in a JSON key.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test transcript_sanitization transcript_sanitization_precedes_durability_and_covers_path_keys_and_values --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-privacy-keys-final.log 2>&1
```

**Notes**

Exact key/value privacy regression passes after key redaction and fail-closed collision handling.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --test transcript_sanitization --locked > /tmp/jj-store-agent-capture-m172-codex-sanitization.log 2>&1
```

**Notes**

Three sanitizer tests pass; ignored cold-child fixture is explicitly executed by parent in a fresh process using the private warmed rule cache.
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
cargo test -p jj-agent --locked > /tmp/jj-store-agent-capture-m172-codex-model.log 2>&1
```

**Notes**

Owned model suite passes before missing-ID refinement.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test agent_catalog native_desktop_messages_publish_sanitized_evidence_without_asserting_execution --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-projection.log 2>&1
```

**Notes**

Test expected Unknown for a message-only session; existing contract correctly selects Discovered. Corrected scaffold without changing product state semantics.
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
cargo test -p jj-store-wal --test agent_catalog native_desktop_messages_publish_sanitized_evidence_without_asserting_execution --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-projection-final.log 2>&1
```

**Notes**

Exact Desktop projection verifies retained evidence/text, old-root pinning and Discovered state with no tool/execution inference.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-agent --test codex_transcript missing_message_ids_cannot_collide_with_real_native_aliases --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-message-id-before.log 2>&1
```

**Notes**

New test referenced kind rather than native_kind; fixed test scaffold field.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-agent --test codex_transcript missing_message_ids_cannot_collide_with_real_native_aliases --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-message-id-regression-before.log 2>&1
```

**Notes**

Corrected test proves a missing-ID fallback collides with a real native message ID.
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
cargo test -p jj-agent --test codex_transcript missing_message_ids_cannot_collide_with_real_native_aliases --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-message-id-final.log 2>&1
```

**Notes**

Exact identity regression passes after separate source-message namespace and withholding fictional native aliases.
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
cargo test -p jj-agent --locked > /tmp/jj-store-agent-capture-m172-codex-model-final.log 2>&1
```

**Notes**

All 30 owned model tests pass after identity refinement.
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
cargo test -p jj-agent --test codex_transcript native_codex_transcripts_project_messages_and_lifecycle_without_inventing_execution --locked -- --exact > /tmp/jj-store-agent-capture-m172-codex-normalize-exact-final.log 2>&1
```

**Notes**

Final exact initial parser regression passes.
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
cargo test -p jj-agent --test codex_transcript --locked > /tmp/jj-store-agent-capture-m172-codex-model-native-final.log 2>&1
```

**Notes**

All six final native parser tests pass, including added legacy hook metadata compatibility regression.
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
cargo test -p jj-store-wal --test agent_catalog --locked > /tmp/jj-store-agent-capture-m172-codex-projection-all.log 2>&1
```

**Notes**

All 10 directly affected shared/native projection tests pass, retaining diagnostic-only publication, cumulative tools and bounded transcript append.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --lib --test reader_progress --test codex_transcript --test transcript_sanitization --locked > /tmp/jj-store-agent-capture-m172-codex-sanitized-runtime-final.log 2>&1
```

**Notes**

22 runtime tests plus explicitly launched fresh-process native sanitizer child pass. One fixture helper is ignored only in parent enumeration.
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
cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --test agent_hook --test agent_kingfisher --locked > /tmp/jj-store-agent-capture-m172-codex-hook-consumers.log 2>&1
```

**Notes**

27 direct CLI tests plus cached-scan fresh-process child pass. Two drainer child fixture tests ignored in parent. Actual cold hook cache build, executable restart and shared redaction regressions pass.
<!-- /run -->
<!-- /evidence-field -->

