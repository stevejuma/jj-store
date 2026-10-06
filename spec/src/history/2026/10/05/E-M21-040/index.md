---
schema: 1
id: E-M21-040
---

# E-M21-040

<!-- evidence-field:id markdown -->
## Id

E-M21-040
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

sswqouumomzl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Ordinary local jj-lib megamerge workspace core with TOML and production UI
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Pinned jj-lib/JJ 0.45.1 Git SHA-1 profile; toml 1.1.6; config and command/state version 1. Existing released foundation ranges remain portable.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local .jj/jj-store/stacks.toml: namespace, revision, active ID and named target/applied/merge-ref/WIP-ref records. Native JJ trees, commits, operations and working-copy state are unchanged. New registered POST /v1/local/repos/{repo}/stacks generated schema; no native manifest/companion/WAL format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/fixtures/stack-workspace/production-http.ts
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/ordinary_stacks.rs
- crates/jj-store-cli/tests/local_browser/prepared_references.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/local_browser/stacks.rs
- crates/jj-store-http/src/local_browser/stacks/tests.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-jj/src/local_browser/references/prepared.rs
- crates/jj-store-jj/src/local_stacks/config.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/settings.rs
- crates/jj-store-jj/tests/local_stacks.rs
- crates/jj-store-service/src/local_browser.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/local.md
- schemas/openapi.json
- scripts/stacks-browser-acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/eligibility.ts
- web/src/lib/controller/stack-workspace/local-references.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/controller/stack-workspace/local-state.ts
- web/src/lib/controller/stack-workspace/local-transport.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/src/lib/controller/stack-workspace/source-range.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/production-stacks-application.test.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-003
- JST-JJ-004
- JST-UI-020
- JST-API-001
- JST-API-002
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The preceding change contains retained-executor scaffolding and read-only production Stacks. Core commands were unavailable. New engine API adaptations initially failed compilation; six focused native regressions subsequently passed. First cold core run failed missing explicit index preparation; the next exposed successful JJ membership settlement blocked by eager new-operation graph history. Both are corrected, with a direct adapter regression.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The approved simplified ordinary-local profile uses normal jj-lib settings, snapshot/ignore/auto-track/file-size policy, native WC/settings locks, merge trees, rebase and checkout. TOML owns names, target and membership; native bookmarks retain merge/WIP. The existing reviewed production UI and pinned browsing are reused. First core checkpoint only: setup/target, discovery/create/switch and apply/unapply, real bounded Unstaged and native conflict notice. Shared graph ranges reset at a new ordinary-local operation instead of blocking confirmed writes on an index. Internal bookmarks are filtered from ordinary and prepared reference windows. Cross-layer validation covers only these owning packages/direct consumers; the complete registered contract-response lane runs because a generated HTTP contract was added. Root aggregate, unrelated Rust/UI, RustFS, publication, installed binaries/services and visual comparisons/baselines are out of scope.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-040 details 2](details-02.md)
- [E-M21-040 details 3](details-03.md)
- [E-M21-040 details 4](details-04.md)
