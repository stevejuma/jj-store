---
schema: 1
id: E-UI-092
---

# E-UI-092

<!-- evidence-field:id markdown -->
## Id

E-UI-092
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

history-query-multiline-feedback-and-local-identity
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wmkpkslo
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main jj-store wmkpkslo/18b51a68 and Design Core snktqqso/57ce147c are clean. Browser inspection finds token classes with ordinary foreground colors and focused pill shadow. Exact new multiline editor regression fails1/4; multiline URL regression fails1/12. Shared compact error lacks invalid styling and diagnostic spacing. User requests direct main-workspace changes and excludes Design Core visual tests. jjbutler current component is read-only and single-line; unrelated changes preserved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Rust1.98.1; pnpm10.32.1; Svelte5/CodeMirror6; macOS ARM64. Design Core0.1.2 source-validation main snktqqso; registry-portable app range ^0.2.0 retained. etcetera0.11.0 and whoami2.1.3 reuse existing locked versions. Qualified source Design Core commit04b1ec1ba7e7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Optional LocalTrusted QueryInfo.default_user_email; explicit context remains query authority. Host JJ_EMAIL/user config then registered repository secure/legacy JJ then Git fallback. Bounded read-only email hints; no alias evaluation/config migration. Multiline query URL whitespace allowed; other controls rejected. No manifest/object/WAL/receipt/companion formats change. Email stays session-only and manual empty/override wins. Registry-only Design Core consumption still requires normal release.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
- catalog/fixtures/HistoryQueryFixture.svelte
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/query/language/identity.rs
- crates/jj-store-jj/src/query/language/mod.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/history-query-route.ts
- web/src/lib/controller/history-query-types.ts
- web/src/lib/controller/history-query.svelte.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/ui/store-history-query/StoreHistoryQuery.svelte
- web/src/lib/ui/store-history-query/history-language.ts
- web/src/lib/ui/store-history-query/history-query.css
- web/tests/browser/history-query.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/history-language.test.ts
- web/tests/vite/history-query.test.ts
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-only Design Core installation still requires a normal package
  release; existing source-validation setup retained. Visual baselines remain
  pending and excluded by user.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared source component crosses repositories, so full required Design Core nonvisual checks apply. App reader context changes justify complete catalog-unit controller lane; native/local language, graph and HTTP contracts remain focused. Root app aggregates, full Rust workspace, unrelated runtime/RustFS and screenshot baseline comparisons are out of scope. Visual tests explicitly excluded; no publishing or release. All requested functional changes and main runtime verification are complete; pending result refers only to registry-only release qualification retained from E-UI-090. Before-state failures are recorded without relabeling.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Requested main changes and runtime feedback are qualified and committed separately. Registry release and wider M13 work remain separate gates.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-092 details 2](details-02.md)
