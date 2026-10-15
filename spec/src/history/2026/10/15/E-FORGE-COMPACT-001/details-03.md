---
schema: 1
id: E-FORGE-COMPACT-001
---

# E-FORGE-COMPACT-001 details 3

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
gh api sample SHA check-runs and actions/runs; gh pr checks 1 --repo stevejuma/jj-sample-repo --json name,state,workflow,link,startedAt,completedAt
```

**Notes**

Read-only comparison found two executions of the same workflow plus four leaf check observations. Prior app total six duplicated aggregates/history. Adapter now selects the latest execution, emits its two leaf contexts with workflow/check/event labels, and preserves actual timing and run inspection. No live push, edit, rerun/cancel, submission or merge was invoked.
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
Changed-file Prettier; pnpm check:progress; pnpm spec:first; pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Final scoped document, evidence and protected-change validation passed. The prior auth-slice post-commit validation race is replaced by a sequential revision-pinned pass; no commit or source mutation occurred during final validation. Root aggregate lanes remain out of scope.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Live additional providers, PAT/Enterprise and Windows/Linux credential
  adapters remain unqualified.
- Native-storage submission, packaged app and disposable-forge mutation
  qualification remain separate M31 gates.
- Wider test-suite repairs, unchanged strict Clippy warnings, unrelated
  contrast/baselines and release qualification remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS 27.0 ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Rust 1.99.0; Vitest 4.1.11/Chromium; Storybook 10.5.7; GitHub API current sample checks.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version-1 ForgeCheck adds optional provider started_at/completed_at strings with backward-compatible defaults. Workflow/check/event is display text from provider data. No new native format, manifest, WAL, credential-store schema or receipt format. UI read cache is bounded session memory, with no browser persistence, previews, effects or recovery proofs.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Implement user-directed GitHub auth styling, icon menus, single-line timed current checks and collapsed flush checks footer, manual bounded UI read caching, and a bottom guarded merge bar. Preserve device authorization polling, account identity, exact pending operations, authored drafts, merge protection and independent diff/floating state. Only direct Forge/Settings consumers and contracts are verified; separate test work is untouched.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep M31 In progress and qualify platform/provider, packaged, native and disposable mutation gates separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-FORGE-004/005/007 and owning UI/Settings/API/compatibility chapters
  explicitly define manual observation, bounded account-isolated caches, current
  check contexts, optional timing, content-sized menus and footer actions.
  JST-FORGE-003 uncertain-intent recovery is retained; stale identity replies
  cannot clear a pending external effect. Native identity, manifest and WAL
  contracts remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress. Focused automated and cold macOS live read
  evidence qualify this UI/provider slice only; native submission, additional
  provider/platform, packaged, mutation and visual gates remain open.

```
<!-- /evidence-field -->

