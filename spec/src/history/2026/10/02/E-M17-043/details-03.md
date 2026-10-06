---
schema: 1
id: E-M17-043
---

# E-M17-043 details 3

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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-factory-boundaries.log 2>&1
```

**Notes**

17 package boundary audits pass.
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
pnpm check:architecture > /tmp/jj-store-agent-capture-m172-factory-architecture.log 2>&1
```

**Notes**

719 web module architecture audit passes.
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
cargo test -p jj-store-cli --test server_executable --locked > /tmp/jj-store-agent-capture-m172-factory-cli-cold.log 2>&1
```

**Notes**

15 ordinary rebuilt executable cases pass; three real-storage/deployment ignores are not passes.
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
cargo test -p jj-store-cli --test local_browser --locked filesystem_runtime_registers_and_reads_without_native_storage -- --exact > /tmp/jj-store-agent-capture-m172-factory-filesystem.log 2>&1
```

**Notes**

Exact direct filesystem runtime consumer passes; disabled factory creates no native capture state.
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
cargo check -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-factory-check-final.log 2>&1
```

**Notes**

Final affected server/CLI all-target checks pass with native message assertion and all fixture corrections.
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
cargo clippy -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-factory-clippy-final.log 2>&1
```

**Notes**

Final affected server/CLI strict all-target lint passes.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-factory-ledger-format.log 2>&1
```

**Notes**

Single-ledger formatting passes after run recording and required date/kind correction.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-factory-spec-validate-final.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass; repeated after final evidence.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-factory-spec-first-final.log 2>&1
```

**Notes**

Nine protected paths and three canonical chapters pass; repeated after final evidence.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-factory-progress-final.log 2>&1
```

**Notes**

129 tasks pass; missing date/kind failure repaired. Prior 297 entries preserved, total 298; repeated after final evidence.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --test capture_authority --test capture_runtime --test capture_supervisor --test configuration --test embedded_storage --locked > /tmp/jj-store-agent-capture-m172-factory-server-final.log 2>&1
```

**Notes**

Complete direct owning/consumer lanes pass: 22 library entries, three authority, seven runtime, 27 supervisors and 16 configuration entries (75 total including three no-op helper registrations). Parent behavior entries dispatch 16 factory, four prior foreground and five retained-runtime fresh-process cases. Two real RustFS ignores are not passes. The new factory active case uses its own cold rules cache and checks the actual native message; shared older fixtures use their existing pinned cache selection. No whole Rust workspace qualification.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-factory-doc-check.log 2>&1
```

**Notes**

All four sealed documents pass formatting; repeated after recording final evidence.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress > /tmp/jj-store-agent-capture-m172-factory-progress-final.log 2>&1
```

**Notes**

A prematurely recorded pending run result is invalid under the ledger run contract; actual commands accept only pass/fail. Replaced that placeholder with the completed actual document-check pass and reran progress. Entry-level pending status was valid; no product failure or prior evidence mutation.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-factory-progress-final.log 2>&1
```

**Notes**

Final corrected ledger bookkeeping passes; all 129 tasks and 298 entries validate with prior 297 entries unchanged. Repeated after final sealing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic Codex CLI 0.156.1 records, MemoryStore native publication, external S3 lifetime client without server requests. Fresh subprocesses and actual private filesystem locks; no real RustFS or live harness acceptance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing current-only AgentStore, installation identity, source registry, sanitized spool, cursor and exact prepared/receipt formats unchanged. Private agent-capture/{sources,spool,kingfisher} server-state layout; body-free internal Disabled/Unavailable/Ready and running states. No alternate startup mode, enablement bypass or extra credentials.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tmsmqvxy
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

One guarded foreground factory selects validated native resources, exact registry installation identity and Codex locations/privacy before private source/spool state. Disabled preparation reads no capture environment/state. Failed preparation maps to a body-free unavailable slot and preserves ordinary serving; no fallback reader or state reset. The ready supervisor retains authoritative storage before dispatch; foreground execution starts that same slot and requests drain before replication/search/storage stop. Capture runtime cancellation retains owners through drain. Test-only fixtures assemble private slots using MemoryStore publication and a retained external-S3 client without requests; normal enabled startup still fails before creating state. Because this changes shared foreground/runtime composition and actor construction, required direct scope is exact factory cases, complete owning library, capture authority/runtime/supervisor and configuration suites, cold executable and exact filesystem runtime consumers, server/CLI all-target check/lint and formatting/boundary/architecture/canonical gates. Root aggregate, unrelated UI/full Rust workspace and real RustFS qualification remain out of scope. Test self-crate alias changes only unit-fixture compilation; complete owning lib checks its direct impact. Previous 297 evidence entries must remain unchanged. M17/M17.2 remain In progress; automatic enabled startup and remaining faults/rotation/root rebinding/loss, Cursor/OpenCode, authenticated controls/coverage/degraded-health presentation and real storage/harness/cold gates remain pending. M17.3-M17.7 remain pending. No installation, publication, daemon, credential, model session, checkpoint, migration or deployment action. A wrapped-context edit miss made no changes before reapplying against actual formatted running code; inspection typos are not product evidence. Final product acceptance passes the complete exact 16-case factory target, 75 direct server entries including three no-op helper registrations and 16 ordinary CLI/runtime consumer cases. All source/framework assertions remain intact, including actual current AgentStore message, exact offset, sanitized queue drain, installation and private-state boundaries and retained shutdown ownership. Cold rules compilation is included without priming; only the total cold observation window follows the canonical synchronous-overdue contract, while configured step deadlines and shutdown bounds remain unchanged. Owning/direct consumer compile and strict lint, Rust formatting, architecture/boundaries and corrected canonical/progress gates pass. Changed production modules have 47-258 lines; capture factory has 213. Public startup remains guarded; only unit fixtures assemble its private ready slot. No configured automatic capture, typed management/degraded-health UI, live harness or real storage acceptance is claimed. Final sealing also records and repairs an invalid pending run placeholder; all actual completed runs now use the required pass/fail contract. The narrow guarded foreground composition slice is complete, with the full M17.1-M17.7 goal active and M17/M17.2 status unchanged. Next work is the remaining discovery/delivery-loss and root/fault qualification, other readers, typed controls and real cold acceptance before lifting startup guard.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

