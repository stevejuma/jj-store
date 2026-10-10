---
schema: 1
id: E-CHAT-FOOTER-003
---

# E-CHAT-FOOTER-003 details 2

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node v26.9.0; pnpm 10.32.1; Vitest 4.1.11; Storybook 10.5.7; released Design Core and Mira dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, schema, appearance, draft or device-local inbox preference format changes. Canonical repository, workspace, agent, Thread/session and receipt identities remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only footer control composition, contextual hover styling, explicit menu sizing and shared Threads/Agents inbox action/header styling. Community inbox CSS and Design Core Button/menu source were inspected first. Standard secondary hover fades onto the same grey group and is visually indistinct; these app-specific compositions intentionally mix semantic secondary/foreground tokens for a distinct hover. Explicit inline menu width is layout through the existing public props; the prior unscanned width utilities did not resize it. No upstream component defect or consumer patch was introduced. No new implementation-mirroring unit tests for reversible styling: existing focused stories and live browser geometry/native pointer checks cover the impact. Root aggregates, Rust, generated public contracts, production built-bundle/release and execution restart flows are out of scope. Fresh browser tabs and cold story processes qualify presentation. Existing development fixture/backend preserved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified UI slice in default workspace, enclosing Jujutsu change pqnxyszz. Continue independent M24 execution, ResizeObserver investigation and explicit visual-baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-015/JST-UI-018: UI 39-intentional-thread-inbox.md specifies
  Agent repositories adjacent to Agent, distinct footer hover, bounded readable
  Workspace menus, compact matching sidebar header insets and bordered split
  actions. API 26-browser-appearance-state.md explicitly confirms unchanged
  selection, authorization, audience, drafts and local pin/archive contracts."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. footer-review.md reviews this slice in
  enclosing change pqnxyszz; evidence/history rows are updated without using a
  future commit hash. Visual baselines remain unapproved. The prior
  ResizeObserver diagnostic reproduced in the initial focused story group and
  remains separately unresolved.

```
<!-- /evidence-field -->

<!-- evidence-field:notes markdown -->
## Notes

Before live inspection: Workspace root menu width=128px despite w-64 utility; Codebase/Workspace resting rgb(240,240,240), inherited secondary hover only fades opacity. Pin/archive group had no border, and secondary hover faded over the same parent background. After: Workspace root width=288px, all three menu rows clientWidth=scrollWidth=278px. The Workspace selection submenu opens and retains two exact repository bindings; no selection was changed. Codebase/Workspace pointer hover uses opaque themed oklab(0.887326 -0.00039352 -0.0010694). Agent repositories uses ghost icon-sm like Agent, positioned 4px before it in the same flex group. Both Threads and Agents sidebar headers show left/right padding=8px, width/client/scroll=358px. Split actions remain absolutely positioned, each 24px, shared border=1px solid rgb(229,229,230), grey resting rgb(240,240,240). Native pointer over either action darkens only that button; the other remains resting grey. No pin/archive action, prompt or repository mutation was submitted, and user drafts/selection are untouched. Temporary browser interaction raced HMR twice when closing a menu; missing locators were re-observed and no action retried blindly. Screenshots under ignored .local/footer-controls-proof preserve live geometry. Restored baseline M24 history table padding and added only the new row; final formatting avoids unrelated historical churn. Both fresh live tabs have empty warning/error logs; no ResizeObserver warning appeared there. The initial focused story group did reproduce it, so no claim of resolution is made. All three agent-created proof tabs were closed; the user tab remains open with HMR changes. No development service was restarted or reset.
<!-- /evidence-field -->

