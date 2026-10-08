---
schema: 1
id: E-GOV-005
---

# E-GOV-005 details 4

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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj cargo test -p jj-store-service --test working_tree_local --test working_tree_observer --locked --no-fail-fast -- --ignored
```

**Notes**

working-tree-service: exit 0; 68.22s. 5 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/working-tree-service.log.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj JJ_STORE_WORKING_TREE_CONTRACT_CAPTURE=/Users/stevejuma/code/lapismd/jj-store/.local/test-audit-2026-10-08/working-tree-capture.json cargo test -p jj-store-http --lib --locked registered_observation_route_reads_real_checkout_without_publication_or_content_leaks -- --ignored
```

**Notes**

working-tree-http: exit 0; 45.83s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/working-tree-http.log.
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
pnpm exec playwright test --config web/playwright.docs.config.ts
```

**Notes**

docs-browser-recheck: exit 1; 30.8s. See retained log and execution exit status. Preview startup again exceeded 30 seconds while package builds were active. Final docs-browser-final run passed all 20 Chromium/WebKit tests. Raw output retained in ignored .local/test-audit-2026-10-08/docs-browser-recheck.log.
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=/Users/stevejuma/code/lapismd/jj-store/.local/test-audit-2026-10-08/working-tree-capture.json pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-working-tree.test.ts --maxWorkers=1
```

**Notes**

working-tree-captured-ui: exit 0; 4.3s. Test Files  1 passed (1); Tests  12 passed (12).  Raw output retained in ignored .local/test-audit-2026-10-08/working-tree-captured-ui.log.
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
JJ_STORE_AGENT_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj cargo test -p jj-store-jj --test agent_workspaces --locked -- --ignored
```

**Notes**

agent-workspaces: exit 0; 48.83s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/agent-workspaces.log.
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
cargo test -p jj-store-server --lib --locked real_private_codex_install_verifies_pinned_versions_and_cold_selection -- --ignored
```

**Notes**

private-tool-install: exit 0; 51.91s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/private-tool-install.log.
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
env -u JJ_STORE_TOKEN cargo test -p jj-store-cli --test executable_native --locked -- --ignored --test-threads=1
```

**Notes**

os-credentials: exit 0; 6.15s. 3 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/os-credentials.log.
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
cargo build --manifest-path runtime/local-agent/Cargo.toml --workspace --locked
```

**Notes**

local-agent-tools-build: exit 0; 23.69s. See retained log and execution exit status.  Raw output retained in ignored .local/test-audit-2026-10-08/local-agent-tools-build.log.
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
python3 scripts/local-agent-tools.acceptance.py runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

local-agent-tools-protocol: exit 0; 16.52s. 5 passed.  Raw output retained in ignored .local/test-audit-2026-10-08/local-agent-tools-protocol.log.
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
python3 scripts/local-agent-acp.acceptance.py --acp /Users/stevejuma/code/lapismd/jj-store/desktop/resources/jj-acp --agent /Users/stevejuma/code/lapismd/jj-store/runtime/local-agent/target/debug/jj-store-local-agent --tools /Users/stevejuma/code/lapismd/jj-store/runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

local-agent-acp-protocol: exit 1; 30.31s. See retained log and execution exit status. Readiness timeout with existing staged desktop/resources/jj-acp. Its help advertises only control protocols 1/2, while acceptance requires 3. Current sibling development executable supports 3 and subsequently passes both scenarios. No installed executable or sibling source was changed. Raw output retained in ignored .local/test-audit-2026-10-08/local-agent-acp-protocol.log.
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
node --input-type=module -e 'import {stageDesktopResources} from '\''./scripts/desktop-resources.mjs'\''; import {resolve} from '\''node:path'\''; await stageDesktopResources({target:resolve('\''target/debug'\''),runtime:resolve('\''runtime/embedded-rustfs/target/debug/jj-store-rustfs'\''),web:resolve('\''web/dist'\''),acp:resolve('\''desktop/resources/jj-acp'\''),destination:resolve('\''.local/test-audit-2026-10-08/desktop-resources'\'')});'
```

**Notes**

desktop-resources-stage: exit 0; 2.82s. See retained log and execution exit status.  Raw output retained in ignored .local/test-audit-2026-10-08/desktop-resources-stage.log.
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
cargo build --locked -p jj-store-http --example browser_preview
```

**Notes**

browser-preview-rebuild: exit 0; 2.66s. See retained log and execution exit status.  Raw output retained in ignored .local/test-audit-2026-10-08/browser-preview-rebuild.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/appearance.spec.ts:14 web/tests/browser/automatic-refresh.spec.ts:11 web/tests/browser/local-bootstrap.spec.ts:11 web/tests/browser/local-capture-loading.spec.ts:42 web/tests/browser/local-capture-loading.spec.ts:115 web/tests/browser/local-capture-settings.spec.ts:411 web/tests/browser/local-search-settings.spec.ts:5 web/tests/browser/local-storage.spec.ts:5 web/tests/browser/local-workspaces.spec.ts:5 web/tests/browser/reads.spec.ts:383 web/tests/browser/reads.spec.ts:450 web/tests/browser/reads.spec.ts:526 web/tests/browser/reads.spec.ts:579 web/tests/browser/repository-navigation.spec.ts:14 web/tests/browser/search.spec.ts:10 web/tests/browser/workspaces.spec.ts:85 --output /Users/stevejuma/code/lapismd/jj-store/.local/test-audit-2026-10-08/web-failures-traces
```

**Notes**

web-failures-recheck: exit 1; 205.27s. 15 failed; 2 passed (3.4m). 15 failures reproduced. The workspace-name/cold-reload case passed on rerun and remains classified intermittent; a second selected automatic-refresh variant also passed. Raw output retained in ignored .local/test-audit-2026-10-08/web-failures-recheck.log.
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
JJ_STORE_DESKTOP_TEST_RESOURCES=/Users/stevejuma/code/lapismd/jj-store/.local/test-audit-2026-10-08/desktop-resources cargo test --manifest-path desktop/Cargo.toml --locked --no-default-features --test managed_runtime -- --ignored
```

**Notes**

desktop-managed-runtime: exit 0; 13.57s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/desktop-managed-runtime.log.
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
pnpm exec playwright test --config web/playwright.docs.config.ts --output /Users/stevejuma/code/lapismd/jj-store/.local/test-audit-2026-10-08/docs-traces
```

**Notes**

docs-browser-final: exit 0; 40.18s. 20 passed (39.4s).  Raw output retained in ignored .local/test-audit-2026-10-08/docs-browser-final.log.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-browser-recheck: exit 1; 18.28s. 0 pass, 1 fail, 0 skipped.  Raw output retained in ignored .local/test-audit-2026-10-08/catalog-browser-recheck.log.
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
cargo test -p jj-store-cli --test local_agent_provenance --locked original_local_agent_descriptor_remains_readable_before_v2_publication -- --exact
```

**Notes**

local-agent-provenance-recheck: exit 101; 6.68s. 0 passed, 1 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/local-agent-provenance-recheck.log.
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
node scripts/local-browser-acceptance.mjs
```

**Notes**

local-browser: exit 0; 358.43s. 5 passed (32.5s).  Raw output retained in ignored .local/test-audit-2026-10-08/local-browser.log.
<!-- /run -->
<!-- /evidence-field -->

