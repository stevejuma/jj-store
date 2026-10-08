---
schema: 1
id: E-CHAT-BODY-004
---

# E-CHAT-BODY-004

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Adjacent rendered-block focus-outline clearance
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change votpxwpu above a14b55fa in chat-body workspace
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The user screenshot shows the focused final Assistant message followed by a shaded shell-code section covering the bottom outline. The owned CSS reserved a collapsed 4px margin while its 1px outline and 4px offset extend 5px. Existing reference interactions passed before the edit; they do not assert this visual defect.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Adjacent text readers, text-to-custom sections and custom-to-text readers now reserve a stable 0.5rem (8px at the catalog root size) start margin. This clears the 5px outline and leaves 3px separation. First-block margins and avatar alignment remain unchanged. Spacing is independent of focus so focus changes introduce no reflow. The outline itself, immutable content, comments, disclosures, code backgrounds and editor integrations are unchanged. No new test was written for this reversible stylesheet-only adjustment; the screenshot, source clearance calculation and existing interaction/accessibility family are the evidence.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-chat-body/text-reader.css
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
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
jj --no-pager st
```

**Notes**

Clean working change before edits; preserve prototype/review parents.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Before: 1 passed, 13 skipped; visual defect remains evidenced by the supplied screenshot and 4px versus 5px source calculation. After: same exact target 1 passed, 13 skipped. Logs: .local/chat-focus-reference-before.log and chat-focus-reference-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

14/14 directly affected interaction/accessibility stories pass in a fresh runner. Includes avatar alignment, immutable comments, focus return, file disclosures, raw details, reactions/XML, navigation, draft/footer, dark and narrow cases. Functional evidence only; no visual baseline comparison.
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
pnpm web:check
```

**Notes**

0 errors, 0 warnings.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community Markdown spacing inspected before public Design Core diff presentation. No new dependencies, copies or checkout imports.
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
pnpm exec prettier --write web/src/lib/ui/store-agent-chat-body/text-reader.css spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/contracts/api/41-interactive-agent-execution.md
```

**Notes**

Changed authored stylesheet and owning chapters formatted.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial evidence listed Storybook references in the named-test field, which accepts named unit/function definitions only. Removed those unsupported references; exact executed story commands remain recorded in runs. No tests or checker rules changed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The same two invalid evidence references prevented trustworthy change-set selection before their correction.
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
pnpm check:progress
```

**Notes**

After correcting evidence metadata: 146 tasks and 671 records; M24 unchanged.
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
pnpm spec:first
```

**Notes**

After correction: 1 protected file and 2 canonical chapters.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

5 document validators pass; 181 requirements. Generated evidence stays within authored Markdown limits.
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
pnpm exec prettier --write spec/src/current/progress/M24/index.md
```

**Notes**

Formatted authored milestone after navigation generation.
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
pnpm exec prettier --check web/src/lib/ui/store-agent-chat-body/text-reader.css spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

All matched changed authored sources pass formatting. Evidence uses generator formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Stories retain visual-pending status; no screenshot baseline approval or
  native IME qualification.
- Independent M24 adapter/permission/desktop/platform gates and the previously
  recorded architecture script-size failure remain open.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Inherited unchanged Delta 0.1.0 fixture; Svelte 5.57.1; Mira 0.0.2; Storybook 10.5.7; Vitest 4.1.11; Node 26.9.0. No package/lockfile changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

All models, retained source/UTF-16 anchors, persistence, HTTP/ACP and AgentStore/ReviewStore formats unchanged. Adjacent owned sections gain only stable layout spacing.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI/catalog stylesheet surface and owning evidence. Existing exact reference and 14-story family, web type check, ownership, formatting and consuming document gates apply. No new test/baseline, browser tooling retry, native harness, root aggregate or Rust lane for this CSS-only correction; startup/lifecycle are unchanged. Unaffected unit models/catalog types and independent qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the focus outline on the running reference; retain separate visual baseline and independent M24 gates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-004 details 2](details-02.md)
