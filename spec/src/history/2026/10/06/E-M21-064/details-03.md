---
schema: 1
id: E-M21-064
---

# E-M21-064 details 3

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
pnpm check:progress; historical ledger preservation check
```

**Notes**

Final ledger validation is run after append. Prior612 evidence records retained; existing milestone states unchanged.
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
Design Core: pnpm checks; pnpm spec:first; pnpm spec:check
```

**Notes**

Source shared-component gate required by Design Core AGENTS.md: formatting and all types/consumer checks,722 unit tests plus3 Turbo tests,213 Storybook files/671 stories, static build/audit,15 workspace pointer,11 shell pointer,3 AI browser and48 shadcn browser tests pass;1 existing browser case skipped. No screenshot comparison or baseline updates. Exact gate logs copied to this evidence directory.
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
pnpm catalog:check; pnpm exec prettier --check spec/src/milestones.md; parent/current Prettier API comparison and guarded table formatting
```

**Notes**

catalog-final-spacing.log:0 errors/warnings,251 modules after final pairwise gap assertion. Initial ledger formatting failed because appended table references were unaligned; formatted the table only under existing prettier-ignore and proved the entire JSON evidence block unchanged. ledger-format-final.log passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Focused controller and cold browser interaction/axe regressions, UI types and governance; shared Design Core source verified separately. No full jj-store suite claim.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- Rust builds/tests and backend changes; root aggregate suites;
  screenshots/baselines; publication; native sample repair or membership
  mutations.
- Direct live browser inspection is unavailable because CUA timed out; live
  server readiness and cold automated browser checks remain separate evidence.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

All requested refinements are in the default/main workspace and refreshed Vite server. Source commit ynmwovuk is separately verified; package publication remains outside this slice.
<!-- /evidence-field -->

