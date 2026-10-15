---
schema: 1
id: E-UI-CI-001
---

# E-UI-CI-001 details 3

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
REMOTE=1 node /tmp/jj-ui-split-probe.mjs; node /tmp/jj-ui-split-probe.mjs
```

**Notes**

Candidate 2px internal gap passed all original compact-ID assertions in Linux and macOS Chromium. Earlier Linux-to-host proxy attempts failed loading assets; direct container-local static serving isolated geometry. These probes inject the candidate stylesheet; final built-source acceptance is recorded separately.
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

Vite built final application assets successfully in 17.82s, then documentation staging failed because the temporary docs_reference copy had disappeared. Reused the unchanged retained public-release executable instead of rebuilding unrelated Rust.
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
JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference node scripts/docs.mjs stage
```

**Notes**

Final docs staging passed: nine Mermaid diagrams rendered without network access; 143 assets, 26067318 bytes; all links and anchors valid.
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
JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Documentation/reference checks passed with the retained executable. No Rust source or public API changed.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate --only governance,summary,fileLimits,verification
```

**Notes**

Draft evidence initially had invalid named-test references and omitted API/documentation topic confirmations. Corrected exact names, explicit unchanged-contract confirmations and file coverage. Final draft checks passed: 155 tasks, 818 evidence records; 31 protected files and 7 canonical chapters; 4 validators, 3893 chapters and 198 requirements.
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
gh api repos/stevejuma/jj-store/branches/main/protection; jj git fetch --remote origin
```

**Notes**

Admin enforcement and strict required check remain enabled. Main is still aeb3e95e68d3. No protection exception, full qualification dispatch, release or installed-service restart is used for this UI repair.
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
pnpm build-storybook; node --test scripts/catalog-browser.test.mjs
```

**Notes**

Final static catalog built successfully, then complete cold_catalog_contract passed in 29.15s on macOS Chromium with final source styles. Width, padding, clipping, native hover/touch and copy assertions remain unchanged. /tmp/jj-ui-static-final.log.
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
docker cp storybook-static/. jj-store-ui-linux:/tmp/jj-ui-static; unmodified expectSplitCopy against container-local static assets
```

**Notes**

Linux Chromium passed all compact-ID assertions on final built assets without injected styles. /tmp/jj-ui-linux-final-built-ids.log. Removed the task-owned Linux container and static-server processes afterwards; existing development data and services were preserved.
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
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=2/3
```

**Notes**

326 passed across 66 files in 335.04s. Vite logged ResizeObserver delivery diagnostics; Vitest exited successfully with no failed tests. /tmp/jj-ui-story-shard2.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 PLAYWRIGHT_JUNIT_OUTPUT_FILE=/tmp/jj-ui-browser-final.xml pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

Second full browser run: 113 passed / 1 failed. Automatic original-receipt observation correctly completed an import before a second synthetic click, leaving no recovery button. /tmp/jj-ui-browser-final.log. Retained initial manual missing-receipt check, awaited automatic receipt inspection/recovery and added one-import assertion.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts local-capture-settings.spec.ts -g "cold capture controls preserve intent and snapshot"
```

**Notes**

Focused recovery scenario passed before the additional one-command assertion; final asserted version remains to be rerun. /tmp/jj-ui-capture-recovery.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=3/3
```

**Notes**

Initial third shard interrupted with SIGINT (exit 130) after 15 failures, including comments and disclosure stories. No completed-shard pass claimed. Global invisible-toolbar pointer rule interfered with synthetic clicks on unrelated controls; narrowed it to prompt context only. /tmp/jj-ui-story-shard3.log.
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
pnpm test:storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Fresh family run after context-only pointer rule: 30 passed / 1 failed. All 23 AgentChatBody and five AgentComments checks passed; review preference scenario failed to find Focus radio. /tmp/jj-ui-comment-families.log.
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
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte -t ProjectsCollection
```

**Notes**

Fresh named reproduction could not find agents region after remount. /tmp/jj-ui-disclosure-exact.log. No persistence contract or timeout changed based on this alone.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Unmodified full families: seven passed / two failed (review preference and operation disclosure). /tmp/jj-ui-disclosure-review-fresh.log. Subsequent diagnostic run of all nine passed; complete third-shard verification is still required.
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
JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build && pnpm build-storybook
```

**Notes**

Final application and static catalog build after narrowing prompt action selectors succeeded. Web build 52.41s; Help build 60.07s. /tmp/jj-ui-web-build-narrow.log and /tmp/jj-ui-catalog-build-narrow.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 PLAYWRIGHT_JUNIT_OUTPUT_FILE=/tmp/jj-ui-help-final.xml pnpm exec playwright test --config web/playwright.docs.config.ts
```

**Notes**

20 Help browser checks passed in Chromium and WebKit (1.8m). /tmp/jj-ui-help-final.log.
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
pnpm test:release; actionlint
```

**Notes**

Revalidated all 53 CI/release regressions and workflow lint after final workflow edits. /tmp/jj-ui-ci-final.log.
<!-- /run -->
<!-- /evidence-field -->

