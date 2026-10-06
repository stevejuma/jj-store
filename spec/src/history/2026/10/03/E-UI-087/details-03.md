---
schema: 1
id: E-UI-087
---

# E-UI-087 details 3

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
pnpm check:types (source qualified)
```

**Notes**

Root and shell/filter/source consumer Svelte checks all0errors0warnings after relocating the example. /tmp/dc-tooltip-restore-types-final.log.
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
pnpm checks (source qualified attempt)
```

**Notes**

Governance/quality pass;715unit tests/111files and3Turbo unit tests pass; static build and visual metadata audit pass. Whole catalog643passes/1emoji-grid focus failure across208files/644tests. It expects heart focus but receives Search emoji. /tmp/dc-tooltip-restore-checks-qualified.log. No unrelated source edits.
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
pnpm ci:browser-workspace; pnpm ci:browser-shell; pnpm ci:browser-ai; pnpm ci:browser-shadcn (sequential source lanes)
```

**Notes**

Complete remaining aggregate browser lanes after its Storybook stop:14workspace,11shell,3AI,35shadcn pass;1shadcn skip. Fresh owned services cleaned up by Playwright. /tmp/dc-tooltip-restore-browser-*.log.
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
pnpm test:storybook src/shared/ai/experimental/emoji-picker/EmojiPicker.stories.svelte (clean main baseline)
```

**Notes**

2/2 on clean parent4922feea. Installed main uses Bits2.19.3/Svelte5.57.1 while source workspace uses locked Bits2.18.1/Svelte5.56.6; this is not a same-runtime baseline. /tmp/dc-tooltip-restore-emoji-baseline.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook src/shared/ai/experimental/emoji-picker/EmojiPicker.stories.svelte (source isolated initial)
```

**Notes**

Same focus failure,1pass/1fail. /tmp/dc-tooltip-restore-emoji-after.log.
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
jj new @- (temporary baseline); pnpm test:storybook src/shared/ai/experimental/emoji-picker/EmojiPicker.stories.svelte (twice); jj edit quoystmy; jj abandon empty baseline; same isolated story after restoration
```

**Notes**

2/2 each on2unchanged parent runs with the same installed dependencies, and2/2 after restoring the tooltip slice. Temporary baseline oplrtkvt removed; source patch preserved. Failure is intermittent; no pre-existing deterministic failure claim. Logs emoji-same-deps-before/repeat and emoji-after-repeat.
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
pnpm spec:check (source); pnpm spec:first; pnpm spec:validate; pnpm check:progress (consumer qualified)
```

**Notes**

Source validation/mdBook/spec-first3lanes; consumer1protected file/3canonical chapters,5validators/21chapters/147requirements and120tasks. Final evidence formatting/validation is repeated before commit. Logs spec-qualified/spec-first-final/spec-validate-final/progress-final.
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
pnpm ci:storybook-interaction (source full rerun)
```

**Notes**

644/644 tests across208files in69.43s. All required nonvisual source lanes now pass through recorded separate runs; initial pnpm checks wrapper failures remain retained and no uninterrupted wrapper pass is claimed. /tmp/dc-tooltip-restore-stories-rerun.log.
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
pnpm exec prettier --write src/shared/shadcn/tooltip/tooltip-content.svelte src/shared/shadcn/tooltip/Tooltip.stories.svelte src/shared/shadcn/tooltip/Tooltip.sizing.example-sources.ts spec/src/shadcn/overlays.md spec/src/verification.md spec/records/tooltip-sizing-restoration.md (source); pnpm exec prettier --write docs/reference/appearance-provenance.md spec/src/ui.md spec/src/architecture.md spec/src/milestones.md (consumer); pnpm spec:check (source); pnpm spec:first; pnpm spec:validate; pnpm check:progress (consumer)
```

**Notes**

Final evidence and changed files formatted. Source3lanes pass; consumer1protected file/3canonical chapters,5validators/21chapters/147requirements and120tasks pass. Logs /tmp/dc-tooltip-restore-evidence-{format,spec}.log and /tmp/jj-tooltip-restore-evidence-{format,spec-first,spec-validate,progress}.log. Source record links full644test rerun while preserving intermittent failure history and visual EACCES gap. No functional changes after the final passing source and consumer suites.
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
pnpm exec prettier --check src/shared/shadcn/tooltip/tooltip-content.svelte src/shared/shadcn/tooltip/Tooltip.stories.svelte src/shared/shadcn/tooltip/Tooltip.sizing.example-sources.ts spec/src/shadcn/overlays.md spec/src/verification.md spec/records/tooltip-sizing-restoration.md (source); pnpm spec:check (source); pnpm exec prettier --check docs/reference/appearance-provenance.md spec/src/ui.md spec/src/architecture.md spec/src/milestones.md (consumer); pnpm spec:first; pnpm spec:validate; pnpm check:progress (consumer); pnpm exec prettier --write spec/src/milestones.md (final ledger insertion)
```

**Notes**

All changed-file formatting and documentation gates pass. Latest user instruction to ignore further Design Core visual tests is recorded; earlier visual failure retained as history. No further visual run or baseline changes. Logs /tmp/dc-tooltip-restore-evidence-{format-check,spec-final}.log and /tmp/jj-tooltip-restore-evidence-{format-check,spec-first-final,spec-validate-final,progress-final,ledger-format}.log. Final ledger validation is repeated after this insertion.
<!-- /run -->
<!-- /evidence-field -->

