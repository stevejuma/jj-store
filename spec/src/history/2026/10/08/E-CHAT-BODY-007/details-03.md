---
schema: 1
id: E-CHAT-BODY-007
---

# E-CHAT-BODY-007 details 3

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

Corrected ledger: 146 tasks and 674 evidence records (.local/chat-body-progress-final.log); milestone bookkeeping only.
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
pnpm spec:first
```

**Notes**

51 protected files, 3 canonical chapters (.local/chat-body-spec-first-final.log).
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Human visual comparison and native platform IME remain separate; stories are
  visual-pending and no baselines are approved.
- Inherited scripts/dev-ui.acceptance.mjs line-limit failure remains open. M24
  live adapter, permissions/cancellation, desktop and platform gates are
  unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; installed Svelte 5.57.1, Mira 0.0.2, Design Core 0.1.2, Storybook 10.5.7, Vitest 4.1.11 and Playwright 1.63.0. Package manifests/lockfile unchanged. Version inspection used filesystem package metadata after package exports refused require(package.json).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Catalog-only Delta projection v2 adds discussions and reply/batch boundaries beside the retained v1 Bedtime reference. UTF-8 source offsets convert to block-local UTF-16 anchors including Markdown splits/trimming. Presentation submission/status types change only inside the prototype; no HTTP, ACP, native storage, WAL, manifest, AgentStore or ReviewStore format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One isolated production-component/catalog surface in the existing jj-store-chat-body workspace. Source, state/adapter, full affected story/axe family, relevant types/boundaries/ownership, exact spacing and cold browser lanes apply. No Rust, root aggregate check/test/build/spec:check, backend restarts, production dispatch, package release or durable persistence. No direct browser-tool bypass of the earlier preview policy rejection. Simulated replies are explicitly labelled; native Delta behavior not directly exercised remains based on the supplied observations and screenshots.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the Delta comments reference and existing Bedtime story. Keep pending visual/native IME and independent live-integration/M24 gates separate from this verified UI prototype.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 in ui/36 govern shared queued drafts, immutable
  deliveries, source/reply navigation, unavailable anchors and consistent
  spacing. JST-API-001 explicitly unchanged in api/41; this presentation-only
  callback does not publish or dispatch. Governance documents the sanitized
  catalog projection and reviewed source mappings.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This isolated UI lifecycle does not close
  visual/native IME, real-adapter, permissions/cancellation, desktop or platform
  gates.

```
<!-- /evidence-field -->

