---
schema: 1
id: E-M15-022
---

# E-M15-022 details 2

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
jj rebase -s msszrzpn -o c347df2d; jj edit msszrzpn
```

**Notes**

Rebased two commits. Three file conflicts resolved by retaining both source profiles, profile-aware route restoration and surface gates, global/reference search and owned CommunityMessage imports. Main checkout remained unchanged during resolution. All 240 main and 233 Threads evidence objects retained unchanged in their union of 243 entries before this new evidence.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Combined controller contracts reached 301 counted lines. Extracted unchanged search presentation types into store-search-types.ts; budget not raised. Obsolete native-only route helper removed, using main profile-aware restorer.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Conflict cleanup removed ApiError import still used by activeSource; restored it. /tmp/jj-rebase-web-check1.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/global-search.test.ts
```

**Notes**

39 directly affected controller tests pass after resolving conflicts; /tmp/jj-rebase-exact-controller1.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/global-search.test.ts -t global_search_does_not_issue_native_requests_for_local_profiles
```

**Notes**

Before the profile guard, global search issues native content/capability requests for a local Git-backed repository. /tmp/jj-rebase-profile-search-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/global-search.test.ts -t global_search_does_not_issue_native_requests_for_local_profiles
```

**Notes**

After guard, mixed queries search native sources, local-only queries return no native requests and explicit unsupported source state; /tmp/jj-rebase-profile-search-after.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm catalog:test
```

**Notes**

Both type checks have zero errors/warnings, 619 production modules within unchanged budgets; full directly impacted catalog/controller suite has 197 tests across 33 files passing. /tmp/jj-rebase-web-check2.log, /tmp/jj-rebase-catalog-check2.log, /tmp/jj-rebase-architecture2.log and /tmp/jj-rebase-catalog-unit.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook (Threads, search, shell and navigation direct consumers)
```

**Notes**

First run: 9 tests pass in 5 files; StoreApplication suite fails to import with Vite/Svelte virtual CSS transform errors. No assertions were removed. Fresh exact Thread alignment run passes 3 tests; /tmp/jj-rebase-stories1.log and /tmp/jj-rebase-story-exact2.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test content_search --test native_collab --test collab_pagination --locked
```

**Notes**

6 content-search, 9 native-collaboration and 8 pagination tests pass; /tmp/jj-rebase-service.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

Fresh exact alignment run: 3 tests pass; /tmp/jj-rebase-story-exact2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Fresh exact shell run: 12 tests pass; /tmp/jj-rebase-shell-exact2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-search/StoreRecentSearch.stories.svelte catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
```

**Notes**

Fresh directly affected family: 27 tests across 7 files pass after the import failure, without assertion changes; /tmp/jj-rebase-stories2.log.
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
pnpm web:build
```

**Notes**

Fresh production bundle succeeds after reverting unrelated actor-capture edit; existing chunk-size advisory only. /tmp/jj-rebase-web-build2.log.
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
CARGO_INCREMENTAL=0 cargo build -p jj-store-cli --bins --locked
```

**Notes**

Explicit isolated development CLI/server binaries built; /tmp/jj-rebase-cli-build.log. No installed CLI replacement.
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
CARGO_INCREMENTAL=0 pnpm contracts:generate
```

**Notes**

Generated combined 85-path native/local contracts; /tmp/jj-rebase-contracts-generate.log.
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
CARGO_INCREMENTAL=0 pnpm contracts:check
```

**Notes**

138 actual HTTP/SSE responses conform; 7 validator tests and TypeScript contract check pass; /tmp/jj-rebase-contracts-check.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:governance
```

**Notes**

41 governance tests pass; /tmp/jj-rebase-governance.log.
<!-- /run -->

<!-- run -->
### Run 18

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

119 tasks validate; /tmp/jj-rebase-spec.log.
<!-- /run -->
<!-- /evidence-field -->

