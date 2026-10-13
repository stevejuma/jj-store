---
schema: 1
id: E-WORKFLOW-001
---

# Workflow selector verification

Before this UI follow-up, timeout's unit addon used a separate tinted surface,
author matching wrapped below selection, and thread/reaction scope used raw text
fields. JST-WF-008 now describes the maintained Command View, avatar and emoji
picker composition. Native source/filter and publication contracts are unchanged.
The selected source uses the existing authorized repository lease; stale reads
are discarded, resources released and failures reported through shared toasts.
No Design Core source, generated API or backend was changed in this follow-up.

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm web:check
pnpm catalog:check
```

**Notes**

Final checks: zero errors/warnings; 422 catalog modules pass ownership boundaries.
Logs: /tmp/jj-workflow-picker-qualified-web.log and
/tmp/jj-workflow-picker-qualified-catalog.log. Initial checks found two nullable
filter prop mismatches and two captured-ID warnings; nullable values now map to
undefined and toast identities are derived. Intermediate rechecks also passed.
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
pnpm test:storybook catalog/components/store-workflows/WorkflowTriggers.stories.svelte catalog/components/store-workflows/WorkflowEditor.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-application/StoreNumericSetting.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte
```

**Notes**

29 tests pass across six focused story files. The shared owned picker changed,
so its two direct consumer stories supplement workflow/editor and numeric-entry
stories. Acceptance covers uniform numeric-unit/popup surfaces, canonical
seconds retained after selecting minutes, author-row alignment and avatars,
existing-thread selection, custom reaction IDs retained through YAML, Unicode
preset selection and an onscreen popup. Existing pending-command and editor
regressions pass. Log: /tmp/jj-workflow-picker-qualified-stories.log. Prior
iterations: 24 stories, three narrowed trigger stories, then eight trigger/shared
picker stories passed. Wider repository/browser suites remain out of scope.
<!-- /run -->
<!-- /evidence-field -->

Cold CUA checks used only the owned Storybook application tab. At 390x844,
Include and Authors share y=411.03125 with non-overlapping widths; the selector
renders an avatar. The workflow emoji popup occupies x=8..382 and y=8..406.921875,
and its Unicode preset selects the exact intended value. The default viewport
was restored. The earlier unconstrained popup rendered its presets above the
screen and a browser click hit an unintended emoji; local height constraints
and eight-pixel collision padding corrected it. Other emoji consumers retain
their default behavior. The user tab and its drafts were untouched.

No Rust check/rebuild or contract regeneration was rerun for these UI-only
changes; the enclosing slice's Rust, real-storage and generated-contract
evidence remains in details-02. M28 stays In progress with the provider,
unchanged Forgejo protocol and visual approval gaps in details-03. These checks
do not approve visual baselines.

Final bookkeeping and documentation commands:

```shell
pnpm check:architecture
pnpm check:ui-ownership
pnpm docs:review application "Documented the source-scoped thread Command View, author avatars and inline matching controls, bounded workflow emoji picker and uniform timeout unit surface; existing owned controls retain their defaults."
pnpm docs:review
PATH=/Users/stevejuma/.local/bin:$PATH pnpm docs:check
pnpm spec:validate
pnpm check:progress
```

All passed: 1803 production/catalog modules, ownership validation, documentation
review, nine Mermaid diagrams, 143-file strict documentation closure, seven spec
validators (3722 chapters/196 requirements), and 153 tasks/778 evidence records.
Logs use /tmp/jj-workflow-picker-qualified-*; the completed documentation log is
/tmp/jj-workflow-picker-qualified-docs-complete.log. The first unprefixed
docs:check failed because uv was absent from inherited PATH; the existing
/Users/stevejuma/.local/bin/uv was used without installation or configuration
changes. Selected authored files were formatted with pnpm exec prettier.

The proof screenshot /tmp/jj-store-workflow-pickers-final.png shows the default
viewport, aligned Include/Authors controls and an avatar in the Command View.
Only temporary Storybook tabs and the isolated port-6068 Storybook process were
closed. The development Vite/backend listeners and preserved fixture remain.

Spec-first passed after the completed source/documentation updates: 138 protected
files and 13 canonical chapters. Actual command: `pnpm spec:first`; log:
/tmp/jj-workflow-picker-qualified-first.log. Selected-file `pnpm exec prettier
--check` also passed, recorded in /tmp/jj-workflow-picker-qualified-format.log.
This final addition records those completed results without changing the source
or contracts. Scoped evidence navigation was regenerated with the existing
writeIndexPages/writeNavigation helpers; unrelated navigation was not rebuilt.
