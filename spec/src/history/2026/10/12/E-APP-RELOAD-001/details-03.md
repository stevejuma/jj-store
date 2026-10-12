---
schema: 1
id: E-APP-RELOAD-001
---

# Validation commands

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
jj --no-pager st
```

**Notes**

Started on lknwuvly above wxxtnnzt; preserve the enclosing implementation and four development repositories. Parent/root/UI guidance, current UI/API/security/publication requirements and M24/M27 reviewed. No unrelated working-copy changes are included. Final file list is in the evidence index.
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
pnpm dev:cargo test --locked -p jj-store-server --lib runtime_reload::tests
```

**Notes**

6 passed, 0 failed. Stale/malformed/startup-only rejection, stable intent and receipt recovery, independent freshly enrolled roles and per-domain failures. .local/app-reload-final-unit.log; this is not the full server suite.
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
pnpm dev:cargo test --locked -p jj-store-wal --test publication configuration_replacement_waits_for_the_original_manifest_cas
pnpm dev:cargo test --locked -p jj-store-wal --test publication
```

**Notes**

Exact CAS-fence regression 1 passed; publication consumer suite 35 passed. Publication retains admitted authority until the original manifest CAS settles. .local/app-reload-publication-test.log and publication-consumers.log.
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
pnpm dev:cargo test --locked -p jj-store-jj --lib retained_query_clones_capture_updated_limits_for_new_jobs
pnpm dev:cargo test --locked --lib -p jj-store-rpc -p jj-store-http hosted_limits_remain_explicit_until_live_policy_is_selected
```

**Notes**

Query clone regression 1 passed. Explicit-default versus opted-in current-policy consumers pass in HTTP and RPC, 1 each. Original pins/admitted limits remain unchanged. .local/app-reload-query-unit.log and opt-in-regression-3.log.
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
pnpm dev:cargo test --locked -p jj-store-service --test authorization --test administration
```

**Notes**

7 authorization and 11 administration tests passed. Configuration/agent operator capabilities remain independent from each other and repository grants. Fresh revocation and original publication protection remain covered. .local/app-reload-authority-final.log.
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
JJ_STORE_AGENT_PRIVACY_CACHE="$(cat .local/app-reload-test-cache-path)" JJ_STORE_TRANSCRIPT_CACHE="$(cat .local/app-reload-test-cache-path)" pnpm dev:cargo test --locked --lib -p jj-store-server -p jj-store-http -p jj-store-rpc -p jj-store-service -p jj-store-object -p jj-store-jj -- --test-threads=2
```

**Notes**

Six directly affected library suites: HTTP 50, JJ 91, object 2, RPC 11, server 216, service 57; 427 passed, 0 failed, 9 ignored. Includes the original foreground stop/cancel proof after owned monitor settlement. Supported external compiled-rules caches avoid repeated Kingfisher compilation; no credentials/transcripts copied. .local/app-reload-consumer-libs-qualified.log.
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
JJ_STORE_AGENT_PRIVACY_CACHE="$(cat .local/app-reload-test-cache-path)" JJ_STORE_TRANSCRIPT_CACHE="$(cat .local/app-reload-test-cache-path)" pnpm dev:cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes -- --exact --nocapture
pnpm dev:cargo test --locked --lib -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-rpc
```

**Notes**

The exact HTTP contract/index-task regression passes 1/1 after separating per-object interactive limits from aggregate preparation budgets. Four direct library consumers pass 209 tests, 4 ignored, after that final correction. Ready assertions are retained. .local/app-reload-index-contract-after.log and index-consumers.log.
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
pnpm dev:cargo check --locked -p jj-store-server -p jj-store-cli
pnpm dev:backend:build
```

**Notes**

Affected Rust composition checks and final explicit development executable build passed. .local/app-reload-check-final.log and index-final-build.log. No installed executable or services are replaced.
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
JJ_STORE_AGENT_PRIVACY_CACHE="$(cat .local/app-reload-test-cache-path)" JJ_STORE_TRANSCRIPT_CACHE="$(cat .local/app-reload-test-cache-path)" pnpm contracts:check
```

**Notes**

Generated contracts match; 300 captured real HTTP/SSE responses conform. Eight schema/contract tests and contract TypeScript check pass. Earlier uncached readiness and index budget failures are recorded separately. .local/app-reload-contracts-complete.log.
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
pnpm catalog:check
```

**Notes**

