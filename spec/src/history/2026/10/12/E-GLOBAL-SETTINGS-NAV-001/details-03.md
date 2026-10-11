---
schema: 1
id: E-GLOBAL-SETTINGS-NAV-001
---

# E-GLOBAL-SETTINGS-NAV-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 22
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Initial architecture audit: two introduced line-count issues, store-route-history and StoreApplication, plus 20 inherited diagnostics. Extracted the browser adapter and removed redundant derived section state; no limits relaxed. .local/settings-navigation/architecture.log.
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
node --input-type=module
```

**Notes**

After refactor, all 20 remaining diagnostics have identical current/parent source line counts. No introduced architecture error remains. .local/settings-navigation/architecture-parent.log.
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
pnpm catalog:test web/tests/vite/global-settings-history.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/repository-settings-restoration.test.ts web/tests/vite/repository-settings-ownership.test.ts
```

**Notes**

27 passed after browser-adapter extraction. Log: .local/settings-navigation/history-delivery.log.
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
pnpm catalog:test web/tests/vite/global-settings-history.test.ts
```

**Notes**

5 passed after strengthening the owner guard regression to jump directly between identical Settings URLs with distinct repository contexts. Log: .local/settings-navigation/global-history-delivery.log.
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
pnpm web:check
```

**Notes**

0 errors/warnings after browser-adapter extraction. Log: .local/settings-navigation/web-delivery.log.
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
pnpm test:storybook catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

8 interaction/Axe stories passed after final application integration. Log: .local/settings-navigation/navigation-delivery.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/GlobalSettingsLoading.stories.svelte
```

**Notes**

6 interaction/Axe stories passed after ensuring empty skeleton footers use a single divider. Log: .local/settings-navigation/skeleton-delivery.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

Documentation closure, Mermaid rendering, links and anchors passed. Log: .local/settings-navigation/docs-delivery.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:links
```

**Notes**

Book links and anchors passed. Log: .local/settings-navigation/spec-links.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Only five inherited E-NPM-REGISTRY-001 errors: missing .pnpmfile.cjs, missing prose pseudo-file, invalid test reference, invalid run result and invalid result. No new ledger diagnostic. Log: .local/settings-navigation/progress.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Same five inherited E-NPM-REGISTRY-001 errors block trustworthy-change-set reporting. Log: .local/settings-navigation/spec-first.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review application 'Global Settings section and tab deep links, validated device-only startup restoration and category-specific loading geometry; existing saved configuration, permissions, controller owners and pending commands retained'
```

**Notes**

Refreshed documentation source review. .local/settings-navigation/docs-review.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review application 'Global Settings section/tab URL history and root-startup resume with an isolated browser navigation adapter; category-specific loading and unchanged saved configuration, controller ownership and authorization'
```

**Notes**

Refreshed documentation source review. .local/settings-navigation/docs-review-final.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:review application 'Global Settings URL/history and validated startup destination with selected-page skeleton geometry, single footer dividers and unchanged server configuration/authorization contracts'
```

**Notes**

Refreshed documentation source review. .local/settings-navigation/docs-review-delivery.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write (changed files from jj --no-pager diff --name-only)
```

**Notes**

Incremental and complete changed-file formatting recorded in format.log, format-refinement.log, format-stories.log, format-final.log, format-extraction.log, format-delivery.log, format-footer.log and format-guard-test.log under .local/settings-navigation.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Five inherited E-NPM-REGISTRY-001 governance diagnostics are separate from
  focused product acceptance. Prior parent reproduction is
  E-GLOBAL-SETTINGS-001; no unrelated evidence repaired here.
- No full-suite, packaged native restart, WKWebView/Linux or release
  qualification. Cold browser document startup and fresh Storybook processes
  qualify the changed web lifecycle. Visual baselines remain unapproved.
- The architecture audit retains 20 inherited source/story line-count
  violations, individually compared against parent 339ca43a. No limit or
  baseline modified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0, pnpm 10.32.1, JJ 0.45.1, Storybook 10.5.7, Vitest 4.1.11. Released Design Core and Mira unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds client-only #/settings?section=...&amp;tab=... and validated jj-store/settings-destination/v1 containing only section/tab. Random history entry IDs retain exact in-session return contexts without persisting repository selections. Server APIs, configuration files, authority, authentication and credentials remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Global Settings skeletons now represent selected section/subtab card groups, aligned fields, previews, disclosures, runtime summaries/actions and shared save footers. Real heading and tabs remain outside ScrollArea during lazy-module loading. Initial data placeholders reuse these geometries; background refresh retains loaded data. The new typed overlay route retains exact repository/Stacks/application context in session and pauses hidden browsing reads. URL parsing rejects unknown, duplicate or incompatible fields. Explicit URLs win; root startup resumes validated last-open settings; leaving clears it. Documentation supersedes the former transient-navigation rule at user request. Published dependencies and .local/dev-demo are preserved; no resets, migrations, Rust lanes or visual baseline changes.
<!-- /evidence-field -->

<!-- evidence-field:live_acceptance markdown -->
## Live acceptance

CUA on localhost: direct Agents/Configuration, reload, Appearance Theme/Preferences Back and Forward restored section/tab. Compute/Resources followed by about:blank and a fresh root document resumed #/settings?section=compute&amp;tab=resources and its selected tab. Fresh-link Back returned to Projects. Profile Settings from jj-store demo, then Local service/Launch and Back restored the byte-identical repository operation/bookmark/commit URL. Loading heading and selected tabs visible before content; restored loaded Compute screenshot at .local/settings-navigation/restored-compute.png. Browser tool timeout recovered by inspecting the existing tab; no native/app services restarted and no live configuration saved. Initial root attempt after concurrent HMR/Projects state returned Projects, so repeated with an explicit Settings selection and isolated fresh document to establish the resume proof.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified slice with Jujutsu. Retain inherited diagnostics and independent native-platform/visual gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 now governs global section/tab URL history and validated
  device startup resume, explicitly replacing transient-only navigation.
  JST-UI-016/018 adds representative global loading presentation. JST-UI-005 and
  JST-API-005/008 retain owned drafts, exact pending identities, stale-response
  handling, read pins and server authorization/contracts. JST-DOC source review
  updated.

```
<!-- /evidence-field -->

