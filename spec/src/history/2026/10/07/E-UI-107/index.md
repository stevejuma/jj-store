---
schema: 1
id: E-UI-107
---

# E-UI-107

<!-- evidence-field:id markdown -->
## Id

E-UI-107
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-007
- JST-UI-017
- JST-GOV-008
- JST-API-004
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-search/StoreRecentSearch.stories.svelte
- spec/src/current/contracts/api/03-captured-application-content-search.md
- spec/src/current/contracts/ui/02-repository-owned-community-presentation.md
- spec/src/current/progress/M15/history/M15.5/index.md
- spec/src/current/progress/M15/index.md
- spec/src/history/2026/10/07/E-UI-107/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-UI-107/details-02.md
- spec/src/history/2026/10/07/E-UI-107/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte
- web/src/lib/ui/community-search-command/model.ts
- web/src/lib/ui/community-search-command/search-command.css
- web/src/lib/ui/store-search/StoreSearchCommand.svelte
- web/src/lib/ui/store-search/StoreSearchOverlay.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change lmqzmwxl; Node 24.10.0, pnpm 10.32.1, Vitest 4.1.11, macOS ARM64. Existing Community search component/style provenance: 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0. Released Design Core Command View public icon-size token reused.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP DTO, search query, collaboration format, storage, receipt, authorization or route changes. Private presentation models carry optional entity kinds; missing kind retains the original generic glyph. Existing provenance entries and source ownership unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Workspace search context, section, browse and create actions show the corresponding existing entity glyph. Browse/search use Community lower-right search badges; create uses plus badges. Results retain their original entity icons. Decorative composites remain aria-hidden. Exact callbacks, permission gating, scope focus and navigation unchanged. UI-only targeted policy: no Rust, root aggregate, contract generation or production bundle lanes needed. Visual baseline approval remains separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-007/JST-UI-017 presentation clarified in the owned Community UI
  chapter. JST-API-004/JST-API-005 authorization, bounded source
  identity/navigation and public formats explicitly unchanged in the owning
  content-search chapter. JST-GOV-008 reuses the already attributed Community
  composite and released Design Core token without new components or external
  imports.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M15.5 remains In progress; decorative search icons qualify no deferred
  native Thread/provider, full-flow, or visual-baseline gate.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-107 details 2](details-02.md)
