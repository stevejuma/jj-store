---
schema: 1
id: E-UI-072
---

# E-UI-072 details 3

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
CUA read-only live inline Agents on HMR 7460
```

**Notes**

Landing session switch, cold reload restoring exact paired IDs, close/opener focus and Back/Forward pass. Selected-project response stays beside the scoped list. Screenshot /tmp/jj-project-agents-inline.png. Locator incorrectly requested region for transcript log and was corrected from DOM evidence; no fixture mutation.
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
pnpm test:storybook catalog/components/store-work catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/RepositoryProjects.stories.svelte
```

**Notes**

64/64 directly affected stories pass, including all project tabs, inline response, layout alignment, discussions and native agent menus, with blocking accessibility.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final web/catalog zero errors/warnings;134 catalog/760 production modules;35 protected files/6 chapters;5 validators/21 chapters/147 requirements. Repository Agents also verified live retains original response column beside session list.
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
pnpm check:progress; pnpm check:ui-ownership
```

**Notes**

Final corrected ledger and repository-owned presentation pass. Visual baseline approval remains separate.
<!-- /run -->
<!-- /evidence-field -->

