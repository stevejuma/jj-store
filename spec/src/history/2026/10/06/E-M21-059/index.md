---
schema: 1
id: E-M21-059
---

# E-M21-059

<!-- evidence-field:id markdown -->
## Id

E-M21-059
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
- M21.2
- M4.11
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

integration
<!-- /evidence-field -->

<!-- evidence-field:change markdown -->
## Change

Enclosing Jujutsu change: Merge local Stacks delivery with bounded repository reads
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Merge the completed Stacks head wuuntsnu/944c7da8 with Main bounded-read head xqkplmwn/9f76be5c in the default workspace. Preserve both original histories, local native JJ semantics, bounded Files reads, portable dependencies, and all historical evidence.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-http/src/local_browser/binding.rs
- crates/jj-store-http/src/local_browser/stacks.rs
- crates/jj-store-http/src/local_browser/working_tree.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/local_browser/git_read.rs
- docs/maintenance/coverage.json
- docs/site/reference/api/code.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
<!-- /evidence-field -->

<!-- evidence-field:baselines markdown -->
## Baselines

Both workspaces clean: Main empty pyyxkkmp/519ea459 above xqkplmwn; Stacks empty trqyosvm/77a6083c above wuuntsnu. Common ancestor yolnqxty/ba084df3. Initial merge had six conflicted files, including three Rust overlaps and additive UI/API/ledger documentation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; Rust 1.98.1; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Existing documented Design Core local validation setup refreshed offline; ^0.2.0 manifest and exact registry lockfile retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No new wire, TOML, native identity, working-copy, companion, WAL, manifest or durable layout format. Combined parent contracts retain their additive last_change_mode and ordinary local Stacks endpoints. No milestone promotion.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-059 details 2](details-02.md)
- [E-M21-059 details 3](details-03.md)
