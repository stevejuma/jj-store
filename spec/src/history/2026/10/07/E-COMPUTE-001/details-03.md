---
schema: 1
id: E-COMPUTE-001
---

# E-COMPUTE-001 details 3

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
cargo build --manifest-path runtime/compute/Cargo.toml --offline
```

**Notes**

TOML macro requires a bound variable for the installed path; fixed expression binding.
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
cargo build --manifest-path runtime/compute/Cargo.toml --offline && python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance
```

**Notes**

Real runtime prepare, explicit GGUF download, startup, authenticated loopback inference and graceful shutdown passed. Logical model identity retained, 8192-token context/two slots bounded. Linker compact-unwind warning retained.
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
pnpm contracts:generate
```

**Notes**

Additive compute/settings/secrets and saved-setup catalog schemas regenerated from owning Rust route contracts.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 59
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

An early rerun preceded generated compute schemas; final contract-aware checks recorded separately.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

After generation, identified inline operation action schema, unknown progress facts and optional refresh callback typing. Fixed against generated operation types.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Compute request is intentionally inline rather than a named component schema. Use the generated local_compute_command request body action type with exact integers.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-server --offline
```

**Notes**

Compute process method visibility excluded its sibling local-agent module. Scoped method to owning agent execution module.
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
pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

1320 production/catalog modules; UI ownership validator passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent*.test.ts web/tests/vite/projects-agent*.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/production-store-settings.test.ts
```

**Notes**

12 files, 59 tests; cold Vite transform. Later catalog/context changes are checked separately.
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
pnpm exec tsx --test web/tests/agent-message-content.test.ts web/tests/agent-execution.test.ts web/tests/agent-mentions.test.ts web/tests/agent-harness-catalog.test.ts
```

**Notes**

15 tests; message context envelopes, exact tags and retained harness models.
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
pnpm check:architecture
```

**Notes**

1316 modules after controller/component decomposition. Final additions require a fresh run.
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
cua final built preview after own-backend restart: Settings → Compute; reset temporary viewport and retain deliverable tab
```

**Notes**

Actual final rebuilt application on 7480 now exposes heading Compute and matching page/region labels, real hardware/model controls, pairing and sharing-off controls. Refresh/back navigation and normal folder registration state remain present. Temporary desktop viewport reset; preview tab remains open. Browser-native rich Sent context/Raw context keyboard acceptance is recorded separately. Own server PID/runtime metrics updated after restart; existing 7460/7474 services preserved.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

An intermediate evidence summary was accidentally assigned to the enum result field; spec-first rejected invalid result. Restored the required enum, retained prose in scope/notes and regenerated histories. This was evidence formatting, not a production failure.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Corrected evidence closure passed: 182 protected files / 10 chapters; 7 validators / 3166 chapters / 181 requirements; progress 146 tasks / 650 evidence records.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/complete-compute-fixture.mjs
```

**Notes**

Review seeding found no Cursor tools because best-effort 3-second hook publication had left 20 durable capture entries pending under host load. Preserved exact batches and prepared identities. Recovered through the public agent drain command; did not recapture, discard pending entries or weaken publication authority.
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
JJ_STORE_DEVELOPMENT_STATE_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/dev-demo target/debug/jj -R .local/dev-demo/repository --no-pager agent --state-dir .local/dev-demo/agent-state --format=json drain
```

**Notes**

Actual native publication recovered 20 batches: committed 20, remaining 0, outcome_unknown false. Kept source ordering, the original request/publication identities, manifest publication and normal authorization.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node .local/complete-compute-fixture.mjs
```

**Notes**

Resumed existing repository/capture data without recapture or reset. Normal idempotent review APIs, repository profile helpers, folder registration and WorkStore seeding completed. Fixture/runtime markers and own process metrics retained; search explicitly disabled in the preview. Startup measurement records the successful recovered process start, not an empty-volume timing qualification.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

Final UI metadata closure passed: 1323 modules and owning UI boundary validator.
<!-- /run -->
<!-- /evidence-field -->

