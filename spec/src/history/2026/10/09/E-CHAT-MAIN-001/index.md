---
schema: 1
id: E-CHAT-MAIN-001
---

# E-CHAT-MAIN-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-MAIN-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M19
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-018
- JST-CLI-004
- JST-API-001
- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/browsing.md
- docs/site/guides/reviews.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/capabilities.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/site/technical/local.md
- docs/site/technical/publication.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/progress/M19/history/M19/index.md
- spec/src/current/progress/M19/index.md
- spec/src/current/progress/M24/footer-review.md
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
jj --no-pager st; jj rebase -r @ -o ruoowsum; jj --no-pager st; jj --no-pager log -r 'conflicts() & e26a177a::@'
```

**Notes**

Main was clean at empty vnowsuxu on e26a177a. Advanced its empty change onto 7ea50d09 (ruoowsum), retaining all 42 chat-body changes and prototype history. No conflicts; main source tree matches the tip. Chat-body workspace retained, not removed.
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
pnpm dev:backend:build
```

**Notes**

Built main target/debug/jj-store and paired jj in 2m03s. Apple linker warns __eh_frame exceeds the compact unwind table 16MB limit; binaries build successfully. No installed executable replaced. Log: .local/chat-body-main-backend-build.log.
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
pnpm docs:build
```

**Notes**

Initial main help build found stale docs/site/reference/api/installation.md. Log: .local/chat-body-main-docs-build.log.
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
pnpm docs:generate
```

**Notes**

Regenerated installation, collaboration and public type API references from landed contracts. Log: .local/chat-body-main-docs-generate.log.
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
pnpm docs:build
```

**Notes**

Generated references now match; source review gate detected the stale publication acknowledgement. Audited all stale topics: publication, collaboration, local, services, configuration, http, application and contracts. Log: .local/chat-body-main-docs-build2.log.
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
node --input-type=module
```

**Notes**

Invoked scripts/docs/review.mjs review(id, reason) for the eight topics after reading owning contracts/source and updating their mapped manual pages. Documented exact workspace reads, qualified file evidence, independent publication receipts, deferred validation, source/reply references and remaining qualification gates. Generated API comparison stays enforced.
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
pnpm exec prettier --write docs/site/guides/agent-execution.md docs/site/guides/reviews.md docs/site/technical/publication.md docs/site/technical/local.md docs/site/technical/architecture.md docs/site/technical/lifecycle.md docs/site/reference/transport.md docs/site/guides/browsing.md docs/site/reference/capabilities.md spec/src/current/contracts/documentation/08-interactive-agent-execution.md
```

**Notes**

Formatted authored manual changes. Coverage JSON and the M24 footer review were also formatted with pnpm exec prettier --write.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:build
```

**Notes**

Final strict manual build: 9 diagrams in both palettes; 141 files, 21,837,831 bytes; links and anchors valid. Log: .local/chat-body-main-docs-build3.log.
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
node --input-type=module
```

**Notes**

prepareUiDevelopment(main) passed before stopping the old server: main-built executable, bundled ACP control version 3, paired JJ, embedded storage, complete existing fixture and built documentation all available.
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
node .local/chat-body-handoff-snapshot.mjs
```

**Notes**

Saved ignored runtime marker and configuration/fixture fingerprints. Previous runtime belonged to chat-body, server PID 78883 and UI PID 38753; fixture already lives at main/.local/dev-demo.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo pnpm dev:stop
```

**Notes**

Ran in jj-store-chat-body. Owned server and Vite stopped; persistent fixture retained. No reset or reseed.
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
node --input-type=module
```

**Notes**

After stop, restored only the ignored runtime marker with main workspaceRoot/uiRunner and no live PIDs. Verified source configuration and fixture hashes first. This lets the existing launcher retain the saved ui-server.toml instead of selecting an older base configuration; no tracked launcher change.
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
pnpm dev:ui
```

**Notes**

Main workspace starts Vite 7460 and its newly built backend 7462; search ready. New PIDs 29929 and 29928. Log: .local/chat-body-main-start.log.
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
ps -p 78883,38753,29928,29929 -o pid=,command=
```

**Notes**

Only new processes remain; both commands resolve within /Users/stevejuma/code/lapismd/jj-store. Runtime workspaceRoot/uiRunner match main. Configuration and fixture SHA-256 values and repository ID are unchanged.
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
cua_repl cold reload of session eeb7049d90fcd39c6610f89c2f7102bc
```

**Notes**

Saved session list, interrupted/published history, agent/model/thinking and workspace binding reload. Workspace menu title is 12px, regular 400 and semantic muted colour. View files opens the expected Files/Changes rail and two retained paths. README.md loads in its inspector with the comment action. Rebuilt Help opens and closes back to the file view with focus restored. No browser warnings/errors; temporary tab closed. No prompt, binding or preference changed.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-MAIN-001 details 2](details-02.md)
