---
schema: 1
id: E-UI-UNAVAILABLE-001
---

# E-UI-UNAVAILABLE-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/UnavailableContent.stories.svelte -t "Missing session uses an icon empty state"
```

**Notes**

Initial display-name filter matched no generated Storybook export; six skipped.
Not acceptance evidence; reran entire new family. Log
/tmp/jj-unavailable-before.log.
<!-- /run -->

<!-- run -->

### Run 2

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/UnavailableContent.stories.svelte
```

**Notes**

Before production changes: all six missing named Empty-status assertions fail.
Log /tmp/jj-unavailable-before-all.log. This includes the exact reported
missing-session branch.
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
pnpm catalog:check
```

**Notes**

New fixture type checks before production edits: zero errors/warnings; catalog
boundary audit passes. Log /tmp/jj-unavailable-type.log.
<!-- /run -->

<!-- run -->

### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/UnavailableContent.stories.svelte
```

**Notes**

First post-change run: five pass, workspace-file case fails because the fixture
leaves its sidebar closed. Corrected fixture to open the owning sidebar; no
production assertion weakened. Log /tmp/jj-unavailable-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/UnavailableContent.stories.svelte
```

**Notes**

Fresh process after fixture repair: all six stories pass with configured Axe
checks. Each asserts the named status, Empty primitive, contextual description
and hidden decorative icon; directory navigation, Thread refresh and file close
remain reachable. Log /tmp/jj-unavailable-after-fixed.log.
<!-- /run -->

<!-- run -->

### Run 6

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte
```

**Notes**

Initial consumer run: 12 pass, 6 fail. Three reviews and two workspace cases
time out during lazy loading; AppearanceFocusPreservesChannelAndReplyDrafts
cannot find Focus. No empty-state assertion fails. Log
/tmp/jj-unavailable-consumers.log.
<!-- /run -->

<!-- run -->

### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
python3 baseline comparison wrapper: temporarily load StoreThreads.svelte, StoreReviewThreads.svelte and StoreWorkspaceInspector.svelte bytes via jj file show -r tsptytzx; run pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte -t "AppearanceFocus|CommunitySections|ReviewPageIgnores|CommunityReviewSelection|ChangedFileHeader|FilesAndChanges"; restore all task source bytes in finally
```

**Notes**

Parent-source comparison: five pass,
AppearanceFocusPreservesChannelAndReplyDrafts fails identically; eight unrelated
tests skipped. Task source bytes restored before final rerun. Log
/tmp/jj-unavailable-baseline-consumers.log. No temporary JJ workspace.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --maxWorkers 1 catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte
```

**Notes**

Fresh serialized confirmation: all 18 stories pass with configured accessibility
checks. No source/test assertion edits between initial failures, comparison and
confirmation. Logs /tmp/jj-unavailable-consumers-confirm.log. Earlier
loading/Focus timing failures retained as diagnostics, not represented as a
full-suite pass.
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
pnpm catalog:check
```

**Notes**

After production implementation: zero Svelte errors/warnings and catalog
boundary audit passes. Log /tmp/jj-unavailable-type-after.log.
<!-- /run -->

<!-- run -->

### Run 10

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

Zero errors/warnings. Log /tmp/jj-unavailable-web-check.log.
<!-- /run -->

<!-- run -->

### Run 11

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

Released Empty primitives, owned presentation helper and unchanged source
attribution pass. Log /tmp/jj-unavailable-ownership.log.
<!-- /run -->

<!-- run -->

### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm spec:first
```

**Notes**

Pre-evidence run reports missing fresh task coverage/ledger entry plus inherited
invalid E-NPM-REGISTRY-001 payload. Log /tmp/jj-unavailable-spec-first.log. Task
coverage is added by this record; parent evidence is preserved.
<!-- /run -->

<!-- run -->

### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_CHECK_REVISION=tsptytzx pnpm check:progress
```

**Notes**

Read-only parent proof reproduces invalid E-NPM-REGISTRY-001 results/run/test
reference, deleted .pnpmfile.cjs missing removed-files bookkeeping and parent
M24/M25 coverage. Log /tmp/jj-unavailable-parent-progress.log.
<!-- /run -->

<!-- run -->

### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec spec-validator first --base tkyzsuoz --head tsptytzx
```

**Notes**

Read-only parent range reproduces the inherited governance errors. Log
/tmp/jj-unavailable-parent-spec-first.log.
<!-- /run -->

<!-- run -->

### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec spec-validator validate --only governance,summary,fileLimits
```

**Notes**

Before task ledger update: existing parent M24 index is 401 physical lines. This
task shortens its introductory reminder to meet the authored limit. Log
/tmp/jj-unavailable-docs.log.
<!-- /run -->

<!-- run -->

### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
CUA: inspect existing port-7460 session tab; verify named status data-ui-component=empty; capture screenshot
```

**Notes**

Existing live HMR page shows centered messages icon, Session unavailable title
and exact description. Screenshot /tmp/jj-store-session-unavailable.jpg. No
dispatch, backend rebuild/reset or service restart.
<!-- /run -->
<!-- /evidence-field -->
