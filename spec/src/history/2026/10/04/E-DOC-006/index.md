---
schema: 1
id: E-DOC-006
---

# E-DOC-006

<!-- evidence-field:id markdown -->
## Id

E-DOC-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M19
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zpoqtoun
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Main integration of application manual and bundled Help
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; MkDocs 1.6.1 / Material 9.6.23; Playwright 1.63.0 Chromium/WebKit; inherited foundational source validation setup.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new repository, API, preference or asset manifest format. CLI/API references regenerated against merged main contracts.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- .prettierignore
- README.md
- crates/jj-store-cli/examples/docs_reference.rs
- crates/jj-store-http/src/assets/help.rs
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/tests/assets.rs
- crates/jj-store-server/tests/configuration.rs
- docs/cli/server-config.md
- docs/cli/work.md
- docs/maintenance/arguments.json
- docs/maintenance/coverage.json
- docs/site/assets/embed.js
- docs/site/assets/layout.css
- docs/site/assets/manual.css
- docs/site/assets/theme.js
- docs/site/cli/index.md
- docs/site/cli/jj-agent.md
- docs/site/cli/jj-project.md
- docs/site/cli/jj-replica.md
- docs/site/cli/jj-review.md
- docs/site/cli/jj-store-client.md
- docs/site/cli/jj-store.md
- docs/site/cli/jj-sync.md
- docs/site/cli/jj-task.md
- docs/site/concepts/collaboration.md
- docs/site/concepts/glossary.md
- docs/site/concepts/projects.md
- docs/site/concepts/repositories.md
- docs/site/concepts/snapshots.md
- docs/site/contributing/development.md
- docs/site/contributing/documentation.md
- docs/site/faq.md
- docs/site/getting-started/contexts.md
- docs/site/getting-started/install.md
- docs/site/getting-started/local.md
- docs/site/getting-started/native.md
- docs/site/getting-started/work.md
- docs/site/guides/browsing.md
- docs/site/guides/capture.md
- docs/site/guides/conversion.md
- docs/site/guides/editing.md
- docs/site/guides/git.md
- docs/site/guides/reviews.md
- docs/site/guides/search.md
- docs/site/guides/settings.md
- docs/site/guides/sync.md
- docs/site/guides/troubleshooting.md
- docs/site/guides/work.md
- docs/site/index.md
- docs/site/reference/api/administration.md
- docs/site/reference/api/code.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/search.md
- docs/site/reference/api/types.md
- docs/site/reference/api/work.md
- docs/site/reference/capabilities.md
- docs/site/reference/configuration.md
- docs/site/reference/defaults.md
- docs/site/reference/environment.md
- docs/site/reference/http-api.md
- docs/site/reference/queries.md
- docs/site/reference/security.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/capture.md
- docs/site/technical/lifecycle.md
- docs/site/technical/local.md
- docs/site/technical/publication.md
- docs/site/technical/reads.md
- docs/site/technical/replication.md
- docs/site/technical/storage.md
- mkdocs.yml
- package.json
- pnpm-lock.yaml
- pyproject.toml
- scripts/check-progress.mjs
- scripts/desktop-resources.mjs
- scripts/desktop.mjs
- scripts/docs/assets.mjs
- scripts/docs/diagrams.mjs
- scripts/docs/hooks.py
- scripts/docs/references.mjs
- scripts/docs/review.mjs
- scripts/docs/site.py
- scripts/docs/theme.mjs
- scripts/docs.mjs
- scripts/docs.test.mjs
- spec/src/SUMMARY.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/desktop.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- spec-validator.config.mjs
- uv.lock
- web/playwright.docs.config.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/src/lib/ui/store-help/StoreHelp.svelte
- web/src/lib/ui/store-help/help-session.ts
- web/tests/browser/help-theme.spec.ts
- web/tests/browser/help.spec.ts
- web/tests/help-session.test.ts
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/docs.test.mjs#shared agent long help links target published manual
  pages
- crates/jj-store-http/tests/assets.rs#documentation_routes_are_preloaded_and_have_scoped_framing_policy
- crates/jj-store-server/tests/configuration.rs#documentation_configuration_examples_use_production_validation
- web/tests/browser/help-theme.spec.ts#embedded Help follows live application
  appearance across pages and reopening
- web/tests/browser/help-theme.spec.ts#standalone documentation retains themed
  search and responsive navigation
- web/tests/browser/help.spec.ts#bundled Help preserves a draft without a
  repository and works without external network
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-DOC-006 details 2](details-02.md)
