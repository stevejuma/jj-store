---
schema: 1
id: E-UI-077
---

# E-UI-077 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (named anchors)
```

**Notes**

Checker accepts named test declarations but not Vitest it alias. Changed these3declarations to equivalent test API; assertions unchanged.
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
pnpm catalog:test web/tests/vite/agent-timeline.test.ts (named declarations); pnpm check:progress (final)
```

**Notes**

3/3 equivalent named regressions and120task evidence bookkeeping pass. /tmp/jj-agent-tool-groups-unit-named.log; /tmp/jj-agent-tool-groups-progress-final.log.
<!-- /run -->
<!-- /evidence-field -->

