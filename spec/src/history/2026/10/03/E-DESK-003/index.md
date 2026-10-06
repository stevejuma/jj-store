---
schema: 1
id: E-DESK-003
---

# E-DESK-003

<!-- evidence-field:id markdown -->
## Id

E-DESK-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.1
- M17.2
- M17.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

desktop-main-rebase
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yuzrmrwx (enclosing change)
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust1.98.1; JJ0.45.1; Node26.9.0; pnpm10.32.1 (official Tauri dlx invoked12.6.0); independently locked Tauri2.11.5 graph. Existing ignored Design Core source hook retained; not registry-only acceptance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/CLI schema, persisted repository, WAL, receipt or device format change. Main rich history/automatic refresh/navigation retained. Typed application props extracted; unsupported fixture explicitly models the optional reader instead of inferring capability from local profile. Latest main header hover/focus/open CSS extracted with existing provenance.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/application-props.ts
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-app-shell/header-hover.css
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- spec/src/desktop.md
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/api.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- desktop/tests/managed_runtime.rs#real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
- crates/jj-store-cli/tests/desktop_supervision.rs#desktop_owner_eof_stops_real_service_and_retains_installation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-DESK-003 details 2](details-02.md)
- [E-DESK-003 details 3](details-03.md)
