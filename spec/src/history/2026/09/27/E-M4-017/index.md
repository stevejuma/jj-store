---
schema: 1
id: E-M4-017
---

# E-M4-017

<!-- evidence-field:id markdown -->
## Id

E-M4-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- .prettierignore
- .prettierrc.json
- Cargo.lock
- README.md
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/examples/browser_preview.rs
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/tests/assets.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/process.rs
- docs/browser.md
- docs/cli/server.md
- docs/evidence/m4-browser.md
- package.json
- pnpm-lock.yaml
- schemas/openapi.json
- scripts/browser-fixtures.mjs
- scripts/contracts.types.ts
- scripts/web-manifest.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec-validator.config.mjs
- web/index.html
- web/playwright.config.ts
- web/src/App.svelte
- web/src/lib/Graph.svelte
- web/src/lib/TextPreview.svelte
- web/src/lib/api/client.ts
- web/src/lib/api/generated.ts
- web/src/lib/graph.ts
- web/src/main.ts
- web/src/style.css
- web/src/tokens.css
- web/svelte.config.js
- web/tests/browser/reads.spec.ts
- web/tests/client.test.ts
- web/tsconfig.json
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/client.test.ts#empty_file_range_retries_a_bounded_full_read
- web/tests/client.test.ts#graph_rendering_bounds_nodes_and_edges_before_layout
- web/tests/browser/reads.spec.ts#native_graphs_files_and_source_rendering_are_accessible
- web/tests/browser/reads.spec.ts#event_reconnect_resumes_and_reset_requires_explicit_refresh
- web/tests/browser/reads.spec.ts#graph_and_directory_pages_retain_exact_snapshots_and_query_context
- web/tests/browser/reads.spec.ts#index_preparation_retries_the_original_query_after_context_edits
- crates/jj-store-http/tests/assets.rs#browser_assets_are_allowlisted_preloaded_and_sandboxed
- crates/jj-store-cli/tests/server_executable.rs#cold_tls_server_serves_only_the_configured_browser_bundle
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-017 details 2](details-02.md)
- [E-M4-017 details 3](details-03.md)
