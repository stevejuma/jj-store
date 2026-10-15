---
schema: 1
id: E-FORGE-FLOAT-STYLE-001
---

# E-FORGE-FLOAT-STYLE-001 details 3

<!-- evidence-field:scope markdown -->
## Scope

Create-PR composer and PR-details float/dock controls use Lucide picture-in-picture-2. The floating PR component drops its own outer border and corner radius, uses the same background for header/body and adds a header bottom divider. Its inline variant retains the existing bordered card. Bottom spacing now belongs to the inline PR component instead of the shared sections container, so detaching leaves the diff at the top and redocking restores separation. Existing maintained PR and Op Log-derived composition and released Design Core window styles were inspected; no new Community source, sibling dependency or foundation override. The narrow regression measures actual file placement across float/redock. No tests were added for the low-impact icon/decorative style values. Verification stays on the changed UI and its 15 direct story consumers; unrelated Stack cases, standalone inspection stories, controller/Rust lanes, root aggregates, broader suite fixes and visual baselines are out of scope under user direction and targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks and float/redock its PR. M31 live-provider, native/platform, packaged and visual qualification remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 now specifies the floating PR surface, icon and inline-owned
  spacing. JST-FORGE-004 editing and JST-API-001 transport/authority contracts
  remain unchanged: only presentation classes and icons changed, and the same
  content/editor is reparented by the existing adapter."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress and links this scoped presentation evidence. No
  live-provider, native/platform, packaged or visual gate is promoted; wider
  suite work remains deferred.

```
<!-- /evidence-field -->

