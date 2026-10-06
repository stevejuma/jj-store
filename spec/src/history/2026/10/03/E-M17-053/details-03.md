---
schema: 1
id: E-M17-053
---

# E-M17-053 details 3

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
pnpm exec prettier --write spec/src/mutations.md > /tmp/jj-store-agent-capture-m172-coverage-mutations-format.log 2>&1
```

**Notes**

Mapped mutation chapter formatted after explaining cached groups carry no authority.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-coverage-first-after.log 2>&1
```

**Notes**

Original canonical missing-map failure one to zero; all protected files have owning chapters.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-coverage-rust-format.log 2>&1
```

**Notes**

Pinned Rust and unchanged embedded-runtime formatting pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management coverage::harness_coverage_is_bounded_cached_and_contains_no_roots --locked -- --exact > /tmp/jj-store-agent-capture-m172-coverage-after3.log 2>&1
```

**Notes**

Final exact regression passes with actual two-profile loopback samples and reversed configuration order after notification indirection. No changed policy/spool bytes while status remains cached across root removal.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-library-after.log 2>&1
```

**Notes**

Serial suite execution with unchanged scenario concurrency and original deadlines. test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 139.61s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_runtime --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-runtime-after.log 2>&1
```

**Notes**

Serial suite execution with unchanged scenario concurrency and original deadlines. Finished `test` profile [unoptimized] target(s) in 5.88s; test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.04s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_management --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-management-after.log 2>&1
```

**Notes**

Serial suite execution with unchanged scenario concurrency and original deadlines. Finished `test` profile [unoptimized] target(s) in 0.77s; test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.93s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_supervisor --locked -- --test-threads=1 > /tmp/jj-store-agent-capture-m172-coverage-supervisor-after.log 2>&1
```

**Notes**

Serial suite execution with unchanged scenario concurrency and original deadlines. test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 209.64s
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm contracts:generate > /tmp/jj-store-agent-capture-m172-coverage-generation.log 2>&1
```

**Notes**

Generated schemas/openapi.json and web/src/lib/api/generated.ts
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-coverage-ledger-format.log
```

**Notes**

Canonical chapter and evidence formatting completed during notification repair.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-coverage-ledger-format2.log
```

**Notes**

Canonical chapter and evidence formatting completed during notification repair.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm contracts:check > /tmp/jj-store-agent-capture-m172-coverage-contracts.log 2>&1
```

**Notes**

All 170 actual HTTP/SSE samples conform; seven contract-tool tests and TypeScript fixture pass.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher pnpm web:check > /tmp/jj-store-agent-capture-m172-coverage-web.log 2>&1
```

**Notes**

Completed direct generated consumer lane; see recorded log.
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
pnpm exec prettier --write scripts/contracts.mjs schemas/openapi.json web/src/lib/api/generated.ts spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-coverage-seal-format.log 2>&1
```

**Notes**

Final canonical/generated formatting and boundary/architecture/spec/progress checks pass; unchanged Rust formatting passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Synthetic Codex CLI 0.156.1 header and configured Codex Desktop 0.159.2 empty root. Native MemoryStore, real private filesystem and loopback transport; no real product edit/RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

All control journal version 1 and native/repository/source/spool formats unchanged; 88 routes unchanged. Additive typed CaptureHarnessCoverage/Profile with at most two current Codex groups, declared product versions and cached root/source counters. Body-free Copy runtime observations retain two fixed profile slots, no raw paths/session keys/import selectors or source proofs. Completion notifications now box the 352-byte body-free status instead of enlarging all enum variants; cached runtime snapshots remain Copy and carry no resources. No scheduling/settlement change.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zzwvqswt
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

Adds pure per-profile root observation groups and source counter projection; deterministic grouping does not depend on configuration order. No scanning, activity, enrollment, cursor, native publication, policy or receipt behavior changes. Current prepared Codex surfaces remain the only runtime-supported groups; schema profile names do not qualify other harnesses or enable startup. Actual HTTP samples qualify two-profile running, paused, cached-after-root-removal and stopped status, with no raw paths and totals matching aggregate counters. Concrete escalation: discovery/status observations cross supervisor/runtime/shared service+HTTP contract boundaries, requiring owning library/runtime/supervisor/management/HTTP entries, direct four-package all-target compile/strict lint and every registered response capture plus contract-tool/TypeScript/generated web consumption. Journal production and root aggregates/full Rust workspace/catalog/Storybook/UI interaction/real storage/live harness/unrelated suites are out of scope. Previous 307 parsed entries remain unchanged. M17/M17.2 remain In progress; source-level coverage and selectors/import, CLI/browser controls, remaining readers/faults and enabled startup/real JJ/storage/live harness/cold full-flow qualification remain pending. M17.3-M17.7 remain Planned and full objective stays active. Current AgentStore only. No installation, migration, publication, deployment, credentials, unrelated daemon restart or subagents. A canonical patch context miss applied no files. Final production reruns pass: 42 server-library, ten runtime, eight management, 27 supervisor and 14 HTTP entries; all 170 actual responses, seven contract-tool tests, TypeScript fixture and generated web consumer pass. These are serial focused boundary lanes, not default-concurrency or full-workspace acceptance.
<!-- /evidence-field -->

