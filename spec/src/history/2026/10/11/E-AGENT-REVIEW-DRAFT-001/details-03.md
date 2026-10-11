---
schema: 1
id: E-AGENT-REVIEW-DRAFT-001
---

# E-AGENT-REVIEW-DRAFT-001 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentReviewDraft.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte
```

**Notes**

Final 17 stories across three files pass: Document/Classic keyboard disclosure
and draft retention, white avatar card, pending Document receipt recovery,
Classic confirmed/unknown session publication, read-only denial, exact
tool/nested reply retention, Focus/Split remounts and repository/project
response settings. Original assertions remain.
/tmp/jj-session-review-consumers-final.log. Each invocation uses a fresh Vitest
browser process; existing Svelte default-config/Node deprecation diagnostics are
non-failing.
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
pnpm check:ui-ownership
```

**Notes**

Maintained local avatar, comment-card CSS and Markdown composer reused with
released Design Core disclosure. Application-authored provenance updated; no
copied source, sibling dependency, package patch or import added.
/tmp/jj-session-review-ownership.log and
/tmp/jj-session-review-ownership-final.log.
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
CUA background stored-session response: fresh page load, expand Add review, inspect controls and save screenshot
```

**Notes**

Actual selected OpenCode recorded session shows the muted disclosure, review
icon/chevron, white repository-reviewer avatar/editor card and retained
formatting/send controls. Final HMR snapshot confirms expanded review controls.
/tmp/jj-store-session-review-draft.jpg. No review submitted, fixture reset or
backend restart.
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

Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Design Core 0.3.1; Svelte 5.57.1;
Storybook 10.5.7; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->

## Formats

Owned Svelte review presentation; maintained comment-card CSS and Markdown
composer; existing ReviewStore immutable session anchors and exact receipt
fields; Markdown evidence schema 1.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->

## Next

Retain existing native collaboration acceptance and visual-pending status.
Inherited E-NPM-REGISTRY-001 evidence bookkeeping remains assigned to its
package-adoption owner.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->

## Contract review

```yaml
status: changed
reason:
  "JST-UI-015/COLLAB-005 owning UI 02/19 now specify Add review disclosure,
  avatar/comment-card layout, retained draft and visible pending checks.
  JST-API-001 unchanged confirmation in API 23: no permission, anchor, command,
  schema, publication or receipt change."
```

<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->

## Milestone review

```yaml
status: unchanged
reason:
  M9.5 remains Done. Seventeen focused response/comment/view stories qualify
  this presentation correction while preserving previously qualified native
  collaboration semantics. No new milestone completion, runtime/platform
  qualification or visual-baseline approval is claimed.
```

<!-- /evidence-field -->

<!-- evidence-field:excluded_scope markdown -->

## Excluded scope

Root aggregate/full Storybook/Rust/build/release/native-service lanes and
baseline updates are out of scope under the UI-only targeted policy. No
functional test substitutes for visual baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:document_checks markdown -->

## Document checks

Targeted pnpm exec prettier --ignore-path /dev/null --write and --check with
explicit changed files pass. pnpm exec spec-validator validate --only
governance,summary,fileLimits passes (3 validators, 3576 chapters, 181
requirements). pnpm check:progress and pnpm spec:first report only the same five
inherited E-NPM-REGISTRY-001 diagnostics: deleted hook file bookkeeping, prose
file/test references and unsupported run/overall results. These were reproduced
on the preserved package-adoption parent earlier in this session; no new
coverage or owning-chapter error remains. Existing parent evidence is unchanged.
Logs: /tmp/jj-session-review-format.log, /tmp/jj-session-review-docs.log,
/tmp/jj-session-review-progress.log and /tmp/jj-session-review-spec-first.log.
<!-- /evidence-field -->
