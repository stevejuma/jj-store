---
schema: 1
id: E-DESK-BROWSER-001
---

# E-DESK-BROWSER-001 details 4

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
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator validate --only governance,summary,fileLimits,verification
```

**Notes**

All four applicable document validators pass; 4617 chapters and 211 requirements.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

186 tasks and 1033 evidence records; evidence bookkeeping only.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_BROWSER_PREVIEW_EXECUTABLE=/home/steve/Projects/jj-store/target/debug/examples/browser_preview pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

Final complete cold production browser run after the verified build: 114 passed, one existing skip. Both earlier transient failures cleared without product or test edits. Uses existing captured HTTP fixtures and prebuilt acceptance host; separate from real-backend desktop development acceptance.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <all changed nonhistorical files>
```

**Notes**

First invocation lacked MISE_PNPM_VERSION; reran with explicit 10.32.1.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec prettier --check <all changed nonhistorical files>
```

**Notes**

All changed production, script, catalog, canonical spec, ledger and development documentation files pass. History Markdown is repository-rendered and excluded by .prettierignore; bounded source validators consume it.
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
MISE_PNPM_VERSION=10.32.1 pnpm spec:links
```

**Notes**

Initial audit read the stale ignored combined book, including manual and Storybook assets with absolute release-site paths. Rebuilt the specification book before auditing current documents.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator build
```

**Notes**

mise required the installed mdbook version to be selected explicitly.
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
MISE_PNPM_VERSION=10.32.1 MISE_MDBOOK_VERSION=0.5.4 pnpm exec spec-validator build
```

**Notes**

Specification book built successfully; existing large search-index advisory retained.
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
MISE_PNPM_VERSION=10.32.1 pnpm spec:links
```

**Notes**

Fresh generated book: 3333 HTML files and 127451 local references pass links and anchors.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator first --base @- --head @
```

**Notes**

43 protected files mapped to six canonical chapters. Fresh coverage includes documentation and UI owning topics.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator validate --only governance,summary,fileLimits,verification
```

**Notes**

All four applicable validators pass; final entry rendering is rechecked before commit.
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

Linux x86_64; Node 26.8.2; pnpm 10.32.1; Svelte 5.57.1; Vite 8.3.1; Vitest 4.1.11; Playwright 1.63.0; released Tauri API ~2.11.1; spec-validator 0.2.0. Existing native binaries are reused; no native/platform requalification. Documentation tooling: uv 0.9.28 and mdbook 0.5.4.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

HostEnvironment is renderer-only. Native marker v1, named IPC payloads, terminal request/subscription identities, API protocol, fixture formats, appearance preference v1 and bounded browser manifest v1 are unchanged. Runtime marker format 2 gains optional presentation; missing values default browser.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One application host abstraction shared by bootstrap, component context, connection/controller owners, appearance, terminal and power. Desktop browser mode uses the existing loopback Vite proxy and isolated real backend. Native host capabilities require supported metadata plus actual IPC; fixture-injected capabilities are separate from real native evidence. Root pnpm check/test/build/spec:check, unrelated Rust/platform qualification and visual baselines are out of scope under the targeted policy. No runtime JS/data authority, API or storage schema changed. Cold test services and the owned temporary Docker validation run were stopped; original fixtures remain untouched.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain independent native WKWebView, macOS effects, signing, relocation, distribution and visual approval gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-DESK-004 explicitly permits development macOS presentation
  independent of capabilities. Owning desktop, architecture and API metadata
  topics updated. JST-DESK-001/002/003/005/006 and JST-CLI-004 retain
  installation/process, authorization, stable terminal identity and
  build/storage contracts; no shell HTTP route, remapping or storage authority
  change. JST-DOC-001/002 and JST-UI-002/006 now explicitly describe the
  debugging workflow and capability-based terminal/dragging behavior; no server
  or native permissions are broadened.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M17 and M17.2 remain In progress. Browser cold development and
  production-CSP evidence cannot qualify WKWebView, native effects or
  distribution. Existing native terminal status and visual-pending stories are
  unchanged.

```
<!-- /evidence-field -->

