---
schema: 1
id: E-UI-INLINE-LINKS-001
---

# E-UI-INLINE-LINKS-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```

**Notes**

Ownership validator passed. All new Community-namespaced presentation files have application-owned declarations; icon paths and Buzz read-only reference have version/license provenance. No checkout dependency.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

1983 modules passed before final evidence-only changes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CUA: inspect http://127.0.0.1:6046/?path=/story/jj-store-appearance-links--inline-links-with-local-site-icons
```

**Notes**

Live Storybook visibly renders baseline-aligned monochrome GitHub, Slack, Figma and other local icons with underlined labels. Vite on 7460 is serving this workspace via HMR. No browser preferences were overwritten. This observation is not visual-baseline approval.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm evidence:new -- --id E-UI-INLINE-LINKS-001 --task M16 --requirement JST-UI-018; writeEvidence; updateNavigation
```

**Notes**

Created the governed Markdown evidence. Repository-wide navigation regeneration also touched unrelated stale indexes; restored those generated changes and retained only the owning pages and new evidence/day navigation. Evidence tests use run references because the named-test parser accepts test()/fn definitions, not these it()/Story declarations.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreLinkRendering.stories.svelte -t RichCardsInANarrowDarkColumn
```

**Notes**

1 passed / 2 filtered after matching fixture/global dark inputs. Link-only dark accessibility remains clean; the unrelated post-redock diff-text finding remains explicit.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreLinkRendering.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

After user requested visible GitLab/Forgejo examples: 8 passed in 2 story files. Both icons are asserted in all link layouts; code.forgejo.org is an explicit recognised host. Settings preview shows all three forges. Log jj-store-inline-icons-stories.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/site-links.test.ts catalog/fixtures/delta-chat/markdown-reading.test.ts
```

**Notes**

Final after GitLab/Forgejo follow-up: 67 passed in 3 files. Log jj-store-inline-icons-units.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```

**Notes**

Both checks passed with zero errors and warnings; catalog boundaries passed across 459 modules. Final checks are repeated only because the user then added visible GitLab/Forgejo examples.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification; pnpm check:progress
```

**Notes**

Initial document/progress validation passed. Rechecked after trimming unrelated navigation regeneration and updating this final evidence. No wider product tests or root aggregate lanes ran.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first
```

**Notes**

The provenance inventory also maps to the owning governance chapter. Added this slice to that chapter under unchanged JST-GOV-002/008 ownership rules; no policy or gate was weakened.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Intermediate evidence rewriting retained two obsolete detail-page paths in its file inventory. Rebuilt the scoped evidence inventory after splitting; no historical evidence or implementation was changed.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm exec spec-validator validate --only summary,fileLimits,governance,verification; pnpm exec spec-validator first; pnpm exec prettier --check <all changed slice files>
```

**Notes**

Final document acceptance passed after scoped inventory correction: progress 156 tasks / 833 evidence records; 4 validators / 3956 chapters / 205 requirements; spec-first 20 protected files / 3 canonical chapters; formatting 32 slice files. Web/catalog also passed with 0 errors and 0 warnings. No Rust, root aggregates, unrelated test repair or visual baseline work.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Unchanged dark PR post-redock diff-text contrast (4.34 versus 4.5) remains
  outside the link-rendering slice; no accessibility suppression or unrelated
  foundation repair.
- Visual baseline approval and inherited M16 release/registry/platform gates
  remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; Storybook 10.5.7 / Vitest 4.1.11; Svelte 5; released Design Core 0.3.2 and Mira 0.0.3; Simple Icons 16.0.0 plus Slack 15.0.0 (CC0-1.0).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing jj-store.appearance.v1 JSON, version=1: links accepts inline/compact/rich; invalid/unset becomes inline while existing compact/rich is retained. No backend API/schema, token, source-Markdown, native entity, file/media or publication format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared application-owned rendered Markdown, local URL/site classification, bundled SVG paths, Appearance controls/previews and contextual PR stories. Temporary preview never persists. Cards require a supported bare/autolink URL in a complete top-level paragraph; labelled/prose/list/heading/table/quote links stay inline. No runtime metadata/favicon/image fetch, CSP relaxation or reference checkout import. Root aggregate suites, Rust, unrelated test repairs and visual baselines are out of scope under the targeted policy and explicit user request.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the named unrelated dark diff accessibility and M16 qualification gates for their owning slices. Link rendering is implemented and live through Vite HMR; metadata-backed cards are not part of this scope.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-018 canonical appearance chapter and browser-owned state contract
  now define Inline default, compatible valid saved choices, selective
  standalone cards and local icon/destination behavior. JST-GOV-002/008
  ownership rules remain unchanged; the owning governance chapter records the
  independently owned renderer and bundled icon provenance.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M16 remains Partial. Focused unit and link/Settings/PR interaction
  evidence does not close inherited dark diff accessibility, visual approval,
  release/registry or platform gates.

```
<!-- /evidence-field -->

