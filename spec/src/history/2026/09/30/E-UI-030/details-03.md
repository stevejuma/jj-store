---
schema: 1
id: E-UI-030
---

# E-UI-030 details 3

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
pnpm spec:first
```

**Notes**

The 31 protected implementation files map to the three updated canonical chapters.
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
pnpm format:check
```

**Notes**

Rust formatting and tracked-text Prettier checks pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Interface and layout reset controls.
- URL-addressable Settings and browser-history integration.
- Replication controls and status, upstream URLs, credentials, storage
  configuration, backup/restore operations, stock-JJ filesystem mirroring and
  Git/Nostr export.
- Human visual review and baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest browser 4.1.11; Playwright 1.63.0; Design Core ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No server, OpenAPI, generated contract, persistence or wire-format change. The transient UI consumes existing bootstrap, repository, capability, snapshot, search-status and index-task contracts; stories remain visual-pending.
<!-- /evidence-field -->

