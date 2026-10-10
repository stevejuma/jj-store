---
schema: 1
id: E-UI-VIEW-LOADING-001
---

# E-UI-VIEW-LOADING-001 details 2

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
jj --no-pager st; jj --no-pager workspace list
```

**Notes**

Default workspace; parent d7a27ff8. Enclosing change ntvyxzvu. Preserved independently owned Design Core AI/Diff work. No extra workspace or subagent.
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
node --test scripts/local-package-pnpmfile.test.mjs
```

**Notes**

2/2 after changing source-folder substitution to the prepared archive. Missing archive retains registry dependency ranges and peers are unchanged. Earlier exact regression failed before implementing the hook. /tmp/view-loading-hook-before.log.
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
pnpm install --lockfile-only --ignore-pnpmfile
```

**Notes**

Portable tracked lockfile resolves Design Core ^0.2.0 from the registry; no file/link/absolute sibling resolution. /tmp/view-loading-registry-lock.log.
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
pnpm install --no-frozen-lockfile
```

**Notes**

Local hook installs ignored Design Core 0.1.2 prepared tarball; restored portable registry lock after local validation. Archive SHA256 c24293ea0a98316508e2bbcb281dd839cb5a188bf4645507facb413391fa3c4a. This is temporary source validation, not registry release qualification.
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
pnpm package:local (Design Core source repository)
```

**Notes**

Source slice lznzkutt / 5c7c9a79 prepares JS rune modules, Svelte, declarations and CSS using @sveltejs/package 3.0.0. Source-specific record spec/records/prepared-local-package.md records its exact commands. Nine missing declarations were repaired with readonly typeof annotations in the compatibility compound.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks (Design Core source repository)
```

**Notes**

Initial source aggregate failed a fifth acceptance bullet and temporary-script formatting. Repaired those source-owned issues; reran focused specification, formatting, type/governance/consumer checks, 722 unit and 3 orchestration tests, static build/audit and all 674 source Storybook tests successfully. Do not claim the original aggregate passed.
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
pnpm test:workspace:pointer (Design Core source repository)
```

**Notes**

14 passed, 1 existing Visual Delta notification readiness timeout. Exact pnpm test:workspace:pointer notification-toasts.spec.ts reproduced it. Fail-fast aggregate did not reach remaining Shell/AI/Shadcn pointer lanes. Source-user work remains unchanged.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Initial lazy-host typing/lifecycle and snippet integration errors were repaired. Latest plugin addition then exposed 8 JS type errors in view-module-urls.mjs; a Vite Plugin JSDoc contract resolves these. /tmp/view-loading-web-initial.log and /tmp/view-loading-web-final-final.log.
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

Final zero errors/warnings after URL plugin types and shell-context repair. /tmp/view-loading-web-complete.log.
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

Final zero errors/warnings; catalog boundaries pass for 370 modules. /tmp/view-loading-catalog-complete.log.
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

Final owned-source audit passes. /tmp/view-loading-ownership-complete.log. No Community source/package/fixture dependency introduced.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 17
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

17 existing violations remain; parent d7a27ff8 diagnostics were compared and count/content match. Changed host components fit budgets. /tmp/view-loading-architecture-proof.log. No architecture baseline relaxed.
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
pnpm exec tsx --test web/tests/lazy-view.test.ts
```

**Notes**

2/2: demand-only import, shared pending/completed imports, rejected-promise recovery. /tmp/view-loading-leaf-final.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

136 pass, 3 fail: agent timestamps and two Node tests importing raw TanStack .svelte. The same three failures were reproduced with unchanged parent source under /tmp/jj-store-view-loading-before. /tmp/view-loading-web-units.log and /tmp/view-loading-web-parent-proof.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test
```

**Notes**

928 pass, 5 fail, 1 skip. Conversation-create and local-browser-merge failures reproduced with parent source (same five failures, three pass). /tmp/view-loading-catalog-parent-proof-retry.log. Not a full-unit pass.
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
pnpm test:storybook catalog/components/store-application/LazyViews.stories.svelte
```

**Notes**

Four new interaction/accessibility stories pass: deferred loading, retry, unmounted completion and retained closed-overlay draft. /tmp/view-loading-final-stories.log. Visual-pending remains.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Stable optimized subset: 5 pass, 11 fail. Existing tests assume grouped calls under default Document and legacy insets/row sizes; others need asynchronous view waits. Added exact readiness waits without weakening assertions. /tmp/view-loading-host-optimized.log.
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
pnpm test:storybook (initial broader run)
```

**Notes**

Initial long-running lane was terminated after source/config repairs and temporary parent-copy tsconfig invalidation made it unsuitable final evidence. Readiness failures, AppShell context errors and ResizeObserver warnings were observed. Not counted as completed full acceptance. Parent proof was moved outside the watched workspace.
<!-- /run -->
<!-- /evidence-field -->

