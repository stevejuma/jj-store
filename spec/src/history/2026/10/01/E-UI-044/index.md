---
schema: 1
id: E-UI-044
---

# E-UI-044

<!-- evidence-field:id markdown -->
## Id

E-UI-044
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M4.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

development-browser-annotation-policy
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rlvvyrkm.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Live demo HTTP 200 serves strict CSP with same-origin scripts and only the pinned inline style hash. User confirms Annotate works in Storybook but overlays are absent in the live app. Current in-app tab inspection also times out at browser focus emulation, so actual Codex Annotate rendering is not claimed from that diagnostic. Main has active M13 work in ptqprvlv touching shared specifications; this slice is isolated from verified parent vqqlmmol in dev-annotations. New exact development policy regression fails once before implementation because configuration rewriting has no annotation option.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/tests/assets.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/tests/configuration.rs
- scripts/dev-demo.mjs
- scripts/dev-demo-state.mjs
- scripts/dev-demo-state.test.mjs
- scripts/real-browser-acceptance.mjs
- web/tests/real/browser-annotations.spec.ts
- spec/src/api.md
- spec/src/cli.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-demo-state.test.mjs#development browser policy can be enabled and
  disabled idempotently
- crates/jj-store-http/tests/assets.rs#development_annotations_allow_inline_helpers_without_widening_resource_sources
- crates/jj-store-server/tests/configuration.rs#development_browser_annotations_require_explicit_loopback_local_trusted_assets
- web/tests/real/browser-annotations.spec.ts#browser annotation helpers render
  only with the explicit development policy
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-044 details 2](details-02.md)
- [E-UI-044 details 3](details-03.md)
