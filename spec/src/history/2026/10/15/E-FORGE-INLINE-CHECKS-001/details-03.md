---
schema: 1
id: E-FORGE-INLINE-CHECKS-001
---

# E-FORGE-INLINE-CHECKS-001 details 3

<!-- evidence-field:scope markdown -->
## Scope

PR header action icons have a visible themed hover/focus/open-menu surface on inline and standalone headers, excluding disabled controls. The collapsible title trigger stays transparent in both docked and floating hosts; docked disclosure remains interactive and floating disclosure remains inert. No new tests mirror this low-impact decorative CSS value. Inspect checks expands the PR as needed and toggles the existing checks component below the description in the same body, without changing surface or reselecting an active PR. Checks start hidden. Jobs, logs, capability-gated run controls and refresh reuse the existing selected-revision controller. Toggling retains the draft, selected jobs/logs and adjacent file diff; floating reparents the same component/editor. Existing checks layout rules now support both owned PR hosts and wrap log text. Verification is limited to changed UI and direct stories, catalog types and ownership/architecture checks. Root aggregates, Rust, full catalog, unrelated suite repairs, screenshot comparison and visual baseline updates are out of scope under the user direction and targeted policy. Stories remain visual-pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks, then Inspect checks or float the PR. M31 provider, native/platform, packaged and visual approval gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 and JST-FORGE-005 specify in-body explicit CI inspection and
  contextual header interaction surfaces. JST-API-001 is explicitly unchanged:
  existing reads, selected revisions, drafts, exact intents and remote-effect
  admission remain the same."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress with this narrow UI evidence linked. No
  live-provider, native/platform, packaged or visual gate is promoted; wider
  test-suite work remains deferred.

```
<!-- /evidence-field -->

