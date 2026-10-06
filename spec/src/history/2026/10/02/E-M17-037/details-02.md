---
schema: 1
id: E-M17-037
---

# E-M17-037 details 2

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
cargo test -p jj-store-capture --test reader_progress preparation_capacity_refusal_reports_exact_relief_without_mutating_intent --locked > /tmp/jj-store-agent-capture-m172-capacity-before.log 2>&1
```

**Notes**

Before implementation, one missing CaptureCapacityRefusal type diagnostic; compilation stops and no regression cases execute.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Affected production/tests formatted; repeated after server regression additions.
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
cargo test -p jj-store-capture --test reader_progress preparation_capacity_refusal_reports_exact_relief_without_mutating_intent --locked > /tmp/jj-store-agent-capture-m172-capacity-after.log 2>&1
```

**Notes**

Exact spool regression passes: required total excludes replaced compact bytes; bytes/cursor unchanged; exact explicit limit permits preparation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor capacity:: --locked > /tmp/jj-store-agent-capture-m172-capacity-supervisor.log 2>&1
```

**Notes**

Two exact native supervisor regressions pass: ingress and compact preparation diagnostics, pause/restart, exact relief, sanitized delivery and clearing.
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
cargo test -p jj-store-capture --locked > /tmp/jj-store-agent-capture-m172-capacity-runtime-final.log 2>&1
```

**Notes**

Complete capture crate passes: 92 main-process cases plus one launched fresh-process privacy helper. Three explicitly ignored child-entry helpers remain distinct from the cases that invoke them. No ignored test is counted as an executed pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --test capture_source --test capture_pipeline --test capture_recovery --locked > /tmp/jj-store-agent-capture-m172-capacity-server-final.log 2>&1
```

**Notes**

34 direct server cases pass, including all 22 supervisor cases, Write denial without false capacity classification and direct native preparation error propagation.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test local_agent_recovery --locked > /tmp/jj-store-agent-capture-m172-capacity-cli-final.log 2>&1
```

**Notes**

31 direct native/local CLI cases pass; two process-entry helpers remain explicitly ignored in the parent run. Exact retries, independent physical accounting and local recovery retained.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-capacity-check-final.log 2>&1
```

**Notes**

Owning packages and direct CLI consumers compile.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-capacity-clippy-final.log 2>&1
```

**Notes**

Owning packages and direct consumers pass strict lint.
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
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-capacity-rust-format-final.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-capacity-doc-format-final.log 2>&1
```

**Notes**

Changed canonical documents and ledger formatted.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-capacity-doc-check-final.log 2>&1
```

**Notes**

Changed canonical documents and ledger have no formatting drift.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-capacity-validate-final.log 2>&1
```

**Notes**

Canonical specification validation.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-capacity-first-final.log 2>&1
```

**Notes**

Same-change ownership chapters accompany protected implementation.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-capacity-boundaries-final.log 2>&1
```

**Notes**

17 package boundaries pass.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-capacity-progress-final.log 2>&1
```

**Notes**

All 129 tasks validate; M17/M17.2 remain In progress and all prior 291 entries preserved.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex 0.156.1 CLI, directory/socket admission and actual native MemoryStore publication. In-process/fresh-process sanitization and CLI spool/native/local recovery fixtures. No live harness, RustFS or cold configured server qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive internal Rust CaptureCapacityRefusal type, server Error variant and CodexSupervisorStatus byte/count observations. Existing fixed error display preserved. No wire, persistence, dependency, source/admission/cursor or exact prepared request/actor/receipt format change. Physical JSON whitespace reservations and compact durable spool retries retain their supported profile independently of current-only AgentStore.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change olokzsmk
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Retains exact body-free byte-limit refusal through shared admission/preparation, native source/pipeline recovery and supervisor delivery. Required total includes other pending physical bytes and reserved whitespace, excludes the replaced entry and authorizes no eviction, implicit increase or publication. Existing arbitrary corrupt/unsafe/read-limit failures keep their fixed generic validation errors and cannot fabricate this typed capacity diagnosis. Internal status has no filesystem work, counts last settled ingress refusals, reports their maximum and a separate last settled delivery requirement. Pause retains observations; completed successful/non-capacity steps replace them, and stopped/lost owners do not report active ingress pressure. Cached observations do not assert current pressure after another writer changes the queue. Exact regressions prove no rejected-capture cursor advancement, unchanged compact accepted bytes/cursor/native root before preparation fits, exact explicit relief, sanitized native recovery and clearing. The before failure is missing API compilation, not a behavioral test pass/fail. Shared spool capacity behavior across native and local consumers justifies complete capture-crate tests plus direct native/server/CLI publication/spool/recovery binaries; unrelated UI/model/whole-Rust-workspace/root aggregate and real-storage lanes are out of scope. All changed behavioral production modules remain below 300 lines. All previous 291 evidence entries preserved. M17/M17.2 remain In progress. Authenticated management/coverage UI, admitted multi-physical generation switching, wider rotation/root rebinding and task-loss/deadline faults, configured startup/ordered storage shutdown, Cursor/OpenCode readers and live/cold qualification remain pending; M17.3-M17.7 remain pending. Automatic enabled startup stays guarded and AgentStore stays current-only. No installed service/binary, unrelated daemon restart, credential, checkpoint, live harness, deployment or publication changed.
<!-- /evidence-field -->

