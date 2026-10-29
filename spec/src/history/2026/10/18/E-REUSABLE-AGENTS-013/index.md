---
schema: 1
id: E-REUSABLE-AGENTS-013
---

# E-REUSABLE-AGENTS-013

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.3
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-DOC-001
- JST-DOC-002
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-conversations/Cargo.toml
- crates/jj-store-conversations/README.md
- crates/jj-store-conversations/src/lib.rs
- crates/jj-store-conversations/src/capture.rs
- crates/jj-store-conversations/tests/capture.rs
- docs/site/technical/conversations.md
- docs/maintenance/coverage.json
- docs/maintenance/reusable-agent-workflows-progress.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-013/index.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-013/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-conversations/tests/capture.rs#bootstrap_facts_and_events_retain_native_binding_and_exact_capture_receipts
- crates/jj-store-conversations/tests/capture.rs#deferred_prefix_and_terminal_projection_are_one_sanitized_durable_batch
- crates/jj-store-conversations/tests/capture.rs#capture_must_finish_before_safe_output_is_returned
- crates/jj-store-conversations/tests/capture.rs#refused_unknown_and_mismatched_receipts_never_release_output_or_retry
- crates/jj-store-conversations/tests/capture.rs#dropped_capture_wait_fences_reuse_without_replaying_an_uncertain_write
- crates/jj-store-conversations/tests/capture.rs#scanner_failure_and_foreign_or_noncontiguous_inputs_never_reach_storage
- crates/jj-store-conversations/tests/capture.rs#joined_projections_are_rescanned_before_any_part_of_the_batch_is_persisted
- crates/jj-store-conversations/tests/capture.rs#facts_consume_an_ordinal_without_advancing_the_native_event_cursor
- crates/jj-store-conversations/tests/capture.rs#input_and_post_scan_batch_byte_limits_fail_before_storage
- crates/jj-store-conversations/tests/capture.rs#content_identity_is_canonical_and_binds_the_host_scope_policy_and_progress
<!-- /evidence-field -->

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
pnpm spec:search JST-ARCH-007
```

**Notes**

Optional QMD index refresh fails because its native module does not match active Node ABI 137. Used the reported rg fallback and opened architecture/03-requirements.md plus the owning reusable-workflow chapter. No cache reinstall or unrelated source changes.
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
rg -n '^\| JST-ARCH-007' spec/src/current/contracts/architecture --glob '*.md'
```

**Notes**

Found and read the authoritative JST-ARCH-007 requirement in architecture/03-requirements.md. Source contract updates precede implementation.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- test -p jj-store-conversations --test capture --locked
```

**Notes**

Completed exact before target fails E0432 because the capture module does not exist. This is absent-capability evidence, not a regression in existing code. Log native-capture-before.log completes before production edits.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- check -p jj-store-conversations --all-targets
```

**Notes**

Initial public target check passes. The unlocked command records only the existing sha2 0.10.9 dependency in this package lock entry; all subsequent source verification is locked.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- test -p jj-store-conversations --test capture --locked
```

**Notes**

Initial six cases pass; expanded ten-case focused target passes. Tests cover exact native binding, sanitized bootstrap facts/events, deferred prefix batching, rescan after accumulation, blocked persistence, rejected/unknown/mismatched receipts, abandoned waits, foreign/noncontiguous/scalar inputs and byte bounds. Logs native-capture-focused.log and native-capture-focused-final.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- test -p jj-store-conversations --locked && CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- test -p jj-store-conversations --test capture --features serde_json/preserve_order --locked && CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- clippy -p jj-store-conversations --all-targets --locked -- -D warnings && CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- build -p jj-store-conversations --locked
```

**Notes**

Complete crate: 20 pass, zero failures/ignored. The ordered-JSON profile independently passes all ten capture cases, including recursively reordered objects and every native scope identity. Strict Clippy/all-target checks and crate build pass. Final source/tests are 208/448 physical lines, within their budgets. Actual source commands also run package fmt. Logs native-capture-crate{,-final}.log, native-capture-ordered-json{,-final}.log, native-capture-clippy{,-final}.log and native-capture-build{,-final}.log. Source-only Rust impact: no root product aggregate or UI suite.
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
pnpm check:boundaries
```

**Notes**

All 25 package boundaries pass. The capture module imports no runtime, Tauri, repository provider, service, WAL or presentation package.
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
node scripts/docs.mjs review conversations 'Reviewed native capture binding, canonical bounded batches, injected privacy and exact durable receipt fences, stopped uncertain owners and complete ordered batch delivery; Notes native storage and transport adoption remain separately qualified.'
```

**Notes**

