---
schema: 1
id: E-UI-045
---

# E-UI-045 details 5

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
pnpm check:progress
```

**Notes**

Final 103 tasks; same-change evidence coverage and all parent evidence retained. .scratch/activity-review-list-progress.log.
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
pnpm exec prettier --check (all changed TS/Svelte/CSS/Markdown paths from jj diff --name-only)
```

**Notes**

All changed paths use Prettier style; checked after source transforms completed.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final enclosing-change evidence block formatted; milestone statuses unchanged.
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

<!-- evidence-field:scope markdown -->
## Scope

One UI presentation/controller behavior with direct consumers. Exact controller, seven direct Storybook files and a cold built-browser read-only owned-demo test. Reviews now use Community full-width list/detail; no sidebar mode controls. Retired column links normalize without losing native identities. Activity uses copied Community measured virtualization and public shadcn ScrollArea with lazy association discovery and deferred session detail. Root aggregates, unrelated Rust, full Storybook and shared release qualification remain out of scope. Adapted work-store/work-seed fixtures are not rerun; the new cold read-only browser test covers the changed flow on existing persisted samples.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence, public API or server format changes. Projects hash tab adds activity. Reads remain separate native code/AgentStore/ReviewStore captures. Default graph pages 20 commits, at most 1000 merged rows, at most 4096 association rows and 16 related sessions; related session details are bounded sanitized captures and disclose truncation. Review comparisons use exact anchored commits/operations and explicit pairs; files are limited to 100 inline patches. Missing original agent snapshots and unsupported code comparison bounds remain explicit. Review additions/deletions come from native text-diff hunks, not path-summary placeholder zeroes. No Review view-mode persistence added; retired review_view=columns is accepted only on Reviews, dropped from canonical routes, and duplicates/invalid values are rejected. Deferred AgentStore discovery is memoized across virtual mounts with at most two readers per captured page.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Rust 1.98.1; Node 24.10.0; pnpm 10.32.1; existing pinned Svelte/Design Core/Community/Chromium and embedded RustFS runtimes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused list/detail, Activity virtualization and history regressions; shared-package release qualification and visual baselines remain separate M11.4 work.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baseline approval remains pending.
- Shared release qualification and independent M11.4 deployment gates remain
  deferred.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/activity-review-*.log, .scratch/activity-history-before.log, .scratch/activity-lazy-*.log and native state captures; web/test-results-real/projects-activity-native-p-693ed-hare-pinned-commit-activity/{project-activity,native-review-list,native-review-detail,anchored-agent-review}.png. Final full-width review list and inset/centered detail screenshots inspected; visual baselines remain pending.
<!-- /evidence-field -->

<!-- evidence-field:live_data markdown -->
## Live data

All three current seeded agent sessions have empty code-association pages (relations=[] and next_cursor=null); the Codex session also has code=[]. Their tools were captured without intervening code changes. Commit-inline context follows only genuine recorded relations and is proven in fixtures, while the live original anchored agent review displays its recorded tools. No timestamp-based or fabricated association, seed or repository mutation added.
<!-- /evidence-field -->

