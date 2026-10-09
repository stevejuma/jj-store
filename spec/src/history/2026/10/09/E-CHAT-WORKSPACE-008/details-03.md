---
schema: 1
id: E-CHAT-WORKSPACE-008
---

# E-CHAT-WORKSPACE-008 details 3

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
pnpm catalog:check
```

**Notes**

New optional multi-binding fixture captured a reactive prop locally. Wrapped that initial fixture setup in existing untrack convention. Log workspace-008-catalog2.log.
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
pnpm catalog:check
```

**Notes**

Type check also rejected the trial modal root prop; removed. Log workspace-008-catalog3.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI boundary check passes. Public Design Core dropdown-menu and button primitives; no dependency changes or external source imports.
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
pnpm exec prettier --check web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/fixtures/WorkspaceFilesFixture.svelte
```

**Notes**

Changed Svelte files use project formatting.
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
cua_repl fresh live inbox page; open repository menu; ArrowRight on Copy; ArrowDown to Workspace name
```

**Notes**

Verified 12px text, Lucide folder-git-2, menu starts within0.18px of trigger left, left-aligned actions, no displayed filesystem path and both Copy submenu entries. Fresh page warn/error logs are empty. Screenshot in ignored .local/workspace-footer-final.png. No agent prompt, publication, fixture reset or backend restart.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- ResizeObserver loop completed with undelivered notifications remains
  reproducible during existing responsive rail stories; this slice does not
  suppress or claim to resolve it.
- M24 and M13.3 wider historical-source, publication/restart/multi-agent and GC
  acceptance gates remain unchanged. Visual baselines stay unapproved.
- Rust, generated API contract and root aggregate lanes are out of scope for
  this UI-only change.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; pnpm10.32.1; Vitest4.1.11; Svelte5.57.1; macOS ARM64. The enclosing Jujutsu change is this evidence revision.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, storage, source anchor, publication or preference format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Conversation repository footer: standard extra-small ghost button with Git folder icon; leading-edge action menu; Copy submenu for verified project directory and exact workspace name; retained View files and multi-repository inspection. Existing owned Community copy submenu composition and released Design Core primitives were inspected and reused.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the separately recorded workspace qualification and investigate the existing resize warning. No development backend restart needed.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-001/015/018 now specify the compact button and left-aligned Copy
  menu with no visible path. JST-API-001 is explicitly unchanged: copies use
  existing verified inspection fields."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress and M13.3 qualification is unchanged. This menu
  slice cannot close the broader historical-source, publication, recovery or
  visual gates.

```
<!-- /evidence-field -->

