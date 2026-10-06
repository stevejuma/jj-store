---
schema: 1
id: E-M17-034
---

# E-M17-034 details 3

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
pnpm check:boundaries > /tmp/jj-store-agent-capture-current-agent-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-current-agent-spec-validate.log 2>&1
```

**Notes**

Initial canonical validation passes: 5 validators, 21 chapters and 160 requirements.
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
pnpm exec prettier --write spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-current-agent-doc-format-final.log 2>&1
```

**Notes**

Current-only descriptor evidence and M17.2 next action formatting.
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
rustc --version
```

**Notes**

Rust 1.98.1 (48a229cea 2026-09-01).
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
jj --version
```

**Notes**

JJ 0.45.1.
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
uname -m
```

**Notes**

macOS ARM64 host.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/mutations.md spec/src/local-jj.md spec/src/cli.md spec/src/verification.md > /tmp/jj-store-agent-capture-current-agent-chapters-check.log 2>&1
```

**Notes**

Changed canonical chapters pass formatting.
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
pnpm exec prettier --write spec/src/milestones.md > /tmp/jj-store-agent-capture-current-agent-ledger-format.log 2>&1
```

**Notes**

Evidence and current task links formatted.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-current-agent-spec-validate-final.log 2>&1
```

**Notes**

Final 5 validators, 21 chapters, 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-current-agent-spec-first.log 2>&1
```

**Notes**

12 protected files and 6 canonical chapters pass same-change gate.
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
pnpm check:progress > /tmp/jj-store-agent-capture-current-agent-progress.log 2>&1
```

**Notes**

129 tasks and 289 evidence entries pass bookkeeping; all preceding evidence retained.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/publication.md spec/src/mutations.md spec/src/local-jj.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-current-agent-doc-check.log 2>&1
```

**Notes**

Every changed document passes final formatting.
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
python3 /tmp/jj-store-agent-capture-current-agent-ledger.py
```

**Notes**

Recorded verified evidence only after every target completed; parsed equality confirms all 288 earlier entries preserved.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64. Native MemoryStore and isolated Git-backed companion fixtures, synthetic Codex transcript and workspace/socket admission; no RustFS or live harness qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Supported native AgentRootV1, TranscriptChunkV1 and TranscriptManifestV1 encodings/identities remain unchanged. Local descriptor version 2 and transcript tail-page schema 2 are the supported profiles. Removed local descriptor version-1 upgrade and forward TranscriptPageV1 type/variant/re-export and read/append/audit compatibility. Obsolete formats are rejected without migration. Durable spool physical reservations, compact-record retry support, source cursors, exact requests and receipts are unchanged. Public generated OpenAPI remains identical.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lnkplqqz
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Implements the user decision to support only current AgentStore formats. Deletes the local descriptor in-memory upgrade, forward transcript type and shared compatibility branches including legacy source-receipt migration proof. Current prefix-before-tail read/audit/append preserves bounded ordering witnesses, other streams and complete selected closure audits. Converts service/HTTP/native-copy and model/WAL fixtures to current tails, reversing two-page construction to retain chronological prefixes. Replaces obsolete migration acceptance tests with meaningful current multi-stream/order and old-byte rejection regressions; historical evidence remains unchanged with explicit retired test references. Canonical chapters and current M17.2 next action remove missing-root initialization/migration gates and distinguish supported current V1-named root/chunk/manifest/provenance from obsolete formats. Native capture already requires its selected current root and keeps failing closed when absent; explicit first-use companion publication creates its current descriptor. No existing repositories are migrated or rewritten. Both new exact regressions failed before removal and pass afterward. Incorrect HTTP module qualifier matched zero tests; corrected exact case passes and no empty result is treated as qualification. Full affected model/WAL suites are justified by shared persistence across native/local consumers; direct service, CLI and supervisor consumers plus focused compile/lint/contracts/boundaries validate the impact surface. Existing publication facade only loses its obsolete re-export; behavioral modules remain below 300 lines. All previous 288 evidence entries preserved. M17/M17.2 and M13.3 stay In progress; M17.1 current-tail qualification stays Done. Remaining M17.2 includes compact spool capacity diagnostics, rotation/retirement/requalification/task-loss/deadline matrices, configured startup and ordered storage shutdown, Cursor/OpenCode readers, authenticated controls/UI and live/cold acceptance. M17.3-M17.7 remain pending. Automatic enabled startup remains guarded. No installed service/binary, daemon restart, credentials, checkpoint, deployment, live harness or publication changed. Root aggregate, unrelated UI, whole Rust workspace and real-storage lanes remain out of scope under targeted policy.
<!-- /evidence-field -->

