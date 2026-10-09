---
schema: 1
id: E-CHAT-LIVE-010
---

# E-CHAT-LIVE-010

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Agent defaults table composition
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kzusyyzv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Configuration rendered a bordered table inside another settings table. The outer row grid compressed all configuration fields to 368px in an 1110px table. Advanced had an additional rounded border.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

The shared configuration table accepts trailing owned content. Agent defaults places Advanced and Save defaults in that table, removing the nested table. Advanced uses unbordered editable Field.Group rows, retaining the shared top divider and collapse/draft semantics. Community forms and Design Core Field/Composer sources were inspected; no foundational defect or CSS workaround was needed.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live

A fresh live browser page inspected Agents settings without saving preferences or entering credentials. All four configuration rows measure 1003px inside a 1005px table; no nested table remains. Advanced opens with full-width environment controls. Screenshot: .local/agent-defaults-fixed.jpg. A pre-existing timed-out defaults toast was present; subsequent advertised model status loaded successfully. Two old tabs timed out during reconnection; the fresh tab succeeded.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-execution/StoreAgentConfiguration.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- catalog/components/store-application/AgentDefaults.stories.svelte
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte -t 'Defaults last|Global defaults Advanced'
```

**Notes**

No tests selected: four skipped. Corrected to actual generated story names; this is not validation.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte
```

**Notes**

Before: two pass/two fail. Advanced exposes an unwanted side border. The new width assertion initially used combobox for the Select button; corrected to its accessible label.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte -t DefaultsLastWithSharedHarnessModelPickerAndJoinedSave
```

**Notes**

Corrected exact before regression against unchanged production: width 368px, expected &gt;1106px. One fail, three skipped. Production restored to fixed composition afterward.
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
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

Final: seven interaction/accessibility stories pass in a fresh Vitest browser process. Full-width label/control geometry, discovery positioning, one Advanced divider, joined save, provider choices, retry and retained drafts.
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
pnpm web:check
```

**Notes**

Zero errors/warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries pass.
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
pnpm check:ui-ownership
```

**Notes**

Owned production boundary passes.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --write web/src/lib/ui/store-agent-execution/StoreAgentConfiguration.svelte web/src/lib/ui/store-settings/StoreAgentDefaults.svelte catalog/components/store-application/AgentDefaults.stories.svelte spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```
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
pnpm progress:history
```

**Notes**

M24 entry refreshed; unrelated generated M25/Bootstrap formatting restored.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,file-limits,book
```

**Notes**

CLI option typo: file-limits is not a validator ID. Corrected to fileLimits.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,book
```
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
pnpm spec:first
```

**Notes**

Three protected implementation/story files mapped to owning UI/API chapters.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M24 independent adapter, permissions, native IME, desktop/platform and visual
  gates remain open.
- Earlier broad agent story failures, development acceptance-script line cap and
  temporary foundational dependency setup are retained separately in
  E-CHAT-LIVE-009.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; macOS ARM64. Existing dependency validation setup retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, request, receipt, stored preference or provider capability changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI settings composition and its direct defaults/Add agent consumers. No Rust, root aggregate or unrelated broad agent family lanes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the requested live Document input alignment and independent M24 qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-010 details 2](details-02.md)
