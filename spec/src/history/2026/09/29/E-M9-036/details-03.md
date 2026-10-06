---
schema: 1
id: E-M9-036
---

# E-M9-036 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Specification validation and the cold catalog browser lane pass. Twenty inherited captured production-browser fixtures still wait for the removed Repository ID and Repository token controls.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The first aggregate check reached the progress gate, which rejected unsupported named references to Storybook interaction tests. The evidence ledger now names only supported source tests.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The final aggregate check passes after the evidence references were corrected.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

The required aggregate test lane passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The required aggregate production build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:reset
```

**Notes**

The final cold reset rebuilt and reseeded repository 3afe5080b9388eeba0688b6514aea1b4 on port 7460 for the complete-card interaction inspection.
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
pnpm format:check
```

**Notes**

The final repository formatting gate passes.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The final documentation-consumer rerun detected that the changed cold browser harness also required its mapped spec-governance chapter. The harness boundary is now canonical there.
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
pnpm spec:first; pnpm check:progress; pnpm format:check; node scripts/web-architecture.mjs
```

**Notes**

The final documentation consumers pass for 56 protected files and six canonical chapters; progress is valid, repository formatting is clean and all 238 browser and catalog modules remain within architecture limits.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
- The inherited captured production-browser fixtures still require migration
  from removed Repository ID and Repository token controls before pnpm
  spec:check can pass.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core ^0.2.0; Community ^0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, AgentStore or ReviewStore encoding changes. HTTP collaboration reads preserve exact operation and comparison context needed for anchored review patches. Agent artwork is bundled with recorded sources; agent avatars are square while people and Git authors remain circular. Tool detail width subtracts its complete transcript inset. Review comments use separator-free compact Community messages plus bounded branch ordering and reply guides. Agent and Review list rows expose one full-row selection surface beneath independent controls. Agent and Review canvases enter their resizable two-column wide presentation at the 720px sum of configured minimum widths.
<!-- /evidence-field -->

