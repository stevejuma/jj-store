---
schema: 1
id: E-M11-005
---

# E-M11-005 details 3

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Initial final 9/9 before new creation regression; final count updated after cold browser fixes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

17/17 existing shell/navigation cases pass. Shared shell, route and build changes require all direct consumers plus cold native browser scenarios.
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
pnpm test:governance
```

**Notes**

34/34, including 2 manifest dependency traversal tests. Shared build manifest changes warrant complete owning governance suite.
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
pnpm check:architecture && pnpm check:boundaries && pnpm spec:validate
```

**Notes**

334 modules, 15 package boundaries and 20 chapters/125 requirements pass.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes.
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Final pre-dialog cold build reused persistent RustFS fixture: first startup 2327 ms, restart 3349 ms, RSS 164784 KiB, search ready.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts
```

**Notes**

Wrong generic Git fixture failed its required Native Git feature assertion; five native repository scenarios passed. WorkStore timed out opening project due to state_unsafe_mutation. Concurrent special Git output overwrote trace artifacts; reruns serialized.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --locked --test server_executable git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored --exact
```

**Notes**

1/1 in 34.62 s; correct owning persistent real-RustFS Git/CLI/browser cold gate resolves generic harness mismatch.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .scratch/projects-inspect.mjs
```

**Notes**

Owned headless diagnostic reproduced Svelte state_unsafe_mutation and absent Create project dialog. hub.work creates a reactive membership entry, now acquired in an effect rather than a derived computation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

6/6 after adding project creation/home selection and retained pending request regression.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

After fixing creation, refresh click blocked by html intercepting pointer events after dialog close; run interrupted once trace established regression. Bits body scroll-lock style restoration conflicts with self-only CSP. Consumer uses established public preventScroll=false, including composition of public AlertDialog confirmation controls.
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Rebuilt after dialog effect and CSP integration fixes: first startup remains 2327 ms; final restart 3533 ms, preparation 0 ms, server 48704 KiB + real RustFS 104864 KiB = 153568 KiB. LocalTrusted and search ready, port 7468. User 7460 untouched.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial sidebar/Reviews steering used unavailable public CommunitySkeleton subpaths twice and insufficient navigation-kind narrowing; corrected public barrel and union narrowing.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Latest steering exceeded unchanged budgets: hub329&gt;300 and shell338&gt;320. First extraction left hub319&gt;300. Extracted named project-home reading, work registry and sidebar selection ownership; no budget increase.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Initial final run had Vite optimizer reload/missing dependency import plus legacy manual link failure. Warm rerun had9/11 passing; Empty assertion used wrong public media selector and Svelte interpolated {32} in a literal input pattern. Corrected selector and bound regex expression.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Storybook getByRole options do not accept Playwright exact; removed three exact options, retaining exact string names.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

11/11 production interaction/accessibility stories including creation, writable-home selection, four landing tabs, expandable sidebar, native Reviews, icon Empty, hierarchy, dialogs and retained unknown requests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

23/23 direct shell/navigation stories; shared shell/build behavior warrants full direct consumer surfaces.
<!-- /run -->
<!-- /evidence-field -->

