---
schema: 1
id: E-UI-041
---

# E-UI-041 details 3

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
pnpm check:architecture
```

**Notes**

Final 297-module boundary and source-size audit passes. Log .scratch/evolution-architecture-final.log.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned live demo ready at 127.0.0.1:7460; PID 53682. Existing explicit binaries and repository data preserved; no reseed or unrelated service restart.
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
CUA live built-UI verification
```

**Notes**

Temporary hidden tab 12 verifies actual native evolution endpoint and JJ Log/Evo Log columns with unchanged main commit/workspace/pinned operation. Compact Repositories top margin is 0px; conflict sheet header is 1px solid. Screenshots .scratch/evolution-final.png and .scratch/evolution-conflict-border.png. Closed sheet and temporary tab; user tabs preserved. No live mutation preview/apply.
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
node .scratch/format-evolution.mjs
```

**Notes**

All 39 changed production, catalog, regression and canonical files formatted. Log .scratch/evolution-format.log.
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

4 validators, 18 chapters and 107 requirements pass. Log .scratch/evolution-spec-validate.log. Final rerun after ledger and matrix formatting also passes; log .scratch/evolution-spec-validate-final.log.
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
pnpm spec:first
```

**Notes**

35 protected files and 3 canonical chapters pass same-change gate. Log .scratch/evolution-spec-first.log. Final rerun after ledger and matrix formatting also passes; log .scratch/evolution-spec-first-final.log.
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
pnpm check:progress
```

**Notes**

81 tasks pass evidence bookkeeping audit. Log .scratch/evolution-progress.log. Final rerun after ledger and matrix formatting also passes; log .scratch/evolution-progress-final.log.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds. Log .scratch/evolution-spec-build.log. Final rerun after ledger and matrix formatting also passes; log .scratch/evolution-spec-build-final.log.
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
pnpm exec prettier --write spec/src/verification.md spec/src/milestones.md
```

**Notes**

Final matrix and evidence ledger formatted; matrix padding kept within existing table width.
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
pnpm exec prettier --check spec/src/verification.md spec/src/milestones.md
```

**Notes**

Final canonical matrix and ledger formatting check passes.
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

<!-- evidence-field:deferred list -->
## Deferred

- Changed stories remain visual-pending; human visual baseline approval is
  separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated UI suites; Rust/native contract generation, real-RustFS and shared
  release qualification are out of scope under targeted UI-only policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing dependency resolution unchanged; no sibling or package manifest edits.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native evolution endpoint consumed with full 128-character commit and pinned operation identities. Independent evolution hash anchor and presentation state preserve the selected main commit and typed mutation target; predecessor edges never substitute commit parents. Opaque pages preserve exact snapshots, deduplicate commits, reject repeated cursors, bound graph display and ignore late responses. Community graph/column canvas reused with local loading, empty, failed and incomplete states. No server DTO, native object, manifest, WAL, receipt or credential format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused evolution identity/paging/history and chrome regressions; visual baseline approval remains separate.
<!-- /evidence-field -->

