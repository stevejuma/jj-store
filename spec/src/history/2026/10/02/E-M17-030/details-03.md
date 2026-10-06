---
schema: 1
id: E-M17-030
---

# E-M17-030 details 3

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
pnpm exec prettier --write spec/src/cli.md > /tmp/jj-store-agent-capture-m172-saturation-cli-format.log 2>&1
```

**Notes**

Mapped CLI pressure/status qualification prose formatted.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-saturation-spec-first-fixed.log 2>&1
```

**Notes**

Mapped CLI chapter now accompanies all protected changes.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-saturation-ledger-format-final.log 2>&1
```

**Notes**

Qualified ledger formatted.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-saturation-doc-check-final.log 2>&1
```

**Notes**

All four changed canonical documents pass formatting.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-saturation-progress-final.log 2>&1
```

**Notes**

Final qualified bookkeeping passes with 129 tasks and 285 evidence entries.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-saturation-spec-first-final.log 2>&1
```

**Notes**

Final same-change canonical chapter map passes.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-saturation-spec-validate-final.log 2>&1
```

**Notes**

Final 5 validators, 21 chapters and 160 requirements pass after mapped CLI update.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Four synthetic qualified Codex CLI 0.156.1 sessions, actual native service/WAL/AgentStore over MemoryStore, synthetic workspace/socket admission and pinned Kingfisher cache. Fresh test fixture and explicit supervisor shutdown/restart are distinct from cold whole-server/RustFS/live harness acceptance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No source/native admission/cursor/pending/prepared request/actor/receipt or public wire format changes. One-byte configured sanitized spool refusal; at most two exact raw source frames under one-reader supervision. Only sanitized progress/native eligibility can survive restart; paths/raw state remain opaque and memory-only. Stopped status excludes actually released reservations while draining status remains conservative.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change yrrqpwvo
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Qualifies configured source-frame and spool-refusal pressure without evicting unacknowledged raw batches or enrolling excess sources. Revoking A while complete B contexts arrive cannot acknowledge or drop rejected batches. Paused immediate shutdown releases source ownership and now reports zero resident/occupied source reservations; outstanding draining owners retain their reservation counts. Explicitly recreating the supervisor with a larger configured capacity backfills only the two admitted native sessions, independently admits current B, safely omits old A and preserves original source identities. The other two historical starts remain withheld until the newly armed probes observe fresh starts, after which all four sessions sanitize/publish under B and the queue empties. The new exact pressure case found the stale shutdown counter regression after fixture defects were fixed; no assertions were weakened. Full direct supervisor binary is required by its shared status/lifecycle impact; unrelated capture-runtime/CLI/UI/whole-workspace/root aggregates are out of scope. M17/M17.2 remain In progress. Saturated durable queue pressure, source rotation/retirement/requalification, task-loss/deadline faults, process startup/ordered storage shutdown, legacy AgentStore initialization, remaining harness readers, typed authenticated controls, coverage/backlog UI and live harness/cold qualification remain pending. M17.3-M17.7 remain pending. No installed services/binaries, daemon restart, harness process, credential, checkpoint, deployment or publication changed; automatic startup remains guarded. Initial spec-first failure for the required CLI chapter was fixed by adding its canonical scope in this same change.
<!-- /evidence-field -->

