---
schema: 1
id: E-M11-006
---

# E-M11-006 details 3

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
CUA cold main Projects, Repositories and Reviews built-UI verification
```

**Notes**

Fresh background tab opens Projects with four landing tabs and icon empty state; native repository loads Maya Chen contributor and 11 commits/8 reviews/0 tasks activity; all eight native review rows load identity, exact artifacts, counts, actor and relative date. Runtime error logs empty. Screenshot .scratch/projects-main-reviews.png. Temporary tab closed; both user tabs preserved.
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
pnpm spec:first --base <full common-ancestor commit ID> --head <full integration commit ID> (final ledger)
```

**Notes**

132 protected files and 11 canonical chapters pass. Actual full SHA command is recorded in .scratch/projects-main-spec-first-final.log. Both aliases and default merge-parent selection failures are resolved by explicit native commit IDs.
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
pnpm check:progress (final ledger)
```

**Notes**

96 tasks pass. .scratch/projects-main-progress-final.log.
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
pnpm exec prettier --check spec/src/milestones.md spec/src/verification.md
```

**Notes**

Both changed canonical documents pass. .scratch/projects-main-format-final.log.
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
Compare parsed evidence to both merge parents and inspect merged diff
```

**Notes**

Every inherited parent evidence entry is structurally identical; no conflict markers. Integration-specific edits are confined to the ledger and verification matrix. Main editor guard and both file-navigation regressions preserved; all other feature production bytes match nuwmlxxk.
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
CUA refresh existing main user tab after verifying no editor draft
```

**Notes**

User main tab reloads rebuilt assets at exact original operation/bookmark/commit/root route. Repository table visible, File name input count 0, workspace picker enabled and runtime errors empty. Existing Storybook tab preserved.
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

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; existing embedded RustFS development runtime.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No integration-specific native object, WAL, receipt, manifest or public contract change. Qualified feature tree and main editor slice preserved in a two-parent merge. Existing repository, installation identity and RustFS volume reused by the owned restart.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse committed E-M11-005 native, API and focused feature qualification. Check merged UI and direct Projects/file-editor stories, then cold built file-editor navigation and actual main server readiness. Native CLI/RustFS build is required only by the explicitly requested owned development restart. Root aggregates, full Rust workspace, unrelated stories and source release qualification are out of scope. M11/M11.4 remain In progress because shared release qualification and human visual baseline approval remain pending; no release or baseline claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain merged Projects and repository editor regressions; shared package release and visual approval remain separate.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Shared Community source release qualification and registry publication remain
  pending as documented in E-M11-005; ignored local snapshot is only development
  validation.
- Changed stories remain visual-pending; human baseline approval is separate.
- Root aggregates, full Rust workspace, unrelated story/test suites and
  additional native fault qualification are out of scope for this integration.
<!-- /evidence-field -->

