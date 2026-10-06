---
schema: 1
id: E-M17-041
---

# E-M17-041 details 3

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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-storage-spec-first.log 2>&1
```

**Notes**

Eleven protected paths and three owning canonical chapters pass; final sealing rerun remains.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-storage-progress.log 2>&1
```

**Notes**

All 129 tasks validate with prior 295 evidence entries preserved, total 296. M17/M17.2 remain In progress; final sealing rerun remains.
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
cargo test -p jj-store-cli --test server_executable --locked > /tmp/jj-store-agent-capture-m172-storage-cli-cold.log 2>&1
```

**Notes**

All 15 ordinary executable cases pass from rebuilt development binaries, including cold capture config/private-state guard, TLS, foreground signals, state lock release and search shutdown. Three real RustFS/deployment gates are ignored and not counted.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-storage-lint-sealed.log 2>&1
```

**Notes**

Final owning/server and direct CLI targets compile and pass strict lint with RetainedWork ordering and all final tests.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-storage-doc-seal.log 2>&1
```

**Notes**

Final chapters and ledger formatting passes; repeated after recording final evidence.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

Inspection-only command rejected: this JJ version has no --check option. Used supported status/stat/diff inspection and the passing Rust/Markdown formatters; not a product failure.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-storage-check.log 2>&1
```

**Notes**

Owning server and direct CLI all-target package checks pass.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-storage-spec-validate-sealed.log 2>&1
```

**Notes**

Final five validators, 21 chapters and 160 requirements pass. Repeated after evidence sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-storage-spec-first-sealed.log 2>&1
```

**Notes**

Final eleven protected paths and three canonical chapters pass. Repeated after evidence sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-storage-progress-sealed.log 2>&1
```

**Notes**

All 129 tasks pass. Parsed parent ledger is also compared directly: all 295 prior entries are unchanged; total 296, M17/M17.2 remain In progress. Repeated after evidence sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-storage-doc-check.log 2>&1
```

**Notes**

All four final documents pass formatting. Repeated after evidence sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI records with native MemoryStore. Disposable mock embedded process/real private locks; no real RustFS or live harness qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Opaque memory-only CaptureStorageLifetime; additive explicit storage-bound lifecycle start. No stored bytes, source cursor, raw/pending/prepared/receipt, public control schemas or AgentStore format changes. Current-only AgentStore remains supported.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lsuylsmq
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

Storage resource lifetime and the shared blocking-owner executor affect explicit capture plus all foreground native storage consumers. Escalation reason before cold CLI/full direct suites: shared storage shutdown/drop and retained task result changes can alter server lifecycle, native serving and diagnostic teardown. Required scope is exact ownership/native runtime tests, complete direct supervisor/pipeline-worker/configuration/ordinary embedded tests, eight executor faults, affected server/CLI all-target lint and ordinary cold executable qualification. Root aggregates, unrelated UI and whole Rust workspace are out of scope. Embedded ownership tests use mock processes, never real RustFS. Authoritative storage and its owned embedded installation lock survive shutdown-wait cancellation, drop, detached work and lost tasks; explicit shutdown waits for the final lease. Configured startup remains guarded and foreground process wiring (including external-mode process state-lock ownership) remains pending. Broader allocation/rotation/root-rebinding/discovery-delivery fault qualification, Cursor/OpenCode, authenticated controls/coverage UI, real storage/harness/cold full-flow and M17.3-M17.7 remain pending. Prior 295 entries are preserved. No installed binaries, services, daemons, credentials, model sessions, checkpoints, deployment or publication changed. Final behavior is qualified by five storage ownership targets (mock child, actual filesystem locks and 128 wakeup races), six main-process runtime behaviors plus five fresh-process helper cases, 27 complete supervisors, two pipeline-worker cases, eight executor faults, 16 configuration and 15 ordinary cold executable cases. The helper registration itself is not a behavioral case; two server and three CLI real-storage/deployment ignores are not passes. A single intermediate independent-B timeout is retained and its exact plus full default-concurrency final reruns pass without test, deadline or assertion changes. No failure is waived. All modified behavioral production modules remain below 300 lines (41-274 physical lines). Public controller/API contracts and durable formats are unchanged. Configured startup and process state-lock wiring remain pending. Final all-target package check and strict lint, Rust/Markdown formatting, boundary/architecture checks and canonical validation/spec-first/progress gates pass. The narrow explicit storage-retention slice is complete; this does not mark M17.2 or the M17.1-M17.7 roadmap complete. Next action is guarded foreground preparation and external process state-lock ownership integration, followed by remaining fault, reader, control and real harness/cold gates.
<!-- /evidence-field -->

