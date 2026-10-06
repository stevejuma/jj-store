---
schema: 1
id: E-M21-052
---

# E-M21-052

<!-- evidence-field:id markdown -->
## Id

E-M21-052
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Cold ordinary-local mutation acceptance, retained diff and graph preparation recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; current pnpm 10.32.1 toolchain; macOS ARM64 Chromium
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local version-1 command/read DTOs and .jj/jj-store/stacks.toml unchanged; optional frontend indexRequired condition
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-stacks-lifecycle.test.ts#required graph preparation
  survives cleared transient feedback until recovery
- web/tests/vite/production-stack-adapter.test.ts#newly loaded ancestor
  destinations are usable without a refresh
- web/tests/real/stacks-browser.spec.ts#cold_native_selected_file_and_hunk_mutations_with_undo_redo
- web/tests/vite/stack-workspace-lazy-controller.test.ts#same-scope refresh and
  history retain loaded file selections
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-052 details 2](details-02.md)
- [E-M21-052 details 3](details-03.md)
- [E-M21-052 details 4](details-04.md)
- [E-M21-052 details 5](details-05.md)
- [E-M21-052 details 6](details-06.md)
- [E-M21-052 details 7](details-07.md)
