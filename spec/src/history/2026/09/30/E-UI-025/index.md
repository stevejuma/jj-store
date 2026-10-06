---
schema: 1
id: E-UI-025
---

# E-UI-025

<!-- evidence-field:id markdown -->
## Id

E-UI-025
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
live computed style inspection of the final branch card at http://127.0.0.1:7460/
```

**Notes**

Before the change, every bookmark card had a 0px radius, while the branch command view container used an 8px radius with hidden overflow and ended on the final card, clipping release/0.1 into rounded lower corners.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte --reporter=dot
```

**Notes**

All three branch-card stories pass. The command view container and every card, including the final visible card, keep square lower corners and a 1px bottom divider. Visual baselines were skipped.
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
pnpm web:build
```

**Notes**

The production browser bundle builds with the existing Design Core global-selector and large-chunk warnings. The rebuilt stylesheet sets the branch command view radius to 0px.
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
live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

After reloading the rebuilt assets, the final release/0.1 card matches the other rows: 0px radius, 1px bottom divider, and a square command view container, including while that card is selected.
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
pnpm web:check && pnpm spec:first && pnpm check:progress
```

**Notes**

Svelte diagnostics report no warnings. The spec-first gate maps the production and catalog changes to the API, UI and progress chapters, and the milestone ledger stays valid.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt or API changes. Branch presentation keeps square card corners by setting the command view container radius to 0px.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


