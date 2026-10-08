---
schema: 1
id: E-GOV-005
---

# E-GOV-005

<!-- evidence-field:id markdown -->
## Id

E-GOV-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-003
- JST-GOV-005
- JST-GOV-007
- JST-GOV-009
- JST-GOV-011
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository-wide requested test audit and preserved development restart
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pzkzmzwu; audited clean product source above parent 98a4115d (vwrskpzn)
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

User explicitly requested a development-server restart followed by all repository tests and a failure report. Initial jj --no-pager st was clean. This explicit request is the JST-GOV-007 escalation reason for full suites. The root test command omits browser, desktop, runtime and several script/fixture lanes, so those were inventoried and run separately. Independent suites continue after failures.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

This is a verification audit, with no product/test fixes. Tests are not all passing. Counts below are per invocation; reruns overlap and must not be added as unique tests.

The completed main Rust workspace run reports 2,078 passed, 4 failed and 53 ignored. Three failures reproduce; the capture-settlement case passes its exact rerun. The ignored count includes helpers invoked by parent tests and gates subsequently run explicitly, so it is not the number of untested features.

### Reproduced unit, script and catalog-browser failures

- scripts/spec/progress-regression.test.mjs: migration_preserves_original_evidence_tasks_requirements_and_byte_archives — 12 historical task state/gate differences plus JST-UI-015 statement mismatch; reproduced.
- web/tests/production-store-mutations.test.ts and production-store-search.test.ts — ERR_UNKNOWN_FILE_EXTENSION for @tanstack/svelte-query HydrationBoundary.svelte; both reproduced.
- scripts/catalog-browser.test.mjs: cold_catalog_contract — README.md button absent at line 251; reproduced.
- crates/jj-store-cli/tests/local_agent_provenance.rs:193: original_local_agent_descriptor_remains_readable_before_v2_publication — Corrupt; reproduced.
- crates/jj-store-http/src/contract/tests.rs:328: contract_inventory_covers_security_methods_media_and_task_bodies — 162 routes versus expected 145; reproduced.

### Main Rust workspace observed failures

- original_local_agent_descriptor_remains_readable_before_v2_publication — Reproduced in exact rerun; see commands and log details.
- contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies — Reproduced in exact rerun; see commands and log details.
- capture_import_state::tests::import_callback_retains_gate_until_settlement — Unavailable in broad run; exact rerun passed (intermittent).
- process::tests::capture_factory::cold_capture_factory_prepares_private_resources_and_retains_ordered_lifetime — Reproduced; foreground-stop child retains store reference at capture_factory/foreground.rs:66. Nested helper is one root failure, not a second case.

### Storybook assertions reproduced

