---
schema: 1
id: E-FORGE-BRANCH-STATUS-002
---

# E-FORGE-BRANCH-STATUS-002 details 2

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

Node v26.9.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11/Chromium; released Design Core 0.3.2 and Mira 0.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only placement and sizing. No forge/controller/API/schema, credentials, exact pending request/receipt, draft, operation-pinned diff, persistence, storage, manifest or WAL change. Existing status models and guarded PR/submission actions remain authoritative.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Every submitted bookmark, including dependent lower segments, uses the existing under-title compact PR/push/check row. The existing topmost-only Create PR and Submit stack entry points remain unchanged in scope. Checks badges match compact PR height/radius/text sizing; neutral states including Running share its background/border and Running retains the spinner. Passed/Failed keep their distinct tones and text/icons. Submit stack and Create PR use released Button xs; branch action bars use xs text controls and icon-xs add-commit/kebab controls. The shared maintained chip and foundational components remain unchanged. The new regression verifies dependent-row placement and activation; existing light/dark/context stories follow the updated lower-row contract. No new tests mirror low-impact CSS values. Verification is limited to changed UI and its direct consumers, type/ownership/architecture and owning document checks. Root aggregates, Rust, full catalog, unrelated suite repairs, screenshot comparison and visual baseline updates are out of scope under user direction and targeted policy. Stories remain visual-pending.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review http://127.0.0.1:6046/?path=/story/jj-store-forge-workspace--pull-request-details-beside-stacks. M31 live-provider, native/platform, packaged and visual approval gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-020 and JST-FORGE-005 now specify dependent under-title status
  rows and matching compact control geometry/surfaces. JST-API-001 is explicitly
  unchanged: same observations, PR selection, action guards, exact pending
  identities and read/publication authority."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress and links this scoped UI evidence. No provider,
  native/platform, packaged or visual gate is promoted; wider suite work remains
  deferred.

```
<!-- /evidence-field -->