Updated the conversation-source fingerprint after final source formatting. Short crate usage and the full manual describe ownership, exact receipt semantics, bounds, uncertainty and remaining native adoption.
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
pnpm docs:check
```

**Notes**

Generated CLI/default/API references, source reviews, nine offline Mermaid diagrams in both themes, strict MkDocs build and manifest closure pass. The command owns its normal Rust docs_reference extraction; it is not a full product test or runtime-restart gate. Native capture source remained separate from Notes adoption.
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
pnpm evidence:new -- --id E-REUSABLE-AGENTS-013 --task M33 --task M33.3 --task M33.4 --requirement JST-ARCH-007 --requirement JST-DOC-001 --requirement JST-GOV-002 --requirement JST-GOV-011
```

**Notes**

Created the new record, then restored 531 unrelated generated projections while preserving the active source slice. The populated record additionally includes JST-DOC-002 for the actual source-review update.
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
pnpm check:progress && pnpm spec:validate && pnpm spec:first
```

**Notes**

Progress passes (171 tasks/881 records), and specification validation passes (seven validators/4,235 chapters/207 requirements). Spec-first rejects the missing owning documentation chapter for this changed-contract feature. Added its explicit native capture documentation boundary and retained this initial failure; no checker or contract was weakened.
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
pnpm check:progress && pnpm spec:validate && pnpm spec:first
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 pnpm dev:cargo -- fmt -p jj-store-conversations -- --check
pnpm exec prettier --check crates/jj-store-conversations/README.md docs/site/technical/conversations.md docs/maintenance/coverage.json docs/maintenance/reusable-agent-workflows-progress.md spec/src/current/contracts/architecture/26-reusable-agent-workflows.md spec/src/current/contracts/documentation/03-delivery-and-validation.md spec/src/current/progress/M33/index.md spec/src/current/progress/M33/history/M33/index.md spec/src/current/progress/M33/history/M33.3/index.md spec/src/current/progress/M33/history/M33.4/index.md spec/src/history/2026/10/18/E-REUSABLE-AGENTS-013/index.md spec/src/history/2026/10/18/E-REUSABLE-AGENTS-013/SUMMARY.nav.md spec/src/history/2026/10/18/SUMMARY.nav.md spec/src/history/2026/10/18/index.md
```

**Notes**

Final checks pass: 171 tasks/881 evidence records; seven validators, 4,235 chapters and 207 requirements; spec-first nine protected files/two canonical chapters. Scoped Rust formatting and all selected Markdown/JSON formatting pass. NOTES-06C1 is complete only within its library scope; NOTES-06C remains the single Active task. Aggregate product and downstream-native gates are out of scope.
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

Enclosing Jujutsu change mnpxkyrs, based on fe4d2f4b. Rust 1.99.0, pnpm 10.34.5, serde_json 1.0.151 and sha2 0.10.9. Native factory remains independently qualified in Lapis AI 87958267 and desktop 0535bb1e; this library is not yet adopted by that native consumer.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New host-private capture schema 1: recursively sorted JSON objects; opaque binding, policy, native ordinal, facts/event cursor and sanitized ordered frames. Domain-separated SHA-256 uses jj-store/native-capture/v1 followed by a NUL byte. Both input and candidate output are bounded to 2 MiB during serialization. This is a private sanitized capture candidate, not a generic native object codec, repository publication, or an AgentStore/ThreadStore record. Existing stored formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

NOTES-06C1 supplies the reusable capture-to-durable-sink boundary required by NOTES-06C. The sink must confirm actual durable bytes/progress, deduplicate across owners and retain unknown intent. Returned hashes alone prove no physical persistence. Prefixes are memory-only; merged projections are rescanned. All safe frames in a committed batch must be delivered in order. No native runtime processor adapter, filesystem journal, JJ publication, power-loss recovery or Notes adoption is claimed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Connect admitted native privacy and a receipt-aware durable sink in Notes, and adapt native delivery to carry every frame in an accepted batch (including deferred prefix frames). Then qualify managed native capture, AgentStore/ThreadStore backend selection and cold conversation/unknown-write recovery. NOTES-06C remains the sole Active task.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-ARCH-007 gains the specified native capture batching/receipt
  boundary in architecture/26-reusable-agent-workflows.md. JST-DOC-001/002 and
  JST-GOV-002/011 remain unchanged: short crate usage, full manual, source
  review and evidence accompany the source change. Runtime/transport authority
  and native storage formats are unchanged."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M32 remains Done; M33/M33.1-M33.4 In progress; M33.5-M33.8 Planned.
  NOTES-06C1 completes only the shared library prerequisite. NOTES-06C stays
  Active and Notes privacy/storage, batching adapter, managed native acceptance
  and 07/08/09 remain open. Fixture sinks are neither actual durable JJ receipts
  nor native downstream acceptance. All previously recorded
  platform/provider/visual/release gaps remain.

```
<!-- /evidence-field -->

