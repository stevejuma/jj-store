---
schema: 1
id: E-FORGE-UNSUBMITTED-001
---

# E-FORGE-UNSUBMITTED-001 details 3

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11 with Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production UI, controller, forge action/schema, session credential, storage or persistence change. Catalog-only fixture state uses the existing ForgePlan/ForgeResponse contracts.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The contextual workspace defaults its first applied stack feature/workspace to no remote PR, showing Create PR without a PR number or CI status, while feature/api keeps its PR/details beside it. Local fixture examples remain available independently of remote PR inventory so Create PR produces a real fixture plan rather than an empty plan. Preview keeps the remote inventory unchanged; only explicit simulated submit adds the PR and changes the badge to #43. The existing alternate-PR story opts into workspacePullRequest to preserve that scenario. Only the two fixture files and their one consuming story file are changed; production components remain unchanged. User scope and targeted policy restrict verification to these six affected stories and the catalog type/boundary checks. Full Storybook, unrelated Stack stories, controller tests, Rust and broader suite work are out of scope. Stories stay visual-pending; no baseline comparisons/updates or production cold-build qualification. The dev server is refreshed only to present the updated fixture.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Inspect http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. Live forge, native/platform, packaged application and visual approval gates remain open in M31.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-020 and JST-FORGE-001 behavior is unchanged. The catalog now
  demonstrates the existing unsubmitted branch/create flow, retains the
  submitted-stack comparison and records fixture-only regression evidence; no
  product requirement or authority changes.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 stays In progress with this evidence linked. No live provider,
  native/platform, packaged or visual gate is promoted; broader tests remain
  deferred.

```
<!-- /evidence-field -->

