---
schema: 1
id: E-UI-028
---

# E-UI-028 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
source and fixture acceptance audit before implementation
```

**Notes**

The repository landing preview supported only README/LICENSE, the application header owned the breadcrumb and exposed no body section tabs, existing text files had no rename/content editor or native EditFile command, and selecting a search result could route before dialog cleanup left the page interactive.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both Svelte projects and the production/catalog ownership and line-budget audit passed after splitting landing-document, mutation-commit, editor-style and editor-story modules.
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
pnpm web:test
```

**Notes**

All 55 web/controller tests passed, including stable five-category landing-document loading and production edit routing.
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
pnpm catalog:test
```

**Notes**

All 60 catalog controller and presentation unit tests passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

After the first two stale expectations were corrected, RepositoryDirectoryNavigation still aligned the file-tree title with the Files breadcrumb segment instead of the new repository-root body edge.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

All 17 focused Chromium interaction and automatic accessibility stories pass for the five landing document tabs, body tabs/breadcrumb, responsive shell, editor preview/apply/rename and search presentation. Stories remain visual-pending.
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
cargo check -p jj-store-jj --locked
```

**Notes**

The native EditFile command and transaction staging compiled with the locked JJ dependency graph.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> cargo test -p jj-store-cli --test native_lifecycle commands --locked
```

**Notes**

A generated vectorscan CMake cache reset BUILD_TOOLS and BUILD_UNIT after the compiler changed, then the unused hscollider target failed because pcre.h was unavailable. Removing only that generated cache let the crate build script retain both targets as OFF.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> cargo test -p jj-store-cli --test native_lifecycle commands --locked
```

**Notes**

The repaired native environment passed all 3 filtered command and HTTP mutation capability cases.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> cargo test -p jj-store-cli --test native_lifecycle typed_edit_file --locked
```

**Notes**

Both EditFile native regressions pass: exact child/rename/content/bookmark publication and ambiguous-target/path/size rejection.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The first governance run identified missing owning updates in cli.md, compatibility.md and jj-integration.md. Those chapters now specify lifecycle coverage, additive format compatibility and native transaction behavior.
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
pnpm spec:first
```

**Notes**

The spec-first gate passes for 43 protected files and all 7 owning canonical chapters.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm web:test:browser
```

**Notes**

The first cold lane reached native response capture and found its registered-handler capability assertion still expected the old eight-command list after edit_file became advertised.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> cargo test --quiet --locked -p jj-store-cli --test native_lifecycle http_mutation_acceptance::http_mutations_preview_apply_status_replay_and_contract_capture -- --exact
```

**Notes**

The exact native HTTP response-capture regression passes with edit_file in the generated capability response.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm web:test:browser
```

**Notes**

After the capability fixture was repaired, Playwright's 30 second web-server startup limit expired while the cold browser_preview example compiled. The example was prebuilt with the same native environment before continuing.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

The prebuilt preview passed 17 of 21 cases. Remaining failures identified dialog scroll lock surviving search navigation, a stale create-change description selector, the edited fixture returning its old log snapshot and a multiple-head assertion that still expected the removed header breadcrumb operation text.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/search.spec.ts
```

**Notes**

After repairing all four cases, 15 of 17 focused tests passed. The mutation assertion omitted code_state_version and the focused command was still serving the previous embedded web build, so it could not exercise the search lifecycle source fix.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts:45 web/tests/browser/search.spec.ts:10; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/search.spec.ts; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/search.spec.ts
```

**Notes**

The corrected mutation regression passed, while repeated focused runs retained only the search body-lock failure because these direct Playwright commands do not rebuild web/dist. The last stale-build run passed 16 of 17 cases.
<!-- /run -->
<!-- /evidence-field -->

