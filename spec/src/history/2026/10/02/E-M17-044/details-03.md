---
schema: 1
id: E-M17-044
---

# E-M17-044 details 3

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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-lane-loss-check-final.log 2>&1
```

**Notes**

Final server/direct CLI all-target compilation passes.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-lane-loss-clippy-final.log 2>&1
```

**Notes**

Final strict server/direct CLI all-target lint passes; duplicate module failure repaired without a waiver.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-lane-loss-rustfmt-final.log 2>&1
```

**Notes**

Final main and embedded pinned Rust formatting passes.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-lane-loss-spec-validate-final.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass; repeated after final sealing.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-lane-loss-spec-first-final.log 2>&1
```

**Notes**

Six protected paths and three owning canonical chapters pass; repeated after final sealing.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-lane-loss-progress-final.log 2>&1
```

**Notes**

129 tasks pass. Previous 298 evidence entries unchanged, total 299; repeated after final sealing.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-lane-loss-doc-check.log 2>&1
```

**Notes**

All four changed documents pass final formatting; repeated after final sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic Codex CLI 0.156.1, MemoryStore native publication, private real filesystem locks, actual Tokio queued blocking-task abortion. Existing qualified synthetic scanner cache; no real storage or live harness fault qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No durable or public shape changed. Only settled delivery-loss unavailable counter changes; cfg(test) task-abort handle helper provides real queued loss without a product fault hook. Current-only AgentStore and exact pending batch/cursor/native request/receipt formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tnmpwvss
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

Real queued discovery/delivery task cancellation settles as loss of that owned lane. The delivery branch previously invalidated queue freshness but omitted its unavailable observation. Count it once at lost completion; cancellation/deadline behavior and ownership stay unchanged. Matrix retains exact sanitized pending bytes, batch and cursor through lost owner and refused pause/resume, then explicitly replaces the supervisor under fresh current source/directory/Write checks. No source/body is read by the aborted queued work. Shared supervisor status/settlement and cfg(test) executor handle helper require complete owning library, existing 27 supervisor and seven runtime targets, package/direct CLI all-target checks and strict lint, Rust formatting, boundaries/architecture and canonical/progress gates. This is the concrete direct escalation reason; unchanged foreground/configuration/CLI product behavior was just qualified in E-M17-043, so cold CLI reruns, UI, root aggregate/full Rust workspace and real storage lanes are out of scope for this diagnostic slice. All previous 298 evidence entries must remain unchanged. M17/M17.2 remain In progress; panic after discovery/delivery dispatch, broader OS/allocation/rotation/root-rebinding faults, other readers, management/coverage, live storage/harness/cold acceptance and M17.3-M17.7 remain pending. Automatic configured startup remains guarded. No installed binaries/services/daemons, credentials, model session, checkpoint, migration, publication or deployment action. Strict duplicate-module lint identifies common unit-fixture reuse. Consolidation changes only cfg(test,unix) module selection at lib root; both factory and new loss targets import the same unchanged fixture. Complete owning-library verification is rerun after this shared fixture change, in addition to the already-required direct supervisor/runtime lanes. Final exact two-lane/current-admission regression, complete owning library and direct runtime/supervisor lanes, all-target compile/strict lint and Rust/document/boundary/architecture/canonical/progress gates pass. Production loss settlement counts the delivery failure once, preserving accepted intent and the existing terminal owner state. No deadline/assertion/format is weakened. Modified behavioral modules are 177-277 lines. The narrow queued-loss slice is complete; panic after dispatch, broader root/fault qualification, Cursor/OpenCode, typed controls/coverage/degraded-health UI and real storage/harness/cold gates still block automatic enablement. M17.3-M17.7 remain planned and the whole goal remains active.
<!-- /evidence-field -->

