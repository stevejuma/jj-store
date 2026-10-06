---
schema: 1
id: E-UI-010
---

# E-UI-010 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Final source builds the static Storybook catalog, production browser and locked Rust workspace. Existing non-failing Design Core CSS minifier and Storybook chunk-size warnings remain.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 canonical lanes pass on final source, including cold catalog, 44 stories, eight production browser scenarios, web/catalog types and units, contracts, Rust workspace, mdBook and spec-first. This completes the graph overlap, native pointer/touch, reduced-motion and aggregate-validation gaps recorded at checkpoint E-UI-009.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Final repository-wide formatting and evidence/specification consumers pass: 43 tasks, 65 requirements, seven protected files and four canonical chapters. Visual baselines remain unapproved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Registry Design Core ^0.2.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Final user refinement: paired controls show four leading characters alone at rest; hover/focus reveals five leading characters, two dots and the final two characters. A single Git or JJ identifier shows that expanded abbreviation immediately at rest. Inactive halves retain their chip. Balanced 5px inline padding; 11px resting IDs, 10px expanded IDs, 9px chips and 8px Copied text with tick. Default paired control fits 122px by 27px with constant outer width. Feedback expands the tapped side and preserves independent feedback tint when the other side expands. CSS animates grid tracks; pointer/focus/feedback select the active half. Graph rows wrap before bookmark/ID overlap. No transport, schema or stored format changes. Stories remain visual-pending; screenshots are not approved baselines. Functional acceptance completes the pending E-UI-009 checkpoint; next action is optional human visual review.
<!-- /evidence-field -->

