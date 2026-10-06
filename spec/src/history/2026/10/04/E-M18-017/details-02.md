---
schema: 1
id: E-M18-017
---

# E-M18-017 details 2

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
cargo test -p jj-store-server --lib --locked process::tests::capture_factory::startup -- --test-threads=1
```

**Notes**

3 startup tests pass; configured OpenCode preparation does not require endpoint contact. /tmp/jj-cursor-startup-final.log.
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
cargo test -p jj-store-capture --test cursor_transcript --locked -- --test-threads=1 (before/after scanner and nullable-path handoff)
```

**Notes**

Before scanner counted257 candidates for one transcript; corrected bounded project/agent-transcripts scan passes. Before nullable-path handoff missed active receipts; final7 reader/binding tests pass. /tmp/jj-cursor-layout-before.log, /tmp/jj-cursor-layout-after.log, /tmp/jj-cursor-null-handoff-final.log. Terminal-only receipts grant no activity; exact pending start/end witness can transfer within the bounded window.
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
cargo test -p jj-store-capture --test codex_transcript --test codex_recovery --test transcript_sanitization --locked
```

**Notes**

Codex reader10 pass; recovery5 pass/1 explicit ignored; sanitation4 pass/1 explicit ignored, with exact Cursor envelope regression1 pass. Earlier source layout invocation selected only1 Cursor regression; final complete7-test binary supersedes it. /tmp/jj-cursor-file-tests.log.
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
cargo test -p jj-store-cli --test agent_integrations --test agent_setup --locked; affected agent_hook regression
```

**Notes**

Registry4 and setup7 pass, plus hook checks. Transcript setup stores metadata-only handlers, preserves unrelated hooks and supports drift-safe removal. /tmp/jj-cursor-cli-final.log, /tmp/jj-cursor-hooks-check.log.
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
cargo test -p jj-store-server --test capture_resume --test capture_runtime --locked -- --test-threads=1 (before/after)
```

**Notes**

Codex resume5 pass. Runtime before9 pass/1 failed: a later shutdown wait could not report proven lane settlement after timeout. Fixed state transition; complete runtime10 pass. /tmp/jj-cursor-codex-runtime-final.log, /tmp/jj-cursor-runtime-qualified.log. No timeout or acceptance assertion weakened.
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
cargo test -p jj-store-server --test cursor_supervisor --test opencode_discovery --locked; serial discovery rerun
```

**Notes**

Cursor2 pass. Initial parallel OpenCode discovery2 pass/2 timeout failures under load; unchanged serial4 pass. /tmp/jj-cursor-server-runtime-fixed.log, /tmp/jj-opencode-discovery-serial.log.
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
cargo test -p jj-store-server --test cursor_supervisor --test opencode_source --test opencode_lifecycle --test opencode_read_api --locked -- --test-threads=1
```

**Notes**

Final20 pass: Cursor2, OpenCode source6, lifecycle7, API5. One earlier complete Cursor run1 pass/1 publication timeout under load; unchanged final complete binary2 pass. /tmp/jj-multi-harness-runtime-final.log, /tmp/jj-multi-harness-recovery-final.log. Includes exact linked admission, pause/resume, rewrites, final flush, endpoint outages and restart.
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
cargo test -p jj-store-server --test opencode_supervisor --locked (partial-assistant regression before/after); complete supervisor binary --test-threads=1
```

**Notes**

Before regression published incomplete mutable assistant snapshots; after pass. Full4 supervisor tests pass. /tmp/jj-opencode-partial-before.log, /tmp/jj-opencode-partial-after.log, /tmp/jj-opencode-supervisor-full-final.log. Pass stops before incomplete assistant and reacquires after final flush without skipping existing intents.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test capture_supervisor --locked -- --test-threads=1; exact generation rerun
```

**Notes**

Complete Codex survey26 pass/1 generation timeout. Unchanged exact generation test then1 pass/26 filtered. Earlier survey26 pass/1 blocked-directory timeout also retained. /tmp/jj-cursor-remaining-serial-fixed.log, /tmp/jj-codex-generation-focused.log, /tmp/jj-cursor-runtime-qualified.log. These are load-sensitive diagnostics, not a full Codex-suite pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-cursor-opencode-rules cargo test -p jj-store-service --test agent_sessions --locked -- --test-threads=1
```

