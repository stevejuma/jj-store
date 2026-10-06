---
schema: 1
id: E-M21-053
---

# E-M21-053

<!-- evidence-field:id markdown -->
## Id

E-M21-053
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

<!-- evidence-field:kind markdown -->
## Kind

integration
<!-- /evidence-field -->

<!-- evidence-field:change markdown -->
## Change

Enclosing Jujutsu change: Integrate Stacks workspace with current main browsing changes
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Rebase the complete Stacks workspace branch onto default workspace yolnqxty / ba084df3. Preserve all native mutation/UI changes and main bounded-read/loading delivery. Resolve source and specification conflicts; retain every historical evidence object without changing decoded content.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
- catalog/fixtures/RepositoryFixture.svelte
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes/archive.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- docs/maintenance/coverage.json
- docs/site/reference/api/administration.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/api/work.md
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
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/http-repository-transports.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/read-query.ts
- web/src/lib/controller/store-settings-types.ts
- web/src/lib/projects/observe-revision-row.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; pnpm 10.32.1; Svelte 5.57.1; existing registry manifests and documented local Design Core validation unchanged
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted, TOML, manifest, WAL, HTTP or RPC format changes. Existing contracts combine both additive route inventories.
<!-- /evidence-field -->

<!-- evidence-field:commands list -->
## Commands

- jj rebase -b @ -o yolnqxty --simplify-parents
- jj squash --from @ --into uqxkytrt --use-destination-message --
  spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-053 details 2](details-02.md)
- [E-M21-053 details 3](details-03.md)
- [E-M21-053 details 4](details-04.md)
- [E-M21-053 details 5](details-05.md)
