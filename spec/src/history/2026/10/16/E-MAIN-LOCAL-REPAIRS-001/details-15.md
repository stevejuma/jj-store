---
schema: 1
id: E-MAIN-LOCAL-REPAIRS-001
---

# E-MAIN-LOCAL-REPAIRS-001 details 15

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
pnpm docs:review application "Reviewed the capture harness listbox name and unchanged Sources filtering, loading and recovery flows; Settings documentation now consistently includes Integrations in the App group. Existing theme-token link contrast, form labels, drafts, manual forge reads and exact commands remain unchanged."
```

**Notes**

docs-application-review-55: exit 0, 0.47 seconds. 
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
pnpm docs:review desktop "Reviewed the equivalent optional ACP regular-file and symlink guard in Resources::from_root against lifecycle documentation. Fresh desktop Clippy, complete workspace tests, native checking and staged restart acceptance pass; private installation, owner lifetime and independent signed-platform gates remain unchanged."
```

**Notes**

docs-desktop-review-55: exit 0, 0.42 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-53: exit 1, 7.76 seconds. 
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
pnpm docs:review local-compute "Reviewed the maintained local-agent hint and bounded catalog simplifications, unchanged item ordering, lexical synthetic request lock and nonempty request-read assertion against the compute, agent and security guides. Permission/cancellation, backend credential custody, single-attempt metadata and uncertain-inference no-replay behavior retain their exact tests and contracts; native GGUF/live-provider/platform gates remain separate."
```

**Notes**

docs-compute-review-56: exit 0, 0.42 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-53: exit 0, 22.96 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts --grep "cold capture loading keeps guarded controls and recovers from busy reads"
```

**Notes**

capture-browser-exact-52: exit 0, 5.67 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts
```

**Notes**

capture-browser-complete-52: exit 1, 20.79 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts --grep "cold source refresh retains coverage and repository link opens its authorized page"
```

**Notes**

capture-refresh-exact-57: exit 0, 4.51 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts
```

**Notes**

capture-browser-complete-52: exit 0, 20.95 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts --max-failures=0
```

**Notes**

served-browser-29: exit 1, 269.27 seconds. 
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
pnpm exec prettier --write web/tests/browser/local-capture-settings.spec.ts web/tests/browser/notifications.spec.ts web/tests/browser/reads.spec.ts
```

**Notes**

browser-repairs-format-58: exit 0, 0.53 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-settings.spec.ts --grep "cold capture controls preserve intent and snapshot"
```

**Notes**

capture-controls-exact-58: exit 0, 7.76 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/notifications.spec.ts
```

**Notes**

notifications-exact-58: exit 0, 5.56 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/reads.spec.ts --grep "branch targets scope native history and expose repository decorations|graph_and_directory_pages_retain_exact_snapshots_and_query_context" --max-failures=0
```

**Notes**

graph-regressions-58: exit 0, 4.2 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-settings.spec.ts web/tests/browser/notifications.spec.ts web/tests/browser/reads.spec.ts --max-failures=0
```

**Notes**

browser-affected-files-58: exit 0, 41.03 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts --max-failures=0
```

**Notes**

served-browser-29: exit 1, 268.22 seconds. 
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
pnpm exec prettier --write web/tests/browser/repository-settings-navigation.ts
```

**Notes**

settings-helper-format-59: exit 0, 0.51 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-sync.spec.ts --grep "Sync reconciliation uses fresh heads and keeps the historical route after unknown recovery"
```

**Notes**

sync-owner-exact-59: exit 0, 6.64 seconds. 
<!-- /run -->
<!-- /evidence-field -->