**Notes**

All9 AgentSession tests pass through the corrected WAL facade; native OpenCode message/session times and tool duration250ms remain exact selected-evidence read projections. Original timing assertion looked at the wrong nested field; corrected owning tool duration path. /tmp/jj-opencode-timing-facade-final.log.
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
cargo test -p jj-store-jj --lib --locked query::language::identity
```

**Notes**

3 exact identity/config-binding tests pass after narrow Clippy conditional modernization. /tmp/jj-cursor-identity-regression.log. Service materialization and server storage conditional cleanups are likewise semantic-preserving.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-cursor-opencode-rules cargo clippy -p jj-agent -p jj-store-capture -p jj-store-wal -p jj-store-service -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Affected packages and targets pass. /tmp/jj-multi-harness-facade-clippy.log. Early shared-target checks linked stale libraries; isolated feature target fixes this. First broad Clippy surfaced existing collapsible-if warnings; narrow conditional modernization passes.
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
pnpm check:boundaries (before/after); pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Before dependency audit rejected service-&gt;jj-agent; removed direct dependency and exposed pure unique native-tool alias lookup through existing WAL facade. After17-package boundary audit passes; architecture923 modules and ownership22 chapters/167 requirements pass. /tmp/jj-multi-harness-boundaries.log and final facade architecture/ownership logs.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-cursor-opencode-rules pnpm contracts:check
```

**Notes**

229 actual HTTP/SSE responses checked;7 contract tests pass. /tmp/jj-cursor-contract-check-cached.log. Generated coverage array bound updated to5; no other wire behavior introduced.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-settings.test.ts; pnpm web:check; pnpm catalog:check; cold multi-harness settings story
```

**Notes**

7 settings units and1 new interaction/axe scenario pass; web/catalog zero errors/warnings. Existing Codex22 settings stories passed in settings parent. /tmp/jj-cursor-settings-unit-final.log, /tmp/jj-cursor-multi-settings-story-final.log, /tmp/jj-cursor-web-check-final.log, /tmp/jj-cursor-catalog-check-final.log. Switch values are numeric render-field indices; corrected new initial string-key story assertion. Visual baseline remains pending.
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
cargo build -p jj-store-cli --bins --locked; pnpm dev:restart with isolated JJ_STORE_DEV_ROOT
```

**Notes**

Development binaries build. First disposable bootstrap refused drifted agent hooks; no reset performed. Recovered same native fixture manually and linked exact workspace through production API. /tmp/jj-multi-harness-bootstrap.log, /tmp/jj-multi-harness-build-final3.log. Rule-cache build date change caused288s rebuild; use isolated cached rules. macOS linker oversized __eh_frame warning retained.
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
Pinned cursor-agent --version / jj agent enable/setup cursor --transcript-capture / disposable Cursor CLI prompt
```

**Notes**

Real approved CLI emitted2 messages with CAPTURE-CURSOR-BOUND-HANDOFF-20261004. AgentStore session36ebc85f272dfa87602c50111e237d86 and Sources match linked repository9e44664671e5d003f6e99ee8a85445b3. Native timestamps absent and remain null. Trace observed sessionStart path null then sessionEnd physical JSONL; bounded exact witness fix enables real capture. Probe exceeded2s once under load; warm pinned retry passed without weakening limit.
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
opencode serve --hostname 127.0.0.1 --port 7486; disposable OpenCode1.18.30 prompt with opencode/big-pickle; production capture/session API reads
```

**Notes**

Real successful native ses_ef96add9cffe6xNRhDllljrsDZ captured2 complete messages, token CAPTURE-OPENCODE-FINAL-FLUSH-20261004; AgentStore7009d66b6ec5dc24a497da24ebaf0e0c, native times1791111799497/1791111799518. Sequence4, records2, receipts4, outstanding0. Earlier MLX endpoint failed runs retained; pre-fix successful diagnostic session retains3 intentionally withheld immutable pending intents. They were not dropped/reset.
<!-- /run -->
<!-- /evidence-field -->

