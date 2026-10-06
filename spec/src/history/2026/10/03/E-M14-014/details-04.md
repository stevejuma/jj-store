---
schema: 1
id: E-M14-014
---

# E-M14-014 details 4

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
JJ_STORE_DEV_PORT=7570 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/storage-browser-acceptance.mjs (final)
```

**Notes**

1/1 real-RustFS browser test in 28.1 seconds after cold native fixture startup. Exact prepared target and request survive service/RustFS restart; explicit publication retains heads/position; bounded integrity completes; second restart retains completed integrity and exact selected checkpoint report. Historical route, mobile reload/navigation, no overflow, axe and no page errors pass. Isolated service and fixture cleaned up. Positive-path test requires this small fixture instead of assuming the large demo fits the 30-second budget. /tmp/jj-local-management-storage-focused-real-register-final.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:validate; pnpm spec:first (new acceptance runner)
```

**Notes**

Progress and canonical validation passed. Spec-first rejected the new tooling file until spec-governance.md documented the bounded native fixture, owned lifecycle and distinction from larger incomplete audits; added the owning chapter without weakening the gate.
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
pnpm exec prettier --check all changed frontend, test, generated-contract and specification files
```

**Notes**

All matched files use Prettier style before the final governance evidence addition.
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
pnpm exec prettier --write spec/src/spec-governance.md spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm spec:first; pnpm check:architecture (final)
```

**Notes**

125-task evidence bookkeeping passes; 5 validators, 22 chapters, 152 requirements; spec-first accepts 44 protected files with 8 canonical chapters; 835-module architecture passes. M14.5 complete for the qualified maintenance scope; M14/M14.6/M14.7 remain separately gated.
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

<!-- evidence-field:scope markdown -->
## Scope

Native Storage overview, bounded integrity, prepared checkpoint publication/cancel and exact-target recovery. Checks are read-only; checkpoints only select immutable folded data through manifest CAS. Full WAL suite justified by shared coordinator behavior; focused UI and native HTTP consumers only. Root aggregates, agents, backup/restore, pruning, replication pressure/soak and visual-baseline approval remain outside this slice. The large demo integrity audit reached its 30-second bound and remains explicitly incomplete; positive full-audit evidence applies to the bounded native fixture. The configured limits were not relaxed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain maintenance regression coverage; remaining M14 agent, integrated settings and expanded Sync qualification are separate work.
<!-- /evidence-field -->

