---
schema: 1
id: E-M17-078
---

# E-M17-078 details 3

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --lib capture_supervisor::opencode:: --locked
```

**Notes**

Two API ownership units pass. /tmp/jj-store-snapshot-management-api-units.log.
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
cargo test -p jj-store-server --lib capture_runtime::sources:: --locked
```

**Notes**

One producer-loss/stopped-cache unit passes. /tmp/jj-store-snapshot-management-runtime-units.log.
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
cargo test -p jj-store-http --lib contract::capture_tests:: --locked
```

**Notes**

Two strict public-contract units pass. /tmp/jj-store-snapshot-management-http-units.log.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries pass.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters, 160 requirements; ledger raw prefix preserved.
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
pnpm check:progress
```

**Notes**

129 tasks, evidence bookkeeping only.
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
pnpm spec:first
```

**Notes**

22 protected files and seven canonical chapters pass.
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
pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/mutations.md spec/src/spec-governance.md spec/src/verification.md spec/src/milestones.md scripts/contracts.mjs web/src/lib/controller/capture-response-validation.ts web/src/lib/controller/capture-snapshot-validation.ts web/src/lib/controller/capture-validation.ts web/tests/vite/capture-management.test.ts
```

**Notes**

Final changed-file formatting gate; rerun after evidence finalization.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; pnpm 10.32.1; macOS ARM64; OpenCode 1.18.30 profile; pinned Kingfisher sanitizer.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional snapshot_publication in version-1 source management DTO/OpenAPI; closed seven-field body-free uint64 record. Source/witness queries remain device-memory-only. Original JSONL publication, source/control/import journals, repository and current AgentStore formats are unchanged.
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

Selected impact surface only. Current Admin/config/control-file admission before and after bounded metadata reads; no harness request, pending-body access or owner creation. Native MemoryStore with controlled OpenCode input remains distinct from real harness/RustFS, configured startup and UI/cold release. Additive shared response schema and capture runner require the complete registered HTTP/SSE inventory. Management module runs sequentially under the previously recorded parallel deadline limitation. Renderer, publication authority and import consent are unchanged. Root aggregates, unrelated suites, visual baselines, installation and publication are out of scope. Full M17 remains active; OpenCode history import/export and common identities, Cursor and M17.3-M17.7 remain open. All 332 parent evidence objects and their raw JSON prefix are preserved. Selected acceptance and canonical gates pass; initial client/module-budget failures and concurrent-build import timeout remain recorded. No full repository or real-harness release claim.
<!-- /evidence-field -->

