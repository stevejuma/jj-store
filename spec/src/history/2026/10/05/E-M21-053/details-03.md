---
schema: 1
id: E-M21-053
---

# E-M21-053 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
python3 -
```

**Notes**

Verification helper initially assumed all unrelated commit hashes would stay fixed despite independent bounded-reads work, then treated jj resolve --list exit2 for no conflicts as an error. Both assumptions were corrected; no preservation assertion on evidence objects failed. Initial stdin helpers were superseded by the preserved verification script recorded below.
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
python3 .local/rebase-main/verify-preservation.py
```

**Notes**

preservation.log: all534 Stacks and529 main decoded entries preserved. All534 Stacks and all57 main-only objects are verbatim; six shared main objects differ only in historical formatting. Main pointer unchanged; other workspace change IDs retained.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

local-browser.log:48 passed /2 failed /3 ignored. Concurrent managed restart observed409 before readiness; working-tree observe exceeded bounded admission with413 under the parallel test load. Original failed output retained; not claimed as a full-suite pass.
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
target/debug/deps/local_browser-0134cba2dabca6a4 filesystem_runtime_registers_and_reads_without_native_storage --exact --nocapture
```

**Notes**

registered-local-exact.log: concurrent namespace preparation returned busy. No contract or bound changed.
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
target/debug/deps/local_browser-0134cba2dabca6a4 managed_http::managed_local_http_activates_without_subscribers_resumes_and_captures_tasks --exact --nocapture
```

**Notes**

managed-local-exact.log: exact managed activation/restart/HTTP capture passes in isolation.
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
target/debug/deps/local_browser-0134cba2dabca6a4 filesystem_runtime_registers_and_reads_without_native_storage --exact --nocapture
```

**Notes**

registered-local-repeat.log: exact registered filesystem startup, advanced preparation, physical observation and ordinary Stacks command capture passes in isolation. Existing non-queuing admission and all timeout/work ceilings retained.
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
cargo test -p jj-store-cli --test native_lifecycle graph_cache --locked
```

**Notes**

native-cache.log:6 graph cache regressions pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceResize.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte
```

**Notes**

stories-final.log:36 interaction/accessibility stories /6 direct families pass in a cold native browser process; no visual baseline changes.
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
pnpm web:check
```

**Notes**

web-check-final.log:zero errors/warnings after adapter/measurement integration.
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
pnpm catalog:check
```

**Notes**

catalog-check-final.log:zero errors/warnings;239 catalog modules pass boundaries.
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
pnpm exec prettier --check
```

**Notes**

format-final.log:all selected files pass. Original24-path argument vector was not retained; the complete31-path final check is separately recorded below.
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
pnpm check:progress
```

**Notes**

progress.log:142 tasks validate; all previous evidence remains immutable.
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
pnpm spec:first
```

**Notes**

spec-first-final.log:combined integration source maps to its owning canonical chapters.
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
pnpm spec:validate
```

**Notes**

spec-validate.log:5 validators /23 chapters /173 requirements pass.
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
target/debug/deps/native_lifecycle-f25450e46b6547bf graph_acceptance::cache_acceptance:: --test-threads=1
```

**Notes**

native-cache-module.log: complete18-test cache acceptance group passes serially.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --test stack_source_recovery indexes --locked
```

**Notes**

recovery-indexes.log: disk exhaustion during stripping left a336-byte non-executable artifact; no recovery tests ran. Retry recovery-indexes-final.log reused that failed artifact and also ran no tests.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

contracts.log: generated artifacts match, but capture-fixture compilation exhausted disk; no completed contract-capture acceptance.
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
pnpm contracts:check
```

**Notes**

contracts-final.log: premature retry raced ignored incremental-cache cleanup and failed copying a removed object. This was an orchestration error, not product acceptance.
<!-- /run -->
<!-- /evidence-field -->

