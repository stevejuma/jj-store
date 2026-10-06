---
schema: 1
id: E-DOC-009
---

# E-DOC-009

<!-- evidence-field:id markdown -->
## Id

E-DOC-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M19
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-DOC-002
- JST-DOC-003
- JST-DOC-004
- JST-UI-019
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uunnzywx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Help startup, themed loading and retained-document navigation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0; pnpm 10.32.1; Vite 8.3.1; Playwright 1.63.0; MkDocs Material from locked uv environment; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing Help asset manifest version 1, same-origin CSP and session-only position retained. No public API, repository, credential or storage format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/maintenance/coverage.json
- docs/site/assets/embed.js
- docs/site/assets/layout.css
- docs/site/assets/theme.js
- scripts/docs/assets.mjs
- scripts/docs-assets.test.mjs
- spec/src/current/contracts/api/33-bundled-application-documentation.md
- spec/src/current/contracts/architecture/13-bundled-application-documentation.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/ui/31-bundled-application-documentation.md
- spec/src/current/governance/19-application-documentation-governance.md
- spec/src/current/progress/M19/history/M19/index.md
- spec/src/current/progress/M19/index.md
- spec/src/history/2026/10/06/E-DOC-009/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-DOC-009/details-02.md
- spec/src/history/2026/10/06/E-DOC-009/details-03.md
- spec/src/history/2026/10/06/E-DOC-009/index.md
- spec/src/history/2026/10/06/SUMMARY.nav.md
- spec/src/history/2026/10/06/index.md
- web/src/lib/ui/store-help/StoreHelp.svelte
- web/src/lib/ui/store-help/StoreHelpSkeleton.svelte
- web/src/lib/ui/store-help/help-session.ts
- web/tests/browser/help-theme.spec.ts
- web/tests/browser/help.spec.ts
- web/tests/help-session.test.ts
- spec/src/current/progress/M19/index.md
- spec/src/current/progress/M19/history/M19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/docs-assets.test.mjs#Vite refuses a missing initial Help bundle before
  listening
- scripts/docs-assets.test.mjs#Vite preview refuses an invalid initial Help
  manifest
- scripts/docs-assets.test.mjs#Help retains validated bytes until a safe
  complete replacement is published
- scripts/docs-assets.test.mjs#a cold Vite server reloads rebuilt Help through
  real filesystem publication events
- scripts/dev-ui.test.mjs#missing or invalid Help leaves running services and
  runtime markers intact
- web/tests/help-session.test.ts#Help only accepts its exact same-origin frame
  message protocol
- web/tests/browser/help.spec.ts#Help shows a skeleton until its initial
  document is ready
- web/tests/browser/help.spec.ts#failed Help shows the response error in an app
  toast and recovers through Retry
- web/tests/browser/help.spec.ts#Help page navigation keeps its document and
  chrome without a frame reload
- web/tests/browser/help.spec.ts#a superseded Help page cannot replace the newer
  selection
- web/tests/browser/help-theme.spec.ts#embedded Help follows live application
  appearance across pages and reopening
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- scripts/docs-assets.test.mjs#Help recovers from a missing initial bundle when
  its manifest is published
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-DOC-009 details 2](details-02.md)
- [E-DOC-009 details 3](details-03.md)
