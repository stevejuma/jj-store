---
schema: 1
id: E-SERVER-CONTROLS-001
---

# E-SERVER-CONTROLS-001

<!-- evidence-field:id markdown -->
## Id

E-SERVER-CONTROLS-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-022
- JST-UI-016
- JST-GOV-004
- JST-API-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ui-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

pypzwott
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-connections/StoreServerSwitcher.svelte
- web/src/lib/ui/store-connections/StoreConnectionDialog.svelte
- web/src/lib/ui/store-connections/presentation.ts
- catalog/components/store-application/ServerConnections.stories.svelte
- catalog/fixtures/ServerConnectionsFixture.svelte
- spec/src/current/contracts/ui/40-server-switcher.md
- spec/src/current/progress/M27/index.md
- spec/src/current/progress/M27/history/M27/index.md
- spec/src/history/2026/10/12/E-SERVER-CONTROLS-001/index.md
- spec/src/history/2026/10/12/E-SERVER-CONTROLS-001/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
- spec/src/history/2026/10/12/E-SERVER-CONTROLS-001/details-02.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
jj --no-pager st; inspect Community navigation/form reference, released Design Core controls and the live main app
```

**Notes**

Before state: Local used secondary styling with a separate accent marker; Add server and Local had different insets from the main sidebar. The existing switcher rendered every saved server. Separate build-storage work was already present; it is preserved and excluded from this slice. The reference inspection copied no Community source and changes no dependencies.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte
```

**Notes**

Before: all four existing stories passed. They did not assert the requested visible-server limit or matching presentation.
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
pnpm dev:ui
```

**Notes**

Main Vite HMR at http://127.0.0.1:7460/ with backend 7462. Existing .local/dev-demo preserved and search ready. Reuse native/ACP/RustFS binaries; no reset or rebuild.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerConnectionOwners.stories.svelte
```

**Notes**

First extended run: 10 pass, 3 fail. Newly authored checks clicked before closing animation released pointer events and expected Connect rather than the existing Loading Connect accessible name. Correct the private story checks by awaiting released pointer events and matching the spinner name; no product diagnostic or assertion was suppressed.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerConnectionOwners.stories.svelte
```

**Notes**

Corrected run: 13 pass across two files, including Axe. Follow-up adds menu-trigger to sign-in-dialog focus and pending guard scenarios; latest cold-process run has 15 pass, zero failures. Logs: /tmp/jj-store-server-controls-stories.log. Inherited Vite default-config and Node module.register deprecation notices are separate from this slice.
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
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
pnpm check:architecture
pnpm check:boundaries
```

**Notes**

Web and catalog have zero errors/warnings; catalog boundaries pass. UI ownership, 1752-module web architecture and 17-package boundaries pass. Repeat type checks after final private presentation-owner/focus changes.
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
cua_repl: reload main app, inspect sidebar, open Add server, test 390x844 layout, close and compare selected button DOM styles
```

**Notes**

Cold HMR app reload preserves all four existing repository entries. Local and selected Agents both measure 36px high, 12px radius and the identical theme foreground 11% background. Dialog has tinted header, edge-to-edge separators, trailing inputs and Connect footer; mobile stacks without overflow. Viewport restored; close returns focus to Add server; no app console errors/warnings. Local-only live configuration is preserved; overflow and synthetic auth are covered in production-component stories. Ignored screenshots: .local/server-controls-review/sidebar.jpg and add-server.jpg.
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
node scripts/check-progress.mjs
node node_modules/@lapismd/spec-validator/dist/cli.js first
node node_modules/@lapismd/spec-validator/dist/cli.js validate
```

**Notes**

Initial bookkeeping: current working revision changed during validation due to separate ongoing build-storage work, whose fresh evidence remains incomplete. The new UI evidence also used descriptive test labels instead of file references and listed deferred M27 gates as slice gaps; correct those metadata fields. General seven-validator validation passes. Verify this slice against its committed revision to isolate the concurrent work.
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
spec-validator first --file (the five changed UI/catalog files and owning UI chapter)
```

**Notes**

Explicit file mode cannot use unchanged-contract confirmations and also maps web files to API requirements. Keep the API boundary unchanged and name JST-API-011 in the qualified evidence confirmation; use a revision-scoped gate on the isolated UI slice rather than changing an API contract.
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
jj commit --no-integrate-operation --tool server-controls-select (explicit UI files; scripted selection of only this entry in shared navigation)
revision-scoped check-progress and spec-first against the temporary operation
```

**Notes**

Noninteractive diff-tool setup required program plus edit-args. The unintegrated revision is not visible to default JJ readers, so select its operation through a temporary read-only JJ wrapper. Initial ledger check then rejected story file paths as named test references; Storybook acceptance is recorded in commands, and the named-test-only list remains empty. No working-tree data or unrelated edits were changed. Retain these bookkeeping attempts separately from zero-failure final UI acceptance.
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
PATH="/tmp/jj-server-controls-bin:$PATH" JJ_STORE_CHECK_REVISION=d1884844b44e5506cb5e9458faeccffd83b404a1 node scripts/check-progress.mjs
PATH="/tmp/jj-server-controls-bin:$PATH" node node_modules/@lapismd/spec-validator/dist/cli.js first --base 5bc387a9 --head d1884844b44e5506cb5e9458faeccffd83b404a1
```

**Notes**

The temporary read-only JJ wrapper selects unintegrated operation da2e870ad32b. This proves the exact isolated UI candidate: 152 tasks/773 records and five protected files/one canonical chapter. The initially parsed snapshot operation pointed to the old parent; select the final commit operation instead. Final selection includes only this record in the two shared day navigation files. The real installed JJ and main working copy are unchanged by this preview.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate
node node_modules/prettier/bin/prettier.cjs --check (changed UI, fixture, stories, UI chapter and M27 pages)
node --test scripts/published-foundations.test.mjs
```

**Notes**

Seven validators pass; changed authored files use Prettier style. The released-foundations regression passes once. Exact focused story target has 15 pass, zero failures; this is not a full suite or visual-baseline approval. All relevant UI checks and metadata correction are complete.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; JJ 0.45.1; Vitest 4.1.11; Storybook 10.5.7; released Design Core/Mira dependencies retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No route, API, configuration, credential, native repository or persisted-data format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Main-workspace server navigation and connection-dialog presentation; two directly affected story families. Local counts toward the two visible servers and the selected remote is promoted. Existing switching/controller/authorization behavior is retained. Root aggregate pnpm check/test/build/spec:check, Rust builds/tests, release and unrelated Settings suites are out of scope under the UI-only targeted policy. The unrelated build-storage edits and their evidence remain uncommitted.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the refreshed main app. Continue M27 provider/platform qualification independently.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-UI-022 application-scoping and JST-API-011 connection/API behavior
  retain owners, guards, drafts, session-only tokens, origins, callbacks and
  request identities. JST-UI-016 form styling is reused unchanged. The UI
  chapter clarifies the user-authorized visual refactor: sidebar presentation,
  two visible entries and an accessible horizontal-kebab overflow retain all
  destinations and commands."

```
<!-- /evidence-field -->

