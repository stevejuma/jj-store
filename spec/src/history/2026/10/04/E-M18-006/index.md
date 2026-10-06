---
schema: 1
id: E-M18-006
---

# E-M18-006

<!-- evidence-field:id markdown -->
## Id

E-M18-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.4
- M9.5
- M18.8
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-002
- JST-COLLAB-004
- JST-COLLAB-005
- JST-API-001
- JST-MUT-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Recover recorded Codex session timing
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change slzksssp
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean main above ylxwsusl/edab2326. Current preview7470 Agents list has6 rows;3 captured Codex sessions have null first/last timing, while3 seeded hook sessions have values. JSONL sanitization intentionally preserves native timestamps inside payloads and leaves occurred_at_millis absent; the shared bounded timing projection reads only that absent field. Exact regression requires native CLI/Desktop list/detail timestamps from selected sanitized evidence without reimport or root publication.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; locked Chrono0.4.45 reused as a service dependency with std and no default features. Existing product/normalizer profiles unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public DTO/schema, canonical encoding, native/local metadata, evidence, receipt, cursor, capture parser/recovery or source identity changes. Existing event-ID matching,4,096-event timing bound and optional millisecond fields retained. Qualified sanitized Codex JSONL record timestamps decode as RFC3339; explicit observation timestamps win; missing/malformed/unsupported evidence stays unknown.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/collaboration/agent_session_timing/tests.rs
- crates/jj-store-service/src/collaboration/agent_session_timing.rs
- crates/jj-store-service/tests/agent_sessions/timing.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-service/tests/local_agent/timing.rs
- crates/jj-store-service/tests/local_agent.rs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/mutations.md
- spec/src/milestones.md
- spec/src/local-jj.md
<!-- /evidence-field -->

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
jj --no-pager st; inspect canonical chapters/ledger, current Cua page and Community/Design Core timing references; pnpm exec tsx .local/agent-session-timing/live.ts before
```

**Notes**

Observed exactly3 missing captured Codex timing pairs. API proof stored in .local/agent-session-timing/before.json; no raw transcript or credential output.
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
cargo test -p jj-store-service --test agent_sessions codex_transcript_timing_recovers_existing_sanitized_native_timestamps --locked (first)
```

**Notes**

Test child module needed an explicit path attribute; fixed test registration.
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
cargo test -p jj-store-service --test agent_sessions codex_transcript_timing_recovers_existing_sanitized_native_timestamps --locked (before)
```

**Notes**

Exact regression fails with first activity None instead of1704164645123; published sanitized record retains timestamp but optional observation field is empty.
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
cargo check -p jj-store-service; cargo fmt --all; cargo check -p jj-store-service --locked
```

**Notes**

Service and direct owned dependencies compile; lock adds only the existing Chrono dependency edge.
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
cargo test -p jj-store-service --test agent_sessions codex_transcript_timing_recovers_existing_sanitized_native_timestamps --locked (after)
```

**Notes**

Exact regression passes for both CLI0.156.1 and Desktop0.159.2. Millisecond fractions, UTC offsets, out-of-order matched records and ignored unsupported records qualified; list/detail and manifest pin unchanged.
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
cargo test -p jj-store-service --lib --test agent_sessions --test local_agent timing --locked
```

**Notes**

4 tests pass:2 timestamp guards,1 native CLI/Desktop list/detail regression,1 local companion pin regression. Unknown/malformed timestamps, wrong envelope profile/source/integration and safe omissions remain absent; explicit timestamp precedence retained; historical local pin retains old timing after later capture.
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
cargo test -p jj-store-service --test agent_sessions session_detail --locked; cargo test -p jj-store-service --test local_agent browser_agent_presentation_pins_metadata_and_transcript_membership --locked
```

**Notes**

3 native existing detail/timing/bounded-activity regressions and1 local existing browser presentation/membership regression pass.
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
pnpm exec prettier --write spec/src/api.md spec/src/collaboration.md spec/src/mutations.md spec/src/architecture.md; pnpm exec prettier --check spec/src/api.md spec/src/collaboration.md; pnpm check:architecture; pnpm check:boundaries
```

**Notes**

Canonical formatting and dependency boundaries17 packages pass. Supplementary architecture905 modules unchanged. No UI component changes; UI type/build/Storybook and contract generation lanes are out of scope. Root aggregates/full Rust workspace not run.
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
pnpm dev:backend:build; pnpm format:rust:check
```

**Notes**

Development CLI/server and direct service consumers build with the locked dependency edge. Existing macOS compact-unwind linker diagnostic is informational; installed executables untouched. Rust formatting passes.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:backend:restart (preview cwd)
```

**Notes**

Only selected preview backend replaced; Vite and fixture retained, Search ready. Main7460/7462 runtime not restarted.
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
pnpm exec tsx .local/agent-session-timing/live.ts after; Python before/after assertions; Cua fresh background Agents tab, AX and screenshot
```

**Notes**

All6 list rows now show captured start/duration. The3 recovered Codex durations are17m26s,34s,3s. Native manifest/AgentStore snapshot, session IDs, states and evidence/event counts remain identical; previous3 hook timing pairs unchanged. Cold fresh page uses retained historical code operation. Screenshot .local/agent-session-timing/after.png. Temporary proof tab closed; existing review page retained.
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
pnpm exec prettier --write spec/src/local-jj.md .local/agent-session-timing/evidence.json; pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check changed canonical chapters and new evidence
```

**Notes**

Spec-first8 protected files/6 chapters;5 validators/22 chapters/167 requirements;136 progress tasks. Milestones retain existing status; focused formatting passes. Historical ledger prefix preserved.
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

<!-- evidence-field:scope markdown -->
## Scope

Timing read projection and current preview verification only. Full harness release, capture runtime recovery, native desktop packaging and visual baselines remain separate. No user repository reimport, evidence rewrite, reset, installed-tool change or unrelated service restart.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use current Agents list; captured start/duration derive from exact selected recorded activity, including previously published Codex records. Preserve deferred harness and broader qualification scope.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