- catalog/components/store-application/RepositoryLayout.stories.svelte: OverviewBodyInset — AssertionError: expected 0 to be greater than or equal to 12     at Instrumenter.invoke (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-ZuY.js?v=83969720:2432:20)     at Instrumenter.intercept (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-ZuY
- catalog/components/store-application/RepositoryLayout.stories.svelte: OverviewMobileBodyInset — AssertionError: expected 0 to be greater than or equal to 12     at Instrumenter.invoke (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-ZuY.js?v=83969720:2432:20)     at Instrumenter.intercept (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-ZuY
- catalog/components/store-application/RepositoryReview.stories.svelte: ShellAndRowLocalGraphSpacing — TestingLibraryElementError: Unable to find an accessible element with the role "status"  Here are the accessible roles:    navigation:    Name "Application navigation":   &lt;nav     aria-label="Application navigation"     class="community-app-shell__header svelte-14oig6g"     style="--community-app-shell-header-main-start: 288px; --community-app-shell-header-main-width: 904px;"   /&gt;    Name "Repository navigation":   &lt;nav     aria-label="Repository
- catalog/components/store-application/StoreCollaboration.stories.svelte: AgentResponseActivity — TestingLibraryElementError: Unable to find an accessible element with the role "region" and name "Changed file src/lib.rs"  Here are the accessible roles:    navigation:    Name "Application navigation":   &lt;nav     aria-label="Application navigation"     class="community-app-shell__header svelte-14oig6g"     style="--community-app-shell-header-main-start: 48px; --community-app-shell-header-main-width: 824px;"   /&gt;    Name "Repository navigation":
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte: RightRailNavigation — AssertionError: expected &lt;button …(8)&gt;…(2)&lt;/button&gt; to be null     at Instrumenter.invoke (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-ZuY.js?v=83969720:2432:20)     at Instrumenter.intercept (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-Z
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte: SelectBranchAndApplyToWorkspace — Error: Unable to find role="button" and name "Apply command"  Ignored nodes: comments, script, style &lt;body   class="sb-main-fullscreen"   data-ui-catalog-full-viewport=""   style="filter: none;" &gt;   &lt;div     class="sb-preparing-story sb-wrapper"   &gt;             &lt;div       class="sb-loader"     /&gt;         &lt;/div&gt;        &lt;div     class="sb-preparing-docs sb-wrapper"   &gt;             &lt;div       class="sb-previewBlock"     &gt;                   &lt;div     
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte: MultipleAdjacentDiffs — TestingLibraryElementError: Unable to find an accessible element with the role "button" and name "src/toolbar.ts"  Here are the accessible roles:    banner:    Name "":   &lt;header     class="stack-workspace__stack-header"     data-ui-component="column-canvas"     data-ui-part="column-header"   /&gt;    --------------------------------------------------   button:    Name "Show all changes in stack feature/workspace":   &lt;button     aria-label="Show all
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte: AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker — TestingLibraryElementError: Unable to find a label with the text of: Search branches  Ignored nodes: comments, script, style &lt;body   style="filter: none;" &gt;   &lt;div     class="sb-preparing-story sb-wrapper"   &gt;             &lt;div       class="sb-loader"     /&gt;         &lt;/div&gt;        &lt;div     class="sb-preparing-docs sb-wrapper"   &gt;             &lt;div       class="sb-previewBlock"     &gt;                   &lt;div         class="sb-previewBlock_header"      
- catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte: DiffHeaderAndWideActionMenus — TestingLibraryElementError: Unable to find an accessible element with the role "button" and name "src/api.ts"  Here are the accessible roles:    presentation:    Name "":   &lt;div     class="ui-minimal-app-shell__mobile-stage"     data-ui-component="app-shell"     data-ui-part="mobile-stage"     role="presentation"     style="touch-action: pan-y;"   /&gt;    --------------------------------------------------   region:    Name "Local stack workspace": 
- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte: EmptyWorkspaceKeepsItsBranchPromptCentered — AssertionError: expected 389.9999694824219 to be 390 // Object.is equality     at Instrumenter.invoke (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps/test-DuTF-ZuY.js?v=83969720:2432:20)     at Instrumenter.intercept (http://127.0.0.1:63315/node_modules/.cache/storybook/10.5.7/06ff3602a41cba2af5618a92d8ce78fdab71ee4cdd8dfd553998f609c3b9af6d/sb-vitest/deps

### Mocked browser assertions reproduced

- web/tests/browser/appearance.spec.ts:14:1 › cold device appearance restores under the production CSP and propagates across tabs — Error: expect(locator).toBeVisible() failed | Locator: getByRole('region', { name: 'Appearance settings' }).getByRole('heading', { name: 'Appearance', exact: true }) | Expected: visible | Error: element(s) not found
- web/tests/browser/automatic-refresh.spec.ts:11:3 › new repository data refreshes automatically with historical selection — Error: expect(received).toBeGreaterThan(expected) | Expected: &gt; 0 | Received:   0
- web/tests/browser/local-bootstrap.spec.ts:11:1 › local trusted bootstrap auto-opens the sole repository without credentials — Error: expect(locator).toBeVisible() failed | Locator: getByText('Local acceptance') | Expected: visible | Error: strict mode violation: getByText('Local acceptance') resolved to 2 elements:
- web/tests/browser/local-capture-loading.spec.ts:42:1 › cold capture loading keeps guarded controls and recovers from busy reads — Error: expect(locator).toBeVisible() failed | Locator: getByRole('button', { name: 'Pause capture' }) | Expected: visible | Error: element(s) not found
- web/tests/browser/local-capture-loading.spec.ts:115:1 › cold grouped Codex configuration and harness-filtered source table retain identities — Test timeout of 30000ms exceeded. | Error: locator.fill: Test timeout of 30000ms exceeded.
- web/tests/browser/local-capture-settings.spec.ts:411:1 › cold local configuration editing retains drafts and announces restart — Test timeout of 30000ms exceeded. | Error: locator.fill: Test timeout of 30000ms exceeded.
- web/tests/browser/local-search-settings.spec.ts:5:1 › Search tabs preserve drafts and separate Save from Apply — Error: expect(locator).toHaveCSS(expected) failed | Locator:  locator('.store-settings') | Expected: "0px" | Received: "16px"
- web/tests/browser/local-storage.spec.ts:5:1 › Storage recovers a lost response without replacing the captured target — Error: expect(locator).toBeVisible() failed | Locator:  getByText('Storage unavailable', { exact: true }) | Expected: visible | Received: hidden
- web/tests/browser/local-workspaces.spec.ts:5:1 › local folder settings preserve drafts and link without changing repository context — Error: expect(locator).toBeVisible() failed | Locator: getByText(/workspace links changed/) | Expected: visible | Error: element(s) not found
- web/tests/browser/reads.spec.ts:383:1 › repository directories load lazily and flat views request recursive paths — Error: expect(locator).toHaveAttribute(expected) failed | Locator: getByRole('button', { name: 'src', exact: true }) | Expected: "false" | Error: element(s) not found
- web/tests/browser/reads.spec.ts:450:1 › index_preparation_retries_the_original_query_after_context_edits — Error: expect(received).toBe(expected) // Object.is equality | Expected: "?at_op=7447085f0e07f2a19f39a67f489fcf2cf845acd75a27e2c57add857f58bafc93267e5e1fafdd321b816e443b0519dbb7509727813eca297d0e4ee2afe0ba8100&amp;limit=50&amp;revset=all%28%29&amp;user_email=&amp;at_time_millis=1791456628327" | Received: "?at_op=7447085f0e07f2a19f39a67f489fcf2cf845acd75a27e2c57add857f58bafc93267e5e1fafdd321b816e443b0519dbb7509727813eca297d0e4ee2afe0ba8100&amp;limit=50&amp;revset=all%28%
- web/tests/browser/reads.spec.ts:526:1 › new_file_selection_discards_an_older_inflight_preview — Test timeout of 30000ms exceeded. | Error: locator.click: Test timeout of 30000ms exceeded.
- web/tests/browser/reads.spec.ts:579:1 › files remain usable while optional header and attribution are unavailable — Error: expect(received).toBe(expected) // Object.is equality | Expected: 0 | Received: 1
- web/tests/browser/repository-navigation.spec.ts:14:1 › right rail destinations match repository tabs and preserve workspace snapshot — Error: expect(received).toBe(expected) // Object.is equality | Expected: true | Received: false
- web/tests/browser/search.spec.ts:10:1 › command search opens native results and routes through stable identities — Test timeout of 30000ms exceeded. | Error: locator.fill: Test timeout of 30000ms exceeded.

### real-browser observed failures

- web/tests/real/event-freshness.spec.ts:135:1 › two clients react to Work events, preserve drafts, and recover missed replay — Error: expect(received).toEqual(expected) // deep equality
- web/tests/real/event-freshness.spec.ts:240:1 › a stale server cursor resets continuity once and exact local preparation retains its pin — Error: expect(locator).toBeVisible() failed | Locator: getByRole('button', { name: 'Prepare graph index', exact: true }) | Expected: visible | Error: element(s) not found
- web/tests/real/git-bridge.spec.ts:7:1 › Git import refreshes automatically — Error: expect(locator).toBeVisible() failed | Locator: getByRole('option', { name: 'main', exact: true }) | Expected: visible | Error: element(s) not found
- web/tests/real/local-mixed.spec.ts:79:1 › merged_native_threads_create_send_and_reload_independent_of_local_browsing — Test timeout of 60000ms exceeded. | Error: locator.click: Test timeout of 60000ms exceeded.
- web/tests/real/native-repository.spec.ts:103:1 › external publication refreshes automatically and preserves selection — Test timeout of 60000ms exceeded. | Error: page.waitForRequest: Test timeout of 60000ms exceeded.
- web/tests/real/native-repository.spec.ts:150:1 › native conflict selection previews and publishes one atomic resolution — Error: expect(locator).toBeVisible() failed | Locator: getByRole('region', { name: 'Resolve native conflicts' }).getByText('Conflicts resolved') | Expected: visible | Error: element(s) not found
- web/tests/real/native-repository.spec.ts:206:1 › browser mutation previews and publishes one native request — Error: expect(locator).toBeVisible() failed | Locator: getByRole('dialog', { name: 'Create change' }).getByText('Preview prepared') | Expected: visible | Error: element(s) not found
- web/tests/real/project-polish.spec.ts:4:1 › repository documents linked projects and Activity filters retain native scope — Error: expect(locator).toBeVisible() failed | Locator: getByRole('button', { name: 'Filter activity', exact: true }) | Expected: visible | Error: element(s) not found
- web/tests/real/project-repository-actions.spec.ts:7:3 › project repository menu and folder dialog desktop — Error: expect(locator).toBeVisible() failed | Locator: getByRole('menuitem', { name: 'Add local repository', exact: true }).locator('svg[aria-hidden="true"]') | Expected: visible | Error: element(s) not found
- web/tests/real/project-repository-actions.spec.ts:7:3 › project repository menu and folder dialog mobile — Error: expect(locator).toBeVisible() failed | Locator: getByRole('menuitem', { name: 'Add local repository', exact: true }).locator('svg[aria-hidden="true"]') | Expected: visible | Error: element(s) not found
- web/tests/real/projects-activity.spec.ts:4:1 › native project task and review pages share pinned commit activity — Error: expect(locator).toHaveAttribute(expected) failed | Locator:  getByRole('region', { name: 'Projects and tasks' }).getByRole('article', { name: 'Task details' }).getByRole('button', { name: 'Activity', exact: true }) | Expected: "false" | Received: "true"
- web/tests/real/settings-layout.spec.ts:38:1 › local settings use shared rows, right controls and icon availability states — Error: expect(locator).toBeVisible() failed | Locator:  locator('.store-settings__action-row').nth(1).locator(':scope &gt; div').first().locator('p') | Expected: visible | Received: hidden
- web/tests/real/task-reactions.spec.ts:5:1 › task reaction picker persists exact comments and refreshes a second client — Error: expect(locator).toBeVisible() failed | Locator: getByRole('listitem', { name: 'Comment 85c3407d', exact: true }).getByRole('button', { name: /1 reaction with like/ }) | Expected: visible | Error: element(s) not found
- web/tests/real/work-seed.spec.ts:4:1 › development work samples survive cold startup and additive reseeding — Error: expect(received).toBe(expected) // Object.is equality | Expected: true | Received: undefined
- web/tests/real/work-store.spec.ts:40:1 › native projects tasks and discussion persist on RustFS and refresh automatically — Error: expect(locator).toHaveText(expected) failed | Locator: getByRole('region', { name: 'Projects and tasks' }).getByRole('tab') | Error: Command failed: pnpm exec playwright test --config web/playwright.real.config.ts

### stacks-browser observed failures

- web/tests/real/stacks-browser.spec.ts:13:1 › cold_focused_stacks_navigation_history_reload_and_return — Test timeout of 60000ms exceeded. | Error: locator.click: Test timeout of 60000ms exceeded.
- web/tests/real/stacks-browser.spec.ts:131:1 › cold_local_megamerges_apply_unapply_and_named_wip_survive_reload — Test timeout of 60000ms exceeded. | Error: locator.click: Test timeout of 60000ms exceeded. | Error: Command failed: pnpm exec playwright test --config web/playwright.real.config.ts stacks-browser.spec.ts --output .scratch/test-results-stacks-browser

### storage-browser observed failures

- web/tests/real/storage-maintenance.spec.ts:11:1 › native Storage keeps exact checkpoint targets through cold restart and verifies captured data — Error: expect(received).toEqual(expected) // deep equality | Error: server service failed: graceful shutdown exceeded five seconds; in-flight outcomes may be unknown | Error: server service failed: graceful shutdown exceeded five seconds; in-flight outcomes may be unknown | Error: Command failed: pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/storage-maintenance.spec.ts

### local-index-browser observed failures

- web/tests/real/local-index-cancel.spec.ts:10:1 › local preparation recovery and settings cancel the captured task — Error: expect(locator).toBeEnabled() failed | Locator: getByRole('button', { name: 'Cancel preparation', exact: true }) | Expected: enabled | Error: element(s) not found

### Dedicated browser qualification details

- Git bridge: the mandatory Playwright child passed with its specialized native Git-cycle fixture; the generic real-browser failure is fixture-dependent.
- Storage: axe rejected .store-repository-picker__value (Storage acceptance), #9a9ea4 on white, contrast 2.69:1 versus 4.5:1; teardown additionally reported graceful shutdown exceeding five seconds.
- Stacks navigation: Prepare graph index detached while clicking at stacks-browser.spec.ts:76.
- Stacks megamerge: Unapply topic button was absent at stacks-browser.spec.ts:281.

### Invocation outcomes

| Lane | Result | Reported tests |
| --- | --- | --- |
| node-tools | fail | 119 pass, 1 fail, 0 skipped |
| storage-harness | pass | See retained log and execution exit status |
| contracts-types | pass | See retained log and execution exit status |
| web-unit | fail | 120 pass, 2 fail, 0 skipped |
| catalog-unit | fail | Test Files  1 failed / 126 passed (127); Tests  1 failed / 788 passed / 1 skipped (790) |
| catalog-unit-bounded | pass | Test Files  127 passed (127); Tests  789 passed / 1 skipped (790) |
| storybook | fail | Test Files  9 failed / 120 passed (139); Tests  10 failed / 615 passed (625); Errors  1 error |
| desktop | pass | 111 passed, 0 failed, 2 ignored (reported Rust test results) |
| local-agent | pass | 715 passed, 0 failed, 1 ignored (reported Rust test results) |
| rust-workspace | fail | 2078 passed, 4 failed, 53 ignored (reported Rust test results) |
| compute | pass | 1 passed, 0 failed, 1 ignored (reported Rust test results) |
| catalog-browser | fail | 0 pass, 1 fail, 0 skipped |
| web-browser | fail | See retained log and execution exit status |
| storybook-recheck | fail | Test Files  8 failed / 11 passed (19); Tests  10 failed / 59 passed (69) |
| docs-browser | fail | See retained log and execution exit status |
| real-browser | fail | 15 failed; 13 skipped; 7 passed (10.7m) |
| embedded-runtime | pass | 0 passed, 0 failed, 0 ignored (reported Rust test results) |
| desktop-supervision | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| work-rustfs | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| capture-rustfs | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| web-browser-recheck | fail | 16 failed; 78 passed (6.7m) |
| conversion-rustfs | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| working-tree-service | pass | 5 passed, 0 failed, 0 ignored (reported Rust test results) |
| working-tree-http | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| docs-browser-recheck | fail | See retained log and execution exit status |
| working-tree-captured-ui | pass | Test Files  1 passed (1); Tests  12 passed (12) |
| agent-workspaces | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| private-tool-install | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| os-credentials | pass | 3 passed, 0 failed, 0 ignored (reported Rust test results) |
| local-agent-tools-build | pass | See retained log and execution exit status |
| local-agent-tools-protocol | pass | 5 passed |
| local-agent-acp-protocol | fail | See retained log and execution exit status |
| desktop-resources-stage | pass | See retained log and execution exit status |
| browser-preview-rebuild | pass | See retained log and execution exit status |
| web-failures-recheck | fail | 15 failed; 2 passed (3.4m) |
| desktop-managed-runtime | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| docs-browser-final | pass | 20 passed (39.4s) |
| catalog-browser-recheck | fail | 0 pass, 1 fail, 0 skipped |
| local-agent-provenance-recheck | fail | 0 passed, 1 failed, 0 ignored (reported Rust test results) |
| local-browser | pass | 5 passed (32.5s) |
| stacks-browser | fail | 2 failed; 1 passed (2.3m) |
| storage-browser | fail | 1 failed |
| git-bridge-browser | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| capture-settlement-recheck | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
| capture-factory-recheck | fail | 0 passed, 1 failed, 0 ignored (reported Rust test results) |
| dev-ui | pass | See retained log and execution exit status |
| local-index-browser | fail | 1 failed |
| seeded-task-browser | fail | See retained log and execution exit status |
| embedded-storage | pass | 2 passed, 0 failed, 0 ignored (reported Rust test results) |
| local-trusted | pass | 1 passed, 0 failed, 0 ignored (reported Rust test results) |
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/progress/Bootstrap/index.md
- spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
- spec/src/history/2026/10/08/E-GOV-005/index.md
- spec/src/history/2026/10/08/E-GOV-005/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-GOV-005/details-02.md
- spec/src/history/2026/10/08/E-GOV-005/details-03.md
- spec/src/history/2026/10/08/E-GOV-005/details-04.md
- spec/src/history/2026/10/08/E-GOV-005/details-05.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-GOV-005 details 2](details-02.md)
- [E-GOV-005 details 3](details-03.md)
- [E-GOV-005 details 4](details-04.md)
- [E-GOV-005 details 5](details-05.md)
