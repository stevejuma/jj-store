---
schema: 1
id: E-GOV-005
---

# E-GOV-005 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test
```

**Notes**

catalog-unit: exit 1; 90.21s. Test Files  1 failed | 126 passed (127); Tests  1 failed | 788 passed | 1 skipped (790). Initial full run hit one disclosure-layout timeout during heavy concurrent compilation. Exact rerun and complete two-worker rerun passed; retain this initial result as resource-sensitive, not an unresolved assertion failure. Raw output retained in ignored .local/test-audit-2026-10-08/catalog-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2
```

**Notes**

catalog-unit-bounded: exit 0; 37.02s. Test Files  127 passed (127); Tests  789 passed | 1 skipped (790). Complete catalog unit suite passed with two workers. The one fixture-dependent skipped case subsequently passed in working-tree-captured-ui. Raw output retained in ignored .local/test-audit-2026-10-08/catalog-unit-bounded.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

storybook: exit 1; 683.22s. Test Files  9 failed | 120 passed (139); Tests  10 failed | 615 passed (625); Errors  1 error. Initial run: 10 assertion failures, one import failure and one browser-connection error; only 129 of 139 files produced results. All 10 missing files and all 9 unsuccessful files were selected in the fresh recheck. Raw output retained in ignored .local/test-audit-2026-10-08/storybook.log.
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
cargo test --manifest-path desktop/Cargo.toml --workspace --locked --no-default-features --no-fail-fast
```

**Notes**

desktop: exit 0; 29.06s. 111 passed, 0 failed, 2 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/desktop.log.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml --workspace --locked --no-fail-fast
```

**Notes**

local-agent: exit 0; 172.13s. 715 passed, 0 failed, 1 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/local-agent.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --workspace --locked --no-fail-fast
```

**Notes**

rust-workspace: exit 101; 2917.48s. 2078 passed, 4 failed, 53 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/rust-workspace.log.
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
cargo test --manifest-path runtime/compute/Cargo.toml --locked --no-fail-fast
```

**Notes**

compute: exit 0; 374.88s. 1 passed, 0 failed, 1 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/compute.log.
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
pnpm catalog:test:browser
```

**Notes**

catalog-browser: exit 1; 83.1s. 0 pass, 1 fail, 0 skipped.  Raw output retained in ignored .local/test-audit-2026-10-08/catalog-browser.log.
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
pnpm web:test:browser
```

**Notes**

web-browser: exit 1; 68.05s. See retained log and execution exit status. Built web assets and captured real native fixtures successfully, then Playwright's preview-server startup exceeded its 30-second limit during compilation. No browser assertions ran in this attempt. Raw output retained in ignored .local/test-audit-2026-10-08/web-browser.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-chip/CommunityChip.stories.svelte catalog/components/community-projects/CommunityProjectsColumnAlert.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSpacing.stories.svelte catalog/components/store-work/LayoutControls.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte --reporter=verbose --reporter=json --outputFile=/Users/stevejuma/code/lapismd/jj-store/.local/test-audit-2026-10-08/storybook-recheck-report.json
```

**Notes**

storybook-recheck: exit 1; 110.69s. Test Files  8 failed | 11 passed (19); Tests  10 failed | 59 passed (69). All 19 selected files completed. The same 10 assertions failed in 8 files; 59 tests passed. The initial StoreApplication import failure and StoreEvolutionColumn browser disconnect cleared. All 139 story files have now been attempted to completion across these two runs. Raw output retained in ignored .local/test-audit-2026-10-08/storybook-recheck.log.
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
pnpm docs:test:browser
```

**Notes**

docs-browser: exit 1; 68.53s. See retained log and execution exit status. Preview startup exceeded 30 seconds; no browser assertions ran. Final run with prebuilt preview and no competing builds passed. Raw output retained in ignored .local/test-audit-2026-10-08/docs-browser.log.
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
JJ_STORE_DEV_PORT=7490 pnpm web:test:real
```

**Notes**

real-browser: exit 1; 1300.42s. 15 failed; 13 skipped; 7 passed (10.7m). Fresh isolated native/mixed fixture on port 7490; interactive .local/dev-demo was not reset. Five-minute renewal passed. Specialized fixture-dependent skips remain explicit; dedicated filesystem, Stacks, storage and Git-bridge lanes are recorded separately. Raw output retained in ignored .local/test-audit-2026-10-08/real-browser.log.
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
cargo test --manifest-path runtime/embedded-rustfs/Cargo.toml --locked --no-fail-fast
```

**Notes**

embedded-runtime: exit 0; 11.74s. 0 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/embedded-runtime.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test desktop_supervision --locked -- --ignored
```

**Notes**

desktop-supervision: exit 0; 18.6s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/desktop-supervision.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test work_rustfs --locked -- --ignored
```

**Notes**

work-rustfs: exit 0; 11.02s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/work-rustfs.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable --locked cold_configured_codex_capture_initializes_publishes_and_preserves_pause -- --ignored
```

**Notes**

capture-rustfs: exit 0; 141.66s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/capture-rustfs.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

web-browser-recheck: exit 1; 403.53s. 16 failed; 78 passed (6.7m). Complete 94-test mocked/registered-fixture browser suite: 78 passed and 16 failed. Exact failing selections were rerun in web-failures-recheck. Raw output retained in ignored .local/test-audit-2026-10-08/web-browser-recheck.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-server --test repository_conversion --locked -- --ignored
```

**Notes**

conversion-rustfs: exit 0; 187.19s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/conversion-rustfs.log.
<!-- /run -->
<!-- /evidence-field -->

