---
schema: 1
id: E-M17-021
---

# E-M17-021 details 3

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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-recovery-clippy-final.log 2>&1
```

**Notes**

Final affected/direct-consumer strict lint passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_recovery queued_other_repository_recovers_exact_context_after_restart --locked -- --exact > /tmp/jj-store-agent-capture-m172-recovery-exact-final.log 2>&1
```

**Notes**

Original exact regression rerun passes after final cursor-resumption safeguards.
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
cargo fmt -p jj-store-capture -p jj-store-server -p jj-store-cli -- --check > /tmp/jj-store-agent-capture-m172-recovery-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes; checked again after final lint-only conditional formatting.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-recovery-boundaries.log 2>&1
```

**Notes**

All 17 package dependency boundaries pass.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-m172-recovery-doc-write.log 2>&1
```

**Notes**

Owning canonical chapters and verification references formatted.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-recovery-first-before-ledger.log 2>&1
```

**Notes**

Initial spec-first passes with 15 protected files and two canonical chapters.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-recovery-doc-final-write.log 2>&1
```

**Notes**

Final canonical and appended ledger formatting.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-recovery-progress.log 2>&1
```

**Notes**

129 tasks validate; 276 evidence structures preserve all 275 parent entries.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-recovery-spec.log 2>&1
```

**Notes**

Five validators, 21 canonical chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-recovery-first.log 2>&1
```

**Notes**

Final protected implementation/spec/ledger scope passes spec-first.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-recovery-doc-format.log 2>&1
```

**Notes**

Final changed canonical document formatting passes.
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
cargo fmt -p jj-store-capture
```

**Notes**

Focused formatting of the added Desktop/incomplete-ending runtime test.
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
cargo test -p jj-store-capture --test codex_recovery --locked > /tmp/jj-store-agent-capture-m172-recovery-runtime-style-final.log 2>&1
```

**Notes**

All five proof regressions pass after the final lint-only ready-condition collapse.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic independently qualified Codex CLI 0.156.1 and Desktop 0.159.2 files. Native pipeline uses actual service/WAL/AgentStore receipts over MemoryStore with the existing pinned Kingfisher cache. Fresh OS process proves retained admitted-channel queued prepared-intent directory reconstruction and unchanged spool; native prepared publication replay separately qualifies reopened driver state. No real model edit, RustFS or cold whole-server startup claim.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No pending entry, request/receipt, cursor journal, native alias, source registry or sanitizer format changes. Additive memory-only bounded recovery proof/poll and lease-preserving reader resume; exact-entry publication selector checks original sanitized core under the drain lease and preserves already-prepared bytes. Path/body-bearing reconstruction types have no Debug or serialization.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change utqtmyyt
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One pending-proof recovery surface validates the admitted channel, physical/header witness, configured product/parser, native UUID, exact complete ending offset and prefix SHA-256, deterministic batch/observation identities and contiguous row count. At most 4 MiB is scanned per step with at most 1 MiB partial bytes and 256 trailing contexts; mixed ranges, incomplete endings, changed witnesses and rewritten prefixes cannot publish. Queued B body from header A uses independently current B directory/socket/catalog and native Write admission. Only exact constant omissions may use separately admitted matching prior/header/current anchor candidates; fallback never grants body admission. Revocation, missing/replaced/rewritten files and lost native Write preserve exact pending intent. Exact-entry draining cannot lend one recovered directory proof to another batch in the same partition; A/B/A queue recovery proceeds in source order before new reading. The live driver resumes latest sanitized durable progress under unchanged physical/source leases only on a later step, so reopen failure cannot discard a committed receipt. Complete direct runtime/native/CLI consumers and focused package checks/lint/format/boundaries qualify the shared context callback and reader/publication consumers. Five final proof cases plus the other 66 runtime primary cases, 14 native server and 26 direct CLI cases pass, with subprocess fixtures explicitly exercised by parents. Initial regression and fixture/lint failures are retained above with their fixes. Restart acquisition still requires current qualified header-directory admission before source leases; alternate queued-directory acquisition remains supervisor composition scope. M17.2 and aggregate M17 remain In progress. Fair bounded blocking-I/O supervision, deadlines/backpressure/pause/restart and controls, legacy absent-domain initialization, Cursor/OpenCode readers, live harness edits and cold full-flow, UI and the full M17.3-M17.7 roadmap remain pending. Enabled startup guard stays in place. No model/wire, native/WAL implementation, browser lifecycle, installed services, credentials, publication, release or deployment changed; root aggregate and unrelated wider lanes remain out of scope.
<!-- /evidence-field -->

