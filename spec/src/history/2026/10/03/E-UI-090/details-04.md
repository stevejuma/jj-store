---
schema: 1
id: E-UI-090
---

# E-UI-090 details 4

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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

11 URL/context codec cases. /tmp/rich-search-route-tests.log
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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

Two production-component keyboard/axe cases. /tmp/rich-search-stories.log
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
pnpm web:build
```

**Notes**

Served production build; existing chunk-size warning. /tmp/rich-search-web-build.log
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/history-query.spec.ts (initial)
```

**Notes**

Cold host startup timed out behind shared Cargo lock; no browser test ran. Isolated target/prebuilt host required. /tmp/rich-search-cold-browser.log
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
pnpm check:architecture (extraction iterations)
```

**Notes**

Counted-line budget failures repaired by extracting route fields, lifecycle cleanup and transports. /tmp/rich-search-architecture*.log
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
pnpm check:architecture (final)
```

**Notes**

738 production modules. /tmp/rich-search-architecture-final.log
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

Production ownership passes. Boundary result recorded separately below. /tmp/rich-search-ui-ownership.log
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Three mapped canonical chapters missing; added architecture, CLI qualification and compatibility scope. /tmp/rich-search-spec-first.log
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
pnpm spec:first; pnpm spec:validate (after mapped chapters)
```

**Notes**

54 protected files,8 chapters;5 validators /21 chapters /146 requirements. /tmp/rich-search-spec-first2.log and spec-validate2.log
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write $(jj --no-pager diff --name-only | rg "\.(ts|svelte|css|json|md|yaml)$")
```

**Notes**

Local rg configuration added line prefixes to piped filenames; replaced with subprocess argument array. /tmp/rich-search-final-format.log
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
pnpm packages:pack (jj-store accidental invocation)
```

**Notes**

Script belongs to Design Core; no such script in consumer. Correct owning package pack run follows. /tmp/rich-search-wrong-pack.log
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
pnpm exec vitest run --project storybook (Design Core final)
```

**Notes**

208 files /644 interaction and accessibility tests passed. /tmp/rich-search-dc-storybook-final.log
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
WORKSPACE_STORYBOOK_PORT=9370 pnpm test:workspace:pointer (Design Core)
```

**Notes**

Fresh Node static host,14 passed. /tmp/rich-search-dc-workspace-node.log
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
SHELL_STORYBOOK_PORT=9370 pnpm test:shell:pointer (Design Core)
```

**Notes**

Fresh static host,11 passed. /tmp/rich-search-dc-shell-node.log
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
SHADCN_STORYBOOK_PORT=9370 pnpm test:shadcn:pointer (Design Core)
```

**Notes**

35 passed,one existing skip. /tmp/rich-search-dc-shadcn-node.log
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
AI_CHAT_STORYBOOK_URL=http://127.0.0.1:9370 pnpm test:ai-chat-browser (Design Core)
```

**Notes**

Three passed. /tmp/rich-search-dc-ai-node.log
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check; pnpm spec:validate (Design Core final)
```

**Notes**

All3 governance lanes pass, final evidence validated. /tmp/rich-search-dc-spec-final2.log and validate-final.log. Source committed as ookknysq/b0a8c96e.
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
pnpm exec prettier --check (changed Design Core files)
```

**Notes**

All files formatted. /tmp/rich-search-dc-format-final.log
<!-- /run -->
<!-- /evidence-field -->

