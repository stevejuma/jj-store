---
schema: 1
id: E-UI-080
---

# E-UI-080 details 5

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
pnpm test:storybook catalog/components/store-work catalog/components/store-application catalog/components/store-threads catalog/components/store-appearance/AppearanceComposer.stories.svelte --shard=1/2 (qualified)
```

**Notes**

23files/92tests passed in a fresh browser. Includes repaired NativeAgentMenus hover assertions. /tmp/jj-agent-comments-consumers-shard1-qualified.log.
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
pnpm test:storybook catalog/components/store-work catalog/components/store-application catalog/components/store-threads catalog/components/store-appearance/AppearanceComposer.stories.svelte --shard=2/2 (qualified)
```

**Notes**

22files/110tests passed in a fresh browser. Combined direct-consumer surface:45files/202tests; no browser closure/runtime errors. /tmp/jj-agent-comments-consumers-shard2-qualified.log.
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
pnpm exec prettier --check (51 changed presentation, generated contract and canonical files; initial)
```

**Notes**

Only milestones.md needed formatting after evidence insertion. /tmp/jj-agent-comments-format-qualified.log.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Formatted ledger after evidence update. /tmp/jj-agent-comments-ledger-format-qualified.log.
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
pnpm spec:validate; pnpm check:progress (after mapped chapter/evidence updates)
```

**Notes**

5validators/21chapters/147requirements;120tasks. Logs spec-validate-qualified/progress-qualified.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check (51 changed files); pnpm spec:first; pnpm spec:validate; pnpm check:progress (final)
```

**Notes**

51files formatted; spec-first49protected files/9chapters;5validators/21chapters/147requirements;120tasks. Final ledger run insertion is formatted and checked again before standing JJ commit. Logs ledger-format-final/format-final/spec-first-final/spec-validate-final/progress-final.
<!-- /run -->
<!-- /evidence-field -->

