---
schema: 1
id: E-GOV-005
---

# E-GOV-005 details 2

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
pnpm dev:restart
```

**Notes**

Existing .local/dev-demo fixture preserved. Loopback LocalTrusted server ready at http://127.0.0.1:7460/; search ready; server plus storage 243824 KiB at restart.
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
docker info --format '{{.ServerVersion}}'; docker version --format '{{.Server.Version}}' (second probe bounded to 15 seconds)
```

**Notes**

Docker status did not respond. First owned probe was terminated; second timed out. Container-backed rustfs-contract --run/--rust-client/--server remains blocked, distinct from passing embedded RustFS fixtures.
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
node --test --test-name-pattern '^migration_preserves_original_evidence_tasks_requirements_and_byte_archives$' scripts/spec/progress-regression.test.mjs
```

**Notes**

Reproduces 12 original task state/gate differences and changed JST-UI-015 statement.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/production-store-mutations.test.ts web/tests/production-store-search.test.ts
```

**Notes**

Both files fail to load @tanstack/svelte-query HydrationBoundary.svelte: ERR_UNKNOWN_FILE_EXTENSION.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/disclosure-layout.test.ts --maxWorkers=1
```

**Notes**

8 passed. Initial broad-run timeout cleared; complete bounded-worker catalog rerun also passed.
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
CARGO_BUILD_JOBS=2 cargo build --locked -p jj-store-http --example browser_preview
```

**Notes**

Prebuild for the browser harness after its 30-second startup limit expired during concurrent compilation. Later retries wait until builds finish.
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
python3 scripts/local-agent-acp.acceptance.py --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --agent /Users/stevejuma/code/lapismd/jj-store/runtime/local-agent/target/debug/jj-store-local-agent --tools /Users/stevejuma/code/lapismd/jj-store/runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

2 passed, allow and deny, against fixture-only inference. Staged desktop/resources/jj-acp only advertises control versions 1/2; existing current development executable supports 1/2/3. No sibling source or installed executable changed.
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
CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 cargo test -p jj-store-http --lib --locked contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies -- --exact
```

**Notes**

Reproduces 162 actual routes versus expected 145 at crates/jj-store-http/src/contract/tests.rs:328.
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
pnpm check:progress
```

**Notes**

Evidence bookkeeping passed: 146 tasks, 661 evidence records. Does not imply passing product tests.
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
pnpm spec:validate
```

**Notes**

7 validators, 3206 chapters, 181 requirements passed after the final evidence refresh.
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
Python urllib GET http://127.0.0.1:7460/ and all four referenced JS/CSS assets; lsof -nP -iTCP -sTCP:LISTEN
```

**Notes**

HTML, three JavaScript assets and one CSS asset returned 200 with correct content types. Only intended 7460 listener remained among owned audit ports at the check; final fixture lanes run separately and clean up.
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
pnpm evidence:new -- --id E-GOV-005 --task Bootstrap --requirement JST-GOV-003 --requirement JST-GOV-005 --requirement JST-GOV-007 --requirement JST-GOV-009 --requirement JST-GOV-011 --requirement JST-CLI-004
```

**Notes**

Created the governed audit record; populated from individual retained logs and exact reruns.
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
python3 .local/test-audit-2026-10-08/summarize.py; node .local/test-audit-2026-10-08/write-evidence.mjs; pnpm progress:history
```

**Notes**

Generated bounded evidence pages and task history. Generator-only formatting changes to M24/M25 and earlier E-FLEET-006/007 and E-GOV-004 were inspected and restored to starting source; previous evidence remains byte-identical.
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
pnpm exec prettier --write spec/src/current/progress/Bootstrap/index.md spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
```

**Notes**

Formatted authored Bootstrap review and history. Historical evidence pages use the bounded evidence renderer and are excluded by the repository prettierignore.
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
node --test scripts/contracts.test.mjs scripts/dev-demo-collaboration.test.mjs scripts/dev-demo-folder.test.mjs scripts/dev-demo-processes.test.mjs scripts/dev-demo-profiles.test.mjs scripts/dev-demo-repository.test.mjs scripts/dev-demo-search.test.mjs scripts/dev-demo-state.test.mjs scripts/dev-demo-work.test.mjs scripts/dev-ui.test.mjs scripts/docs-assets.test.mjs scripts/docs.test.mjs scripts/governance.test.mjs scripts/install-cli.test.mjs scripts/local-package-pnpmfile.test.mjs scripts/progress-mapping.test.mjs scripts/ui-ownership.test.mjs scripts/web-architecture.test.mjs scripts/web-manifest.test.mjs scripts/spec/book-references.test.mjs scripts/spec/evidence-format.test.mjs scripts/spec/navigation-regression.test.mjs scripts/spec/progress-regression.test.mjs scripts/spec/vcs-regression.test.mjs
```

**Notes**

node-tools: exit 1; 4.71s. 119 pass, 1 fail, 0 skipped.  Raw output retained in ignored .local/test-audit-2026-10-08/node-tools.log.
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
pnpm test:storage-harness
```

**Notes**

storage-harness: exit 0; 0.8s. See retained log and execution exit status.  Raw output retained in ignored .local/test-audit-2026-10-08/storage-harness.log.
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
pnpm exec tsc -p scripts/tsconfig.contracts.json
```

**Notes**

contracts-types: exit 0; 2.25s. See retained log and execution exit status.  Raw output retained in ignored .local/test-audit-2026-10-08/contracts-types.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

web-unit: exit 1; 3.81s. 120 pass, 2 fail, 0 skipped.  Raw output retained in ignored .local/test-audit-2026-10-08/web-unit.log.
<!-- /run -->
<!-- /evidence-field -->