Final normalized-label source: 0 errors and 0 warnings for each; catalog boundaries cover 416 modules. .local/app-reload-normalized-web-check.log and normalized-catalog-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/runtime-reload.test.ts web/tests/vite/server-settings.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/server-agent-administration.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/global-settings-history.test.ts web/tests/vite/repository-settings-ownership.test.ts web/tests/vite/repository-settings-restoration.test.ts web/tests/vite/production-store-settings.test.ts
```

**Notes**

Nine directly affected reload/settings/history/owner files pass 52 tests. Stable unknown request identity, stale owner rejection, saved drafts and restoration are retained. Earlier final subset separately passed 45 tests. .local/app-reload-complete-owners.log.
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
pnpm test:storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/GlobalSettingsLoading.stories.svelte catalog/components/store-application/StoreNumericSetting.stories.svelte catalog/components/store-application/RuntimeReload.stories.svelte catalog/components/store-application/ResponsiveNavigation.stories.svelte catalog/components/store-application/SettingsShell.stories.svelte
```

**Notes**

Six directly affected story files pass 55 stories with interaction/Axe. All four Capture slider ranges, keyboard editing, explicit save, exact arbitrary bytes, unit-only draft preservation, normalized endpoints, mobile overflow and default shell behavior covered. Nonfailing ResizeObserver notices in the resized overflow fixture are separate from cold built browser errors. .local/app-reload-normalized-stories.log.
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
pnpm test:storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

After extracting unchanged configuration plays to the existing helper, the complete Capture story family passes 23 stories. .local/app-reload-capture-extraction-stories.log.
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
pnpm check:ui-ownership
pnpm check:architecture
pnpm check:boundaries
```

**Notes**

UI ownership passes. The source-size check found Capture story and StoreApplication over their limits; unchanged story extraction and reuse of the existing editorOpen derivation repair both without raising baselines. Package boundaries pass for 17 packages. Final architecture passes 1,770 modules. .local/app-reload-closure-architecture-3.log; ownership and package checks also pass.
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
pnpm docs:review application "Reviewed Capture sliders and normalized selected-unit endpoint labels, unchanged canonical ranges and exact drafts, persistent responsive header menu, loading geometry and lifecycle guards."
PATH=/Users/stevejuma/.local/bin:$PATH pnpm web:build
```

**Notes**

Topic-specific source reviews cover application, publication, local/query, service, configuration, CLI, HTTP and contracts. The built UI includes strict documentation compilation, 9 light/dark Mermaid diagrams, documentation closure and assets. Final normalized source bundle passes, including strict docs and 141-file closure (25,020,368 bytes), with inherited bundle notices. .local/app-reload-normalized-web-build.log.
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
pnpm format:rust:check
python3 - <<'FORMAT'
import subprocess
paths=subprocess.check_output(["jj","--no-pager","diff","--name-only"],text=True).splitlines()
paths=[p for p in paths if p.endswith((".md",".json",".ts",".svelte",".css",".mjs"))]
raise SystemExit(subprocess.call(["pnpm","exec","prettier","--check",*paths]))
FORMAT
```

**Notes**

Rust format check passes; changed-file formatting is rerun after final evidence completion. Python acceptance sources use the existing maintained style. No root aggregate check/test/build/spec lane or visual baseline is run.
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
pnpm check:progress
pnpm spec:first
pnpm spec:links
```

**Notes**

Book links/anchors pass (2,679 HTML files; 101,087 local references). The first final progress and spec-first commands fail due to descriptive test names where path#name references are required, pending run metadata, and independent milestone gates incorrectly listed as in-slice gaps. Correct those metadata fields; scope and milestone reviews keep independent gates explicit. These two failed commands are not acceptance. .local/app-reload-complete-progress.log and complete-spec-first.log.
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
pnpm spec:first
python3 -m py_compile scripts/runtime-reload-acceptance.py scripts/remote-agent-administration-acceptance.py
node --check scripts/runtime-reload-browser.mjs
node --check scripts/remote-agent-administration-browser.mjs
```

**Notes**

Corrected metadata passes progress (152 tasks, 777 evidence records). Spec-first
then finds one missing Local JJ owning chapter; updating the bounded code-read
chapter preserves original pin/authorization/publication contracts and passes
123 protected files / 14 canonical chapters. Script syntax checks pass. These
are specification/ledger corrections, not weakened tests or promoted milestones.
Final changed-file format and progress/spec-first are checked again with the
complete record. M24/M27 remain In progress; independent gates are in the review.
<!-- /run -->

<!-- /evidence-field -->
