---
schema: 1
id: E-FORGE-INTEGRATIONS-001
---

# E-FORGE-INTEGRATIONS-001 details 2

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
jj --no-pager st; owning forge, Settings and credential chapters; JJButler account/authentication source review
```

**Notes**

Before state: clean isolated forge-stacks workspace znyqwmzm over df0cfe6c. Existing forge setup was repository-scoped, session-token only, with no global accounts; checks were a flat unbounded list. Read JJButler account.rs/auth.rs/GithubIntegration.svelte at a9763eebc79c, existing owned Community account/avatar compositions and released Design Core APIs. No reference checkout code or dependency was copied.
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
pnpm dev:cargo -- check -p jj-forge -p jj-store-http
```

**Notes**

Initial cross-layer check found a mismatched HTTP error conversion in the new inventory handler; corrected it.
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
pnpm dev:cargo -- check -p jj-store-http -p jj-store-cli
```

**Notes**

Rejected package selector: directory jj-store-cli publishes package jj-store. Retried with the actual package name; no build script changes.
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
pnpm dev:cargo -- check -p jj-store-http -p jj-store --locked
```

**Notes**

Affected CLI/HTTP boundary compiled. Removed the reported unused import; final package builds and targeted tests compile the corrected sources.
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
pnpm dev:cargo -- test -p jj-forge --test accounts --test submission --locked
```

**Notes**

9 passed: seven injected account/credential/OAuth/remote parsing tests and two submission recovery tests, including late cancellation and locked-store inventory errors. No real tokens or OS-keychain changes in fixtures. .local/forge-integrations-rust-tests-final.log.
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
pnpm dev:cargo -- test -p jj-store-http --test forge_admission --locked
```

**Notes**

2 passed: foreign-origin rejection, exact LocalTrusted Admin admission for GET/POST integration management, and no secret echoes. .local/forge-integrations-http-tests-final.log.
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
pnpm dev:cargo -- test -p jj-store-jj --lib local_stacks::forge::tests::github_detection_binds_one_verified_account_and_preserves_ambiguity_and_pending_intent --locked
```

**Notes**

1 passed, 98 unrelated tests filtered out. Real disposable JJ repository with injected accounts: default trunk, one-match binding, account/remote ambiguity, revoked access, operation unchanged and blocked pending-intent account switches. .local/forge-integrations-detection-test-final.log.
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
pnpm contracts:generate; pnpm test:contracts
```

**Notes**

Generated OpenAPI and TypeScript contracts; Swagger validation and eight contract tests plus contract TypeScript passed. Additive optional account references retain legacy request/receipt decoding. Does not run the broad contracts:check capture matrix. .local/forge-integrations-contract-tests.log.
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
pnpm web:check
```

**Notes**

Early checks ran before generation and caught missing new types; subsequent checks caught an unsupported Avatar export, a rune-shadowing name and an unsupported menu prop. Reused the owned avatar and generated request-body union, and fixed the new UI code.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final web and catalog checks: zero errors/warnings; catalog boundary passes for 455 modules. .local/forge-integrations-web-complete.log and .local/forge-integrations-catalog-completion.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/integrations.test.ts web/tests/vite/forge.test.ts web/tests/vite/forge-details.test.ts web/tests/vite/forge-branch-status.test.ts web/tests/vite/global-settings-history.test.ts
```

**Notes**

41 passed in five focused files. Includes exact Settings return, setup-toast dismissal/navigation, unavailable PR controls, transient drafts, successful secret clearing, error toast, existing selection/receipt/revision guards. .local/forge-integrations-unit-reviewed.log.
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
pnpm exec tsx --test web/tests/global-settings-navigation.test.ts
```

**Notes**

5 passed: canonical routes, invalid/duplicate query rejection, exact return, startup resume and loading geometry. .local/forge-integrations-navigation.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreIntegrations.stories.svelte catalog/components/store-forge/StoreForgeChecksLayout.stories.svelte
```

**Notes**

New browser regressions exposed modal input blocking, missing viewport height constraint, async-view preload and toast portal lookup. Fixed product controls/scroll bounds and fixture setup; retained assertions. Failed iterative logs .local/forge-integrations-stories.log through .local/forge-integrations-stories-6.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge
```

**Notes**

43 passed across nine direct Forge story files, including accessibility, account forms/OAuth/forget/failure, large mixed checks, grouping/disclosure/scrolling, Markdown editing, selection, jobs/logs and float/redock. .local/forge-integrations-story-acceptance.log. Visual baselines remain pending.
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
node scripts/web-architecture.mjs; pnpm check:ui-ownership; node --test scripts/forge-boundaries.test.mjs
```

**Notes**

Architecture passes for 1970 modules, UI ownership passes, forge dependency boundary test passes. Initial module-size failure was repaired by extracting checks presentation and using the existing settings owner; formatting exposed a further five-line overflow, repaired by moving readiness policy into the connection lifecycle; no thresholds were changed.
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
pnpm dev:cargo -- build -p jj-store --bins --locked
```

**Notes**

Built task-owned server and paired custom JJ. Final build succeeded; macOS linker emitted its existing large unwind-section warning. No installed executable was replaced. .local/forge-integrations-backend-build-reviewed.log.
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
Explicit task-owned JJ_STORE_DEV_ROOT/JJ_STORE_DEV_SERVER/JJ_STORE_DEV_RUSTFS_RUNTIME/ACP/JJ overrides; pnpm dev:backend:restart
```

**Notes**

Replaced the task-owned backend while preserving Vite and the shared development fixture. Backend/UI 7462/7460. Cold browser reload renders Integrations and obtains a real GitHub verification code. Initial HMR requests during backend replacement failed; cold reload recovered. No sample branches or PRs were mutated. .local/forge-integrations-backend-restart.log.
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
GitHub browser registration and device verification
```

**Notes**

Registered jj-store OAuth app 3935935 under stevejuma, using the project homepage, device flow and expiring tokens. Disabled wildcard redirect matching. Configured only the public client ID outside checkout. Reached the real GitHub authorization review; final repository/workflow/profile permission grant awaits explicit browser-policy confirmation. No additional organization grants requested.
<!-- /run -->
<!-- /evidence-field -->

