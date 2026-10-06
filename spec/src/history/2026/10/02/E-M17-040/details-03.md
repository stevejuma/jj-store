---
schema: 1
id: E-M17-040
---

# E-M17-040 details 3

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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-runtime-doc-check.log 2>&1
```

**Notes**

Final sealed canonical documents and ledger pass formatting; repeated after recording this result.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-runtime-terminal-final.log 2>&1
```

**Notes**

Four native runtime cases pass after terminal command ownership is retained inside the driver for all error/stop drain paths.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-runtime-terminal-lint.log 2>&1
```

**Notes**

Server/CLI targets pass strict lint after the internal terminal latch change.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked > /tmp/jj-store-agent-capture-m172-runtime-fault-final.log 2>&1
```

**Notes**

All five final native runtime cases pass, including failed dispatch, terminal controls and repeated completed failed join. Earlier complete 27 supervisor and eight executor cases are unchanged; total 40 distinct relevant lifecycle cases.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-runtime-fault-lint.log 2>&1
```

**Notes**

Final source and all test/direct consumer targets compile and pass strict lint after the failure regression and shared runtime fixture extraction; Rust formatting also repeated and passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI source records and native MemoryStore publication. No configured foreground-server, live harness or real RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive explicit RunningCodexCapture and bounded last-observed scalar snapshot. Internal watch command state is terminal after stop; no serialized control schema, raw persistence, cursor, pending, prepared request or receipt format changes. Current-only AgentStore remains supported.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ztzxnzpq
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

The explicit lifecycle driver owns the existing prepared supervisor and schedules discovery, admitted reading and exact delivery on retained independent blocking owners. Pause/resume uses one bounded internal command state; stop is terminal and ordered with synchronous dispatch. Timeout retains the join for later waiting; dropping the handle requests stop while its task drains. Driver errors also stop/drain remaining lanes before returning; a completed failed join is never repolled. Existing supervisor shutdown gains only an internal reusable stop path. This shared lifecycle change requires the complete direct supervisor suite, exact native runtime acceptance and underlying executor fault tests; server/CLI all-target compilation and strict lint cover the direct consumer boundary. No CLI currently consumes the new driver, so unrelated CLI capture test reruns, UI, capture/model/storage suites, whole Rust workspace, root aggregate and real storage qualification are out of scope. Changed behavioral production modules remain below 300 lines. Prior 294 evidence entries preserved. M17/M17.2 stay In progress. Configured startup, authoritative-storage lifetime/shutdown integration, broader rotation/root rebinding and discovery/delivery task-loss faults, Cursor/OpenCode readers, authenticated controls/coverage UI and real harness/cold acceptance remain pending; M17.3-M17.7 remain pending. The enabled server-startup guard is unchanged. No installed binaries/services, unrelated daemon restart, credentials, checkpoint, harness execution, deployment or publication changed. Final gates qualify four explicit runtime cases, 27 complete supervisor cases and eight underlying executor fault cases, owning/direct consumer compile and strict lint, Rust/document formatting, architecture/boundaries, canonical validation, spec-first and evidence bookkeeping. The configured foreground-server guard remains unchanged; no configured startup, real storage or cold harness qualification is claimed. Final review retains a bounded command sender inside the driver so internal failure latches terminal controls before drain. The new exact failed-dispatch test and all four previous runtime cases pass; final relevant count is 40 (five runtime, 27 supervisor and eight executor), with final all-target strict lint and Rust formatting passing. The final runtime module has 168 lines; supervisor remains 293. No unrelated source or contract changed.
<!-- /evidence-field -->

